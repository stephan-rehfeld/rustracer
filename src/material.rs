use std::iter::Sum;
use std::ops::{Add, Div, Mul};

use crate::light::Light;
use crate::math::{Normal3, Point3};
use crate::traits::Zero;

pub trait Material<T> where
    T: Div
{
    type ColorType;

    fn color_for(&self, p: Point3<T>, n: Normal3<<T as Div>::Output>, lights: &Vec<Box<dyn Light<T, Self::ColorType>>>) -> Self::ColorType;
}

pub struct SingleColorMaterial<C> {
    color: C,
}

impl<C> SingleColorMaterial<C> {
    pub fn new(color: C) -> SingleColorMaterial<C> {
        SingleColorMaterial { color }
    }
}

impl<T, C> Material<T> for SingleColorMaterial<C> where
    T: Div,
    C: Copy
{
    type ColorType = C;

    fn color_for(&self, _p: Point3<T>, _n: Normal3<<T as Div>::Output>, _lights: &Vec<Box<dyn Light<T, C>>>) -> C {
        self.color
    }
}

pub struct LambertMaterial<C> {
    color: C,
}

impl<C> LambertMaterial<C> {
    pub fn new(color: C) -> LambertMaterial<C> {
        LambertMaterial { color }
    }
}

impl<T, C> Material<T> for LambertMaterial<C> where
    T: Div + Copy,
    <T as Div>::Output: Add<Output=<T as Div>::Output> + Mul<Output=<T as Div>::Output> + Copy + PartialOrd + Zero,
    C: Mul<<T as Div>::Output, Output=C> + Sum + Copy
{
    type ColorType = C;

    fn color_for(&self, p: Point3<T>, n: Normal3<<T as Div>::Output>, lights: &Vec<Box<dyn Light<T, C>>>) -> C {
        lights.iter()
            .filter(|light| light.illuminates(p, n))
            .filter(|light| Normal3::dot(light.direction_from(p), n) > Zero::zero())
            .map(|light| self.color * Normal3::dot(light.direction_from(p), n))
            .sum()
    }
}


