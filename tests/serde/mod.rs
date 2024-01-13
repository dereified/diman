#[cfg(any(feature = "f32", feature = "f64"))]
macro_rules! gen_tests_for_float {
    ($float_name: ident, $assert_is_close: path) => {
        mod $float_name {
            use crate::example_system::$float_name::Energy;
            use crate::example_system::$float_name::Length;
            use crate::example_system::$float_name::Time;
            use crate::example_system::$float_name::Velocity;
            use $assert_is_close as assert_is_close;

            #[test]
            fn deserialize_float() {
                let q: Length = serde_yaml::from_str("5.0 km").unwrap();
                assert_is_close(q, Length::kilometers(5.0));
                let q: Velocity = serde_yaml::from_str("5.0 km s^-1").unwrap();
                assert_is_close(q, Length::kilometers(5.0) / Time::seconds(1.0));
            }

            #[test]
            #[should_panic(expected = "mismatch in dimensions")]
            fn deserialize_float_dimension_mismatch() {
                let q: Length = serde_yaml::from_str("5.0 kg").unwrap();
                assert_is_close(q, Length::kilometers(5.0));
            }

            #[test]
            fn serialize_float() {
                assert_eq!(
                    serde_yaml::to_string(&Length::kilometers(5.0))
                        .unwrap()
                        .trim(),
                    "5000 m"
                );
                assert_eq!(
                    serde_yaml::to_string(&Energy::joules(5.0)).unwrap().trim(),
                    "5 J"
                );
            }

            #[test]
            fn serialize_float_unnamed_dimension() {
                let unnamed_dimension = Energy::joules(5.0) * Length::meters(1.0);
                assert_eq!(
                    serde_yaml::to_string(&unnamed_dimension).unwrap().trim(),
                    "5 m^3 s^-2 kg"
                );
            }
        }
    };
}

#[cfg(any(feature = "glam-vec2", feature = "glam-dvec2"))]
macro_rules! gen_tests_for_vector_2 {
    ($float_name: ident, $mod_name: ident, $vec_name: ty, $assert_is_close: path) => {
        mod $mod_name {
            use crate::example_system::$float_name::Length;
            use crate::example_system::$mod_name::Dimensionless as Vec2Dimensionless;
            use crate::example_system::$mod_name::Length as Vec2Length;
            use $assert_is_close as assert_is_close;

            #[test]
            fn deserialize_vector() {
                let q: Vec2Length = serde_yaml::from_str("(5.0 3.0) km").unwrap();
                assert_is_close(q.x(), Length::kilometers(5.0));
                assert_is_close(q.y(), Length::kilometers(3.0));
            }

            #[test]
            #[should_panic]
            fn deserialize_vector_fails_with_fewer_than_2_components() {
                let _: Vec2Length = serde_yaml::from_str("(5.0) km").unwrap();
            }

            #[test]
            #[should_panic]
            fn deserialize_vector_fails_with_more_than_2_components() {
                let _: Vec2Length = serde_yaml::from_str("(5.0 3.0 7.0) km").unwrap();
            }

            #[test]
            fn serialize_vector() {
                let x = Vec2Length::meters(5.3, 1.1);
                let result: String = serde_yaml::to_string(&x).unwrap();
                assert_eq!(result, "(5.3 1.1) m\n");
            }

            #[test]
            fn serialize_dimensionless_vector() {
                let x = Vec2Dimensionless::dimensionless(5.3, 1.1);
                let result: String = serde_yaml::to_string(&x).unwrap();
                assert_eq!(result, "(5.3 1.1)\n");
            }
        }
    };
}

#[cfg(any(feature = "glam-vec3", feature = "glam-dvec3"))]
macro_rules! gen_tests_for_vector_3 {
    ($float_name: ident, $mod_name: ident, $vec_name: ty, $assert_is_close: path) => {
        mod $mod_name {
            use crate::example_system::$float_name::Length;
            use crate::example_system::$mod_name::Dimensionless as Vec3Dimensionless;
            use crate::example_system::$mod_name::Length as Vec3Length;
            use $assert_is_close as assert_is_close;

            #[test]
            fn deserialize_vector() {
                let q: Vec3Length = serde_yaml::from_str("(5.0 3.0 7.0) km").unwrap();
                assert_is_close(q.x(), Length::kilometers(5.0));
                assert_is_close(q.y(), Length::kilometers(3.0));
                assert_is_close(q.z(), Length::kilometers(7.0));
            }

            #[test]
            #[should_panic]
            fn deserialize_vector_fails_with_fewer_than_3_components() {
                let _: Vec3Length = serde_yaml::from_str("(5.0 4.0) km").unwrap();
            }

            #[test]
            #[should_panic]
            fn deserialize_vector_fails_with_more_than_3_components() {
                let _: Vec3Length = serde_yaml::from_str("(5.0 3.0 7.0 9.0) km").unwrap();
            }

            #[test]
            fn serialize_vector() {
                let x = Vec3Length::meters(5.3, 1.1, 2.2);
                let result: String = serde_yaml::to_string(&x).unwrap();
                assert_eq!(result, "(5.3 1.1 2.2) m\n");
            }

            #[test]
            fn serialize_dimensionless_vector() {
                let x = Vec3Dimensionless::dimensionless(5.3, 1.1, 2.2);
                let result: String = serde_yaml::to_string(&x).unwrap();
                assert_eq!(result, "(5.3 1.1 2.2)\n");
            }
        }
    };
}

#[cfg(feature = "f32")]
gen_tests_for_float!(f32, crate::utils::assert_is_close_f32);

#[cfg(feature = "f64")]
gen_tests_for_float!(f64, crate::utils::assert_is_close_f64);

#[cfg(all(feature = "f32", feature = "glam-vec2"))]
gen_tests_for_vector_2!(f32, vec2, glam::Vec2, crate::utils::assert_is_close_f32);

#[cfg(all(feature = "f64", feature = "glam-dvec2"))]
gen_tests_for_vector_2!(f64, dvec2, glam::DVec2, crate::utils::assert_is_close_f64);

#[cfg(all(feature = "f32", feature = "glam-vec3"))]
gen_tests_for_vector_3!(f32, vec3, glam::Vec3, crate::utils::assert_is_close_f32);

#[cfg(all(feature = "f64", feature = "glam-dvec3"))]
gen_tests_for_vector_3!(f64, dvec3, glam::DVec3, crate::utils::assert_is_close_f64);
