use std::ops::Neg;

use cg_basics::camera::OrthographicCamera;
use math::geometry::ParametricLine;
use math::{Point2, Point3, Vector2, Vector3};
use traits::Half;
use units::length::Length;

use crate::camera::RaycasterCamera;

impl<T> RaycasterCamera<T> for OrthographicCamera<T>
where
    T: Length + Neg<Output = T>,
    T::ValueType: Half,
{
    fn ray_for(
        &self,
        size: Vector2<T::ValueType>,
        p: Point2<T::ValueType>,
    ) -> Option<ParametricLine<Point3<T>, Vector3<T>>> {
        let aspect_ratio = size.x / size.y;

        let d = -(self.w * T::one());

        let x = (p.x - size.x.half()) / size.x;
        let y = (p.y - size.y.half()) / size.y;

        let o = self.e
            + self.u * T::one() * aspect_ratio * self.scale * x
            + self.v * T::one() * self.scale * y;

        Some(ParametricLine::new(o, d))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use units::length::Meter;

    macro_rules! orthographic_camera_ray_for {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let e = Point3::new(
                    Meter::new(3 as $type),
                    Meter::new(2 as $type),
                    Meter::new(1 as $type),
                );
                let g = Vector3::new(
                    Meter::new(1 as $type),
                    Meter::new(0 as $type),
                    Meter::new(0 as $type),
                );
                let t = Vector3::new(
                    Meter::new(0 as $type),
                    Meter::new(1 as $type),
                    Meter::new(0 as $type),
                );
                let size = Vector2::new(640.0, 480.0);

                let orth = OrthographicCamera::new(e, g, t, 480.0);

                let center = ParametricLine::new(e, g);
                let upper_left = ParametricLine::new(
                    Point3::new(
                        Meter::new(3 as $type),
                        Meter::new(242.0 as $type),
                        Meter::new(-319 as $type),
                    ),
                    g,
                );
                let lower_left = ParametricLine::new(
                    Point3::new(
                        Meter::new(3 as $type),
                        Meter::new(-238.0 as $type),
                        Meter::new(-319 as $type),
                    ),
                    g,
                );
                let lower_right = ParametricLine::new(
                    Point3::new(
                        Meter::new(3 as $type),
                        Meter::new(-238.0 as $type),
                        Meter::new(321 as $type),
                    ),
                    g,
                );
                let upper_right = ParametricLine::new(
                    Point3::new(
                        Meter::new(3 as $type),
                        Meter::new(242.0 as $type),
                        Meter::new(321 as $type),
                    ),
                    g,
                );

                assert_eq!(orth.ray_for(size, Point2::new(320.0, 240.0)), Some(center));
                assert_eq!(
                    orth.ray_for(size, Point2::new(0.0, 480.0)),
                    Some(upper_left)
                );
                assert_eq!(orth.ray_for(size, Point2::new(0.0, 0.0)), Some(lower_left));
                assert_eq!(
                    orth.ray_for(size, Point2::new(640.0, 0.0)),
                    Some(lower_right)
                );
                assert_eq!(
                    orth.ray_for(size, Point2::new(640.0, 480.0)),
                    Some(upper_right)
                );
            }
        };
    }

    orthographic_camera_ray_for! { f32, orthographic_camera_ray_for_f32 }
    orthographic_camera_ray_for! { f64, orthographic_camera_ray_for_f64 }
}
