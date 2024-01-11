mod error;
mod ident_storage;

use std::{collections::HashMap, result::Result};

use proc_macro2::Span;
use syn::Ident;

use crate::types::{Definition, Defs, DimensionEntry, Unit, UnitEntry, UnresolvedDefs};

use self::{
    error::{
        emit_if_err, BaseUnitForNonBaseDimensionError, Emit, MultipleBaseUnitsForDimensionError,
        NoSymbolForBaseUnitError, SymbolDefinedMultipleTimes, TypeDefinitionsError,
    },
    ident_storage::IdentStorage,
};

fn get_single_ident(
    mut dimension_types: Vec<Ident>,
    type_name: &'static str,
    default_name: &'static str,
) -> (Ident, Result<(), TypeDefinitionsError>) {
    if dimension_types.len() == 1 {
        (dimension_types.remove(0), Ok(()))
    } else if dimension_types.is_empty() {
        // Construct an identifier to be able to continue
        (
            Ident::new(default_name, Span::call_site()),
            Err(TypeDefinitionsError::None {
                type_name,
                default_name,
            }),
        )
    } else {
        let dimension_type = dimension_types[0].clone();
        (
            dimension_type,
            Err(TypeDefinitionsError::Multiple {
                idents: dimension_types,
                type_name,
            }),
        )
    }
}

/// A helper function for emitting all the errors contained in the
/// result types but continuing with a partial result anyways. This is
/// done so that we at least define all the quantities that can be
/// partially resolved in order to keep the amount of error messages
/// manageable.
fn emit_errors<T, E: Emit>((input, result): (T, Result<(), E>)) -> T {
    if let Err(err) = result {
        err.emit();
    }
    input
}

impl UnresolvedDefs {
    pub fn resolve(self) -> Defs {
        let quantity_type = emit_errors(get_single_ident(
            self.quantity_types,
            "quantity type",
            "Quantity",
        ));
        let dimension_type = emit_errors(get_single_ident(
            self.dimension_types,
            "dimension type",
            "Dimension",
        ));
        let mut idents = IdentStorage::default();
        let base_dimensions = get_base_dimensions(&self.dimensions, &self.units);
        check_multiply_defined_symbols(&self.units);
        idents.add(self.dimensions);
        idents.add(self.units);
        idents.add(self.constants);
        emit_if_err(idents.filter_undefined());
        idents.filter_autogenerated_invalid();
        emit_if_err(idents.filter_multiply_defined());
        idents.filter_autogenerated_invalid();
        idents.check_kinds_in_definitions();
        idents.filter_autogenerated_invalid();
        emit_if_err(idents.resolve());
        emit_if_err(idents.check_type_annotations());
        let dimensions = idents.get_items();
        let units = idents.get_items();
        let constants = idents.get_items();
        check_for_base_units_without_symbol(&units);
        Defs {
            dimension_type,
            quantity_type,
            dimensions,
            units,
            constants,
            base_dimensions,
        }
    }
}

fn check_for_base_units_without_symbol(units: &[Unit]) {
    for unit in units {
        if unit.is_base_unit {
            if unit.symbol.is_none() {
                NoSymbolForBaseUnitError(unit).emit();
            }
        }
    }
}

fn check_multiply_defined_symbols(units: &[UnitEntry]) {
    let mut units_by_symbol: HashMap<&Ident, Vec<&Ident>> = HashMap::new();
    for unit in units {
        if let Some(ref symbol) = unit.symbol {
            units_by_symbol
                .entry(&symbol.0)
                .or_default()
                .push(&unit.name);
        }
    }
    for (symbol, units) in units_by_symbol {
        if units.len() > 1 {
            SymbolDefinedMultipleTimes { symbol, units }.emit()
        }
    }
}

pub fn get_base_dimensions(dimensions: &[DimensionEntry], units: &[UnitEntry]) -> Vec<Ident> {
    let base_dimensions: Vec<_> = dimensions
        .iter()
        .filter(|d| d.is_base_dimension())
        .collect();
    check_invalid_base_units(units, &base_dimensions);
    base_dimensions
        .into_iter()
        .map(|x| x.dimension_entry_name())
        .collect()
}

pub fn check_invalid_base_units(units: &[UnitEntry], base_dimensions: &[&DimensionEntry]) {
    let mut counter: HashMap<&Ident, usize> =
        base_dimensions.iter().map(|dim| (&dim.name, 0)).collect();
    for unit in units {
        if unit.autogenerated_from.is_some() {
            continue;
        }
        if let Definition::Base(ref dimension) = unit.definition {
            if let Some(count) = counter.get_mut(dimension) {
                *count += 1;
                if *count > 1 {
                    MultipleBaseUnitsForDimensionError {
                        dimension,
                        unit: &unit.name,
                    }
                    .emit()
                }
            } else {
                BaseUnitForNonBaseDimensionError {
                    dimension,
                    unit: &unit.name,
                }
                .emit()
            }
        }
    }
}
