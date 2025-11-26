use std::ops::Mul;

use crate::light::DiffuseRayTracerLight;
use cg_basics::material::{LambertMaterial, PhongMaterial, UnshadedMaterial};
use colors::Color;
use image::Image;
use math::geometry::SurfacePoint;
use math::{Point2, Vector3};
use random::{RandomNumberGenerator, WichmannHillPRNG};
use sampling::{PatternMapping, SamplingPattern};
use scene_graph::Material;
use traits::floating_point::{Max, Powf, Sqrt};
use traits::{FloatingPoint, Zero};
use units::length::Length;

pub trait DiffuseRayTracerMaterial<T: Length> {
    type ColorType: Color;

    fn color_for(
        &self,
        sp: SurfacePoint<T>,
        d: Vector3<T>,
        lights: Vec<&scene_graph::Light<T, Self::ColorType>>,
    ) -> Self::ColorType;
}

impl<T: Length, C: Color<ChannelType = T::ValueType>> DiffuseRayTracerMaterial<T>
    for Material<Box<dyn Image<ColorType = C, PointType = Point2<T::ValueType>>>>
where
    T::ValueType: FloatingPoint,
    T::AreaType: Sqrt<Output = T>,
    WichmannHillPRNG: RandomNumberGenerator<T::ValueType>,
    SamplingPattern<Point2<T::ValueType>>: PatternMapping<T::ValueType>,
{
    type ColorType = C;

    fn color_for(
        &self,
        sp: SurfacePoint<T>,
        d: Vector3<T>,
        lights: Vec<&scene_graph::Light<T, Self::ColorType>>,
    ) -> Self::ColorType {
        match self {
            Material::UnshadedMaterial(mat) => mat.color_for(sp, d, lights),
            Material::LambertMaterial(mat) => mat.color_for(sp, d, lights),
            Material::PhongMaterial(mat) => mat.color_for(sp, d, lights),
        }
    }
}

impl<T: Length, I: Image<PointType = Point2<<T as Length>::ValueType>>> DiffuseRayTracerMaterial<T>
    for UnshadedMaterial<I>
{
    type ColorType = <I as Image>::ColorType;

    fn color_for(
        &self,
        sp: SurfacePoint<T>,
        _d: Vector3<T>,
        _lights: Vec<&scene_graph::Light<T, Self::ColorType>>,
    ) -> Self::ColorType {
        self.texture.get(sp.uv)
    }
}

impl<T: Length, I: Image<PointType = Point2<<T as Length>::ValueType>>> DiffuseRayTracerMaterial<T>
    for LambertMaterial<I>
where
    <T as Length>::ValueType: FloatingPoint + Mul<T, Output = T>,
    <T as Length>::AreaType: Sqrt<Output = T>,
    WichmannHillPRNG: RandomNumberGenerator<T::ValueType>,
    SamplingPattern<Point2<T::ValueType>>: PatternMapping<T::ValueType>,
    <I as Image>::ColorType: Color<ChannelType = <T as Length>::ValueType>,
{
    type ColorType = <I as Image>::ColorType;

    fn color_for(
        &self,
        sp: SurfacePoint<T>,
        _d: Vector3<T>,
        lights: Vec<&scene_graph::Light<T, Self::ColorType>>,
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

impl<T: Length, I: Image<PointType = Point2<<T as Length>::ValueType>>> DiffuseRayTracerMaterial<T>
    for PhongMaterial<I>
where
    <T as Length>::ValueType: FloatingPoint + Sqrt<Output = <T as Length>::ValueType>,
    <T as Length>::AreaType: Sqrt<Output = T>,
    <I as Image>::ColorType: Color<ChannelType = <T as Length>::ValueType>,
    WichmannHillPRNG: RandomNumberGenerator<T::ValueType>,
    SamplingPattern<Point2<T::ValueType>>: PatternMapping<T::ValueType>,
{
    type ColorType = <I as Image>::ColorType;

    fn color_for(
        &self,
        sp: SurfacePoint<T>,
        d: Vector3<T>,
        lights: Vec<&scene_graph::Light<T, Self::ColorType>>,
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
