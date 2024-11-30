use std::ops::{Div, Mul};

use crate::math::geometry::ParametricLine;
use crate::math::{Point2, Point3, Vector2, Vector3};
use crate::sampling::SamplingPattern;
use crate::traits::{
    ConvenientNumber, Cos, FloatingPoint, Half, Min, Number, One, SelfMulNumber, Sin, Sqrt,
};
use crate::units::angle::Radians;

use super::RaytracingCamera;

pub struct FisheyeCamera<T>
where
    T: Div,
{
    e: Point3<T>,
    u: Vector3<<T as Div>::Output>,
    v: Vector3<<T as Div>::Output>,
    w: Vector3<<T as Div>::Output>,
    psi: Radians<<T as Div>::Output>,
}

impl<T> FisheyeCamera<T>
where
    T: Number<<T as Div>::Output> + SelfMulNumber<<T as Div>::Output>,
    <T as Div>::Output: FloatingPoint + ConvenientNumber,
    <T as Mul>::Output: Number<<T as Div>::Output> + ConvenientNumber + Sqrt<Output = T>,
{
    pub fn new(
        e: Point3<T>,
        g: Vector3<T>,
        t: Vector3<T>,
        psi: Radians<<T as Div>::Output>,
    ) -> FisheyeCamera<T> {
        let w = -g.normalized();
        let u = Vector3::cross(t, w).normalized();
        let v = Vector3::cross(w, u).normalized();

        FisheyeCamera {
            e,
            u,
            v,
            w,
            psi: psi.half(),
        }
    }
}

impl<T> RaytracingCamera<T> for FisheyeCamera<T>
where
    T: Number<<T as Div>::Output>,
    <T as Div>::Output: FloatingPoint + ConvenientNumber + Mul<T, Output = T>,
{
    fn ray_for(
        &self,
        size: Vector2<<T as Div>::Output>,
        p: Point2<<T as Div>::Output>,
        _pattern: &SamplingPattern<Point2<<T as Div>::Output>>,
    ) -> Option<ParametricLine<Point3<T>, Vector3<T>>> {
        let half_size = size.half();
        let centerd_p = p - half_size;

        let min_dim = half_size.x.min(half_size.y);
        let normalized_p = centerd_p / min_dim;
        let r = normalized_p.as_vector().magnitude();

        if r <= One::one() {
            let psi = self.psi * r;
            let sin_alpha = normalized_p.y / r;
            let cos_alpha = normalized_p.x / r;

            let direction = self.u * psi.sin() * cos_alpha + self.v * psi.sin() * sin_alpha
                - self.w * psi.cos();

            Some(ParametricLine::new(self.e, direction * T::one()))
        } else {
            None
        }
    }
}
