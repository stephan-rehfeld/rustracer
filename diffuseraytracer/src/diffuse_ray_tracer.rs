use std::ops::{AddAssign, Div, DivAssign, Mul, Neg};

use crate::camera::RaytracingCamera;
use crate::light::DiffuseRayTracerLight;
use crate::material::DiffuseRayTracerMaterial;
use crate::geometry::Renderable;
use colors::Color;
use cg_basics::geometry::{AxisAlignedBox, Cylinder, Disc, Plane, Sphere, Triangle};
use image::{Image, ImageBuffer, WritableImage};
use math::geometry::{Intersect, ParametricLine, SurfacePoint};
use math::{Point2, Point3, Vector2, Vector3};
use random::{RandomNumberGenerator, WichmannHillPRNG};
use sampling::SamplingPatternSet;
use sampling::{PatternMapping, SamplingPattern};
use scene_graph::Scene3;
use traits::{ConvenientNumber, Cos, FloatingPoint, Number, One, SelfMulNumber, Sin, Sqrt, Zero};
use units::angle::{Angle, Radians};
use units::length::Length;

pub struct DiffuseRayTracer<T: Length> {
    sampling_patterns: SamplingPatternSet<Point2<T::ValueType>>,
    shadow_tolerance: T::ValueType,
}

impl<T: Length> DiffuseRayTracer<T>
where
    T: ConvenientNumber + Neg<Output = T> + SelfMulNumber<<T as Div>::Output>,
    T::ValueType: FloatingPoint+ ConvenientNumber,
    Radians<<T as Div>::Output>:
        Angle + Cos<Output = <T as Div>::Output> + Sin<Output = <T as Div>::Output>,
    WichmannHillPRNG: RandomNumberGenerator<T::ValueType>,
    SamplingPattern<Point2<T::ValueType>>: PatternMapping<T::ValueType>,
    ParametricLine<Point3<T>, Vector3<T>>: Intersect<AxisAlignedBox<T>, Output = Vec<(T::ValueType, SurfacePoint<T>)>>
        + Intersect<Cylinder<T>, Output = Vec<(T::ValueType, SurfacePoint<T>)>>
        + Intersect<Disc<T>, Output = Vec<(T::ValueType, SurfacePoint<T>)>>
        + Intersect<Plane<T>, Output = Vec<(T::ValueType, SurfacePoint<T>)>>
        + Intersect<Sphere<T>, Output = Vec<(T::ValueType, SurfacePoint<T>)>>
        + Intersect<Triangle<T>, Output = Vec<(T::ValueType, SurfacePoint<T>)>>,
{
    pub fn new(
        sampling_patterns: SamplingPatternSet<Point2<T::ValueType>>,
        shadow_tolerance: T::ValueType,
    ) -> DiffuseRayTracer<T> {
        DiffuseRayTracer {
            sampling_patterns,
            shadow_tolerance,
        }
    }
    pub fn render<C: Color<ChannelType = T::ValueType>>(
        self,
        mut scene: Scene3<T, C>,
        camera_id: &str,
        size: Vector2<usize>,
        rnd: WichmannHillPRNG,
    ) -> ImageBuffer<C>
    where
        C: AddAssign + DivAssign<C::ChannelType>,
        C::ChannelType: Zero + One,
        u16: Into<T::ValueType>,
    {
        let mut rnd = rnd;

        let mut image_buffer = ImageBuffer::new(size, C::default());

        let camera = scene.cameras.remove(camera_id).unwrap();

        let float_size =
            Vector2::<T::ValueType>::new((size.x as u16).into(), (size.y as u16).into());

        for x in 0..size.x {
            for y in 0..size.y {
                let p = Point2::new(x, y);
                let pattern = self.sampling_patterns.draw_pattern(&mut rnd);

                let mut counter = C::ChannelType::zero();

                let color = image_buffer.get_mut(p);

                for i in 0..pattern.len() {
                    let sp = Point2::<T::ValueType>::new(
                        (p.x as u16).into(),
                        ((size.y - p.y - 1) as u16).into(),
                    ) + pattern[i].as_vector();

                    let ray = camera.ray_for(
                        float_size,
                        sp,
                        self.sampling_patterns.draw_pattern(&mut rnd),
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
                            *color += scene.bg_color;
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
                                        self.sampling_patterns.draw_pattern(&mut rnd),
                                        &mut rnd,
                                    )
                                })
                                .collect();

                            *color += material.color_for(sp, r.direction, lights)
                        }
                    }
                }

                *color /= counter;
            }
        }

        image_buffer
    }
}
