use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Add, AddAssign, Div, DivAssign, Neg, Mul, MulAssign, Sub, SubAssign};
use std::marker::PhantomData;

use crate::traits::{Half, One, Zero};

pub mod angle;
pub mod area;
pub mod length;
pub mod prefix;
pub mod second_moment_of_area;
pub mod volume;

use prefix::Prefix;

pub trait Unit {
    const UNIT: &'static str; 
}

#[derive(Debug,PartialEq,PartialOrd,Clone,Copy)]
pub struct ValueWithPrefixAndUnit<T, P, U> {
    value: T,
    _prefix: PhantomData<P>,
    _unit: PhantomData<U>,
} 

impl<T, P, U> ValueWithPrefixAndUnit<T, P, U> {
    pub fn new(value: T) -> ValueWithPrefixAndUnit<T, P, U> {
        ValueWithPrefixAndUnit { value: value, _prefix: PhantomData, _unit: PhantomData }
    }
}

impl<T: Add, P, U> Add for ValueWithPrefixAndUnit<T, P, U> {
    type Output = ValueWithPrefixAndUnit<<T as Add>::Output, P, U>;

    fn add(self, rhs: Self) -> Self::Output {
        ValueWithPrefixAndUnit::new(self.value + rhs.value)
    }
}

impl<T: AddAssign, P, U> AddAssign for ValueWithPrefixAndUnit<T, P, U> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl<T: Sub, P, U> Sub for ValueWithPrefixAndUnit<T, P, U> {
    type Output = ValueWithPrefixAndUnit<<T as Sub>::Output, P, U>;

    fn sub(self, rhs: Self) -> Self::Output {
        ValueWithPrefixAndUnit::new(self.value - rhs.value)
    }
}

impl<T: SubAssign, P, U> SubAssign for ValueWithPrefixAndUnit<T, P, U> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl<T: Mul, P, U> Mul<T> for ValueWithPrefixAndUnit<T, P, U> {
    type Output = ValueWithPrefixAndUnit<<T as Mul>::Output, P, U>;

    fn mul(self, rhs: T) -> Self::Output {
        ValueWithPrefixAndUnit::new(self.value * rhs)
    }
}

macro_rules! impl_mul_scalar_with_value_with_prefix_and_unit {
    ($($type: ty)+ ) => ($(
        impl<T: Mul, P, U> Mul<ValueWithPrefixAndUnit<T, P, U>> for $type where
            $type: Mul<T>
        {
            type Output = ValueWithPrefixAndUnit<<$type as Mul<T>>::Output, P, U>;

            fn mul(self, rhs: ValueWithPrefixAndUnit<T, P, U>) -> Self::Output {
                ValueWithPrefixAndUnit::new( self * rhs.value )
            }
        }
    )*)
}

impl_mul_scalar_with_value_with_prefix_and_unit! { u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 }

impl<T: MulAssign, P, U> MulAssign<T> for ValueWithPrefixAndUnit<T, P, U> {
    fn mul_assign(&mut self, rhs: T) {
        self.value *= rhs;
    }
}

impl<T: Div, P, U> Div<T> for ValueWithPrefixAndUnit<T, P, U> {
    type Output = ValueWithPrefixAndUnit<<T as Div>::Output, P, U>;

    fn div(self, rhs: T) -> Self::Output {
        ValueWithPrefixAndUnit::new(self.value / rhs)
    }
}

impl<T: DivAssign, P, U> DivAssign<T> for ValueWithPrefixAndUnit<T, P, U> {
    fn div_assign(&mut self, rhs: T) {
        self.value /= rhs;
    }
}

impl<T: Div, P, U> Div for ValueWithPrefixAndUnit<T, P, U> {
    type Output = <T as Div>::Output;

    fn div(self, rhs: Self) -> Self::Output {
        self.value / rhs.value
    }
}

impl<T: Neg, P, U> Neg for ValueWithPrefixAndUnit<T, P, U> {
    type Output = ValueWithPrefixAndUnit<<T as Neg>::Output, P, U>;

    fn neg(self) -> Self::Output {
        ValueWithPrefixAndUnit::new(-self.value)
    }
}

impl<T: Default, P, U> Default for ValueWithPrefixAndUnit<T, P, U> {
    fn default() -> Self {
        ValueWithPrefixAndUnit::new(T::default())
    }
}

impl<T: Display, P: Prefix, U: Unit> Display for ValueWithPrefixAndUnit<T, P, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}{}{}", self.value, P::PREFIX, U::UNIT)
    }
}

impl<T: Half, P, U> Half for ValueWithPrefixAndUnit<T, P, U> {
    fn half(&self) -> ValueWithPrefixAndUnit<T, P, U> {
        Self::new(self.value.half())
    }
}

impl<T: One, P, U> One for ValueWithPrefixAndUnit<T, P, U> {
    fn one() -> ValueWithPrefixAndUnit<T, P, U> {
        Self::new(One::one())
    }
}

impl<T: Zero, P, U> Zero for ValueWithPrefixAndUnit<T, P, U> {
    fn zero() -> ValueWithPrefixAndUnit<T, P, U> {
        Self::new(Zero::zero())
    }
}

