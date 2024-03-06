use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::math::geometry::ParametricLine;
use crate::math::{Normal3, NormalizableVector, Point2, Point3, Vector2, Vector3};
use crate::traits::{Half, One, Sqrt, Tan, Zero};
use crate::units::angle::Radians;

pub trait RaytracingCamera<T>
where
    T: Div,
{
    fn size(&self) -> Vector2<<T as Div>::Output>;
    fn ray_for(&self, p: Point2<<T as Div>::Output>) -> ParametricLine<Point3<T>, Vector3<T>>;
}

pub struct Orthographic<T>
where
    T: Div,
{
    e: Point3<T>,
    u: Normal3<<T as Div>::Output>,
    v: Normal3<<T as Div>::Output>,
    w: Normal3<<T as Div>::Output>,
    scale: <T as Div>::Output,
    size: Vector2<<T as Div>::Output>,
    aspect_ratio: <T as Div>::Output,
}

impl<T> Orthographic<T>
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
        size: Vector2<<T as Div>::Output>,
    ) -> Orthographic<T> {
        let w = -g.normalized();
        let u = Vector3::cross(t, w.as_vector()).normalized();
        let v = Vector3::cross(w.as_vector(), u.as_vector()).normalized();

        let aspect_ratio = size.x / size.y;

        Orthographic {
            e,
            u,
            v,
            w,
            scale,
            size,
            aspect_ratio,
        }
    }
}

impl<T> RaytracingCamera<T> for Orthographic<T>
where
    T: Add<Output = T>
        + Div
        + Mul<<T as Div>::Output, Output = T>
        + Neg<Output = T>
        + One
        + Mul<Normal3<<T as Div>::Output>, Output = Vector3<T>>
        + Copy,
    <T as Div>::Output: Div<Output = <T as Div>::Output>
        + Half
        + Mul<T, Output = T>
        + Mul<<T as Div>::Output, Output = <T as Div>::Output>
        + Sub<Output = <T as Div>::Output>
        + Copy,
{
    fn size(&self) -> Vector2<<T as Div>::Output> {
        self.size
    }

    fn ray_for(&self, p: Point2<<T as Div>::Output>) -> ParametricLine<Point3<T>, Vector3<T>> {
        let d = -(self.w.as_vector() * T::one());

        let x = (p.x - self.size.x.half()) / self.size.x;
        let y = (p.y - self.size.y.half()) / self.size.y;

        let o = self.e
            + T::one() * self.aspect_ratio * self.scale * x * self.u
            + T::one() * self.scale * y * self.v;

        ParametricLine::new(o, d)
    }
}

pub struct Perspective<T>
where
    T: Div,
{
    e: Point3<T>,
    u: Normal3<<T as Div>::Output>,
    v: Normal3<<T as Div>::Output>,
    w: Normal3<<T as Div>::Output>,
    vertical_field_of_view: Radians<<T as Div>::Output>,
    size: Vector2<<T as Div>::Output>,
}

impl<T> Perspective<T>
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
        size: Vector2<<T as Div>::Output>,
    ) -> Perspective<T> {
        let w = -g.normalized();
        let u = Vector3::cross(t, w.as_vector()).normalized();
        let v = Vector3::cross(w.as_vector(), u.as_vector()).normalized();

        let vertical_field_of_view = vertical_field_of_view.half();

        Perspective {
            e,
            u,
            v,
            w,
            vertical_field_of_view,
            size,
        }
    }
}

