use std::ops::Neg;

use colors::Color;
use cg_basics::geometry::{AxisAlignedBox, Cylinder, Disc, Plane, Sphere, Triangle};
use image::{Image, ImageBuffer, WritableImage};
use math::{Point2, Point3, Vector2, Vector3};
use math::geometry::{Intersect, ParametricLine, SurfacePoint};
use scene_graph::Scene3;
use traits::{ConvenientNumber, One, Zero, FloatingPoint, Sin, Cos};
use units::angle::{Angle, Radians};
use units::length::Length;

use crate::camera::RaycasterCamera;
use crate::geometry::Renderable;
use crate::light::RaycasterLight;
use crate::material::RaycasterMaterial;

pub struct Raycaster<T: Length> {
    shadow_tolerance: T::ValueType,
}

impl<T: Length> Raycaster<T> {

    pub fn new(shadow_tolerance: T::ValueType) -> Self {
        Raycaster { shadow_tolerance }
    }

    pub fn render<C: Color<ChannelType = T::ValueType>>
    (
        self,
        mut scene: Scene3<T, C>,
        camera_id: &str,
        size: Vector2<usize>,
    ) -> ImageBuffer<C>  where 
        T: Neg<Output = T>,
        T::ValueType: FloatingPoint + ConvenientNumber,
        u16: Into<T::ValueType>,
        Radians<T::ValueType>: Angle + Sin<Output = T::ValueType> + Cos<Output = T::ValueType>,

        ParametricLine<Point3<T>, Vector3<T>>: Intersect<AxisAlignedBox<T>, Output = Vec<(T::ValueType, SurfacePoint<T>)>>
            + Intersect<Cylinder<T>, Output = Vec<(T::ValueType, SurfacePoint<T>)>>
            + Intersect<Disc<T>, Output = Vec<(T::ValueType, SurfacePoint<T>)>>
            + Intersect<Plane<T>, Output = Vec<(T::ValueType, SurfacePoint<T>)>>
            + Intersect<Sphere<T>, Output = Vec<(T::ValueType, SurfacePoint<T>)>>
            + Intersect<Triangle<T>, Output = Vec<(T::ValueType, SurfacePoint<T>)>>,
    {
        let mut image_buffer = ImageBuffer::new(size, C::default());

        let camera = scene.cameras.remove(camera_id).unwrap();

        let float_size =
            Vector2::<T::ValueType>::new((size.x as u16).into(), (size.y as u16).into());

        for x in 0..size.x {
            for y in 0..size.y {
                let p = Point2::new(x, y);

                let mut counter = C::ChannelType::zero();

                let color = image_buffer.get_mut(p);

                let float_p = Point2::<T::ValueType>::new((p.x as u16).into(), ((size.y - p.y - 1) as u16).into());

                    let ray = camera.ray_for(
                        float_size,
                        float_p,
                    );

                    if let Some(r) = ray {
                        let mut hits: Vec<(
                            T::ValueType,
                            SurfacePoint<T>,
                            &scene_graph::Material<
                                Box<dyn Image<ColorType = C, PointType = Point2<T::ValueType>>>,
                            >,
                        )> = scene
                            .geometries
                            .iter()
                            .flat_map(|g| g.intersect(r))
                            .filter(|(t, _, _)| *t > Zero::zero())
                            .collect();

                        hits.sort_by(|(t1, _, _), (t2, _, _)| t1.partial_cmp(t2).unwrap());

                        counter += C::ChannelType::one();

                        if hits.is_empty() {
                            *color = scene.bg_color;
                        } else {
                            let (_, sp, material) = hits.remove(0);
                            let lights = scene
                                .lights
                                .iter()
                                .filter(|light| {
                                    let shadow_function = |shadow_ray, min_distance| {
                                        let mut hits: Vec<T::ValueType> = scene
                                            .geometries
                                            .iter()
                                            .flat_map(|g| g.intersect(shadow_ray))
                                            .map(|(t, _, _)| t)
                                            .filter(|t| *t > self.shadow_tolerance)
                                            .filter(|t| {
                                                if let Some(min_d) = min_distance {
                                                    *t < min_d / T::one()
                                                } else {
                                                    true
                                                }
                                            })
                                            .collect();
                                        hits.sort_by(|t1, t2| t1.partial_cmp(t2).unwrap());
                                        hits.first().copied()
                                    };
                                    light.illuminates(
                                        sp,
                                        &shadow_function,
                                    )
                                })
                                .collect();

                            *color = material.color_for(sp, r.direction, lights)
                        }
                    }
            }
        }

        image_buffer
    }
}