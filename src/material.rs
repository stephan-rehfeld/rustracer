use crate::color::Color;
use crate::light::Light;
use crate::math::vector::NormalizableVector;
use crate::math::{Normal3, Point3, Vector3};
use crate::traits::floating_point::{Max, Powf, Sqrt};
use crate::traits::{FloatingPoint, Zero};
use crate::units::length::Length;

pub trait Material<T: Length> {
    type ColorType: Color;

    fn color_for(
        &self,
        p: Point3<T>,
        n: Normal3<<T as Length>::ValueType>,
        d: Vector3<T>,
        lights: Vec<&Box<dyn Light<T, Self::ColorType>>>,
        ambient_light: Self::ColorType,
    ) -> Self::ColorType;
}

pub struct SingleColorMaterial<C> {
    color: C,
}

impl<C> SingleColorMaterial<C> {
    pub fn new(color: C) -> SingleColorMaterial<C> {
        SingleColorMaterial { color }
    }
}

impl<T: Length, C: Color> Material<T> for SingleColorMaterial<C> {
    type ColorType = C;

    fn color_for(
        &self,
        _p: Point3<T>,
        _n: Normal3<<T as Length>::ValueType>,
        _d: Vector3<T>,
        _lights: Vec<&Box<dyn Light<T, C>>>,
        _ambient_light: C,
    ) -> C {
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

impl<T: Length, C> Material<T> for LambertMaterial<C>
where
    C: Color<ChannelType = <T as Length>::ValueType>,
{
    type ColorType = C;

    fn color_for(
        &self,
        p: Point3<T>,
        n: Normal3<<T as Length>::ValueType>,
        _d: Vector3<T>,
        lights: Vec<&Box<dyn Light<T, C>>>,
        ambient_light: C,
    ) -> C {
        self.color * ambient_light
            + lights
                .iter()
                .map(|light| {
                    self.color * light.get_color() * Normal3::dot(light.direction_from(p), n)
                })
                .sum()
    }
}

pub struct PhongMaterial<C: Color> {
    diffuse: C,
    specular: C,
    exponent: <C as Color>::ChannelType,
}

impl<C: Color> PhongMaterial<C> {
    pub fn new(diffuse: C, specular: C, exponent: <C as Color>::ChannelType) -> PhongMaterial<C> {
        PhongMaterial {
            diffuse,
            specular,
            exponent,
        }
    }
}

impl<T: Length, C> Material<T> for PhongMaterial<C>
where
    <T as Length>::ValueType: FloatingPoint + Sqrt<Output = <T as Length>::ValueType>,
    <T as Length>::AreaType: Sqrt<Output = T>,
    C: Color<ChannelType = <T as Length>::ValueType>,
{
    type ColorType = C;

    fn color_for(
        &self,
        p: Point3<T>,
        n: Normal3<<T as Length>::ValueType>,
        d: Vector3<T>,
        lights: Vec<&Box<dyn Light<T, C>>>,
        ambient_light: C,
    ) -> C {
        lights
            .iter()
            .map(|light| {
                let ambient_term = self.diffuse * ambient_light;
                let diffuse_term =
                    self.diffuse * light.get_color() * Normal3::dot(light.direction_from(p), n);
                let reflected_light = light
                    .direction_from(p)
                    .as_vector()
                    .reflect_on(n)
                    .normalized();
                let specular_term = self.specular
                    * light.get_color()
                    * Normal3::dot(reflected_light, d.normalized())
                        .max(Zero::zero())
                        .powf(self.exponent);
                ambient_term + diffuse_term + specular_term
            })
            .sum()
    }
}
