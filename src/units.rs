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

// Add
// AddAsign
// Sub
// SubAssign
// Mul T
// MulAssign T
// Div T

impl<T: ops::Div, P, U> ops::Div<T> for ValueWithPrefixAndUnit<T, P, U> {
    type Output = ValueWithPrefixAndUnit<<T as ops::Div>::Output, P, U>;

    fn div(self, rhs: T) -> Self::Output {
        ValueWithPrefixAndUnit::new(self.value / rhs)
    }
}

// DivAssign T
// Div -> T
// Neg
// Default
// Display


