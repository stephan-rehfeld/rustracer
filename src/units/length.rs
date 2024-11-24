use super::prefix::None;
use super::ValueWithPrefixAndUnit;

use crate::traits::number::{Number, SelfMultiply};
use crate::units::area::{Area, SquareMeter};
use crate::units::volume::{CubicMeter, Volume};

use std::ops::{Div, Mul};

pub trait Length:
    Number<Self::ValueType>
    + SelfMultiply<Self::ValueType>
    + Mul<Output = Self::AreaType>
    + Mul<Self::AreaType, Output = Self::VolumeType>
    + Div<Output = Self::ValueType>
{
    type ValueType: Number + Mul<Self, Output = Self>;
    type AreaType: Area<ValueType = Self::ValueType>;
    type VolumeType: Volume<ValueType = Self::ValueType>;
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct MeterUnit;

impl super::Unit for MeterUnit {
    const UNIT: &'static str = "m";
}

pub type Meter<T> = ValueWithPrefixAndUnit<T, None, MeterUnit>;

impl<T: Mul> Mul for Meter<T> {
    type Output = SquareMeter<<T as Mul>::Output>;

    fn mul(self, rhs: Meter<T>) -> Self::Output {
        SquareMeter::new(self.value * rhs.value)
    }
}

impl<T: Mul> Mul<SquareMeter<T>> for Meter<T> {
    type Output = CubicMeter<<T as Mul>::Output>;

    fn mul(self, rhs: SquareMeter<T>) -> Self::Output {
        CubicMeter::new(self.value * rhs.value)
    }
}

impl<T: SelfMultiply> SelfMultiply<T> for Meter<T> {}

impl<T> Length for Meter<T>
where
    T: Number
        + SelfMultiply
        + Mul<Meter<T>, Output = Meter<T>>
        + Mul<SquareMeter<T>, Output = SquareMeter<T>>
        + Mul<CubicMeter<T>, Output = CubicMeter<T>>,
{
    type ValueType = T;
    type AreaType = SquareMeter<T>;
    type VolumeType = CubicMeter<T>;
}
