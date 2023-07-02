use std::ops;

use crate::math::Vector3;
use crate::math::Point3;

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct ParametricLine<T> {
    origin: Point3<T>,
    direction: Vector3<T>
}

impl<T> ParametricLine<T> {
    pub fn new( origin: Point3<T>, direction: Vector3<T> ) -> ParametricLine<T> {
        ParametricLine { origin, direction }
    }

    pub fn at(&self, t: T) -> Point3<T>
    where
        T: Clone,
        T: Copy,
        T: ops::Add<Output = T>,
        T: ops::Mul<Output = T>,
    {
       self.origin + self.direction * t
    }
}

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct Plane3<T> {
    anchor: Point3<T>,
    normal: Vector3<T>
}

impl<T> Plane3<T> {
    pub fn new(anchor: Point3<T>, normal: Vector3<T>) -> Plane3<T> {
        Plane3 { anchor, normal }
    }
}

pub trait Intersect<T> {
    type Output;

    fn intersect(self, other: T) -> Vec<Self::Output>;
}

impl<T: ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T> + ops::Div<Output = T> + std::cmp::PartialEq<T> + Default + Clone + Copy> Intersect<Plane3<T>> for ParametricLine<T> {
    type Output = T;

    fn intersect(self, plane: Plane3<T>) -> Vec<Self::Output> {
        if self.direction * plane.normal == Default::default() {
            Vec::new()
        } else {
            vec![((plane.anchor - self.origin) * plane.normal) / (self.direction * plane.normal)]
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere<T> {
    center: Point3<T>,
    radius: T,
}

impl<T> Sphere<T> {
    pub fn new(center: Point3<T>, radius: T) -> Sphere<T> {
        Sphere { center, radius }
    }
}

impl Intersect<Sphere<f32>> for ParametricLine<f32> {
    type Output = f32;

    fn intersect(self, sphere: Sphere<f32>) -> Vec<Self::Output> {
        let a = self.direction * self.direction;
        let b = self.direction * (2.0 * (self.origin - sphere.center));
        let c = (self.origin - sphere.center) * (self.origin - sphere.center) - sphere.radius * sphere.radius;

        let helper = b * b - 4.0 * a * c;
        
        if helper < 0.0 {
            Vec::new()
        } else if helper == 0.0 {
            vec![ (-b / (2.0 * a) ) ]
        } else {
            let helper = helper.sqrt();

            vec![ ((-b - helper) / (2.0 * a) ), ((-b + helper) / (2.0 * a) ) ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_parametric_line_f32() {
        let origin = Point3::new( 1.0f32, 2.0f32, 3.0f32 );
        let direction = Vector3::new( 4.0f32, 5.0f32, 6.0f32 );

        let ray = ParametricLine::new(origin, direction);

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn new_parametric_line_f64() {
        let origin = Point3::new( 1.0f64, 2.0f64, 3.0f64 );
        let direction = Vector3::new( 4.0f64, 5.0f64, 6.0f64);

        let ray = ParametricLine::new(origin, direction);

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn parametric_line_at_f32() {
        let origin = Point3::new( 1.0f32, 2.0f32, 3.0f32 );
        let direction = Vector3::new( 4.0f32, 5.0f32, 6.0f32);

        let t = 10.0f32;

        let ray = ParametricLine::new(origin, direction);

        assert_eq!(ray.at(t), origin + direction * t);
    }

    #[test]
    fn parametric_line_at_f64() {
        let origin = Point3::new( 1.0f64, 2.0f64, 3.0f64 );
        let direction = Vector3::new( 4.0f64, 5.0f64, 6.0f64);

        let t = 10.0f64;

        let ray = ParametricLine::new(origin, direction);

        assert_eq!(ray.at(t), origin + direction * t);
    }

    #[test]
    fn new_plane3_f32() {
        let anchor = Point3::new( 1.0f32, 2.0f32, 3.0f32 );
        let normal = Vector3::new( 4.0f32, 5.0f32, 6.0f32 );

        let plane = Plane3::new(anchor, normal);

        assert_eq!(plane.anchor, anchor);
        assert_eq!(plane.normal, normal);
    }

    #[test]
    fn new_plane3_f64() {
        let anchor = Point3::new( 1.0f64, 2.0f64, 3.0f64 );
        let normal = Vector3::new( 4.0f64, 5.0f64, 6.0f64);

        let plane = Plane3::new(anchor, normal);

        assert_eq!(plane.anchor, anchor);
        assert_eq!(plane.normal, normal);
    }

    #[test]
    fn parametric_line_intersect_plane_f64() {
        let ray1 = ParametricLine::new(
            Point3::new(0.0f64, 1.0f64, 0.0f64),
            Vector3::new(0.0f64, 0.0f64, -1.0f64)
        );

        let plane = Plane3::new(
            Point3::new(0.0f64, 0.0f64, 0.0f64),
            Vector3::new(0.0f64, 1.0f64, 0.0f64)
        );

        assert_eq!(ray1.intersect(plane), Vec::new());

        let ray2 = ParametricLine::new(
            Point3::new(0.0f64, 1.0f64, 0.0f64),
            Vector3::new(0.0f64, -1.0f64, 0.0f64)
        );

        assert_eq!(ray2.intersect(plane), vec![1.0]);
    }

    #[test]
    fn parametric_line_intersect_plane_f32() {
        let ray1 = ParametricLine::new(
            Point3::new(0.0f32, 1.0f32, 0.0f32),
            Vector3::new(0.0f32, 0.0f32, -1.0f32)
        );

        let plane = Plane3::new(
            Point3::new(0.0f32, 0.0f32, 0.0f32),
            Vector3::new(0.0f32, 1.0f32, 0.0f32)
        );

        assert_eq!(ray1.intersect(plane), Vec::new());

        let ray2 = ParametricLine::new(
            Point3::new(0.0f32, 1.0f32, 0.0f32),
            Vector3::new(0.0f32, -1.0f32, 0.0f32)
        );

        assert_eq!(ray2.intersect(plane), vec![1.0]);
    }


}

