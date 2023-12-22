use super::ValueWithPrefixAndUnit;
use super::prefix::None;

use std::ops::Div;

use crate::units::length::Meter;
use crate::units::area::SquareMeter;

#[derive(Debug,PartialEq,PartialOrd,Clone,Copy)]
pub struct CubicMeterUnit;

impl super::Unit for CubicMeterUnit {
    const UNIT: &'static str = "m³"; 
}

pub type CubicMeter<T> = ValueWithPrefixAndUnit<T, None, CubicMeterUnit>;

impl<T: Div> Div<Meter<T>> for CubicMeter<T> {

    type Output = SquareMeter<<T as Div>::Output>;

    fn div(self, rhs: Meter<T>) -> Self::Output {
        SquareMeter::new(self.value / rhs.value)
    }
}

