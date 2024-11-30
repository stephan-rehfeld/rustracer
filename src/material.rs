use std::ops::Deref;

use crate::color::Color;
use crate::image::Image;
use crate::light::Light;
use crate::math::geometry::SurfacePoint;
use crate::math::{Point2, Vector3};
use crate::traits::floating_point::{Max, Powf, Sqrt};
use crate::traits::{FloatingPoint, Zero};
use crate::units::length::Length;

pub trait Material<T: Length> {
    type ColorType: Color;

    fn color_for(
        &self,
        sp: SurfacePoint<T>,
        d: Vector3<T>,
        lights: Vec<&Box<dyn Light<T, Self::ColorType>>>,
    ) -> Self::ColorType;
}

impl<T: Length, C: Color> Material<T> for Box<dyn Material<T, ColorType = C>> {
    type ColorType = C;

    fn color_for(
        &self,
        sp: SurfacePoint<T>,
        d: Vector3<T>,
        lights: Vec<&Box<dyn Light<T, Self::ColorType>>>,
    ) -> Self::ColorType {
        self.deref().color_for(sp, d, lights)
    }
}

pub struct UnshadedMaterial<I: Image> {
    texture: I,
}

impl<I: Image> UnshadedMaterial<I> {
    pub fn new(texture: I) -> UnshadedMaterial<I> {
        UnshadedMaterial { texture }
    }
}

impl<T: Length, I: Image<PointType = Point2<<T as Length>::ValueType>>> Material<T>
    for UnshadedMaterial<I>
{
    type ColorType = <I as Image>::ColorType;

    fn color_for(
        &self,
        sp: SurfacePoint<T>,
        _d: Vector3<T>,
        _lights: Vec<&Box<dyn Light<T, Self::ColorType>>>,
    ) -> Self::ColorType {
        self.texture.get(sp.uv)
    }
}

pub struct LambertMaterial<I: Image> {
    texture: I,
}

impl<I: Image> LambertMaterial<I> {
    pub fn new(texture: I) -> LambertMaterial<I> {
        LambertMaterial { texture }
    }
}

impl<T: Length, I: Image<PointType = Point2<<T as Length>::ValueType>>> Material<T>
    for LambertMaterial<I>
where
    <I as Image>::ColorType: Color<ChannelType = <T as Length>::ValueType>,
{
    type ColorType = <I as Image>::ColorType;

    fn color_for(
        &self,
        sp: SurfacePoint<T>,
        _d: Vector3<T>,
        lights: Vec<&Box<dyn Light<T, Self::ColorType>>>,
    ) -> Self::ColorType {
        lights
            .iter()
            .map(|light| {
                self.texture.get(sp.uv)
                    * light.get_color()
                    * light.direction_from(sp).dot(sp.n.as_vector())
            })
            .sum()
    }
}

pub struct PhongMaterial<I: Image> {
    diffuse_texture: I,
    specular_texture: I,
    exponent: <<I as Image>::ColorType as Color>::ChannelType,
}

impl<I: Image> PhongMaterial<I> {
    pub fn new(
        diffuse_texture: I,
        specular_texture: I,
        exponent: <<I as Image>::ColorType as Color>::ChannelType,
    ) -> PhongMaterial<I> {
        PhongMaterial {
            diffuse_texture,
            specular_texture,
            exponent,
        }
    }
}

impl<T: Length, I: Image<PointType = Point2<<T as Length>::ValueType>>> Material<T>
    for PhongMaterial<I>
where
    <T as Length>::ValueType: FloatingPoint + Sqrt<Output = <T as Length>::ValueType>,
    <T as Length>::AreaType: Sqrt<Output = T>,
    <I as Image>::ColorType: Color<ChannelType = <T as Length>::ValueType>,
{
    type ColorType = <I as Image>::ColorType;

    fn color_for(
        &self,
        sp: SurfacePoint<T>,
        d: Vector3<T>,
        lights: Vec<&Box<dyn Light<T, Self::ColorType>>>,
    ) -> Self::ColorType {
        lights
            .iter()
            .map(|light| {
                let diffuse_term = self.diffuse_texture.get(sp.uv)
                    * light.get_color()
                    * light.direction_from(sp).dot(sp.n.as_vector());
                let reflected_light = light.direction_from(sp).reflect_on(sp.n).normalized();
                let specular_term = self.specular_texture.get(sp.uv)
                    * light.get_color()
                    * reflected_light
                        .dot(d.normalized())
                        .max(Zero::zero())
                        .powf(self.exponent);
                diffuse_term + specular_term
            })
            .sum()
    }
}
