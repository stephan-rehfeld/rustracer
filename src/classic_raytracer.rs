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

pub struct ClassicRaytracer<T: Length, C>
where
    C: Color<ChannelType = <T as Length>::ValueType>,
{
    camera: Box<dyn RaytracingCamera<T>>,
    scene: Vec<Box<<Self as Raytracer>::RenderableTraitType>>,
    lights: Vec<Box<dyn Light<T, C>>>,
    bg_color: C,
}

impl<T: Length, C> ClassicRaytracer<T, C>
where
    C: Color<ChannelType = <T as Length>::ValueType>,
{
    pub fn new(
        camera: Box<dyn RaytracingCamera<T>>,
        scene: Vec<Box<<Self as Raytracer>::RenderableTraitType>>,
        lights: Vec<Box<dyn Light<T, C>>>,
        bg_color: C,
    ) -> ClassicRaytracer<T, C> {
        ClassicRaytracer {
            camera,
            scene,
            lights,
            bg_color,
        }
    }
}

impl<T: Length, C> Image for ClassicRaytracer<T, C>
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
            material.color_for(p, n, ray.direction, &self.lights)
        }
    }
}

impl<T, C> Raytracer for ClassicRaytracer<T, C>
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
