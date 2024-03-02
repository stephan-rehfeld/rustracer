use super::ValueWithPrefixAndUnit;
use super::prefix::None;

use crate::traits::number::{Number, SelfMultiply};
use crate::units::area::{Area, SquareMeter};
use crate::units::volume::{CubicMeter, Volume};

use std::ops::{Div, Mul};

pub trait Length: Number<Self::ValueType>
                + SelfMultiply<Self::ValueType>
                + Mul<Output=Self::AreaType>
                + Mul<Self::AreaType, Output=Self::VolumeType>
                + Div<Output=Self::ValueType> {
    type ValueType: Number;
    type AreaType: Area;
    type VolumeType: Volume;
}

#[derive(Debug,PartialEq,PartialOrd,Clone,Copy)]
pub struct MeterUnit;

impl super::Unit for MeterUnit {
    const UNIT: &'static str = "m"; 
}

pub type Meter<T> = ValueWithPrefixAndUnit<T, None, MeterUnit>;

impl<T: Mul> Mul for Meter<T> {

    type Output = SquareMeter< <T as Mul >::Output>;

    fn mul(self, rhs: Meter<T>) -> Self::Output {
        SquareMeter::new(self.value * rhs.value)
    }
}

impl<T: Mul> Mul<SquareMeter<T>> for Meter<T> {

    type Output = CubicMeter< <T as Mul >::Output>;

    fn mul(self, rhs: SquareMeter<T>) -> Self::Output {
        CubicMeter::new(self.value * rhs.value)
    }
}


impl<T: SelfMultiply> SelfMultiply<T> for Meter<T> {
}

impl<T: Number + SelfMultiply> Length for Meter<T> {
    type ValueType = T;
    type AreaType = SquareMeter<T>;
    type VolumeType = CubicMeter<T>;
}


