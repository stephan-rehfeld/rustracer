use std::ops::{Div, Mul};

use cg_basics::camera::FisheyeCamera;
use math::geometry::ParametricLine;
use math::{Point2, Point3, Vector2, Vector3};
use sampling::SamplingPattern;
use traits::{ConvenientNumber, Cos, FloatingPoint, Half, Min, Number, One, Sin};

use crate::camera::RaytracingCamera;

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
