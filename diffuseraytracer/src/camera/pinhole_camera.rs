use std::ops::{Div, Mul};

use cg_basics::camera::PinholeCamera;
use math::geometry::ParametricLine;
use math::{Point2, Point3, Vector2, Vector3};
use sampling::SamplingPattern;
use traits::{ConvenientNumber, FloatingPoint, Half, Number, SelfMulNumber, Sqrt, Tan};

use crate::camera::RaytracingCamera;

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
