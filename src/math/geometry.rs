use std::ops;

use crate::math::vector::Vector3;
use crate::math::point::Point3;
use crate::traits::Sqrt;

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct ParametricLine<P,V> {
    origin: P,
    direction: V
}

impl<P,V> ParametricLine<P,V> {
    pub fn new( origin: P, direction: V ) -> ParametricLine<P,V> {
        ParametricLine { origin, direction }
    }

    pub fn at<T>(self, t: T) -> <P as ops::Add<<V as ops::Mul<T>>::Output>>::Output
    where
        V: ops::Mul<T>,
        P: ops::Add<<V as ops::Mul<T>>::Output>

    {
       self.origin + self.direction * t
    }
}

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct ImplicitPlane3<T> {
    anchor: Point3<T>,
    normal: Vector3<T>
}

impl<T> ImplicitPlane3<T> {
    pub fn new(anchor: Point3<T>, normal: Vector3<T>) -> ImplicitPlane3<T> {
        ImplicitPlane3 { anchor, normal }
    }
}

pub trait Intersect<T> {
    type Output;

    fn intersect(self, other: T) -> Vec<Self::Output>;
}

impl<T: ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T> + ops::Div<Output = T> + PartialEq + Default + Clone + Copy> Intersect<ImplicitPlane3<T>> for ParametricLine<Point3<T>, Vector3<T>> {
    type Output = T;

