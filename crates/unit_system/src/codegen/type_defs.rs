use proc_macro2::TokenStream;
use quote::{quote};
use crate::types::{Defs, Dimensions};

impl Defs {
    pub(crate) fn type_definition(&self) -> TokenStream {
        let Self {
            quantity_type,
            dimension_type,
            ..
        } = &self;
        quote! {
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Default)]
            #[repr(transparent)]
            pub struct #quantity_type<S: 'static, const D: #dimension_type>(pub(crate) S);
        }
    }

    pub(crate) fn type_functions(&self) -> TokenStream {
        let Self {
            quantity_type,
            dimension_type,
            ..
        } = &self;
        quote! {
            impl<S> #quantity_type<S, { #dimension_type::none() }> {
                /// Get the value of a dimensionless quantity
                pub fn value(self) -> S {
                    self.0
                }

                /// Get a reference to the value of a dimensionless quantity
                pub fn value_ref(&self) -> &S {
                    &self.0
                }
            }

            impl<S, const D: #dimension_type> #quantity_type<S, D> {
                /// Return the value of a quantity, regardless of whether
                /// it is dimensionless or not. Use this carefully, since the
                /// result depends on the underlying base units
                pub fn value_unchecked(self) -> S {
                    self.0
                }

                /// Create a new quantity for the dimension with a given value.
                /// Use carefully, since the constructed quantity depends on the
                /// used base units.
                pub const fn new_unchecked(s: S) -> Self {
                    Self(s)
                }
            }
        }
    }

    pub fn get_dimension_expr(&self, dim: &Dimensions) -> TokenStream {
        let dimension_type = &self.dimension_type;
        let field_updates: TokenStream = 
            dim
            .fields
            .iter()
            .map(|field| {
                let ident = &field.ident;
                let value = &field.value;
                quote! { #ident: #value, }
            })
            .collect();
        quote! {
            #dimension_type {
                #field_updates
                ..#dimension_type::none()
            }
        }
    }

    pub fn vector_quantity_definitions(&self) -> TokenStream {
        self.vector_types()
            .iter()
            .map(|vector_type| {
                self.quantity_definitions_for_type(&vector_type.name, &vector_type.module_name)
            })
            .collect()
    }

    pub fn float_quantity_definitions(&self) -> TokenStream {
        self.float_types()
            .iter()
            .map(|float_type| {
                self.quantity_definitions_for_type(&float_type.name, &float_type.module_name)
            })
            .collect()
    }

    pub fn quantity_definitions_for_type(
        &self,
        type_: &TokenStream,
        module_name: &TokenStream,
    ) -> TokenStream {
        let Self {
            dimension_type,
            quantity_type,
            ..
        } = &self;
        let quantities: TokenStream = self
            .quantities
            .iter()
            .map(|quantity| {
                let dimension = self.get_dimension_expr(&quantity.dimension);
                let quantity_type = &self.quantity_type;
                let quantity_name = &quantity.name;
                quote! {
                    pub type #quantity_name = #quantity_type::<#type_, { #dimension }>;
                }
            })
            .collect();
        // TODO: The use statements here are quite hacky and will probably
        // not work if dimension is declared in a different place from
        // the macro invocation.
        quote! {
            pub mod #module_name {
                use super::#dimension_type;
                use super::#quantity_type;
                #quantities
            }
        }
    }
}
