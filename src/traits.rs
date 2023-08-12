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

create_and_implement_proxy_trait! { Acos, acos, f32 f64 }
create_and_implement_proxy_trait! { Acosh, acosh, f32 f64 }
create_and_implement_proxy_trait! { Asin, asin, f32 f64 }
create_and_implement_proxy_trait! { Asinh, asinh, f32 f64 }
create_and_implement_proxy_trait! { Atan, atan, f32 f64 }
// atan2

pub trait Atan2 {
    type Output;

    fn atan2(self, other: Self) -> Self::Output;
}

impl Atan2 for f32 {
    type Output = f32;

    fn atan2(self, other: f32) -> Self::Output {
        self.atan2(other)
    }
}

impl Atan2 for f64 {
    type Output = f64;

    fn atan2(self, other: f64) -> Self::Output {
        self.atan2(other)
    }
}

create_and_implement_proxy_trait! { Atanh, atanh, f32 f64 }
create_and_implement_proxy_trait! { Cos, cos, f32 f64 }
create_and_implement_proxy_trait! { Cosh, cosh, f32 f64 }
create_and_implement_proxy_trait! { Sin, sin, f32 f64 }

pub trait SinCos {
    type Output;

    fn sin_cos(self) -> Self::Output;
}

impl SinCos for f32 {
    type Output = (f32,f32);

    fn sin_cos(self) -> Self::Output {
        self.sin_cos()
    }
}

impl SinCos for f64 {
    type Output = (f64,f64);

    fn sin_cos(self) -> Self::Output {
        self.sin_cos()
    }
}

create_and_implement_proxy_trait! { Sinh, sinh, f32 f64 }
create_and_implement_proxy_trait! { Tan, tan, f32 f64 }
create_and_implement_proxy_trait! { Tanh, tanh, f32 f64 }
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
    implement_test_for! { Acos, acos, acos_test, 1.0, [acos_f32, acos_f64], [f32, f64] }
    implement_test_for! { Acosh, acosh, acosh_test, 1.0, [acosh_f32, acosh_f64], [f32, f64] }
    implement_test_for! { Asin, asin, asin_test, 1.0, [asin_f32, asin_f64], [f32, f64] }
    implement_test_for! { Asinh, asinh, asinh_test, 1.0, [asinh_f32, asinh_f64], [f32, f64] }
    implement_test_for! { Atan, atan, atan_test, 1.0, [atan_f32, atan_f64], [f32, f64] }
    
    fn atan2_helper<T: Atan2>(v: T, other: T) -> <T as Atan2>::Output {
        v.atan2(other)
    }
    
    #[test]
    fn atan2_f32() {
        assert_eq!(atan2_helper(1 as f32, 1 as f32), (1 as f32).atan2(1 as f32));
    }

    #[test]
    fn atan2_f64() {
        assert_eq!(atan2_helper(1 as f64, 1 as f64), (1 as f64).atan2(1 as f64));
    }

    implement_test_for! { Atanh, atanh, atanh_test, 1.0, [atanh_f32, atanh_f64], [f32, f64] }
    implement_test_for! { Cos, cos, cos_test, 1.0, [cos_f32, cos_f64], [f32, f64] }
    implement_test_for! { Cosh, cosh, cosh_test, 1.0, [cosh_f32, cosh_f64], [f32, f64] }
    implement_test_for! { Sin, sin, sin_test, 1.0, [sin_f32, sin_f64], [f32, f64] }
    implement_test_for! { SinCos, sin_cos, sin_cos_test, 1.0, [sin_cos_f32, sin_cos_f64], [f32, f64] }
    implement_test_for! { Sinh, sinh, sinh_test, 1.0, [sinh_f32, sinh_f64], [f32, f64] }
    implement_test_for! { Tan, tan, tan_test, 1.0, [tan_f32, tan_f64], [f32, f64] }
    implement_test_for! { Tanh, tanh, tanh_test, 1.0, [tanh_f32, tanh_f64], [f32, f64] }
    implement_test_for! { ToDegrees, to_degrees, to_degrees_test, 3.1415, [to_degrees_f32, to_degrees_f64], [f32, f64] }
    implement_test_for! { ToRadians, to_radians, to_radians_test, 180.0, [to_radians_f32, to_radians_f64], [f32, f64] }
}
