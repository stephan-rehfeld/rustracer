use std::ops::{Div, Mul};

use math::geometry::ParametricLine;
use math::{Point2, Point3, Vector2, Vector3};
use sampling::SamplingPattern;
use traits::{ConvenientNumber, FloatingPoint, Half, Number, SelfMulNumber, Sqrt, Tan};
use units::angle::Radians;

use super::RaytracingCamera;

pub struct PinholeCamera<T>
where
    T: Div,
{
    e: Point3<T>,
    u: Vector3<<T as Div>::Output>,
    v: Vector3<<T as Div>::Output>,
    w: Vector3<<T as Div>::Output>,
    vertical_field_of_view: Radians<<T as Div>::Output>,
}

impl<T> PinholeCamera<T>
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
    ) -> PinholeCamera<T> {
        let w = -g.normalized();
        let u = Vector3::cross(t, w).normalized();
        let v = Vector3::cross(w, u).normalized();

        let vertical_field_of_view = vertical_field_of_view.half();

        PinholeCamera {
            e,
            u,
            v,
            w,
            vertical_field_of_view,
        }
    }
}

impl<T> RaytracingCamera<T> for PinholeCamera<T>
where
    T: SelfMulNumber<<T as Div>::Output> + ConvenientNumber,
    <T as Div>::Output: FloatingPoint + ConvenientNumber + Mul<T, Output = T>,
    <T as Mul>::Output: Number<<T as Div>::Output> + ConvenientNumber + Sqrt<Output = T>,
{
    fn ray_for(
        &self,
        size: Vector2<<T as Div>::Output>,
        p: Point2<<T as Div>::Output>,
        _pattern: &SamplingPattern<Point2<<T as Div>::Output>>,
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

    use sampling::{RegularPatternGenerator, SamplingPatternSet};
    use traits::ToRadians;
    use units::angle::Degrees;

    macro_rules! new_pinhole_camera {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let e = Point3::new(1 as $type, 2 as $type, 3 as $type);
                let g = Vector3::new(0 as $type, 0 as $type, -1 as $type);
                let t = Vector3::new(0 as $type, -1 as $type, 0 as $type);

                let fov = Degrees::<$type>::new(90.0).to_radians();

                let persp = PinholeCamera::new(e, g, t, fov);

                assert_eq!(persp.e, e);
                assert_eq!(persp.u, Vector3::new(-1 as $type, 0 as $type, 0 as $type));
                assert_eq!(persp.v, Vector3::new(0 as $type, -1 as $type, 0 as $type));
                assert_eq!(persp.w, Vector3::new(0 as $type, 0 as $type, 1 as $type));

                assert_eq!(persp.vertical_field_of_view, fov.half());
            }
        };
    }

    new_pinhole_camera! { f32, new_pinhole_camera_f32 }
    new_pinhole_camera! { f64, new_pinhole_camera_f64 }

    macro_rules! pinhole_camera_ray_for {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let e = Point3::new(1 as $type, 2 as $type, 3 as $type);
                let g = Vector3::new(0 as $type, 0 as $type, -1 as $type);
                let t = Vector3::new(0 as $type, 1 as $type, 0 as $type);

                let fov = Degrees::<$type>::new(90.0).to_radians();
                let size = Vector2::new(640.0, 480.0);

                let persp = PinholeCamera::new(e, g, t, fov);

                let center = ParametricLine::new(e, g);
                let upper_left = ParametricLine::new(
                    e,
                    Vector3::new(-0.6859943405700354, 0.5144957554275266, -0.5144957554275266),
                );
                let lower_left = ParametricLine::new(
                    e,
                    Vector3::new(
                        -0.6859943405700354,
                        -0.5144957554275266,
                        -0.5144957554275266,
                    ),
                );
                let lower_right = ParametricLine::new(
                    e,
                    Vector3::new(0.6859943405700354, -0.5144957554275266, -0.5144957554275266),
                );
                let upper_right = ParametricLine::new(
                    e,
                    Vector3::new(0.6859943405700354, 0.5144957554275266, -0.5144957554275266),
                );

                let patterns = SamplingPatternSet::<Point2<$type>>::regular_pattern(1, 1);

                assert_eq!(
                    persp.ray_for(size, Point2::new(320.0, 240.0), &patterns[0]),
                    Some(center)
                );
                assert_eq!(
                    persp.ray_for(size, Point2::new(0.0, 480.0), &patterns[0]),
                    Some(upper_left)
                );
                assert_eq!(
                    persp.ray_for(size, Point2::new(0.0, 0.0), &patterns[0]),
                    Some(lower_left)
                );
                assert_eq!(
                    persp.ray_for(size, Point2::new(640.0, 0.0), &patterns[0]),
                    Some(lower_right)
                );
                assert_eq!(
                    persp.ray_for(size, Point2::new(640.0, 480.0), &patterns[0]),
                    Some(upper_right)
                );
            }
        };
    }

    pinhole_camera_ray_for! { f32, pinhole_camera_ray_for_f32 }
    pinhole_camera_ray_for! { f64, pinhole_camera_ray_for_f64 }
}
