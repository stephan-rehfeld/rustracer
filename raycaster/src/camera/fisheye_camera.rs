use cg_basics::camera::FisheyeCamera;
use math::geometry::ParametricLine;
use math::{Point2, Point3, Vector2, Vector3};
use traits::{ConvenientNumber, Cos, FloatingPoint, Half, Min, One, Sin};
use units::angle::{Angle, Radians};
use units::length::Length;

use crate::camera::RaycasterCamera;

impl<T> RaycasterCamera<T> for FisheyeCamera<T>
where
    T: Length,
    T::ValueType: FloatingPoint + ConvenientNumber,
    Radians<T::ValueType>: Angle + Sin<Output = T::ValueType> + Cos<Output = T::ValueType>,
{
    fn ray_for(
        &self,
        size: Vector2<T::ValueType>,
        p: Point2<T::ValueType>,
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
