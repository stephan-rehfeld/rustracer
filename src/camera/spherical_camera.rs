use std::ops::{Add, Div, Mul, Sub};

use crate::math::geometry::ParametricLine;
use crate::math::{Point2, Point3, Vector2, Vector3};
use crate::sampling::SamplingPattern;
use crate::traits::{
    ConvenientNumber, Cos, FloatingPoint, Half, Min, Number, SelfMulNumber, Sin, Sqrt, Zero,
};
use crate::units::angle::{Angle, Radians};

use super::RaytracingCamera;

pub struct SphericalCamera<T>
where
    T: Div,
{
    e: Point3<T>,
    u: Vector3<<T as Div>::Output>,
    v: Vector3<<T as Div>::Output>,
    w: Vector3<<T as Div>::Output>,
    vertical_field_of_view: Radians<<T as Div>::Output>,
}

impl<T> SphericalCamera<T>
where
    T: Number<<T as Div>::Output> + SelfMulNumber<<T as Div>::Output>,
    <T as Div>::Output: FloatingPoint<<T as Div>::Output> + ConvenientNumber,
    <T as Mul>::Output: Add<Output = <T as Mul>::Output>
        + Sub<Output = <T as Mul>::Output>
        + Sqrt<Output = T>
        + Zero,
{
    pub fn new(
        e: Point3<T>,
        g: Vector3<T>,
        t: Vector3<T>,
        vertical_field_of_view: Radians<<T as Div>::Output>,
    ) -> SphericalCamera<T> {
        let w = -g.normalized();
        let u = Vector3::cross(t, w).normalized();
        let v = Vector3::cross(w, u).normalized();

        let vertical_field_of_view = vertical_field_of_view.half();

        SphericalCamera {
            e,
            u,
            v,
            w,
            vertical_field_of_view,
        }
    }
}

impl<T> RaytracingCamera<T> for SphericalCamera<T>
where
    T: Number<<T as Div>::Output> + ConvenientNumber,
    <T as Div>::Output: FloatingPoint + ConvenientNumber + Mul<T, Output = T>,
    Radians<<T as Div>::Output>:
        Angle + Cos<Output = <T as Div>::Output> + Sin<Output = <T as Div>::Output>,
{
    fn ray_for(
        &self,
        size: Vector2<<T as Div>::Output>,
        p: Point2<<T as Div>::Output>,
        _pattern: &SamplingPattern<Point2<<T as Div>::Output>>,
    ) -> Vec<ParametricLine<Point3<T>, Vector3<T>>> {
        let half_size = size.half();
        let centerd_p = p - half_size;

        let min_dim = half_size.x.min(half_size.y);
        let normalized_p = centerd_p / min_dim;

        let ratio = size.x / size.y;

        let horizontal_field_of_view = self.vertical_field_of_view * ratio;

        let lambda = horizontal_field_of_view * normalized_p.x;
        let psi = self.vertical_field_of_view * normalized_p.y;

        let phi = Radians::half_turn() - lambda;
        let theta = Radians::quarter_turn() - psi;

        let direction = self.u * theta.sin() * phi.sin()
            + self.v * theta.cos()
            + self.w * theta.sin() * phi.cos();

        vec![ParametricLine::new(self.e, direction * T::one())]
    }
}
