use crate::traits::{
    Acos, Acosh, Asin, Asinh, Atan, Atan2, Atanh, Cos, Cosh, Sin, SinCos, Sinh, Tan, Tanh,
    ToDegrees, ToRadians,
};

use super::prefix::None;
use super::ValueWithPrefixAndUnit;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct DegreesUnit;

impl super::Unit for DegreesUnit {
    const UNIT: &'static str = "°";
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct RadiansUnit;

impl super::Unit for RadiansUnit {
    const UNIT: &'static str = "rad";
}

pub trait Angle: Cos + ToDegrees + ToRadians + Sin {}

pub type Degrees<T> = ValueWithPrefixAndUnit<T, None, DegreesUnit>;
pub type Radians<T> = ValueWithPrefixAndUnit<T, None, RadiansUnit>;

impl<T> Angle for Degrees<T>
where
    T: ToDegrees + ToRadians,
    <T as ToRadians>::Output: Cos + Sin,
{
}

impl<T> Angle for Radians<T> where T: Cos + ToDegrees + ToRadians + Sin {}

impl<T> Radians<T> {
    pub fn acos(v: T) -> Radians<<T as Acos>::Output>
    where
        T: Acos,
    {
        Radians::new(v.acos())
    }

    pub fn acosh(v: T) -> Radians<<T as Acosh>::Output>
    where
        T: Acosh,
    {
        Radians::new(v.acosh())
    }

    pub fn asin(v: T) -> Radians<<T as Asin>::Output>
    where
        T: Asin,
    {
        Radians::new(v.asin())
    }

    pub fn asinh(v: T) -> Radians<<T as Asinh>::Output>
    where
        T: Asinh,
    {
        Radians::new(v.asinh())
    }

    pub fn atan(v: T) -> Radians<<T as Atan>::Output>
    where
        T: Atan,
    {
        Radians::new(v.atan())
    }

    pub fn atan2(v: T, other: T) -> Radians<<T as Atan2>::Output>
    where
        T: Atan2,
    {
        Radians::new(v.atan2(other))
    }

    pub fn atanh(v: T) -> Radians<<T as Atanh>::Output>
    where
        T: Atanh,
    {
        Radians::new(v.atanh())
    }
}

impl<T> Degrees<T> {
    pub fn acos(v: T) -> Degrees<<<T as ToRadians>::Output as Acos>::Output>
    where
        T: ToRadians,
        <T as ToRadians>::Output: Acos,
    {
        Degrees::new(v.to_radians().acos())
    }

    pub fn acosh(v: T) -> Degrees<<<T as ToRadians>::Output as Acosh>::Output>
    where
        T: ToRadians,
        <T as ToRadians>::Output: Acosh,
    {
        Degrees::new(v.to_radians().acosh())
    }

    pub fn asin(v: T) -> Degrees<<<T as ToRadians>::Output as Asin>::Output>
    where
        T: ToRadians,
        <T as ToRadians>::Output: Asin,
    {
        Degrees::new(v.to_radians().asin())
    }

    pub fn asinh(v: T) -> Degrees<<<T as ToRadians>::Output as Asinh>::Output>
    where
        T: ToRadians,
        <T as ToRadians>::Output: Asinh,
    {
        Degrees::new(v.to_radians().asinh())
    }

    pub fn atan(v: T) -> Degrees<<<T as ToRadians>::Output as Atan>::Output>
    where
        T: ToRadians,
        <T as ToRadians>::Output: Atan,
    {
        Degrees::new(v.to_radians().atan())
    }

    pub fn atan2(
        v: T,
        other: <T as ToRadians>::Output,
    ) -> Degrees<<<T as ToRadians>::Output as Atan2>::Output>
    where
        T: ToRadians,
        <T as ToRadians>::Output: Atan2,
    {
        Degrees::new(v.to_radians().atan2(other))
    }

    pub fn atanh(v: T) -> Degrees<<<T as ToRadians>::Output as Atanh>::Output>
    where
        T: ToRadians,
        <T as ToRadians>::Output: Atanh,
    {
        Degrees::new(v.to_radians().atanh())
    }
}

impl<T: Sin> Sin for Radians<T> {
    type Output = <T as Sin>::Output;

