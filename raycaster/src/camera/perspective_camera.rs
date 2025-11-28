use cg_basics::camera::PerspectiveCamera;
use math::geometry::ParametricLine;
use math::{Point2, Point3, Vector2, Vector3};
use traits::{ConvenientNumber, FloatingPoint, Half, Tan};
use units::length::Length;

use crate::camera::RaycasterCamera;

impl<T> RaycasterCamera<T> for PerspectiveCamera<T>
where
    T: Length,
    T::ValueType: ConvenientNumber + FloatingPoint,
{
    fn ray_for(
        &self,
        size: Vector2<T::ValueType>,
        p: Point2<T::ValueType>,
    ) -> Option<ParametricLine<Point3<T>, Vector3<T>>> {
        let o = self.e;

        let a = -self.w * (size.y.half() / self.vertical_field_of_view.tan());
        let b = self.u * (p.x - size.x.half());
        let c = self.v * (p.y - size.y.half());

        let r = a + b + c;
        let d = r.normalized() * T::one();

        Some(ParametricLine::new(o, d))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use traits::ToRadians;
    use units::angle::Degrees;
    use units::length::Meter;

    macro_rules! perspective_camera_ray_for {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let e = Point3::new(
                    Meter::new(1 as $type),
                    Meter::new(2 as $type),
                    Meter::new(3 as $type),
                );
                let g = Vector3::new(
                    Meter::new(0 as $type),
                    Meter::new(0 as $type),
                    Meter::new(-1 as $type),
                );
                let t = Vector3::new(
                    Meter::new(0 as $type),
                    Meter::new(1 as $type),
                    Meter::new(0 as $type),
                );

                let fov = Degrees::<$type>::new(90.0).to_radians();
                let size = Vector2::new(640.0, 480.0);

                let persp = PerspectiveCamera::new(
                    e,
                    g,
                    t,
                    fov,
                    Meter::new(0 as $type),
                    Meter::new(0 as $type),
                );

                let center = ParametricLine::new(e, g);
                let upper_left = ParametricLine::new(
                    e,
                    Vector3::new(
                        Meter::new(-0.6859943405700354),
                        Meter::new(0.5144957554275266),
                        Meter::new(-0.5144957554275266),
                    ),
                );
                let lower_left = ParametricLine::new(
                    e,
                    Vector3::new(
                        Meter::new(-0.6859943405700354),
                        Meter::new(-0.5144957554275266),
                        Meter::new(-0.5144957554275266),
                    ),
                );
                let lower_right = ParametricLine::new(
                    e,
                    Vector3::new(
                        Meter::new(0.6859943405700354),
                        Meter::new(-0.5144957554275266),
                        Meter::new(-0.5144957554275266),
                    ),
                );
                let upper_right = ParametricLine::new(
                    e,
                    Vector3::new(
                        Meter::new(0.6859943405700354),
                        Meter::new(0.5144957554275266),
                        Meter::new(-0.5144957554275266),
                    ),
                );

                assert_eq!(persp.ray_for(size, Point2::new(320.0, 240.0)), Some(center));
                assert_eq!(
                    persp.ray_for(size, Point2::new(0.0, 480.0)),
                    Some(upper_left)
                );
                assert_eq!(persp.ray_for(size, Point2::new(0.0, 0.0)), Some(lower_left));
                assert_eq!(
                    persp.ray_for(size, Point2::new(640.0, 0.0)),
                    Some(lower_right)
                );
                assert_eq!(
                    persp.ray_for(size, Point2::new(640.0, 480.0)),
                    Some(upper_right)
                );
            }
        };
    }

    perspective_camera_ray_for! { f32, pinhole_camera_ray_for_f32 }
    perspective_camera_ray_for! { f64, pinhole_camera_ray_for_f64 }
}
