use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::math::geometry::ParametricLine;
use crate::math::{NormalizableVector, Point2, Point3, Vector2, Vector3};
use crate::sampling::SamplingPattern;
use crate::traits::{Cos, Half, Min, One, Sin, Sqrt, Zero};
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
    T: Div + Mul + Mul<<T as Div>::Output, Output = T> + Sub<Output = T> + Clone + Copy,
    <T as Div>::Output: Add<Output = <T as Div>::Output>
        + Div<Output = <T as Div>::Output>
        + Half
        + Neg<Output = <T as Div>::Output>
        + Mul<Output = <T as Div>::Output>
        + Sub<Output = <T as Div>::Output>
        + Sqrt<Output = <T as Div>::Output>
        + Zero
        + Copy,
    <T as Mul>::Output: Add<Output = <T as Mul>::Output>
        + Sub<Output = <T as Mul>::Output>
        + Sqrt<Output = T>
        + Zero,
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
    T: Copy + Div + One,
    <T as Div>::Output: Add<Output = <T as Div>::Output>
        + Copy
        + Cos<Output = <T as Div>::Output>
        + Div<Output = <T as Div>::Output>
        + Half
        + Min
        + Mul<Output = <T as Div>::Output>
        + Mul<T, Output = T>
        + One
        + PartialOrd
        + Sin<Output = <T as Div>::Output>
        + Sub<Output = <T as Div>::Output>
        + Sqrt<Output = <T as Div>::Output>
        + Zero,
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
        let r = normalized_p.as_vector().magnitude();

        if r <= One::one() {
            let psi = self.psi * r;
            let sin_alpha = normalized_p.y / r;
            let cos_alpha = normalized_p.x / r;

            let direction = self.u * psi.sin() * cos_alpha + self.v * psi.sin() * sin_alpha
                - self.w * psi.cos();

            vec![ParametricLine::new(self.e, direction * T::one())]
        } else {
            Vec::new()
        }
    }
}
