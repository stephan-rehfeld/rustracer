use std::ops;

use std::marker::PhantomData;
use crate::traits;

pub trait Angle<T>: traits::ToDegrees + traits::ToRadians {
//    fn to_degrees(self) -> Degrees<T>;
//    fn to_radians(self) -> Radians<T>;
}

pub type Degrees<T> = ValueWithPrefixAndUnit<T, None, DegreesUnit>;
pub type Radians<T> = ValueWithPrefixAndUnit<T, None, RadiansUnit>;


// Add
// AddAsign
// Sub
// SubAssign
// Mul T
// MulAssign T
// Div T

impl<T: ops::Div<T>, P: Prefix, U: Unit> ops::Div<T> for ValueWithPrefixAndUnit<T, P, U> {
    type Output = ValueWithPrefixAndUnit<<T as ops::Div>::Output, P, U>;

    fn div(self, rhs: T) -> Self::Output {
        ValueWithPrefixAndUnit::new(self.value / rhs)
    }
}

// DivAssign T
// Div -> T
// Neg
// Default
// Display

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

pub trait Unit {
    const UNIT: &'static str; 
}

#[derive(Debug,PartialEq,PartialOrd,Clone,Copy)]
pub struct DegreesUnit;

impl Unit for DegreesUnit {
    const UNIT: &'static str = "°"; 
}

#[derive(Debug,PartialEq,PartialOrd,Clone,Copy)]
pub struct RadiansUnit;

impl Unit for RadiansUnit {
    const UNIT: &'static str = "rad"; 
}

#[derive(Debug,PartialEq,PartialOrd,Clone,Copy)]
pub struct ValueWithPrefixAndUnit<T, P: Prefix, U: Unit> {
    value: T,
    _prefix: PhantomData<P>,
    _unit: PhantomData<U>,
} 

impl<T, P: Prefix, U: Unit> ValueWithPrefixAndUnit<T, P, U> {
    pub fn new(value: T) -> ValueWithPrefixAndUnit<T, P, U> {
        ValueWithPrefixAndUnit { value: value, _prefix: PhantomData, _unit: PhantomData }
    }
}

pub trait Prefix {
    const NUMERATOR: u64;
    const DENOMINATOR: u64;
    const PREFIX: &'static str;
}

pub struct Milli;

impl Prefix for Milli {
    const NUMERATOR: u64 = 1;
    const DENOMINATOR: u64 = 1000;
    const PREFIX: &'static str = "m";
}


pub struct Centi;

impl Prefix for Centi {
    const NUMERATOR: u64 = 1;
    const DENOMINATOR: u64 = 100;
    const PREFIX: &'static str = "c";

}

pub struct Deci;

impl Prefix for Deci {
    const NUMERATOR: u64 = 1;
    const DENOMINATOR: u64 = 10;
    const PREFIX: &'static str = "d";

}

#[derive(Debug,PartialEq,PartialOrd,Clone,Copy)]
pub struct None;

impl Prefix for None {
    const NUMERATOR: u64 = 1;
    const DENOMINATOR: u64 = 1;
    const PREFIX: &'static str = "";
}

pub struct Deca;

impl Prefix for Deca {
    const NUMERATOR: u64 = 10;
    const DENOMINATOR: u64 = 1;
    const PREFIX: &'static str = "da";
}

pub struct Hecto;

impl Prefix for Hecto {
    const NUMERATOR: u64 = 100;
    const DENOMINATOR: u64 = 1;
    const PREFIX: &'static str = "h";
}

pub struct Kilo;

impl Prefix for Kilo {
    const NUMERATOR: u64 = 1000;
    const DENOMINATOR: u64 = 1;
    const PREFIX: &'static str = "k";
}
