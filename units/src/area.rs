use super::prefix::None;
use super::second_moment_of_area::SecondMomentOfArea;
use super::ValueWithPrefixAndUnit;

use std::ops::{Div, Mul};

use crate::length::{Length, Meter};
use crate::second_moment_of_area::MeterToThePowerOfFour;
use crate::volume::{CubicMeter, Volume};
use traits::{ConvenientNumber, Number, SelfMulNumber, Sqrt};

pub trait Area:
    Number<Self::ValueType>
    + ConvenientNumber
    + Mul<Self::LengthType, Output = Self::VolumeType>
    + Mul<Output = Self::SecondMomentOfAreaType>
    + Div<Self::LengthType, Output = Self::LengthType>
    + Div<Output = Self::ValueType>
{
    type ValueType: Number + Mul<Self, Output = Self>;
    type LengthType: Length<
        ValueType = Self::ValueType,
        AreaType = Self,
        VolumeType = Self::VolumeType,
        SecondMomentOfAreaType = Self::SecondMomentOfAreaType,
    >;
    type VolumeType: Volume<
        ValueType = Self::ValueType,
        LengthType = Self::LengthType,
        AreaType = Self,
        SecondMomentOfAreaType = Self::SecondMomentOfAreaType,
    >;
    type SecondMomentOfAreaType: SecondMomentOfArea<
        ValueType = Self::ValueType,
        LengthType = Self::LengthType,
        AreaType = Self,
        VolumeType = Self::VolumeType,
    >;
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct SquareMeterUnit;

impl super::Unit for SquareMeterUnit {
    const UNIT: &'static str = "m²";
}

pub type SquareMeter<T> = ValueWithPrefixAndUnit<T, None, SquareMeterUnit>;

impl<T: Mul> Mul<Meter<T>> for SquareMeter<T> {
    type Output = CubicMeter<<T as Mul>::Output>;

    fn mul(self, rhs: Meter<T>) -> Self::Output {
        CubicMeter::new(self.value * rhs.value)
    }
}

impl<T: Mul> Mul for SquareMeter<T> {
    type Output = MeterToThePowerOfFour<<T as Mul>::Output>;

    fn mul(self, rhs: SquareMeter<T>) -> Self::Output {
        MeterToThePowerOfFour::new(self.value * rhs.value)
    }
}

impl<T: SelfMulNumber<T>> SelfMulNumber<T> for SquareMeter<T> {}

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

impl<T> Area for SquareMeter<T>
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
    type VolumeType = CubicMeter<T>;
    type SecondMomentOfAreaType = MeterToThePowerOfFour<T>;
}
