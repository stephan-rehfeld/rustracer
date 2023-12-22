use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::math::{NormalizableVector, Point2, Point3, Vector2, Vector3};
use crate::math::geometry::ParametricLine;
use crate::traits::{Half, Sqrt, Tan, Zero};
use crate::units::angle::Radians;

pub trait RaytracingCamera<T> {
    fn size(&self) -> Vector2<T>;
    fn ray_for(&self, p: Point2<T>) -> ParametricLine<Point3<T>, Vector3<T>>;
}

pub struct Orthographic<T> where T: Div {
    e: Point3<T>,
    u: Vector3<T>,
    v: Vector3<T>,
    w: Vector3<T>,
    scale: T,
    size: Vector2<T>,
    aspect_ratio: <T as Div>::Output,
}

impl<T> Orthographic<T> where
    T: Div<Output = T>,
    T: Mul<Output = T>,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Sqrt<Output = T>,
    T: Clone + Copy + Zero,
{
    pub fn new(e: Point3<T>, g: Vector3<T>, t: Vector3<T>, scale: T, size: Vector2<T>) -> Orthographic<T> 
    {
        let w = -g.normalized().as_vector();
        let u = Vector3::cross(t, w).normalized().as_vector();
        let v = Vector3::cross(w, u);

        let aspect_ratio = size.x/size.y;

        Orthographic { e, u, v, w, scale, size, aspect_ratio }
    }
}

impl<T> RaytracingCamera<T> for Orthographic<T> where
    T: Div<Output = T>,
    T: Mul<Vector3<T>, Output=Vector3<T>>,
    T: Mul<Output = T>,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Sqrt<Output = T>,
    T: Half,
    T: Clone + Copy,
{
    fn size(&self) -> Vector2<T> {
        self.size
    }

    fn ray_for(&self, p: Point2<T>) -> ParametricLine<Point3<T>, Vector3<T>> {
        let d = -self.w;

        let x = (p.x - self.size.x.half()) / self.size.x;
        let y = (p.y - self.size.y.half()) / self.size.y;

        let o = self.e + self.aspect_ratio * self.scale * x * self.u + self.scale * y * self.v;

        ParametricLine::new(o, d)
    }
}

pub struct Perspective<T> {
    e: Point3<T>,
    u: Vector3<T>,
    v: Vector3<T>,
    w: Vector3<T>,
    vertical_field_of_view: Radians<T>,
    size: Vector2<T>
}

impl<T> Perspective<T> where
    T: Div<Output = T>,
    T: Mul<Vector3<T>, Output=Vector3<T>>,
    T: Mul<Output = T>,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Sqrt<Output = T>,
    T: Half,
    T: Clone + Copy + Zero,
{
    pub fn new(e: Point3<T>, g: Vector3<T>, t: Vector3<T>, vertical_field_of_view: Radians<T>, size: Vector2<T>) -> Perspective<T> {
        let w = -g.normalized().as_vector();
        let u = Vector3::cross(t, w).normalized().as_vector();
        let v = Vector3::cross(w, u);

        let vertical_field_of_view = vertical_field_of_view.half();

        Perspective { e, u, v, w, vertical_field_of_view, size }
    }
}

