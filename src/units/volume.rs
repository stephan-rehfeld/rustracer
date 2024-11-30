use super::prefix::None;
use super::ValueWithPrefixAndUnit;

use std::ops::{Div, Mul};

use crate::traits::number::Number;
use crate::traits::ConvenientNumber;
use crate::units::area::{Area, SquareMeter};
use crate::units::length::{Length, Meter};
use crate::units::second_moment_of_area::{MeterToThePowerOfFour, SecondMomentOfArea};

pub trait Volume:
    Number<Self::ValueType>
    + ConvenientNumber
    + Div<Output = Self::ValueType>
    + Div<Self::LengthType, Output = Self::AreaType>
    + Div<Self::AreaType, Output = Self::LengthType>
{
    type ValueType: Number + Mul<Self, Output = Self>;
    type LengthType: Length<
        ValueType = Self::ValueType,
        AreaType = Self::AreaType,
        VolumeType = Self,
        SecondMomentOfAreaType = Self::SecondMomentOfAreaType,
    >;
    type AreaType: Area<
        ValueType = Self::ValueType,
        LengthType = Self::LengthType,
        VolumeType = Self,
        SecondMomentOfAreaType = Self::SecondMomentOfAreaType,
    >;
    type SecondMomentOfAreaType: SecondMomentOfArea<
        ValueType = Self::ValueType,
        LengthType = Self::LengthType,
        AreaType = Self::AreaType,
        VolumeType = Self,
    >;
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct CubicMeterUnit;

impl super::Unit for CubicMeterUnit {
    const UNIT: &'static str = "m³";
}

pub type CubicMeter<T> = ValueWithPrefixAndUnit<T, None, CubicMeterUnit>;

impl<T: Mul> Mul<Meter<T>> for CubicMeter<T> {
    type Output = MeterToThePowerOfFour<<T as Mul>::Output>;

    fn mul(self, rhs: Meter<T>) -> Self::Output {
        MeterToThePowerOfFour::new(self.value * rhs.value)
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

impl<T> Volume for CubicMeter<T>
where
    T: Number
        + ConvenientNumber
        + Mul<Meter<T>, Output = Meter<T>>
        + Mul<SquareMeter<T>, Output = SquareMeter<T>>
        + Mul<CubicMeter<T>, Output = CubicMeter<T>>
        + Mul<MeterToThePowerOfFour<T>, Output = MeterToThePowerOfFour<T>>,
{
    type ValueType = T;
    type LengthType = Meter<T>;
    type AreaType = SquareMeter<T>;
    type SecondMomentOfAreaType = MeterToThePowerOfFour<T>;
}
