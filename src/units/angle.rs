use crate::traits;
use super::ValueWithPrefixAndUnit;
use super::prefix;

#[derive(Debug,PartialEq,PartialOrd,Clone,Copy)]
pub struct DegreesUnit;

impl super::Unit for DegreesUnit {
    const UNIT: &'static str = "°"; 
}

#[derive(Debug,PartialEq,PartialOrd,Clone,Copy)]
pub struct RadiansUnit;

impl super::Unit for RadiansUnit {
    const UNIT: &'static str = "rad"; 
}


pub trait Angle<T>: traits::ToDegrees + traits::ToRadians {
//    fn to_degrees(self) -> Degrees<T>;
//    fn to_radians(self) -> Radians<T>;
}

pub type Degrees<T> = ValueWithPrefixAndUnit<T, prefix::None, DegreesUnit>;
pub type Radians<T> = ValueWithPrefixAndUnit<T, prefix::None, RadiansUnit>;

// acos
// acosh
// asin
// asinh
// atan
// atan2
// atanh
// cos
// cosh
// sin
// sin_cos
// sinh
// tan

impl<T: traits::Tan> traits::Tan for Radians<T> {
    type Output = <T as traits::Tan>::Output;

    fn tan(self) -> Self::Output {
        self.value.tan()
    }
}

// tanh

impl<T: traits::ToDegrees> traits::ToDegrees for Radians<T> {
    type Output = Degrees<<T as traits::ToDegrees>::Output>;

    fn to_degrees(self) -> Self::Output {
        Degrees::new( self.value.to_degrees() )
    }
}

impl<T> traits::ToDegrees for Degrees<T> {
    type Output = Self;

    fn to_degrees(self) -> Self::Output {
        self
    }
}

impl<T> traits::ToRadians for Radians<T> {
    type Output = Self;

    fn to_radians(self) -> Self::Output {
        self
    }
}

impl<T: traits::ToRadians> traits::ToRadians for Degrees<T> {
    type Output = Radians<<T as traits::ToRadians>::Output>;

    fn to_radians(self) -> Self::Output {
        Radians::new( self.value.to_radians() )
    }
}


// to_degrees
// to_radians


