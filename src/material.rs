use crate::color::Color;
use crate::image::Image;
use crate::light::Light;
use crate::math::vector::NormalizableVector;
use crate::math::{Normal3, Point2, Point3, Vector3};
use crate::traits::floating_point::{Max, Powf, Sqrt};
use crate::traits::{FloatingPoint, Zero};
use crate::units::length::Length;

pub trait Material<T: Length> {
    type ColorType: Color;

    fn color_for(
        &self,
        p: Point3<T>,
        n: Normal3<<T as Length>::ValueType>,
        tex: Point2<<T as Length>::ValueType>,
        d: Vector3<T>,
        lights: Vec<&Box<dyn Light<T, Self::ColorType>>>,
        ambient_light: Self::ColorType,
    ) -> Self::ColorType;
}

pub struct UnshadedMaterial<C: Color> {
    texture: Box<dyn Image<ColorType = C, PointType = Point2<<C as Color>::ChannelType>>>,
}

impl<C: Color> UnshadedMaterial<C> {
    pub fn new(
        texture: Box<dyn Image<ColorType = C, PointType = Point2<<C as Color>::ChannelType>>>,
    ) -> UnshadedMaterial<C> {
        UnshadedMaterial { texture }
    }
}

impl<T: Length, C: Color<ChannelType = <T as Length>::ValueType>> Material<T>
    for UnshadedMaterial<C>
{
    type ColorType = C;

    fn color_for(
        &self,
        _p: Point3<T>,
        _n: Normal3<<T as Length>::ValueType>,
        tex: Point2<<T as Length>::ValueType>,
        _d: Vector3<T>,
        _lights: Vec<&Box<dyn Light<T, Self::ColorType>>>,
        _ambient_light: Self::ColorType,
    ) -> Self::ColorType {
        self.texture.get(tex)
    }
}

pub struct LambertMaterial<C: Color> {
    texture: Box<dyn Image<ColorType = C, PointType = Point2<<C as Color>::ChannelType>>>,
}

impl<C: Color> LambertMaterial<C> {
    pub fn new(
        texture: Box<dyn Image<ColorType = C, PointType = Point2<<C as Color>::ChannelType>>>,
    ) -> LambertMaterial<C> {
        LambertMaterial { texture }
    }
}

impl<T: Length, C: Color<ChannelType = <T as Length>::ValueType>> Material<T>
    for LambertMaterial<C>
{
    type ColorType = C;

    fn color_for(
        &self,
        p: Point3<T>,
        n: Normal3<<T as Length>::ValueType>,
        tex: Point2<<T as Length>::ValueType>,
        _d: Vector3<T>,
        lights: Vec<&Box<dyn Light<T, Self::ColorType>>>,
        ambient_light: Self::ColorType,
    ) -> Self::ColorType {
        self.texture.get(tex) * ambient_light
            + lights
                .iter()
                .map(|light| {
                    self.texture.get(tex)
                        * light.get_color()
                        * Normal3::dot(light.direction_from(p), n)
                })
                .sum()
    }
}

pub struct PhongMaterial<C: Color> {
    diffuse_texture: Box<dyn Image<ColorType = C, PointType = Point2<<C as Color>::ChannelType>>>,
    specular_texture: Box<dyn Image<ColorType = C, PointType = Point2<<C as Color>::ChannelType>>>,
    exponent: <C as Color>::ChannelType,
}

impl<C: Color> PhongMaterial<C> {
    pub fn new(
        diffuse_texture: Box<
            dyn Image<ColorType = C, PointType = Point2<<C as Color>::ChannelType>>,
        >,
        specular_texture: Box<
            dyn Image<ColorType = C, PointType = Point2<<C as Color>::ChannelType>>,
        >,
        exponent: <C as Color>::ChannelType,
    ) -> PhongMaterial<C> {
        PhongMaterial {
            diffuse_texture,
            specular_texture,
            exponent,
        }
    }
}

impl<T: Length, C: Color<ChannelType = <T as Length>::ValueType>> Material<T> for PhongMaterial<C>
where
    <T as Length>::ValueType: FloatingPoint + Sqrt<Output = <T as Length>::ValueType>,
    <T as Length>::AreaType: Sqrt<Output = T>,
{
    type ColorType = C;

    fn color_for(
        &self,
        p: Point3<T>,
        n: Normal3<<T as Length>::ValueType>,
        tex: Point2<<T as Length>::ValueType>,
        d: Vector3<T>,
        lights: Vec<&Box<dyn Light<T, Self::ColorType>>>,
        ambient_light: Self::ColorType,
    ) -> Self::ColorType {
        lights
            .iter()
            .map(|light| {
                let ambient_term = self.diffuse_texture.get(tex) * ambient_light;
                let diffuse_term = self.diffuse_texture.get(tex)
                    * light.get_color()
                    * Normal3::dot(light.direction_from(p), n);
                let reflected_light = light
                    .direction_from(p)
                    .as_vector()
                    .reflect_on(n)
                    .normalized();
                let specular_term = self.specular_texture.get(tex)
                    * light.get_color()
                    * Normal3::dot(reflected_light, d.normalized())
                        .max(Zero::zero())
                        .powf(self.exponent);
                ambient_term + diffuse_term + specular_term
            })
            .sum()
    }
}
