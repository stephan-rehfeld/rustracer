use super::ValueWithPrefixAndUnit;
use super::prefix::None;

use std::ops::{Div, Mul};

use crate::traits::floating_point::Sqrt;
use crate::traits::number::{Number, SelfMultiply};
use crate::units::length::{Length, Meter};
use crate::units::second_moment_of_area::SecondMomentOfArea;
use crate::units::volume::{CubicMeter, Volume};

pub trait Area: Number<Self::ValueType>
              + Mul<Self::LengthType, Output = Self::VolumeType>
              + Div<Self::LengthType, Output = Self::LengthType> {
    type ValueType: Number;
    type LengthType: Length;
    type VolumeType: Volume;
}

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

impl<T: Number + SelfMultiply> Area for SquareMeter<T> {
    type ValueType = T;
    type LengthType = Meter<T>;
    type VolumeType = CubicMeter<T>;
}