impl<T> RaytracingCamera<T> for Perspective<T> where
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
    T: Mul<Vector3<T>, Output=Vector3<T>>,
    T: Div<Output = T>,
    T: Sqrt<Output = T>,
    T: Half,
    T: Tan<Output = T>,
    T: Neg<Output = T>,
    T: Clone + Copy + Zero,
{
    fn size(&self) -> Vector2<T> {
        self.size
    }

    fn ray_for(&self, p: Point2<T> ) -> ParametricLine<Point3<T>, Vector3<T>> {
        let o = self.e;

        let a = -self.w * self.size.y.half()/self.vertical_field_of_view.tan();
        let b = (p.x - self.size.x.half()) * self.u;
        let c = (p.y - self.size.y.half()) * self.v;

        let r = a + b + c; 
        let d = r.normalized().as_vector();

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
                let e = Point3::new( 1 as $type, 2 as $type, 3 as $type );
                let g = Vector3::new( 0 as $type, 0 as $type, -1 as $type );
                let t = Vector3::new( 0 as $type, -1 as $type, 0 as $type );
                let size = Vector2::new( 640.0, 480.0 );

                let orth = Orthographic::new(
                    e,
                    g,
                    t,
                    15.0,
                    size
                );

                assert_eq!(orth.e, e);
                assert_eq!(orth.u, Vector3::new(-1 as $type, 0 as $type, 0 as $type));
                assert_eq!(orth.v, Vector3::new(0 as $type, -1 as $type, 0 as $type));
                assert_eq!(orth.w, Vector3::new(0 as $type, 0 as $type, 1 as $type));

                assert_eq!(orth.scale, 15.0);
                assert_eq!(orth.size, size);
                assert_eq!(orth.aspect_ratio, 640.0/480.0);
            }
        }
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
                let size = Vector2::new( 640.0, 480.0 );

                let orth = Orthographic::new(
                    e,
                    g,
                    t,
                    480.0,
                    size
                );

                let center = ParametricLine::new(e, g);
                let upper_left = ParametricLine::new(Point3::new( 3 as $type, 242.0 as $type ,-319 as $type), g); 
                let lower_left = ParametricLine::new(Point3::new( 3 as $type, -238.0 as $type ,-319 as $type), g); 
                let lower_right = ParametricLine::new(Point3::new( 3 as $type, -238.0 as $type ,321 as $type), g); 
                let upper_right = ParametricLine::new(Point3::new( 3 as $type, 242.0 as $type ,321 as $type), g); 

                assert_eq!(orth.ray_for(Point2::new(320.0, 240.0)), center);
                assert_eq!(orth.ray_for(Point2::new(0.0, 480.0)), upper_left);
                assert_eq!(orth.ray_for(Point2::new(0.0, 0.0)), lower_left);
                assert_eq!(orth.ray_for(Point2::new(640.0, 0.0)), lower_right);
                assert_eq!(orth.ray_for(Point2::new(640.0, 480.0)), upper_right);
            }
        }
    }

    orthographic_ray_for! { f32, orthographic_ray_for_f32 }
    orthographic_ray_for! { f64, orthographic_ray_for_f64 }

    macro_rules! new_perspective {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let e = Point3::new( 1 as $type, 2 as $type, 3 as $type );
                let g = Vector3::new( 0 as $type, 0 as $type, -1 as $type );
                let t = Vector3::new( 0 as $type, -1 as $type, 0 as $type );

                let fov = Degrees::<$type>::new( 90.0 ).to_radians();
                let size = Vector2::new( 640.0, 480.0 );

                let persp = Perspective::new(
                    e,
                    g,
                    t,
                    fov,
                    size
                );

                assert_eq!(persp.e, e);
                assert_eq!(persp.u, Vector3::new(-1 as $type, 0 as $type, 0 as $type));
                assert_eq!(persp.v, Vector3::new(0 as $type, -1 as $type, 0 as $type));
                assert_eq!(persp.w, Vector3::new(0 as $type, 0 as $type, 1 as $type));

                assert_eq!(persp.vertical_field_of_view, fov.half());

                assert_eq!(persp.size, size);
            }
        }
    }

    new_perspective! { f32, new_perspective_f32 }
    new_perspective! { f64, new_perspective_f64 }

    macro_rules! perspective_ray_for {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let e = Point3::new( 1 as $type, 2 as $type, 3 as $type );
                let g = Vector3::new( 0 as $type, 0 as $type, -1 as $type );
                let t = Vector3::new( 0 as $type, 1 as $type, 0 as $type );

                let fov = Degrees::<$type>::new( 90.0 ).to_radians();
                let size = Vector2::new( 640.0, 480.0 );

                let persp = Perspective::new(
                    e,
                    g,
                    t,
                    fov,
                    size,
                );

                let center = ParametricLine::new(e, g);
                let upper_left = ParametricLine::new(e, Vector3::new( -0.6859943405700354, 0.5144957554275266, -0.5144957554275266));
                let lower_left = ParametricLine::new(e, Vector3::new( -0.6859943405700354, -0.5144957554275266, -0.5144957554275266));
                let lower_right = ParametricLine::new(e, Vector3::new( 0.6859943405700354, -0.5144957554275266, -0.5144957554275266));
                let upper_right = ParametricLine::new(e, Vector3::new( 0.6859943405700354, 0.5144957554275266, -0.5144957554275266));

                assert_eq!(persp.ray_for(Point2::new(320.0, 240.0)), center);
                assert_eq!(persp.ray_for(Point2::new(0.0, 480.0)), upper_left);
                assert_eq!(persp.ray_for(Point2::new(0.0, 0.0)), lower_left);
                assert_eq!(persp.ray_for(Point2::new(640.0, 0.0)), lower_right);
                assert_eq!(persp.ray_for(Point2::new(640.0, 480.0)), upper_right);
            }
        }
    }

    perspective_ray_for! { f32, perspective_ray_for_f32 }
    perspective_ray_for! { f64, perspective_ray_for_f64 }
}
