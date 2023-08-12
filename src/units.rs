use std::fmt;
use std::ops;
use std::marker::PhantomData;

pub mod prefix;
pub mod angle;

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

impl<T: ops::Add, P, U> ops::Add for ValueWithPrefixAndUnit<T, P, U> {
    type Output = ValueWithPrefixAndUnit<<T as ops::Add>::Output, P, U>;

    fn add(self, rhs: Self) -> Self::Output {
        ValueWithPrefixAndUnit::new(self.value + rhs.value)
    }
}

impl<T: ops::AddAssign, P, U> ops::AddAssign for ValueWithPrefixAndUnit<T, P, U> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl<T: ops::Sub, P, U> ops::Sub for ValueWithPrefixAndUnit<T, P, U> {
    type Output = ValueWithPrefixAndUnit<<T as ops::Sub>::Output, P, U>;

    fn sub(self, rhs: Self) -> Self::Output {
        ValueWithPrefixAndUnit::new(self.value - rhs.value)
    }
}

impl<T: ops::SubAssign, P, U> ops::SubAssign for ValueWithPrefixAndUnit<T, P, U> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl<T: ops::Mul, P, U> ops::Mul<T> for ValueWithPrefixAndUnit<T, P, U> {
    type Output = ValueWithPrefixAndUnit<<T as ops::Mul>::Output, P, U>;

    fn mul(self, rhs: T) -> Self::Output {
        ValueWithPrefixAndUnit::new(self.value * rhs)
    }
}

impl<T: ops::MulAssign, P, U> ops::MulAssign<T> for ValueWithPrefixAndUnit<T, P, U> {
    fn mul_assign(&mut self, rhs: T) {
        self.value *= rhs;
    }
}

impl<T: ops::Div, P, U> ops::Div<T> for ValueWithPrefixAndUnit<T, P, U> {
    type Output = ValueWithPrefixAndUnit<<T as ops::Div>::Output, P, U>;

    fn div(self, rhs: T) -> Self::Output {
        ValueWithPrefixAndUnit::new(self.value / rhs)
    }
}

impl<T: ops::DivAssign, P, U> ops::DivAssign<T> for ValueWithPrefixAndUnit<T, P, U> {
    fn div_assign(&mut self, rhs: T) {
        self.value /= rhs;
    }
}

impl<T: ops::Div, P, U> ops::Div for ValueWithPrefixAndUnit<T, P, U> {
    type Output = <T as ops::Div>::Output;

    fn div(self, rhs: Self) -> Self::Output {
        self.value / rhs.value
    }
}

impl<T: ops::Neg, P, U> ops::Neg for ValueWithPrefixAndUnit<T, P, U> {
    type Output = ValueWithPrefixAndUnit<<T as ops::Neg>::Output, P, U>;

    fn neg(self) -> Self::Output {
        ValueWithPrefixAndUnit::new(-self.value)
    }
}

impl<T: Default, P, U> Default for ValueWithPrefixAndUnit<T, P, U> {
    fn default() -> Self {
        ValueWithPrefixAndUnit::new(T::default())
    }
}

impl<T: fmt::Display, P: prefix::Prefix, U: Unit> fmt::Display for ValueWithPrefixAndUnit<T, P, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.value, P::PREFIX, U::UNIT)
    }
}
