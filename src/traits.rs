macro_rules! create_and_implement_proxy_trait {
    ($traitName: ident, $function: ident, $($type: ty)*  ) => {
        pub trait $traitName {
            type Output;

            fn $function(self) -> Self::Output;
        }

        $(
        impl $traitName for $type  {
            type Output = $type;

            fn $function(self) -> <$type as $traitName>::Output {
                self.$function()
            }
        }
        )*

    }
}

create_and_implement_proxy_trait! { Sqrt, sqrt, f32 f64 }

// acos
// acosh
// asin
// asinh
// atan
// atan2
// atanh
// cos
// cosh
// sin
// sin_cos
// sinh
create_and_implement_proxy_trait! { Tan, tan, f32 f64 }
// tanh

create_and_implement_proxy_trait! { ToDegrees, to_degrees, f32 f64 }
create_and_implement_proxy_trait! { ToRadians, to_radians, f32 f64 }



#[cfg(test)]
mod tests {

    use super::*;

    macro_rules! implement_test_for {
        ($traitName: ident, $function: ident, $helperName: ident, $testValue: literal, [$($testName: ident),*], [$($type: ty),*] ) => {
            fn $helperName<T: $traitName>(v: T) -> <T as $traitName>::Output {
                v.$function()
            }

            $(
                #[test]
                fn $testName() {
                    assert_eq!($helperName($testValue as $type), ($testValue as $type).$function());
                }
            )*
        }
    }

    implement_test_for! { Sqrt, sqrt, sqrt_test, 2.0, [sqrt_f32, sqrt_f64], [f32, f64] }
    implement_test_for! { Tan, tan, tan_test, 1.0, [tan_f32, tan_f64], [f32, f64] }
    implement_test_for! { ToDegrees, to_degrees, to_degrees_test, 3.1415, [to_degrees_f32, to_degrees_f64], [f32, f64] }
    implement_test_for! { ToRadians, to_radians, to_radians_test, 180.0, [to_radians_f32, to_radians_f64], [f32, f64] }
}
