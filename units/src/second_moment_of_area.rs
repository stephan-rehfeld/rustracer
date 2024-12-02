use super::prefix::None;
use super::ValueWithPrefixAndUnit;

use std::ops::{Div, Mul};

use crate::area::{Area, SquareMeter};
use crate::length::{Length, Meter};
use crate::volume::{CubicMeter, Volume};
use traits::{ConvenientNumber, Number, Sqrt};

pub trait SecondMomentOfArea:
    Number<Self::ValueType>
    + Div<Output = Self::ValueType>
    + Div<Self::ValueType, Output = Self>
    + Div<Self::LengthType, Output = Self::VolumeType>
    + Div<Self::AreaType, Output = Self::AreaType>
{
    type ValueType: Number + Mul<Self, Output = Self>;
    type LengthType: Length<
        ValueType = Self::ValueType,
        AreaType = Self::AreaType,
        VolumeType = Self::VolumeType,
    >;
    type AreaType: Area<
        ValueType = Self::ValueType,
        LengthType = Self::LengthType,
        VolumeType = Self::VolumeType,
    >;
    type VolumeType: Volume<
        ValueType = Self::ValueType,
        LengthType = Self::LengthType,
        AreaType = Self::AreaType,
    >;
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct MeterToThePowerOfFourUnit;

impl super::Unit for MeterToThePowerOfFourUnit {
    const UNIT: &'static str = "m⁴";
}

pub type MeterToThePowerOfFour<T> = ValueWithPrefixAndUnit<T, None, MeterToThePowerOfFourUnit>;

impl<T: Div> Div<Meter<T>> for MeterToThePowerOfFour<T> {
    type Output = CubicMeter<<T as Div>::Output>;

    fn div(self, rhs: Meter<T>) -> Self::Output {
        CubicMeter::new(self.value / rhs.value)
    }
}

impl<T: Div> Div<SquareMeter<T>> for MeterToThePowerOfFour<T> {
    type Output = SquareMeter<<T as Div>::Output>;

    fn div(self, rhs: SquareMeter<T>) -> Self::Output {
        SquareMeter::new(self.value / rhs.value)
    }
}

impl<T: Sqrt> Sqrt for MeterToThePowerOfFour<T> {
    type Output = SquareMeter<<T as Sqrt>::Output>;

    fn sqrt(self) -> Self::Output {
        SquareMeter::new(self.value.sqrt())
    }
}

impl<T> SecondMomentOfArea for MeterToThePowerOfFour<T>
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
    type VolumeType = CubicMeter<T>;
}
