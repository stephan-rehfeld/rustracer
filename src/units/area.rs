use super::ValueWithPrefixAndUnit;
use super::prefix;

use std::ops;

use crate::units::length::Meter;
use crate::units::volume::CubicMeter;

#[derive(Debug,PartialEq,PartialOrd,Clone,Copy)]
pub struct SquareMeterUnit;

impl super::Unit for SquareMeterUnit {
    const UNIT: &'static str = "m²"; 
}

pub type SquareMeter<T> = ValueWithPrefixAndUnit<T, prefix::None, SquareMeterUnit>;

impl<T: ops::Mul> ops::Mul<Meter<T>> for SquareMeter<T> {

    type Output = CubicMeter< <T as ops::Mul >::Output>;

    fn mul(self, rhs: Meter<T>) -> Self::Output {
        CubicMeter::new(self.value * rhs.value)
    }
}

impl<T: ops::Div> ops::Div<Meter<T>> for SquareMeter<T> {

    type Output = Meter<<T as ops::Div>::Output>;

    fn div(self, rhs: Meter<T>) -> Self::Output {
        Meter::new(self.value / rhs.value)
    }
}

