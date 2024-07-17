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
