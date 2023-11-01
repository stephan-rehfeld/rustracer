use std::ops;

use crate::math::NormalizableVector;
use crate::math::Point3;
use crate::math::Vector3;
use crate::math::geometry::ParametricLine;
use crate::traits;
use crate::traits::Half;
use crate::traits::Tan;
use crate::traits::Zero;
use crate::units::angle;

pub trait RaytracingCamera<T> {
    fn ray_for(&self, x: T, y: T) -> ParametricLine<Point3<T>, Vector3<T>>;
}

pub struct Orthographic<T> where T: ops::Div {
    e: Point3<T>,
    u: Vector3<T>,
    v: Vector3<T>,
    w: Vector3<T>,
    scale: T,
    width: T,
    height: T,
    aspect_ratio: <T as ops::Div>::Output,
}

impl<T> Orthographic<T> where
    T: ops::Div<Output = T>,
    T: ops::Mul<Output = T>,
    T: ops::Add<Output = T>,
    T: ops::Sub<Output = T>,
    T: ops::Neg<Output = T>,
    T: traits::Sqrt<Output = T>,
    T: Clone + Copy + Zero,
{
    pub fn new(e: Point3<T>, g: Vector3<T>, t: Vector3<T>, scale: T, width: T, height: T) -> Orthographic<T> 
    {
        let w = -g.normalized().as_vector();
        let u = Vector3::cross(t, w).normalized().as_vector();
        let v = Vector3::cross(w, u);

        let aspect_ratio = width/height;

        Orthographic { e, u, v, w, scale, width, height, aspect_ratio }
    }
}

impl<T> RaytracingCamera<T> for Orthographic<T> where
    T: ops::Div<Output = T>,
    T: ops::Mul<Vector3<T>, Output=Vector3<T>>,
    T: ops::Mul<Output = T>,
    T: ops::Add<Output = T>,
    T: ops::Sub<Output = T>,
    T: ops::Neg<Output = T>,
    T: traits::Sqrt<Output = T>,
    T: traits::Half,
    T: Clone + Copy,
{
    fn ray_for(&self, x: T, y: T) -> ParametricLine<Point3<T>, Vector3<T>> {
        let d = -self.w;

        let x = (x - self.width.half()) / self.width;
        let y = (y - self.height.half()) / self.height;

        let o = self.e + self.aspect_ratio * self.scale * x * self.u + self.scale * y * self.v;

        ParametricLine::new(o, d)
    }
}

pub struct Perspective<T> {
    e: Point3<T>,
    u: Vector3<T>,
    v: Vector3<T>,
    w: Vector3<T>,
    vertical_field_of_view: angle::Radians<T>,
    width: T,
    height: T 
}

impl<T> Perspective<T> where
    T: ops::Div<Output = T>,
    T: ops::Mul<Vector3<T>, Output=Vector3<T>>,
    T: ops::Mul<Output = T>,
    T: ops::Add<Output = T>,
    T: ops::Sub<Output = T>,
    T: ops::Neg<Output = T>,
    T: traits::Sqrt<Output = T>,
    T: traits::Half,
    T: Clone + Copy + Zero,
{
    pub fn new(e: Point3<T>, g: Vector3<T>, t: Vector3<T>, vertical_field_of_view: angle::Radians<T>, width: T, height: T) -> Perspective<T> {
        let w = -g.normalized().as_vector();
        let u = Vector3::cross(t, w).normalized().as_vector();
        let v = Vector3::cross(w, u);

        let vertical_field_of_view = vertical_field_of_view.half();

        Perspective { e, u, v, w, vertical_field_of_view, width, height }
    }
}

