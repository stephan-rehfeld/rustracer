use crate::traits::Number;

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

pub trait ConvenienceNumber<T = Self>: Number<T> + Half + One + Zero {}

impl<T: Number + Half + One + Zero> ConvenienceNumber<T> for T {}

#[cfg(test)]
mod tests {

    use super::*;

    fn half_test<T: Half>(v: T) -> T {
        v.half()
    }

    macro_rules! implement_half_test {
        ($testValue: literal, $testName: ident, $type: ty) => {
            #[test]
            fn $testName() {
                assert_eq!(
                    half_test($testValue as $type),
                    ($testValue as $type) / (2 as $type)
                );
            }
        };
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
        };
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
        };
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
