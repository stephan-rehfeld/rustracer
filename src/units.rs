use std::fmt::{Debug, Display, Formatter, LowerExp, Result as FmtResult, UpperExp};
use std::iter::Sum;
use std::marker::PhantomData;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use std::str::FromStr;

use crate::traits::{
    Abs, ConvenientNumber, DivEuclid, Half, Number, One, RemEuclid, SignedNumber, Signum, Zero,
};

pub mod angle;
pub mod area;
pub mod length;
pub mod prefix;
pub mod second_moment_of_area;
pub mod volume;

use prefix::Prefix;

pub trait Unit: Debug + PartialEq + PartialOrd + Copy + Clone {
    const UNIT: &'static str;
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct ValueWithPrefixAndUnit<T, P, U> {
    value: T,
    _prefix: PhantomData<P>,
    _unit: PhantomData<U>,
}

impl<T, P, U> ValueWithPrefixAndUnit<T, P, U> {
    pub fn new(value: T) -> ValueWithPrefixAndUnit<T, P, U> {
        ValueWithPrefixAndUnit {
            value: value,
            _prefix: PhantomData,
            _unit: PhantomData,
        }
    }
}

impl<T: DivEuclid, P, U> DivEuclid for ValueWithPrefixAndUnit<T, P, U> {
    type Output = ValueWithPrefixAndUnit<<T as DivEuclid>::Output, P, U>;

    fn div_euclid(self, rhs: Self) -> Self::Output {
        ValueWithPrefixAndUnit::new(self.value.div_euclid(rhs.value))
    }
}

impl<T: RemEuclid, P, U> RemEuclid for ValueWithPrefixAndUnit<T, P, U> {
    type Output = ValueWithPrefixAndUnit<<T as RemEuclid>::Output, P, U>;

    fn rem_euclid(self, rhs: Self) -> Self::Output {
        ValueWithPrefixAndUnit::new(self.value.rem_euclid(rhs.value))
    }
}

impl<T: Add, P, U> Add for ValueWithPrefixAndUnit<T, P, U> {
    type Output = ValueWithPrefixAndUnit<<T as Add>::Output, P, U>;

    fn add(self, rhs: Self) -> Self::Output {
        ValueWithPrefixAndUnit::new(self.value + rhs.value)
    }
}

impl<T: for<'a> Add<&'a T, Output = T>, P, U> Add<&Self> for ValueWithPrefixAndUnit<T, P, U> {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        ValueWithPrefixAndUnit::new(self.value + &rhs.value)
    }
}

impl<T: AddAssign, P, U> AddAssign for ValueWithPrefixAndUnit<T, P, U> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl<T: for<'a> AddAssign<&'a T>, P, U> AddAssign<&Self> for ValueWithPrefixAndUnit<T, P, U> {
    fn add_assign(&mut self, rhs: &Self) {
        self.value += &rhs.value;
    }
}

impl<T: Div, P, U> Div<T> for ValueWithPrefixAndUnit<T, P, U> {
    type Output = ValueWithPrefixAndUnit<<T as Div>::Output, P, U>;

    fn div(self, rhs: T) -> Self::Output {
        ValueWithPrefixAndUnit::new(self.value / rhs)
    }
}

impl<T: for<'a> Div<&'a T, Output = T>, P, U> Div<&T> for ValueWithPrefixAndUnit<T, P, U> {
    type Output = Self;

    fn div(self, rhs: &T) -> Self::Output {
        ValueWithPrefixAndUnit::new(self.value / rhs)
    }
}

impl<T: DivAssign, P, U> DivAssign<T> for ValueWithPrefixAndUnit<T, P, U> {
    fn div_assign(&mut self, rhs: T) {
        self.value /= rhs;
    }
}

impl<T: for<'a> DivAssign<&'a T>, P, U> DivAssign<&T> for ValueWithPrefixAndUnit<T, P, U> {
    fn div_assign(&mut self, rhs: &T) {
        self.value /= rhs;
    }
}

impl<T: Div, P, U> Div for ValueWithPrefixAndUnit<T, P, U> {
    type Output = <T as Div>::Output;

    fn div(self, rhs: Self) -> Self::Output {
        self.value / rhs.value
    }
}

impl<T: for<'a> Div<&'a T, Output = <T as Div>::Output> + Div, P, U> Div<&Self>
    for ValueWithPrefixAndUnit<T, P, U>
{
    type Output = <T as Div<T>>::Output;

    fn div(self, rhs: &Self) -> Self::Output {
        self.value / &rhs.value
    }
}

impl<T: From<bool>, P: Prefix, U: Unit> From<bool> for ValueWithPrefixAndUnit<T, P, U> {
    fn from(value: bool) -> Self {
        ValueWithPrefixAndUnit::new(T::from(value))
    }
}

impl<T: FromStr, P: Prefix, U: Unit> FromStr for ValueWithPrefixAndUnit<T, P, U> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match T::from_str(s) {
            Ok(v) => Ok(ValueWithPrefixAndUnit::new(v)),
            Err(e) => Err(e),
        }
    }
}

impl<T: LowerExp, P: Prefix, U: Unit> LowerExp for ValueWithPrefixAndUnit<T, P, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:e}{}{}", self.value, P::PREFIX, U::UNIT)
    }
}

impl<T: Mul, P, U> Mul<T> for ValueWithPrefixAndUnit<T, P, U> {
    type Output = ValueWithPrefixAndUnit<<T as Mul>::Output, P, U>;

    fn mul(self, rhs: T) -> Self::Output {
        ValueWithPrefixAndUnit::new(self.value * rhs)
    }
}

impl<T: for<'a> Mul<&'a T, Output = T>, P, U> Mul<&T> for ValueWithPrefixAndUnit<T, P, U> {
    type Output = Self;