impl<T> RaytracingCamera<T> for Perspective<T> where
    T: ops::Add<Output = T>,
    T: ops::Sub<Output = T>,
    T: ops::Mul<Output = T>,
    T: ops::Mul<Vector3<T>, Output=Vector3<T>>,
    T: ops::Div<Output = T>,
    T: traits::Sqrt<Output = T>,
    T: traits::Half,
    T: traits::Tan<Output = T>,
    T: ops::Neg<Output = T>,
    T: Clone + Copy + Zero,
{
    fn ray_for(&self, x: T, y: T ) -> ParametricLine<Point3<T>, Vector3<T>> {
        let o = self.e;

        let a = -self.w * self.height.half()/self.vertical_field_of_view.tan(); 
        let b = (x - self.width.half()) * self.u;
        let c = (y - self.height.half()) * self.v;

        let r = a + b + c; 
        let d = r.normalized().as_vector();

        ParametricLine::new(o, d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::traits::ToRadians;

    macro_rules! new_orthographic {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let e = Point3::new( 1 as $type, 2 as $type, 3 as $type );
                let g = Vector3::new( 0 as $type, 0 as $type, -1 as $type );
                let t = Vector3::new( 0 as $type, -1 as $type, 0 as $type );
                let orth = Orthographic::new(
                    e,
                    g,
                    t,
                    15.0,
                    640.0,
                    480.0
                );

                assert_eq!(orth.e, e);
                assert_eq!(orth.u, Vector3::new(-1 as $type, 0 as $type, 0 as $type));
                assert_eq!(orth.v, Vector3::new(0 as $type, -1 as $type, 0 as $type));
                assert_eq!(orth.w, Vector3::new(0 as $type, 0 as $type, 1 as $type));

                assert_eq!(orth.scale, 15.0);
                assert_eq!(orth.width, 640.0);
                assert_eq!(orth.height, 480.0);
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

                let orth = Orthographic::new(
                    e,
                    g,
                    t,
                    480.0,
                    640.0,
                    480.0
                );

                let center = ParametricLine::new(e, g);
                let upper_left = ParametricLine::new(Point3::new( 3 as $type, 242.0 as $type ,-319 as $type), g); 
                let lower_left = ParametricLine::new(Point3::new( 3 as $type, -238.0 as $type ,-319 as $type), g); 
                let lower_right = ParametricLine::new(Point3::new( 3 as $type, -238.0 as $type ,321 as $type), g); 
                let upper_right = ParametricLine::new(Point3::new( 3 as $type, 242.0 as $type ,321 as $type), g); 

                assert_eq!(orth.ray_for(320.0, 240.0), center);
                assert_eq!(orth.ray_for(0.0, 480.0), upper_left);
                assert_eq!(orth.ray_for(0.0, 0.0), lower_left);
                assert_eq!(orth.ray_for(640.0, 0.0), lower_right);
                assert_eq!(orth.ray_for(640.0, 480.0), upper_right);
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

                let fov = angle::Degrees::<$type>::new( 90.0 ).to_radians();

                let persp = Perspective::new(
                    e,
                    g,
                    t,
                    fov,
                    640.0,
                    480.0
                );

                assert_eq!(persp.e, e);
                assert_eq!(persp.u, Vector3::new(-1 as $type, 0 as $type, 0 as $type));
                assert_eq!(persp.v, Vector3::new(0 as $type, -1 as $type, 0 as $type));
                assert_eq!(persp.w, Vector3::new(0 as $type, 0 as $type, 1 as $type));

                assert_eq!(persp.vertical_field_of_view, fov.half());

                assert_eq!(persp.width, 640.0);
                assert_eq!(persp.height, 480.0);
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

                let fov = angle::Degrees::<$type>::new( 90.0 ).to_radians();

                let persp = Perspective::new(
                    e,
                    g,
                    t,
                    fov,
                    640.0,
                    480.0
                );

                let center = ParametricLine::new(e, g);
                let upper_left = ParametricLine::new(e, Vector3::new( -0.6859943405700354, 0.5144957554275266, -0.5144957554275266));
                let lower_left = ParametricLine::new(e, Vector3::new( -0.6859943405700354, -0.5144957554275266, -0.5144957554275266));
                let lower_right = ParametricLine::new(e, Vector3::new( 0.6859943405700354, -0.5144957554275266, -0.5144957554275266));
                let upper_right = ParametricLine::new(e, Vector3::new( 0.6859943405700354, 0.5144957554275266, -0.5144957554275266));

                assert_eq!(persp.ray_for(320.0, 240.0), center);
                assert_eq!(persp.ray_for(0.0, 480.0), upper_left);
                assert_eq!(persp.ray_for(0.0, 0.0), lower_left);
                assert_eq!(persp.ray_for(640.0, 0.0), lower_right);
                assert_eq!(persp.ray_for(640.0, 480.0), upper_right);
            }
        }
    }

    perspective_ray_for! { f32, perspective_ray_for_f32 }
    perspective_ray_for! { f64, perspective_ray_for_f64 }
}
