use std::ops::{AddAssign, Div, DivAssign, Mul, Neg};

use crate::camera::RaytracingCamera;
use crate::light::DiffuseRayTracerLight;
use crate::material::Material;
use crate::Renderable;
use colors::Color;
use image::{ImageBuffer, WritableImage};
use math::geometry::SurfacePoint;
use math::{Point2, Vector2};
use random::{RandomNumberGenerator, WichmannHillPRNG};
use sampling::SamplingPatternSet;
use sampling::{PatternMapping, SamplingPattern};
use scene_graph::Scene3;
use traits::{ConvenientNumber, SelfMulNumber, Number, Sin, Cos, FloatingPoint, One, Sqrt, Zero};
use units::length::Length;
use units::angle::{Angle, Radians};

pub struct DiffuseRayTracer<T: Length> {
    sampling_patterns: SamplingPatternSet<Point2<T::ValueType>>,
    shadow_tolerance: T::ValueType,
}

impl<T: Length> DiffuseRayTracer<T>
where
    T : ConvenientNumber + Neg<Output = T> + SelfMulNumber<<T as Div>::Output>,
    <T as Div>::Output: FloatingPoint + ConvenientNumber,
    <T as Mul>::Output: Number<<T as Div>::Output> + ConvenientNumber + Sqrt<Output = T>,
    T: Number<<T as Div>::Output> + ConvenientNumber,
    <T as Div>::Output: FloatingPoint + ConvenientNumber + Mul<T, Output = T>,
    Radians<<T as Div>::Output>: Angle + Cos<Output = <T as Div>::Output> + Sin<Output = <T as Div>::Output>,
    T::ValueType: FloatingPoint,
    <T as Length>::AreaType: Sqrt<Output = T>,
    WichmannHillPRNG: RandomNumberGenerator<T::ValueType>,
    SamplingPattern<Point2<T::ValueType>>: PatternMapping<T::ValueType>,
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
        mut scene: Scene3<T, C, Box<dyn Renderable<T, C>>>,
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
                            &dyn Material<T, ColorType = C>,
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