impl<T> RaytracingCamera<T> for Perspective<T>
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
    fn size(&self) -> Vector2<<T as Div>::Output> {
        self.size
    }

    fn ray_for(&self, p: Point2<<T as Div>::Output>) -> ParametricLine<Point3<T>, Vector3<T>> {
        let o = self.e;

        let a = -self.w * (self.size.y.half() / self.vertical_field_of_view.tan());
        let b = self.u * (p.x - self.size.x.half());
        let c = self.v * (p.y - self.size.y.half());

        let r = a + b + c;
        let d = r.normalized() * T::one();

        ParametricLine::new(o, d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::traits::ToRadians;
    use crate::units::angle::Degrees;

    macro_rules! new_orthographic {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let e = Point3::new(1 as $type, 2 as $type, 3 as $type);
                let g = Vector3::new(0 as $type, 0 as $type, -1 as $type);
                let t = Vector3::new(0 as $type, -1 as $type, 0 as $type);
                let size = Vector2::new(640.0, 480.0);

                let orth = Orthographic::new(e, g, t, 15.0, size);

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
                assert_eq!(orth.size, size);
                assert_eq!(orth.aspect_ratio, 640.0 / 480.0);
            }
        };
    }

    new_orthographic! { f32, new_orthographic_f32 }
    new_orthographic! { f64, new_orthographic_f64 }

    macro_rules! orthographic_ray_for {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let e = Point3::new(3 as $type, 2 as $type, 1 as $type);
                let g = Vector3::new(1 as $type, 0 as $type, 0 as $type);
                let t = Vector3::new(0 as $type, 1 as $type, 0 as $type);
                let size = Vector2::new(640.0, 480.0);

                let orth = Orthographic::new(e, g, t, 480.0, size);

                let center = ParametricLine::new(e, g);
                let upper_left =
                    ParametricLine::new(Point3::new(3 as $type, 242.0 as $type, -319 as $type), g);
                let lower_left =
                    ParametricLine::new(Point3::new(3 as $type, -238.0 as $type, -319 as $type), g);
                let lower_right =
                    ParametricLine::new(Point3::new(3 as $type, -238.0 as $type, 321 as $type), g);
                let upper_right =
                    ParametricLine::new(Point3::new(3 as $type, 242.0 as $type, 321 as $type), g);

                assert_eq!(orth.ray_for(Point2::new(320.0, 240.0)), center);
                assert_eq!(orth.ray_for(Point2::new(0.0, 480.0)), upper_left);
                assert_eq!(orth.ray_for(Point2::new(0.0, 0.0)), lower_left);
                assert_eq!(orth.ray_for(Point2::new(640.0, 0.0)), lower_right);
                assert_eq!(orth.ray_for(Point2::new(640.0, 480.0)), upper_right);
            }
        };
    }

    orthographic_ray_for! { f32, orthographic_ray_for_f32 }
    orthographic_ray_for! { f64, orthographic_ray_for_f64 }

    macro_rules! new_perspective {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let e = Point3::new(1 as $type, 2 as $type, 3 as $type);
                let g = Vector3::new(0 as $type, 0 as $type, -1 as $type);
                let t = Vector3::new(0 as $type, -1 as $type, 0 as $type);

                let fov = Degrees::<$type>::new(90.0).to_radians();
                let size = Vector2::new(640.0, 480.0);

                let persp = Perspective::new(e, g, t, fov, size);

                assert_eq!(persp.e, e);
                assert_eq!(persp.u, Normal3::new(-1 as $type, 0 as $type, 0 as $type));
                assert_eq!(persp.v, Normal3::new(0 as $type, -1 as $type, 0 as $type));
                assert_eq!(persp.w, Normal3::new(0 as $type, 0 as $type, 1 as $type));

                assert_eq!(persp.vertical_field_of_view, fov.half());

                assert_eq!(persp.size, size);
            }
        };
    }

    new_perspective! { f32, new_perspective_f32 }
    new_perspective! { f64, new_perspective_f64 }

    macro_rules! perspective_ray_for {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let e = Point3::new(1 as $type, 2 as $type, 3 as $type);
                let g = Vector3::new(0 as $type, 0 as $type, -1 as $type);
                let t = Vector3::new(0 as $type, 1 as $type, 0 as $type);

                let fov = Degrees::<$type>::new(90.0).to_radians();
                let size = Vector2::new(640.0, 480.0);

                let persp = Perspective::new(e, g, t, fov, size);

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

                assert_eq!(persp.ray_for(Point2::new(320.0, 240.0)), center);
                assert_eq!(persp.ray_for(Point2::new(0.0, 480.0)), upper_left);
                assert_eq!(persp.ray_for(Point2::new(0.0, 0.0)), lower_left);
                assert_eq!(persp.ray_for(Point2::new(640.0, 0.0)), lower_right);
                assert_eq!(persp.ray_for(Point2::new(640.0, 480.0)), upper_right);
            }
        };
    }

    perspective_ray_for! { f32, perspective_ray_for_f32 }
    perspective_ray_for! { f64, perspective_ray_for_f64 }
}
