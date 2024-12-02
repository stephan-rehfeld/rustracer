use std::ops::Deref;

use crate::light::Light;
use cg_basics::material::{LambertMaterial, PhongMaterial, UnshadedMaterial};
use colors::Color;
use image::Image;
use math::geometry::SurfacePoint;
use math::{Point2, Vector3};
use traits::floating_point::{Max, Powf, Sqrt};
use traits::{FloatingPoint, Zero};
use units::length::Length;

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
