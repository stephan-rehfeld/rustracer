use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::math::geometry::ParametricLine;
use crate::math::{NormalizableVector, Point2, Point3, Vector2, Vector3};
use crate::traits::{Half, One, Sqrt, Tan, Zero};
use crate::units::angle::Radians;

pub trait RaytracingCamera<T>
where
    T: Div,
{
    fn ray_for(
        &self,
        size: Vector2<<T as Div>::Output>,
        p: Point2<<T as Div>::Output>,
    ) -> Vec<ParametricLine<Point3<T>, Vector3<T>>>;
}

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
    T: Div + Mul + Mul<<T as Div>::Output, Output = T> + Sub<Output = T> + Clone + Copy,
    <T as Div>::Output: Add<Output = <T as Div>::Output>
        + Div<Output = <T as Div>::Output>
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
    ) -> Vec<ParametricLine<Point3<T>, Vector3<T>>> {
        let aspect_ratio = size.x / size.y;

        let d = -(self.w * T::one());

        let x = (p.x - size.x.half()) / size.x;
        let y = (p.y - size.y.half()) / size.y;

        let o = self.e
            + self.u * T::one() * aspect_ratio * self.scale * x
            + self.v * T::one() * self.scale * y;

        vec![ParametricLine::new(o, d)]
    }
}

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
    T: Div + Mul + One + Copy,
    <T as Div>::Output: Add<Output = <T as Div>::Output>
        + Div<Output = <T as Div>::Output>
        + Half
        + Mul<T, Output = T>
        + Mul<Output = <T as Div>::Output>
        + Neg<Output = <T as Div>::Output>
        + Sub<Output = <T as Div>::Output>
        + Sqrt<Output = <T as Div>::Output>
        + Tan<Output = <T as Div>::Output>
        + Zero
        + Copy,
    <T as Mul>::Output: Add<Output = <T as Mul>::Output> + Sqrt<Output = T> + Zero,
{
    fn ray_for(
        &self,
        size: Vector2<<T as Div>::Output>,
        p: Point2<<T as Div>::Output>,
    ) -> Vec<ParametricLine<Point3<T>, Vector3<T>>> {
        let o = self.e;

        let a = -self.w * (size.y.half() / self.vertical_field_of_view.tan());
        let b = self.u * (p.x - size.x.half());
        let c = self.v * (p.y - size.y.half());

        let r = a + b + c;
        let d = r.normalized() * T::one();

        vec![ParametricLine::new(o, d)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::traits::ToRadians;
    use crate::units::angle::Degrees;

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

                assert_eq!(orth.ray_for(size, Point2::new(320.0, 240.0)), vec![center]);
                assert_eq!(
                    orth.ray_for(size, Point2::new(0.0, 480.0)),
                    vec![upper_left]
                );
                assert_eq!(orth.ray_for(size, Point2::new(0.0, 0.0)), vec![lower_left]);
                assert_eq!(
                    orth.ray_for(size, Point2::new(640.0, 0.0)),
                    vec![lower_right]
                );
                assert_eq!(
                    orth.ray_for(size, Point2::new(640.0, 480.0)),
                    vec![upper_right]
                );
            }
        };
    }

    orthographic_camera_ray_for! { f32, orthographic_camera_ray_for_f32 }
    orthographic_camera_ray_for! { f64, orthographic_camera_ray_for_f64 }

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

                assert_eq!(persp.ray_for(size, Point2::new(320.0, 240.0)), vec![center]);
                assert_eq!(
                    persp.ray_for(size, Point2::new(0.0, 480.0)),
                    vec![upper_left]
                );
                assert_eq!(persp.ray_for(size, Point2::new(0.0, 0.0)), vec![lower_left]);
                assert_eq!(
                    persp.ray_for(size, Point2::new(640.0, 0.0)),
                    vec![lower_right]
                );
                assert_eq!(
                    persp.ray_for(size, Point2::new(640.0, 480.0)),
                    vec![upper_right]
                );
            }
        };
    }

    pinhole_camera_ray_for! { f32, pinhole_camera_ray_for_f32 }
    pinhole_camera_ray_for! { f64, pinhole_camera_ray_for_f64 }
}