    fn mul(self, rhs: &T) -> Self::Output {
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

impl<T: for<'a> MulAssign<&'a T>, P, U> MulAssign<&T> for ValueWithPrefixAndUnit<T, P, U> {
    fn mul_assign(&mut self, rhs: &T) {
        self.value *= &rhs;
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

impl<T: Rem, P, U> Rem<T> for ValueWithPrefixAndUnit<T, P, U> {
    type Output = ValueWithPrefixAndUnit<<T as Rem>::Output, P, U>;

    fn rem(self, rhs: T) -> Self::Output {
        ValueWithPrefixAndUnit::new(self.value % rhs)
    }
}

impl<T: for<'a> RemAssign<&'a T>, P, U> RemAssign<&Self> for ValueWithPrefixAndUnit<T, P, U> {
    fn rem_assign(&mut self, rhs: &Self) {
        self.value %= &rhs.value;
    }
}

impl<T: for<'a> RemAssign<&'a T>, P, U> RemAssign<&T> for ValueWithPrefixAndUnit<T, P, U> {
    fn rem_assign(&mut self, rhs: &T) {
        self.value %= &rhs;
    }
}

impl<T: Rem, P, U> Rem for ValueWithPrefixAndUnit<T, P, U> {
    type Output = ValueWithPrefixAndUnit<<T as Rem>::Output, P, U>;

    fn rem(self, rhs: Self) -> Self::Output {
        ValueWithPrefixAndUnit::new(self.value % rhs.value)
    }
}

impl<T: for<'a> Rem<&'a T, Output = T>, P, U> Rem<&Self> for ValueWithPrefixAndUnit<T, P, U> {
    type Output = Self;

    fn rem(self, rhs: &Self) -> Self::Output {
        ValueWithPrefixAndUnit::new(self.value % &rhs.value)
    }
}

impl<T: for<'a> Rem<&'a T, Output = T>, P, U> Rem<&T> for ValueWithPrefixAndUnit<T, P, U> {
    type Output = Self;

    fn rem(self, rhs: &T) -> Self::Output {
        ValueWithPrefixAndUnit::new(self.value % rhs)
    }
}

impl<T: RemAssign, P, U> RemAssign for ValueWithPrefixAndUnit<T, P, U> {
    fn rem_assign(&mut self, rhs: Self) {
        self.value %= rhs.value;
    }
}

impl<T: RemAssign, P, U> RemAssign<T> for ValueWithPrefixAndUnit<T, P, U> {
    fn rem_assign(&mut self, rhs: T) {
        self.value %= rhs;
    }
}

impl<T: Sub, P, U> Sub for ValueWithPrefixAndUnit<T, P, U> {
    type Output = ValueWithPrefixAndUnit<<T as Sub>::Output, P, U>;

    fn sub(self, rhs: Self) -> Self::Output {
        ValueWithPrefixAndUnit::new(self.value - rhs.value)
    }
}

impl<T: for<'a> Sub<&'a T, Output = T>, P, U> Sub<&Self> for ValueWithPrefixAndUnit<T, P, U> {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        ValueWithPrefixAndUnit::new(self.value - &rhs.value)
    }
}

impl<T: SubAssign, P, U> SubAssign for ValueWithPrefixAndUnit<T, P, U> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl<T: for<'a> SubAssign<&'a T>, P, U> SubAssign<&Self> for ValueWithPrefixAndUnit<T, P, U> {
    fn sub_assign(&mut self, rhs: &Self) {
        self.value -= &rhs.value;
    }
}

impl<T: Add<Output = T> + Zero, P, U> Sum for ValueWithPrefixAndUnit<T, P, U> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Zero::zero(), |a, b| a + b)
    }
}

impl<'a, T: for<'b> Add<&'b T, Output = T> + Zero, P, U> Sum<&'a Self>
    for ValueWithPrefixAndUnit<T, P, U>
{
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Zero::zero(), |a, b| a + b)
    }
}

impl<T: UpperExp, P: Prefix, U: Unit> UpperExp for ValueWithPrefixAndUnit<T, P, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:E}{}{}", self.value, P::PREFIX, U::UNIT)
    }
}

impl<T: Number, P: Prefix, U: Unit> Number<T> for ValueWithPrefixAndUnit<T, P, U> {
    const MAX: Self = ValueWithPrefixAndUnit {
        value: T::MAX,
        _prefix: PhantomData,
        _unit: PhantomData,
    };
    const MIN: Self = ValueWithPrefixAndUnit {
        value: T::MIN,
        _prefix: PhantomData,
        _unit: PhantomData,
    };
}

impl<T: Neg, P, U> Neg for ValueWithPrefixAndUnit<T, P, U> {
    type Output = ValueWithPrefixAndUnit<<T as Neg>::Output, P, U>;

    fn neg(self) -> Self::Output {
        ValueWithPrefixAndUnit::new(-self.value)
    }
}

impl<T: Abs, P, U> Abs for ValueWithPrefixAndUnit<T, P, U> {
    fn abs(self) -> Self {
        ValueWithPrefixAndUnit::new(self.value.abs())
    }
}

impl<T: Signum, P, U> Signum for ValueWithPrefixAndUnit<T, P, U> {
    fn signum(self) -> Self {
        ValueWithPrefixAndUnit::new(self.value.signum())
    }
}

impl<T: SignedNumber, P: Prefix, U: Unit> SignedNumber<T> for ValueWithPrefixAndUnit<T, P, U> {}

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

impl<T: ConvenientNumber, P: Prefix, U: Unit> ConvenientNumber for ValueWithPrefixAndUnit<T, P, U> {}

//impl<T: FloatingPoint, P: Prefix, U: Unit> FloatingPoint<T> for ValueWithPrefixAndUnit<T, P, U> {}
