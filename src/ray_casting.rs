use std::collections::HashMap;
use std::ops::Div;

use crate::camera::RaytracingCamera;
use crate::color::Color;
use crate::image::Image;
use crate::light::Light;
use crate::material::Material;
use crate::math::geometry::ParametricLine;
use crate::math::{Normal3, Point2, Point3, Vector2, Vector3};
use crate::traits::{One, Zero};
use crate::units::length::Length;
use crate::{Raytracer, Renderable};

pub struct Scene<T: Length, C>
where
    C: Color<ChannelType = <T as Length>::ValueType>,
{
    pub bg_color: C,
    pub ambient_light: C,
    pub lights: Vec<Box<dyn Light<T, C>>>,
    pub cameras: HashMap<String, Box<dyn RaytracingCamera<T>>>,
    pub geometries: Vec<
        Box<
            dyn Renderable<
                T,
                ScalarType = <T as Length>::ValueType,
                ColorType = C,
                LengthType = T,
                VectorType = Vector3<T>,
                PointType = Point3<T>,
                NormalType = Normal3<<T as Length>::ValueType>,
            >,
        >,
    >,
}

impl<T: Length, C> Scene<T, C>
where
    C: Color<ChannelType = <T as Length>::ValueType>,
{
    pub fn new(
        bg_color: C,
        ambient_light: C,
        lights: Vec<Box<dyn Light<T, C>>>,
        cameras: HashMap<String, Box<dyn RaytracingCamera<T>>>,
        geometries: Vec<
            Box<
                dyn Renderable<
                    T,
                    ScalarType = <T as Length>::ValueType,
                    ColorType = C,
                    LengthType = T,
                    VectorType = Vector3<T>,
                    PointType = Point3<T>,
                    NormalType = Normal3<<T as Length>::ValueType>,
                >,
            >,
        >,
    ) -> Scene<T, C> {
        Scene {
            bg_color,
            ambient_light,
            lights,
            cameras,
            geometries,
        }
    }
}

pub struct RayCaster<T: Length, C>
where
    C: Color<ChannelType = <T as Length>::ValueType>,
{
    camera: Box<dyn RaytracingCamera<T>>,
    scene: Vec<Box<<Self as Raytracer>::RenderableTraitType>>,
    lights: Vec<Box<dyn Light<T, C>>>,
    ambient_light: C,
    bg_color: C,
    shadow_tolerance: <T as Length>::ValueType,
}

impl<T: Length, C> RayCaster<T, C>
where
    C: Color<ChannelType = <T as Length>::ValueType>,
{
    pub fn new(
        camera: Box<dyn RaytracingCamera<T>>,
        scene: Vec<Box<<Self as Raytracer>::RenderableTraitType>>,
        lights: Vec<Box<dyn Light<T, C>>>,
        ambient_light: C,
        bg_color: C,
        shadow_tolerance: <T as Length>::ValueType,
    ) -> RayCaster<T, C> {
        RayCaster {
            camera,
            scene,
            lights,
            ambient_light,
            bg_color,
            shadow_tolerance,
        }
    }
}

impl<T: Length, C> Image for RayCaster<T, C>
where
    C: Color<ChannelType = <T as Length>::ValueType>,
{
    type ColorType = C;
    type PointType = Point2<<T as Length>::ValueType>;

    fn size(&self) -> Vector2<<T as Length>::ValueType> {
        self.camera.size()
    }

    fn get(&self, p: Self::PointType) -> Self::ColorType {
        let p = Point2::new(p.x, self.size().y - p.y - <<T as Length>::ValueType>::one());
        let ray = self.camera.ray_for(p);

        let mut hits: Vec<(
            <Self as Raytracer>::ScalarType,
            <Self as Raytracer>::NormalType,
            &dyn Material<T, ColorType = Self::ColorType>,
        )> = self
            .scene
            .iter()
            .flat_map(|g| g.intersect(ray))
            .filter(|(t, _, _)| *t > Zero::zero())
            .collect();
        hits.sort_by(|(t1, _, _), (t2, _, _)| t1.partial_cmp(t2).unwrap());

        if hits.is_empty() {
            self.bg_color
        } else {
            let (t, n, material) = hits[0];
            let p = ray.at(t);
            let lights: Vec<&Box<dyn Light<T, C>>> = self
                .lights
                .iter()
                .filter(|light| {
                    light.illuminates(p, n, &|shadow_ray| {
                        let mut hits: Vec<<Self as Raytracer>::ScalarType> = self
                            .scene
                            .iter()
                            .flat_map(|g| g.intersect(shadow_ray))
                            .map(|(t, _, _)| t)
                            .filter(|t| *t > self.shadow_tolerance)
                            .collect();
                        hits.sort_by(|t1, t2| t1.partial_cmp(t2).unwrap());
                        hits.first().copied()
                    })
                })
                .collect();

            let tex = Point2::new(Zero::zero(), Zero::zero());
            material.color_for(p, n, tex, ray.direction, lights, self.ambient_light)
        }
    }
}

impl<T, C> Raytracer for RayCaster<T, C>
where
    T: Length,
    C: Color<ChannelType = <T as Div>::Output>,
{
    type ScalarType = <T as Div>::Output;
    type LengthType = T;
    type PointType = Point3<T>;
    type VectorType = Vector3<T>;
    type NormalType = Normal3<Self::ScalarType>;
    type ColorType = C;

    type RenderableTraitType = dyn Renderable<
        Self::LengthType,
        ScalarType = Self::ScalarType,
        LengthType = Self::LengthType,
        VectorType = Self::VectorType,
        PointType = <Self as Raytracer>::PointType,
        NormalType = Self::NormalType,
        ColorType = <Self as Raytracer>::ColorType,
    >;

    type Ray = ParametricLine<<Self as Raytracer>::PointType, <Self as Raytracer>::VectorType>;
}
