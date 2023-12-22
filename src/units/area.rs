use super::ValueWithPrefixAndUnit;
use super::prefix::None;

use std::ops::{Div, Mul};

use crate::units::length::Meter;
use crate::units::volume::CubicMeter;

#[derive(Debug,PartialEq,PartialOrd,Clone,Copy)]
pub struct SquareMeterUnit;

impl super::Unit for SquareMeterUnit {
    const UNIT: &'static str = "m²"; 
}

pub type SquareMeter<T> = ValueWithPrefixAndUnit<T, None, SquareMeterUnit>;

impl<T: Mul> Mul<Meter<T>> for SquareMeter<T> {

    type Output = CubicMeter< <T as Mul >::Output>;

    fn mul(self, rhs: Meter<T>) -> Self::Output {
        CubicMeter::new(self.value * rhs.value)
    }
}

impl<T: Div> Div<Meter<T>> for SquareMeter<T> {

    type Output = Meter<<T as Div>::Output>;

    fn div(self, rhs: Meter<T>) -> Self::Output {
        Meter::new(self.value / rhs.value)
    }
}

