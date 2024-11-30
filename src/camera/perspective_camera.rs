use std::ops::{Div, Mul};

use crate::math::geometry::ParametricLine;
use crate::math::{Point2, Point3, Vector2, Vector3};
use crate::random::WichmannHillPRNG;
use crate::sampling::SamplingPattern;
use crate::traits::{ConvenientNumber, FloatingPoint, Half, Number, SelfMulNumber, Sqrt, Tan};
use crate::units::angle::Radians;

use super::RaytracingCamera;

pub struct PerspectiveCamera<T>
where
    T: Div,
{
    e: Point3<T>,
    u: Vector3<<T as Div>::Output>,
    v: Vector3<<T as Div>::Output>,
    w: Vector3<<T as Div>::Output>,
    vertical_field_of_view: Radians<<T as Div>::Output>,
    lens_radius: T,
    focal_length: T,
}

impl<T> PerspectiveCamera<T>
where
    T: SelfMulNumber<<T as Div>::Output>,
    <T as Div>::Output: FloatingPoint + ConvenientNumber,
    <T as Mul>::Output: Number<<T as Div>::Output> + ConvenientNumber + Sqrt<Output = T>,
{
    pub fn new(
        e: Point3<T>,
        g: Vector3<T>,
        t: Vector3<T>,
        vertical_field_of_view: Radians<<T as Div>::Output>,
        lens_radius: T,
        focal_length: T,
    ) -> PerspectiveCamera<T> {
        let w = -g.normalized();
        let u = Vector3::cross(t, w).normalized();
        let v = Vector3::cross(w, u).normalized();

        let vertical_field_of_view = vertical_field_of_view.half();

        PerspectiveCamera {
            e,
            u,
            v,
            w,
            vertical_field_of_view,
            lens_radius,
            focal_length,
        }
    }
}

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
