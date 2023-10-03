use crate::traits;
use crate::traits::Acos;
use crate::traits::Acosh;
use crate::traits::Asin;
use crate::traits::Asinh;
use crate::traits::Atan;
use crate::traits::Atan2;
use crate::traits::Atanh;
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

impl<T> Radians<T> {
    pub fn acos(v: T) -> Radians<<T as traits::Acos>::Output> where
        T: traits::Acos
    {
         Radians::new( v.acos() )
    }
    
    pub fn acosh(v: T) -> Radians<<T as traits::Acosh>::Output> where
        T: traits::Acosh
    {
        Radians::new( v.acosh() )
    }

    pub fn asin(v: T) -> Radians<<T as traits::Asin>::Output> where
        T: traits::Asin
    {
        Radians::new( v.asin() )
    }
    
    pub fn asinh(v: T) -> Radians<<T as traits::Asinh>::Output> where
        T: traits::Asinh
    {
        Radians::new( v.asinh() )
    }

    pub fn atan(v: T) -> Radians<<T as traits::Atan>::Output> where
        T: traits::Atan
    {
        Radians::new( v.atan() )
    }

    pub fn atan2(v: T, other: T) -> Radians<<T as traits::Atan2>::Output> where
        T: traits::Atan2
    {
        Radians::new( v.atan2(other) )
    }
    
    pub fn atanh(v: T) -> Radians<<T as traits::Atanh>::Output> where
        T: traits::Atanh
    {
        Radians::new( v.atanh() )
    }
}

impl<T> Degrees<T> {
    pub fn acos(v: T) -> Degrees<<<T as traits::ToRadians>::Output as traits::Acos>::Output> where
        T: traits::ToRadians,
        <T as traits::ToRadians>::Output: traits::Acos
    {
        Degrees::new( v.to_radians().acos() )
    }
    
    pub fn acosh(v: T) -> Degrees<<<T as traits::ToRadians>::Output as traits::Acosh>::Output> where
        T: traits::ToRadians,
        <T as traits::ToRadians>::Output: traits::Acosh
    {
        Degrees::new( v.to_radians().acosh() )
    }

    pub fn asin(v: T) -> Degrees<<<T as traits::ToRadians>::Output as traits::Asin>::Output> where
        T: traits::ToRadians,
        <T as traits::ToRadians>::Output: traits::Asin
    {
        Degrees::new( v.to_radians().asin() )
    }
    
    pub fn asinh(v: T) -> Degrees<<<T as traits::ToRadians>::Output as traits::Asinh>::Output> where
        T: traits::ToRadians,
        <T as traits::ToRadians>::Output: traits::Asinh
    {
        Degrees::new( v.to_radians().asinh() )
    }

    pub fn atan(v: T) -> Degrees<<<T as traits::ToRadians>::Output as traits::Atan>::Output> where
        T: traits::ToRadians,
        <T as traits::ToRadians>::Output: traits::Atan
    {
        Degrees::new( v.to_radians().atan() )
    }

    pub fn atan2(v: T, other: <T as traits::ToRadians>::Output) -> Degrees<<<T as traits::ToRadians>::Output as traits::Atan2>::Output> where
        T: traits::ToRadians,
        <T as traits::ToRadians>::Output: traits::Atan2
    {
        Degrees::new( v.to_radians().atan2(other) )
    }
    
    pub fn atanh(v: T) -> Degrees<<<T as traits::ToRadians>::Output as traits::Atanh>::Output> where
        T: traits::ToRadians,
        <T as traits::ToRadians>::Output: traits::Atanh
    {
        Degrees::new( v.to_radians().atanh() )
    }
}

impl<T: traits::Sin> traits::Sin for Radians<T> {
    type Output = <T as traits::Sin>::Output;

    fn sin(self) -> Self::Output {
        self.value.sin()
    }
}

impl<T> traits::Sin for Degrees<T> where 
    T: traits::ToRadians,
    <T as traits::ToRadians>::Output: traits::Sin
{
    type Output = <<T as traits::ToRadians>::Output as traits::Sin>::Output;

    fn sin(self) -> Self::Output {
        self.value.to_radians().sin()
    }
}


impl<T: traits::SinCos> traits::SinCos for Radians<T> {
    type Output = <T as traits::SinCos>::Output;

    fn sin_cos(self) -> Self::Output {
        self.value.sin_cos()
    }
}

impl<T> traits::SinCos for Degrees<T> where 
    T: traits::ToRadians,
    <T as traits::ToRadians>::Output: traits::SinCos
{
    type Output = <<T as traits::ToRadians>::Output as traits::SinCos>::Output;

    fn sin_cos(self) -> Self::Output {
        self.value.to_radians().sin_cos()
    }
}

impl<T: traits::Sinh> traits::Sinh for Radians<T> {
    type Output = <T as traits::Sinh>::Output;

    fn sinh(self) -> Self::Output {
        self.value.sinh()
    }
}

impl<T> traits::Sinh for Degrees<T> where 
    T: traits::ToRadians,
    <T as traits::ToRadians>::Output: traits::Sinh
{
    type Output = <<T as traits::ToRadians>::Output as traits::Sinh>::Output;

    fn sinh(self) -> Self::Output {
        self.value.to_radians().sinh()
    }
}

impl<T: traits::Cos> traits::Cos for Radians<T> {
    type Output = <T as traits::Cos>::Output;

    fn cos(self) -> Self::Output {
        self.value.cos()
    }
}

impl<T> traits::Cos for Degrees<T> where 
    T: traits::ToRadians,
    <T as traits::ToRadians>::Output: traits::Cos
{
    type Output = <<T as traits::ToRadians>::Output as traits::Cos>::Output;

    fn cos(self) -> Self::Output {
        self.value.to_radians().cos()
    }
}

impl<T: traits::Cosh> traits::Cosh for Radians<T> {
    type Output = <T as traits::Cosh>::Output;

    fn cosh(self) -> Self::Output {
        self.value.cosh()
    }
}

impl<T> traits::Cosh for Degrees<T> where 
    T: traits::ToRadians,
    <T as traits::ToRadians>::Output: traits::Cosh
{
    type Output = <<T as traits::ToRadians>::Output as traits::Cosh>::Output;

    fn cosh(self) -> Self::Output {
        self.value.to_radians().cosh()
    }
}

impl<T: traits::Tan> traits::Tan for Radians<T> {
    type Output = <T as traits::Tan>::Output;

    fn tan(self) -> Self::Output {
        self.value.tan()
    }
}

impl<T> traits::Tan for Degrees<T> where 
    T: traits::ToRadians,
    <T as traits::ToRadians>::Output: traits::Tan
{
    type Output = <<T as traits::ToRadians>::Output as traits::Tan>::Output;

    fn tan(self) -> Self::Output {
        self.value.to_radians().tan()
    }
}

impl<T: traits::Tanh> traits::Tanh for Radians<T> {
    type Output = <T as traits::Tanh>::Output;

    fn tanh(self) -> Self::Output {
        self.value.tanh()
    }
}

impl<T> traits::Tanh for Degrees<T> where 
    T: traits::ToRadians,
    <T as traits::ToRadians>::Output: traits::Tanh
{
    type Output = <<T as traits::ToRadians>::Output as traits::Tanh>::Output;

    fn tanh(self) -> Self::Output {
        self.value.to_radians().tanh()
    }
}

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
