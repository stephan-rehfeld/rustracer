use super::ValueWithPrefixAndUnit;
use super::prefix::None;

use std::ops::Div;

use crate::traits::Sqrt;
use crate::units::area::SquareMeter;
use crate::units::length::Meter;
use crate::units::volume::CubicMeter;

#[derive(Debug,PartialEq,PartialOrd,Clone,Copy)]
pub struct MeterToThePowerOfFourUnit;

impl super::Unit for MeterToThePowerOfFourUnit {
    const UNIT: &'static str = "m⁴"; 
}

pub type SecondMomentOfArea<T> = ValueWithPrefixAndUnit<T, None, MeterToThePowerOfFourUnit>;

impl<T: Div> Div<Meter<T>> for SecondMomentOfArea<T> {

    type Output = CubicMeter<<T as Div>::Output>;

    fn div(self, rhs: Meter<T>) -> Self::Output {
        CubicMeter::new(self.value / rhs.value)
    }
}

impl<T: Div> Div<SquareMeter<T>> for SecondMomentOfArea<T> {

    type Output = SquareMeter<<T as Div>::Output>;

    fn div(self, rhs: SquareMeter<T>) -> Self::Output {
        SquareMeter::new(self.value / rhs.value)
    }
}

impl<T: Sqrt> Sqrt for SecondMomentOfArea<T> {
    type Output = SquareMeter<<T as Sqrt>::Output>;

    fn sqrt(self) -> Self::Output {
        SquareMeter::new(self.value.sqrt())
    }
}
