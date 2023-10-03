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

pub trait Half {
    fn half(&self) -> Self;
}

macro_rules! implement_half_for {
    ($($type: ty)*) => {$(
        impl Half for $type {
            fn half(&self) -> $type {
                self / (2 as $type)
            }
        }
    )*}
}

implement_half_for! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 } 

pub trait Zero {
    fn zero() -> Self;
}

macro_rules! implement_zero_for {
    ($($type: ty)*) => {$(
        impl Zero for $type {
            fn zero() -> $type {
                0 as $type
            }
        }
    )*}
}

implement_zero_for! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 } 

pub trait One {
    fn one() -> Self;
}


macro_rules! implement_one_for {
    ($($type: ty)*) => {$(
        impl One for $type {
            fn one() -> $type {
                1 as $type
            }
        }
    )*}
}

implement_one_for! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 } 

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

    fn half_test<T: Half>(v: T) -> T {
        v.half()
    }

    macro_rules! implement_half_test {
        ($testValue: literal, $testName: ident, $type: ty) => {
            #[test]
            fn $testName() {
                assert_eq!(half_test($testValue as $type), ($testValue as $type) / (2 as $type));
            }
        }
    }

    implement_half_test! { 84, half_u8, u8 } 
    implement_half_test! { 98, half_u16, u16 } 
    implement_half_test! { 84, half_u32, u32 } 
    implement_half_test! { 56, half_u64, u64 } 
    implement_half_test! { 8, half_u128, u128 } 
    implement_half_test! { 10, half_usize, usize } 
    implement_half_test! { 24, half_i8, i8 } 
    implement_half_test! { 30, half_i16, i16 } 
    implement_half_test! { 40, half_i32, i32 } 
    implement_half_test! { 20, half_i64, i64 } 
    implement_half_test! { 100, half_i128, i128 } 
    implement_half_test! { 2, half_isize, isize } 
    implement_half_test! { 5.0, half_f32, f32 } 
    implement_half_test! { 7.0, half_f64, u8 } 

    macro_rules! implement_zero_test {
        ($testName: ident, $type: ty) => {
            #[test]
            fn $testName() {
                assert_eq!(<$type>::zero(), 0 as $type);
            }
        }
    }

    implement_zero_test! { zero_u8, u8 } 
    implement_zero_test! { zero_u16, u16 } 
    implement_zero_test! { zero_u32, u32 } 
    implement_zero_test! { zero_u64, u64 } 
    implement_zero_test! { zero_u128, u128 } 
    implement_zero_test! { zero_usize, usize } 
    implement_zero_test! { zero_i8, i8 } 
    implement_zero_test! { zero_i16, i16 } 
    implement_zero_test! { zero_i32, i32 } 
    implement_zero_test! { zero_i64, i64 } 
    implement_zero_test! { zero_i128, i128 } 
    implement_zero_test! { zero_isize, isize } 
    implement_zero_test! { zero_f32, f32 } 
    implement_zero_test! { zero_f64, u8 } 

    macro_rules! implement_one_test {
        ($testName: ident, $type: ty) => {
            #[test]
            fn $testName() {
                assert_eq!(<$type>::one(), 1 as $type);
            }
        }
    }

    implement_one_test! { one_u8, u8 } 
    implement_one_test! { one_u16, u16 } 
    implement_one_test! { one_u32, u32 } 
    implement_one_test! { one_u64, u64 } 
    implement_one_test! { one_u128, u128 } 
    implement_one_test! { one_usize, usize } 
    implement_one_test! { one_i8, i8 } 
    implement_one_test! { one_i16, i16 } 
    implement_one_test! { one_i32, i32 } 
    implement_one_test! { one_i64, i64 } 
    implement_one_test! { one_i128, i128 } 
    implement_one_test! { one_isize, isize } 
    implement_one_test! { one_f32, f32 } 
    implement_one_test! { one_f64, u8 } 
}
