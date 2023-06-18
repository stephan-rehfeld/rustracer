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

impl<T: ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T> + ops::Div<Output = T>+ std::cmp::PartialEq<f64> + Clone + Copy> Intersect<Plane3<T>> for ParametricLine<T> {
    type Output = T;

    fn intersect(self, plane: Plane3<T>) -> Vec<Self::Output> {
        if self.direction * plane.normal == 0.0 {
            Vec::new()
        } else {
            vec![((plane.anchor - self.origin) * plane.normal) / (self.direction * plane.normal)]
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
}

