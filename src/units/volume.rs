use super::ValueWithPrefixAndUnit;
use super::prefix::None;

use std::ops::{Div, Mul};

use crate::traits::number::{Number, SelfMultiply};
use crate::units::area::{Area, SquareMeter};
use crate::units::second_moment_of_area::SecondMomentOfArea;
use crate::units::length::{Length, Meter};

pub trait Volume: Number<Self::ValueType>
                + Div<Self::LengthType, Output = Self::AreaType>
                + Div<Self::AreaType, Output = Self::LengthType> {
    type ValueType: Number;
    type LengthType: Length;
    type AreaType: Area;
}

#[derive(Debug,PartialEq,PartialOrd,Clone,Copy)]
pub struct CubicMeterUnit;

impl super::Unit for CubicMeterUnit {
    const UNIT: &'static str = "m³"; 
}

pub type CubicMeter<T> = ValueWithPrefixAndUnit<T, None, CubicMeterUnit>;

impl<T: Mul> Mul<Meter<T>> for CubicMeter<T> {

    type Output = SecondMomentOfArea<<T as Mul>::Output>;

    fn mul(self, rhs: Meter<T>) -> Self::Output {
        SecondMomentOfArea::new(self.value * rhs.value)
    }

}

impl<T: Div> Div<Meter<T>> for CubicMeter<T> {

    type Output = SquareMeter<<T as Div>::Output>;

    fn div(self, rhs: Meter<T>) -> Self::Output {
        SquareMeter::new(self.value / rhs.value)
    }
}

impl<T: Div> Div<SquareMeter<T>> for CubicMeter<T> {

    type Output = Meter<<T as Div>::Output>;

    fn div(self, rhs: SquareMeter<T>) -> Self::Output {
        Meter::new(self.value / rhs.value)
    }
}


impl<T: Number + SelfMultiply> Volume for CubicMeter<T> {
    type ValueType = T;
    type LengthType = Meter<T>;
    type AreaType = SquareMeter<T>;
}
