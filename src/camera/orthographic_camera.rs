use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::math::geometry::ParametricLine;
use crate::math::{Point2, Point3, Vector2, Vector3};
use crate::sampling::SamplingPattern;
use crate::traits::{ConvenientNumber, FloatingPoint, Half, Number, One, SelfMulNumber, Sqrt};

use super::RaytracingCamera;

pub struct OrthographicCamera<T>
where
    T: Div,
{
    e: Point3<T>,
    u: Vector3<<T as Div>::Output>,
    v: Vector3<<T as Div>::Output>,
    w: Vector3<<T as Div>::Output>,
    scale: <T as Div>::Output,
}

impl<T> OrthographicCamera<T>
where
    T: Number<<T as Div>::Output> + SelfMulNumber<<T as Div>::Output>,
    <T as Div>::Output: FloatingPoint + ConvenientNumber,
    <T as Mul>::Output: Number<<T as Div>::Output> + ConvenientNumber + Sqrt<Output = T>,
{
    pub fn new(
        e: Point3<T>,
        g: Vector3<T>,
        t: Vector3<T>,
        scale: <T as Div>::Output,
    ) -> OrthographicCamera<T> {
        let w = -g.normalized();
        let u = Vector3::cross(t, w).normalized();
        let v = Vector3::cross(w, u).normalized();

        OrthographicCamera { e, u, v, w, scale }
    }
}

impl<T> RaytracingCamera<T> for OrthographicCamera<T>
where
    T: Add<Output = T> + Div + Mul<<T as Div>::Output, Output = T> + Neg<Output = T> + One + Copy,
    <T as Div>::Output: Div<Output = <T as Div>::Output>
        + Half
        + Mul<T, Output = T>
        + Mul<<T as Div>::Output, Output = <T as Div>::Output>
        + Sub<Output = <T as Div>::Output>
        + Copy,
{
    fn ray_for(
        &self,
        size: Vector2<<T as Div>::Output>,
        p: Point2<<T as Div>::Output>,
        _pattern: &SamplingPattern<Point2<<T as Div>::Output>>,
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

    use crate::sampling::{RegularPatternGenerator, SamplingPatternSet};

    macro_rules! new_orthographic_camera {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let e = Point3::new(1 as $type, 2 as $type, 3 as $type);
                let g = Vector3::new(0 as $type, 0 as $type, -1 as $type);
                let t = Vector3::new(0 as $type, -1 as $type, 0 as $type);

                let orth = OrthographicCamera::new(e, g, t, 15.0);

                assert_eq!(orth.e, e);
                assert_eq!(
                    orth.u,
                    Vector3::new(-1 as $type, 0 as $type, 0 as $type).normalized()
                );
                assert_eq!(
                    orth.v,
                    Vector3::new(0 as $type, -1 as $type, 0 as $type).normalized()
                );
                assert_eq!(
                    orth.w,
                    Vector3::new(0 as $type, 0 as $type, 1 as $type).normalized()
                );

                assert_eq!(orth.scale, 15.0);
            }
        };
    }

    new_orthographic_camera! { f32, new_orthographic_camera_f32 }
    new_orthographic_camera! { f64, new_orthographic_camera_f64 }

    macro_rules! orthographic_camera_ray_for {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let e = Point3::new(3 as $type, 2 as $type, 1 as $type);
                let g = Vector3::new(1 as $type, 0 as $type, 0 as $type);
                let t = Vector3::new(0 as $type, 1 as $type, 0 as $type);
                let size = Vector2::new(640.0, 480.0);

                let orth = OrthographicCamera::new(e, g, t, 480.0);

                let center = ParametricLine::new(e, g);
                let upper_left =
                    ParametricLine::new(Point3::new(3 as $type, 242.0 as $type, -319 as $type), g);
                let lower_left =
                    ParametricLine::new(Point3::new(3 as $type, -238.0 as $type, -319 as $type), g);
                let lower_right =
                    ParametricLine::new(Point3::new(3 as $type, -238.0 as $type, 321 as $type), g);
                let upper_right =
                    ParametricLine::new(Point3::new(3 as $type, 242.0 as $type, 321 as $type), g);

                let patterns = SamplingPatternSet::<Point2<$type>>::regular_pattern(1, 1);

                assert_eq!(
                    orth.ray_for(size, Point2::new(320.0, 240.0), &patterns[0]),
                    Some(center)
                );
                assert_eq!(
                    orth.ray_for(size, Point2::new(0.0, 480.0), &patterns[0]),
                    Some(upper_left)
                );
                assert_eq!(
                    orth.ray_for(size, Point2::new(0.0, 0.0), &patterns[0]),
                    Some(lower_left)
                );
                assert_eq!(
                    orth.ray_for(size, Point2::new(640.0, 0.0), &patterns[0]),
                    Some(lower_right)
                );
                assert_eq!(
                    orth.ray_for(size, Point2::new(640.0, 480.0), &patterns[0]),
                    Some(upper_right)
                );
            }
        };
    }

    orthographic_camera_ray_for! { f32, orthographic_camera_ray_for_f32 }
    orthographic_camera_ray_for! { f64, orthographic_camera_ray_for_f64 }
}