    fn sin(self) -> Self::Output {
        self.value.sin()
    }
}

impl<T> Sin for Degrees<T>
where
    T: ToRadians,
    <T as ToRadians>::Output: Sin,
{
    type Output = <<T as ToRadians>::Output as Sin>::Output;

    fn sin(self) -> Self::Output {
        self.value.to_radians().sin()
    }
}

impl<T: SinCos> SinCos for Radians<T> {
    type Output = <T as SinCos>::Output;

    fn sin_cos(self) -> Self::Output {
        self.value.sin_cos()
    }
}

impl<T> SinCos for Degrees<T>
where
    T: ToRadians,
    <T as ToRadians>::Output: SinCos,
{
    type Output = <<T as ToRadians>::Output as SinCos>::Output;

    fn sin_cos(self) -> Self::Output {
        self.value.to_radians().sin_cos()
    }
}

impl<T: Sinh> Sinh for Radians<T> {
    type Output = <T as Sinh>::Output;

    fn sinh(self) -> Self::Output {
        self.value.sinh()
    }
}

impl<T> Sinh for Degrees<T>
where
    T: ToRadians,
    <T as ToRadians>::Output: Sinh,
{
    type Output = <<T as ToRadians>::Output as Sinh>::Output;

    fn sinh(self) -> Self::Output {
        self.value.to_radians().sinh()
    }
}

impl<T: Cos> Cos for Radians<T> {
    type Output = <T as Cos>::Output;

    fn cos(self) -> Self::Output {
        self.value.cos()
    }
}

impl<T> Cos for Degrees<T>
where
    T: ToRadians,
    <T as ToRadians>::Output: Cos,
{
    type Output = <<T as ToRadians>::Output as Cos>::Output;

    fn cos(self) -> Self::Output {
        self.value.to_radians().cos()
    }
}

impl<T: Cosh> Cosh for Radians<T> {
    type Output = <T as Cosh>::Output;

    fn cosh(self) -> Self::Output {
        self.value.cosh()
    }
}

impl<T> Cosh for Degrees<T>
where
    T: ToRadians,
    <T as ToRadians>::Output: Cosh,
{
    type Output = <<T as ToRadians>::Output as Cosh>::Output;

    fn cosh(self) -> Self::Output {
        self.value.to_radians().cosh()
    }
}

impl<T: Tan> Tan for Radians<T> {
    type Output = <T as Tan>::Output;

    fn tan(self) -> Self::Output {
        self.value.tan()
    }
}

impl<T> Tan for Degrees<T>
where
    T: ToRadians,
    <T as ToRadians>::Output: Tan,
{
    type Output = <<T as ToRadians>::Output as Tan>::Output;

    fn tan(self) -> Self::Output {
        self.value.to_radians().tan()
    }
}

impl<T: Tanh> Tanh for Radians<T> {
    type Output = <T as Tanh>::Output;

    fn tanh(self) -> Self::Output {
        self.value.tanh()
    }
}

impl<T> Tanh for Degrees<T>
where
    T: ToRadians,
    <T as ToRadians>::Output: Tanh,
{
    type Output = <<T as ToRadians>::Output as Tanh>::Output;

    fn tanh(self) -> Self::Output {
        self.value.to_radians().tanh()
    }
}

impl<T: ToDegrees> ToDegrees for Radians<T> {
    type Output = Degrees<<T as ToDegrees>::Output>;

    fn to_degrees(self) -> Self::Output {
        Degrees::new(self.value.to_degrees())
    }
}

impl<T> ToDegrees for Degrees<T> {
    type Output = Self;

    fn to_degrees(self) -> Self::Output {
        self
    }
}

impl<T> ToRadians for Radians<T> {
    type Output = Self;

    fn to_radians(self) -> Self::Output {
        self
    }
}

impl<T: ToRadians> ToRadians for Degrees<T> {
    type Output = Radians<<T as ToRadians>::Output>;

    fn to_radians(self) -> Self::Output {
        Radians::new(self.value.to_radians())
    }
}
