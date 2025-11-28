use crate::camera::RaycasterCamera;

use cg_basics::camera::SphericalCamera;
use math::geometry::ParametricLine;
use math::{Point2, Point3, Vector2, Vector3};
use traits::{ConvenientNumber, Cos, FloatingPoint, Min, Half, Sin};
use units::angle::{Angle, Radians};
use units::length::Length;

impl<T> RaycasterCamera<T> for SphericalCamera<T>
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

        let ratio = size.x / size.y;

        let horizontal_field_of_view = self.vertical_field_of_view * ratio;

        let lambda = horizontal_field_of_view * normalized_p.x;
        let psi = self.vertical_field_of_view * normalized_p.y;

        let phi = Radians::half_turn() - lambda;
        let theta = Radians::quarter_turn() - psi;

        let direction = self.u * theta.sin() * phi.sin()
            + self.v * theta.cos()
            + self.w * theta.sin() * phi.cos();

        Some(ParametricLine::new(self.e, direction * T::one()))
    }
}
