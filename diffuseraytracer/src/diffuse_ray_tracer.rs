use std::ops::{AddAssign, DivAssign};

use crate::ray_casting::Scene;
use cg_basics::material::Material;
use colors::Color;
use image::{ImageBuffer, WritableImage};
use math::geometry::SurfacePoint;
use math::{Point2, Vector2};
use random::WichmannHillPRNG;
use sampling::SamplingPatternSet;
use traits::{One, Zero};
use units::length::Length;

pub struct DiffuseRayTracer<T: Length> {
    sampling_patterns: SamplingPatternSet<Point2<T::ValueType>>,
    shadow_tolerance: T::ValueType,
}

impl<T: Length> DiffuseRayTracer<T> {
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
        mut scene: Scene<T, C>,
        camera_id: &str,
        size: Vector2<usize>,
        rnd: WichmannHillPRNG,
    ) -> ImageBuffer<C>
    where
        C: AddAssign + DivAssign<C::ChannelType>,
        C::ChannelType: Zero + One,
        i32: Into<T::ValueType>,
    {
        let mut rnd = rnd;

        let mut image_buffer = ImageBuffer::new(size, C::default());

        let camera = scene.cameras.remove(camera_id).unwrap();

        let float_size =
            Vector2::<T::ValueType>::new((size.x as i32).into(), (size.y as i32).into());

        for x in 0..size.x {
            for y in 0..size.y {
                let p = Point2::new(x, y);
                let pattern = self.sampling_patterns.draw_pattern(&mut rnd);

                let mut counter = C::ChannelType::zero();

                let color = image_buffer.get_mut(p);

                for i in 0..pattern.len() {
                    let sp = Point2::<T::ValueType>::new(
                        (p.x as i32).into(),
                        ((size.y - p.y - 1) as i32).into(),
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
                                    light.illuminates(sp, &|shadow_ray, min_distance| {
                                        let mut hits: Vec<T::ValueType> = scene
                                            .geometries
                                            .iter()
                                            .flat_map(|g| g.intersect(shadow_ray))
                                            .map(|(t, _, _)| t)
                                            .filter(|t| *t > self.shadow_tolerance)
                                            .filter(|t| {
                                                if let Some(min_d) = min_distance {
                                                    *t > min_d / T::one()
                                                } else {
                                                    true
                                                }
                                            })
                                            .collect();
                                        hits.sort_by(|t1, t2| t1.partial_cmp(t2).unwrap());
                                        hits.first().copied()
                                    })
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
