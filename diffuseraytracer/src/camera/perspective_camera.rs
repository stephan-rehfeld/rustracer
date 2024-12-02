use std::ops::{Div, Mul};

use cg_basics::camera::PerspectiveCamera;

use math::geometry::ParametricLine;
use math::{Point2, Point3, Vector2, Vector3};
use random::WichmannHillPRNG;
use sampling::SamplingPattern;
use traits::{ConvenientNumber, FloatingPoint, Half, Number, SelfMulNumber, Sqrt, Tan};

use crate::camera::RaytracingCamera;

impl<T> RaytracingCamera<T> for PerspectiveCamera<T>
where
    T: SelfMulNumber<<T as Div>::Output>,
    <T as Div>::Output: FloatingPoint + ConvenientNumber + Mul<T, Output = T>,
    <T as Mul>::Output: Number<<T as Div>::Output> + ConvenientNumber + Sqrt<Output = T>,
{
    fn ray_for(
        &self,
        size: Vector2<<T as Div>::Output>,
        p: Point2<<T as Div>::Output>,
        pattern: &SamplingPattern<Point2<<T as Div>::Output>>,
    ) -> Option<ParametricLine<Point3<T>, Vector3<T>>> {
        let o = self.e;

        let unit_plane_distance = size.y.half() / self.vertical_field_of_view.tan();

        let focal_length_factor = (self.focal_length / T::one()) / unit_plane_distance;

        let a = -self.w * (self.focal_length / T::one());
        let b = self.u * (p.x - size.x.half()) * focal_length_factor;
        let c = self.v * (p.y - size.y.half()) * focal_length_factor;

        let r = a + b + c;
        let fp = o + r * T::one();

        let mut rnd = WichmannHillPRNG::new_random();

        let sampling_point = pattern.draw_point(&mut rnd);
        let lo = o
            + self.u * sampling_point.x * self.lens_radius
            + self.v * sampling_point.y * self.lens_radius;

        let direction = (fp - lo).normalized() * T::one();

        Some(ParametricLine::new(lo, direction))
    }
}
