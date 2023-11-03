use super::ValueWithPrefixAndUnit;
use super::prefix;

use std::ops;

use crate::units::length::Meter;
use crate::units::area::SquareMeter;

#[derive(Debug,PartialEq,PartialOrd,Clone,Copy)]
pub struct CubicMeterUnit;

impl super::Unit for CubicMeterUnit {
    const UNIT: &'static str = "m³"; 
}

pub type CubicMeter<T> = ValueWithPrefixAndUnit<T, prefix::None, CubicMeterUnit>;

impl<T: ops::Div> ops::Div<Meter<T>> for CubicMeter<T> {

    type Output = SquareMeter<<T as ops::Div>::Output>;

    fn div(self, rhs: Meter<T>) -> Self::Output {
        SquareMeter::new(self.value / rhs.value)
    }
}

