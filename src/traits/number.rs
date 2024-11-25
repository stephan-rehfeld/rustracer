use std::fmt::{Debug, Display, LowerExp, UpperExp};
use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use std::str::FromStr;

use super::{One, Zero};

macro_rules! implement_one_parameter_proxy_trait_with_output {
    ($traitName: ident, $function: ident, $($type: ty)*  ) => {
        $(
        impl $traitName for $type  {
            type Output = $type;

            fn $function(self, param: $type) -> <$type as $traitName>::Output {
                self.$function(param)
            }
        }
        )*
    }
}

pub trait DivEuclid<RHS = Self> {
    type Output;

    fn div_euclid(self, rhs: RHS) -> Self::Output;
}

implement_one_parameter_proxy_trait_with_output! { DivEuclid, div_euclid, f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }

/*pub trait Midpoint {
    fn midpoint(self, rhs: Self) -> Self;
}

macro_rules! implement_one_parameter_proxy_trait {
    ($traitName: ident, $function: ident, $($type: ty)*  ) => {
        $(
        impl $traitName for $type  {
            fn $function(self, rhs: $type) -> $type {
                self.$function(rhs)
            }
        }
        )*
    }
}

implement_one_parameter_proxy_trait! { Midpoint, midpoint, f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }*/

pub trait RemEuclid<RHS = Self> {
    type Output;

    fn rem_euclid(self, rhs: RHS) -> Self::Output;
}

implement_one_parameter_proxy_trait_with_output! { RemEuclid, rem_euclid, f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }

// N stands for neutral
pub trait Number<N=Self>: DivEuclid
             /* + Midpoint */
                + RemEuclid
                + Add<Output=Self>
                + for<'a> Add<&'a Self, Output=Self>
                + AddAssign
                + for<'a> AddAssign<&'a Self>
                + Clone
                + Copy
                + Debug
                + Default
                + Display
                + Div
                + Div<N, Output=Self>
                + for<'a> Div<&'a Self>
                + for<'a> Div<&'a N, Output=Self>
                + DivAssign<N>
                + for<'a> DivAssign<&'a N>
                + From<bool>
                + FromStr
                + LowerExp
                + Mul<N, Output=Self>
                + for<'a> Mul<&'a N, Output=Self>
                + MulAssign<N>
                + for<'a> MulAssign<&'a N>
                + One
                + PartialEq
                + PartialOrd
                + Rem<Output=Self>
                + Rem<N, Output=Self>
                + for<'a> Rem<&'a Self, Output=Self>
                + for<'a> Rem<&'a N, Output=Self>
                + RemAssign
                + RemAssign<N>
                + for<'a> RemAssign<&'a Self>
                + for<'a> RemAssign<&'a N>
                + Sub<Output=Self>
                + for<'a> Sub<&'a Self, Output=Self>
                + SubAssign
                + for<'a> SubAssign<&'a Self>
                + Sum
                + for<'a> Sum<&'a Self>
                + UpperExp
                + Zero
                + Sized  {
    const MAX: Self;
    const MIN: Self;
}

pub trait SelfMulNumber<N>: Number<N> + Mul {}

pub trait InvariantMulNumber:
    Number + Div<Output = Self> + Mul<Output = Self> + Product + for<'a> Product<&'a Self>
{
}

macro_rules! implement_number_trait {
    ($($type: ty)*  ) => {
        $(
        impl Number for $type  {
            const MAX: $type = <$type>::MAX;
            const MIN: $type = <$type>::MIN;
        }
        )*
    }
}

implement_number_trait! { f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }

impl<T: Number<T>> SelfMulNumber<T> for T {}

implement_marker_trait! { InvariantMulNumber, f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }

#[cfg(test)]
mod tests {

    use super::*;

    fn test<T: DivEuclid<Output = T>>(a: T, b: T) -> T {
        a.div_euclid(b)
    }

    #[test]
    fn test_function() {
        assert_eq!(test(5, 2), 2);
    }
}
