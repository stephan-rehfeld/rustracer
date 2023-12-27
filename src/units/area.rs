use super::ValueWithPrefixAndUnit;
use super::prefix::None;

use std::ops::{Div, Mul};

use crate::traits::Sqrt;
use crate::units::length::Meter;
use crate::units::second_moment_of_area::SecondMomentOfArea;
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

impl<T: Mul> Mul for SquareMeter<T> {

    type Output = SecondMomentOfArea< <T as Mul >::Output>;

    fn mul(self, rhs: SquareMeter<T>) -> Self::Output {
        SecondMomentOfArea::new(self.value * rhs.value)
    }
}

impl<T: Div> Div<Meter<T>> for SquareMeter<T> {

    type Output = Meter<<T as Div>::Output>;

    fn div(self, rhs: Meter<T>) -> Self::Output {
        Meter::new(self.value / rhs.value)
    }
}

impl<T: Sqrt> Sqrt for SquareMeter<T> {
    type Output = Meter<<T as Sqrt>::Output>;

    fn sqrt(self) -> Self::Output {
        Meter::new(self.value.sqrt())
    }
}