    fn intersect(self, plane: ImplicitPlane3<T>) -> Vec<Self::Output> {
        if self.direction * plane.normal == Default::default() {
            Vec::new()
        } else {
            vec![((plane.anchor - self.origin) * plane.normal) / (self.direction * plane.normal)]
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ImplicitNSphere<P,T> {
    center: P,
    radius: T,
}

impl<P, T> ImplicitNSphere<P, T> {
    pub fn new(center: P, radius: T) -> ImplicitNSphere<P, T> {
        ImplicitNSphere { center, radius }
    }
}

impl<P, V, T> Intersect<ImplicitNSphere<P, T>> for ParametricLine<P, V>
where
    V: ops::Mul,
    P: ops::Sub<Output = V>,
    V: ops::Add<Output = V>,
    T: ops::Mul,
    V: ops::Mul<Output = <T as ops::Mul>::Output>,
    <T as ops::Mul>::Output: ops::Sub<Output = <T as ops::Mul>::Output>,
    <T as ops::Mul>::Output: ops::Mul,
    <<T as ops::Mul>::Output as ops::Mul>::Output: ops::Add<Output = <<T as ops::Mul>::Output as ops::Mul>::Output>,
    <<T as ops::Mul>::Output as ops::Mul>::Output: ops::Sub<Output = <<T as ops::Mul>::Output as ops::Mul>::Output>,
    <<T as ops::Mul>::Output as ops::Mul>::Output: Sqrt<Output = <T as ops::Mul>::Output>,
    <<T as ops::Mul>::Output as ops::Mul>::Output: PartialEq + PartialOrd + Default,
    <T as ops::Mul>::Output: ops::Neg<Output = <T as ops::Mul>::Output>,
    <T as ops::Mul>::Output: ops::Add<Output = <T as ops::Mul>::Output>,
    <T as ops::Mul>::Output: ops::Div,
    P: Clone + Copy,
    V: Clone + Copy,
    T: Clone + Copy,
    <T as ops::Mul>::Output: Clone + Copy,
    <<T as ops::Mul>::Output as ops::Mul>::Output: Clone + Copy, 
{
    type Output = <<T as ops::Mul>::Output as ops::Div>::Output;

    fn intersect(self, sphere: ImplicitNSphere<P, T>) -> Vec<Self::Output> {
        let a = self.direction * self.direction;
        let b = self.direction * ((self.origin - sphere.center) + (self.origin - sphere.center));
        let c = (self.origin - sphere.center) * (self.origin - sphere.center) - sphere.radius * sphere.radius;

        let helper = b * b - (a * c + a * c + a * c + a * c);
        
        if helper < Default::default() {
            Vec::new()
        } else if helper == Default::default() {
            vec![ (-b / (a + a) ) ]
        } else {
            let helper = helper.sqrt();
            vec![ ((-b - helper) / (a + a) ), ((-b + helper) / (a + a) ) ]
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AxisAlignedBox<T> {
    lower_left_far: Point3<T>,
    upper_right_near: Point3<T>,
}

impl<T> AxisAlignedBox<T> {
    pub fn new(lower_left_far: Point3<T>, upper_right_near: Point3<T>) -> AxisAlignedBox<T> {
        AxisAlignedBox { lower_left_far, upper_right_near }
    }
}

impl Intersect<AxisAlignedBox<f32>> for ParametricLine<Point3<f32>, Vector3<f32>> {
    type Output = f32;

    fn intersect(self, aab: AxisAlignedBox<f32>) -> Vec<f32> {
        let left = ImplicitPlane3::new( aab.lower_left_far, Vector3::new( -1.0, 0.0, 0.0 ));
        let lower = ImplicitPlane3::new( aab.lower_left_far, Vector3::new( 0.0, -1.0, 0.0 ));
        let far = ImplicitPlane3::new( aab.lower_left_far, Vector3::new( 0.0, 0.0, -1.0 ));

        let right = ImplicitPlane3::new( aab.upper_right_near, Vector3::new( 1.0, 0.0, 0.0 ));
        let upper = ImplicitPlane3::new( aab.upper_right_near, Vector3::new( 0.0, 1.0, 0.0 ));
        let near = ImplicitPlane3::new( aab.upper_right_near, Vector3::new( 0.0, 0.0, 1.0 ));

        let mut results: Vec<f32> = Vec::new();

        let t = self.intersect(left);

        if t.len() > 0 {
            let p = self.at(t[0]);

            if p.y > aab.lower_left_far.y && p.y < aab.upper_right_near.y &&
               p.z > aab.lower_left_far.z && p.z < aab.upper_right_near.z {
                results.push(t[0]);
            }
        }
        
        let t = self.intersect(right);

        if t.len() > 0 {
            let p = self.at(t[0]);

            if p.y > aab.lower_left_far.y && p.y < aab.upper_right_near.y &&
               p.z > aab.lower_left_far.z && p.z < aab.upper_right_near.z {
                results.push(t[0]);
            }
        }

        let t = self.intersect(lower);

        if t.len() > 0 {
            let p = self.at(t[0]);

            if p.x > aab.lower_left_far.x && p.x < aab.upper_right_near.x &&
               p.z > aab.lower_left_far.z && p.z < aab.upper_right_near.z {
                results.push(t[0]);
            }
        }

        let t = self.intersect(upper);

        if t.len() > 0 {
            let p = self.at(t[0]);

            if p.x > aab.lower_left_far.x && p.x < aab.upper_right_near.x &&
               p.z > aab.lower_left_far.z && p.z < aab.upper_right_near.z {
                results.push(t[0]);
            }
        }

        let t = self.intersect(near);

        if t.len() > 0 {
            let p = self.at(t[0]);

            if p.x > aab.lower_left_far.x && p.x < aab.upper_right_near.x &&
               p.y > aab.lower_left_far.z && p.y < aab.upper_right_near.y {
                results.push(t[0]);
            }
        }

        let t = self.intersect(far);

        if t.len() > 0 {
            let p = self.at(t[0]);

            if p.x > aab.lower_left_far.x && p.x < aab.upper_right_near.x &&
               p.y > aab.lower_left_far.z && p.y < aab.upper_right_near.y {
                results.push(t[0]);
            }
        }
        
        results
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
    fn new_implicit_plane3_f32() {
        let anchor = Point3::new( 1.0f32, 2.0f32, 3.0f32 );
        let normal = Vector3::new( 4.0f32, 5.0f32, 6.0f32 );

        let plane = ImplicitPlane3::new(anchor, normal);

        assert_eq!(plane.anchor, anchor);
        assert_eq!(plane.normal, normal);
    }

    #[test]
    fn new_implicit_plane3_f64() {
        let anchor = Point3::new( 1.0f64, 2.0f64, 3.0f64 );
        let normal = Vector3::new( 4.0f64, 5.0f64, 6.0f64);

        let plane = ImplicitPlane3::new(anchor, normal);

        assert_eq!(plane.anchor, anchor);
        assert_eq!(plane.normal, normal);
    }

    #[test]
    fn parametric_line_intersect_implicit_plane_f64() {
        let ray1 = ParametricLine::new(
            Point3::new(0.0f64, 1.0f64, 0.0f64),
            Vector3::new(0.0f64, 0.0f64, -1.0f64)
        );

        let plane = ImplicitPlane3::new(
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
    fn parametric_line_intersect_implicit_plane_f32() {
        let ray1 = ParametricLine::new(
            Point3::new(0.0f32, 1.0f32, 0.0f32),
            Vector3::new(0.0f32, 0.0f32, -1.0f32)
        );

        let plane = ImplicitPlane3::new(
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

