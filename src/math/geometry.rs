use std::ops;

use crate::math::vector::Vector3;
use crate::math::vector::Orthonormal3;
use crate::math::point::Point3;
use crate::traits::Sqrt;

pub trait Intersect<T> {
    type Output;

    fn intersect(self, other: T) -> Vec<Self::Output>;
}

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

    pub fn test<U>(self, p: Point3<U>) -> <Vector3<<U as ops::Sub<T>>::Output> as ops::Mul<Vector3<T>>>::Output 
    where
        U: ops::Sub<T>,
        Vector3<<U as ops::Sub<T>>::Output>: ops::Mul<Vector3<T>>  {
        (p - self.anchor) * self.normal
    }
}


impl<T> Intersect<ImplicitPlane3<T>> for ParametricLine<Point3<T>, Vector3<T>> where
    T: Clone + Copy,
    Vector3<T> : ops::Mul,
    <Vector3<T> as ops::Mul>::Output: PartialEq + Default,
    Point3<T>: ops::Sub,
    <Point3<T> as ops::Sub>::Output: ops::Mul<Vector3<T>>,
    <<Point3<T> as ops::Sub>::Output as ops::Mul<Vector3<T>>>::Output: ops::Div<<Vector3<T> as ops::Mul>::Output>
    {
    type Output = <<<Point3<T> as ops::Sub>::Output as ops::Mul<Vector3<T>>>::Output as ops::Div<<Vector3<T> as ops::Mul>::Output>>::Output;

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

    pub fn test(self, point: P) -> <<<P as ops::Sub>::Output as ops::Mul>::Output as ops::Sub<<T as ops::Mul>::Output>>::Output where 
        P: ops::Sub,
        <P as ops::Sub>::Output: ops::Mul + Copy + Clone,
        T: ops::Mul + Copy + Clone,
        <<P as ops::Sub>::Output as ops::Mul>::Output: ops::Sub<<T as ops::Mul>::Output>,
    {
        let d = point - self.center;
        d * d - self.radius * self.radius
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
pub struct AxisAlignedBox<P> {
    a: P,
    b: P,
}

impl<P> AxisAlignedBox<P> {
    pub fn new(a: P, b: P) -> AxisAlignedBox<P> {
        AxisAlignedBox { a, b }
    }
}

impl<T> Intersect<AxisAlignedBox<Point3<T>>> for ParametricLine<Point3<T>, Vector3<T>>
where
    T: ops::Neg<Output = T> + Copy + Clone + PartialOrd,
    Vector3<T>: Orthonormal3<T> + ops::Mul,
    <Vector3<T> as ops::Mul>::Output: ops::Div<Output = T> + PartialEq + Default,
    Point3<T>: ops::Sub<Output = Vector3<T>>,
    Vector3<T>: ops::Mul<T, Output = Vector3<T>>,
    Point3<T>: ops::Add<Vector3<T>, Output = Point3<T>>,
    {
    type Output = T;

    fn intersect(self, aab: AxisAlignedBox<Point3<T>>) -> Vec<Self::Output> {
        let left = ImplicitPlane3::new( aab.a, -Vector3::<T>::x_axis());
        let lower = ImplicitPlane3::new( aab.a, -Vector3::<T>::y_axis());
        let far = ImplicitPlane3::new( aab.a, -Vector3::<T>::z_axis());

        let right = ImplicitPlane3::new( aab.b, Vector3::<T>::x_axis());
        let upper = ImplicitPlane3::new( aab.b, Vector3::<T>::y_axis());
        let near = ImplicitPlane3::new( aab.b, Vector3::<T>::z_axis());

        let mut results: Vec<T> = Vec::new();

        let t = self.intersect(left);

        if t.len() > 0 {
            let p = self.at(t[0]);

            if p.y > aab.a.y && p.y < aab.b.y &&
               p.z > aab.a.z && p.z < aab.b.z {
                results.push(t[0]);
            }
        }
        
        let t = self.intersect(right);

        if t.len() > 0 {
            let p = self.at(t[0]);

            if p.y > aab.a.y && p.y < aab.b.y &&
               p.z > aab.a.z && p.z < aab.b.z {
                results.push(t[0]);
            }
        }

        let t = self.intersect(lower);

        if t.len() > 0 {
            let p = self.at(t[0]);

            if p.x > aab.a.x && p.x < aab.b.x &&
               p.z > aab.a.z && p.z < aab.b.z {
                results.push(t[0]);
            }
        }

        let t = self.intersect(upper);

        if t.len() > 0 {
            let p = self.at(t[0]);

            if p.x > aab.a.x && p.x < aab.b.x &&
               p.z > aab.a.z && p.z < aab.b.z {
                results.push(t[0]);
            }
        }

        let t = self.intersect(near);

        if t.len() > 0 {
            let p = self.at(t[0]);

            if p.x > aab.a.x && p.x < aab.b.x &&
               p.y > aab.a.z && p.y < aab.b.y {
                results.push(t[0]);
            }
        }

        let t = self.intersect(far);

        if t.len() > 0 {
            let p = self.at(t[0]);

            if p.x > aab.a.x && p.x < aab.b.x &&
               p.y > aab.a.z && p.y < aab.b.y {
                results.push(t[0]);
            }
        }
      
        results
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! new_parametric_line {
        ( $type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let origin = Point3::new( 1 as $type, 2 as $type, 3 as $type );
                let direction = Vector3::new( 4 as $type, 5 as $type, 6 as $type );

                let ray = ParametricLine::new(origin, direction);

                assert_eq!(ray.origin, origin);
                assert_eq!(ray.direction, direction);
            }
        }
    }

    new_parametric_line! { u8, new_parametric_line_u8 }
    new_parametric_line! { u16, new_parametric_line_u16 }
    new_parametric_line! { u32, new_parametric_line_u32 }
    new_parametric_line! { u64, new_parametric_line_u64 }
    new_parametric_line! { u128, new_parametric_line_u128 }
    new_parametric_line! { i8, new_parametric_line_i8 }
    new_parametric_line! { i16, new_parametric_line_i16 }
    new_parametric_line! { i32, new_parametric_line_i32 }
    new_parametric_line! { i64, new_parametric_line_i64 }
    new_parametric_line! { i128, new_parametric_line_i128 }
    new_parametric_line! { f32, new_parametric_line_f32 }
    new_parametric_line! { f64, new_parametric_line_f64 }

    macro_rules! parametric_line_at {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let origin = Point3::new( 1 as $type, 2 as $type, 3 as $type );
                let direction = Vector3::new( 4 as $type, 5 as $type, 6 as $type);

                let t = 10.0 as $type;

                let ray = ParametricLine::new(origin, direction);

                assert_eq!(ray.at(t), origin + direction * t);
            }    
        }
    }

    parametric_line_at! { u8, parametric_line_at_u8 }
    parametric_line_at! { u16, parametric_line_at_u16 }
    parametric_line_at! { u32, parametric_line_at_u32 }
    parametric_line_at! { u64, parametric_line_at_u64 }
    parametric_line_at! { u128, parametric_line_at_u128 }
    parametric_line_at! { i8, parametric_line_at_i8 }
    parametric_line_at! { i16, parametric_line_at_i16 }
    parametric_line_at! { i32, parametric_line_at_i32 }
    parametric_line_at! { i64, parametric_line_at_i64 }
    parametric_line_at! { i128, parametric_line_at_i128 }
    parametric_line_at! { f32, parametric_line_at_f32 }
    parametric_line_at! { f64, parametric_line_at_f64 }

    macro_rules! new_implicit_plane3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let anchor = Point3::new( 1 as $type, 2 as $type, 3 as $type );
                let normal = Vector3::new( 4 as $type, 5 as $type, 6 as $type );

                let plane = ImplicitPlane3::new(anchor, normal);

                assert_eq!(plane.anchor, anchor);
                assert_eq!(plane.normal, normal);
            }
        }
    }

    new_implicit_plane3! { u8, new_implicit_plane3_u8 }
    new_implicit_plane3! { u16, new_implicit_plane3_u16 }
    new_implicit_plane3! { u32, new_implicit_plane3_u32 }
    new_implicit_plane3! { u64, new_implicit_plane3_u64 }
    new_implicit_plane3! { u128, new_implicit_plane3_u128 }
    new_implicit_plane3! { i8, new_implicit_plane3_i8 }
    new_implicit_plane3! { i16, new_implicit_plane3_i16 }
    new_implicit_plane3! { i32, new_implicit_plane3_i32 }
    new_implicit_plane3! { i64, new_implicit_plane3_i64 }
    new_implicit_plane3! { i128, new_implicit_plane3_i128 }
    new_implicit_plane3! { f32, new_implicit_plane3_f32 }
    new_implicit_plane3! { f64, new_implicit_plane3_f64 }

    macro_rules! implicit_plane3_test_point {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let plane = ImplicitPlane3::new(
                    Point3::new(1 as $type, 1 as $type, 1 as $type),
                    Vector3::new(0 as $type, 1 as $type, 0 as $type)
                );

                assert_eq!(plane.test(Point3::new(5 as $type, 1 as $type, 10 as $type)), 0 as $type);
                assert_ne!(plane.test(Point3::new(1 as $type, 2 as $type, 1 as $type)), 0 as $type);
            }
        }
    }

    implicit_plane3_test_point! { u8, implicit_plane3_test_point_u8 }
    implicit_plane3_test_point! { u16, implicit_plane3_test_point_u16 }
    implicit_plane3_test_point! { u32, implicit_plane3_test_point_u32 }
    implicit_plane3_test_point! { u64, implicit_plane3_test_point_u64 }
    implicit_plane3_test_point! { u128, implicit_plane3_test_point_u128 }
    implicit_plane3_test_point! { i8, implicit_plane3_test_point_i8 }
    implicit_plane3_test_point! { i16, implicit_plane3_test_point_i16 }
    implicit_plane3_test_point! { i32, implicit_plane3_test_point_i32 }
    implicit_plane3_test_point! { i64, implicit_plane3_test_point_i64 }
    implicit_plane3_test_point! { i128, implicit_plane3_test_point_i128 }
    implicit_plane3_test_point! { f32, implicit_plane3_test_point_f32 }
    implicit_plane3_test_point! { f64, implicit_plane3_test_point_f64 }

    macro_rules! parametric_line_intersect_implicit_plane3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let ray1 = ParametricLine::new(
                    Point3::new(0 as $type, 1 as $type, 0 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type)
                );

                let plane = ImplicitPlane3::new(
                    Point3::new(0 as $type, 0 as $type, 0 as $type),
                    Vector3::new(0 as $type, 1 as $type, 0 as $type)
                );

                assert_eq!(ray1.intersect(plane), Vec::new());

                let ray2 = ParametricLine::new(
                    Point3::new(0 as $type, 1 as $type, 0 as $type),
                    Vector3::new(0 as $type, -1 as $type, 0 as $type)
                );

                assert_eq!(ray2.intersect(plane), vec![1 as $type]);
            }
        }
    }

    parametric_line_intersect_implicit_plane3! { i8, parametric_line_intersect_implicit_plane3_i8 }
    parametric_line_intersect_implicit_plane3! { i16, parametric_line_intersect_implicit_plane3_i16 }
    parametric_line_intersect_implicit_plane3! { i32, parametric_line_intersect_implicit_plane3_i32 }
    parametric_line_intersect_implicit_plane3! { i64, parametric_line_intersect_implicit_plane3_i64 }
    parametric_line_intersect_implicit_plane3! { i128, parametric_line_intersect_implicit_plane3_i128 }
    parametric_line_intersect_implicit_plane3! { f32, parametric_line_intersect_implicit_plane3_f32 }
    parametric_line_intersect_implicit_plane3! { f64, parametric_line_intersect_implicit_plane3_f64 }

    macro_rules! new_implicit_3_sphere {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let center = Point3::new( 1 as $type, 2 as $type, 3 as $type );
                let radius = 4 as $type;

                let sphere = ImplicitNSphere::new(center, radius);

                assert_eq!(sphere.center, center);
                assert_eq!(sphere.radius, radius);
            }
        }
    }

    new_implicit_3_sphere! { u8, new_implicit_3_sphere_u8 }
    new_implicit_3_sphere! { u16, new_implicit_3_sphere_u16 }
    new_implicit_3_sphere! { u32, new_implicit_3_sphere_u32 }
    new_implicit_3_sphere! { u64, new_implicit_3_sphere_u64 }
    new_implicit_3_sphere! { u128, new_implicit_3_sphere_u128 }
    new_implicit_3_sphere! { i8, new_implicit_3_sphere_i8 }
    new_implicit_3_sphere! { i16, new_implicit_3_sphere_i16 }
    new_implicit_3_sphere! { i32, new_implicit_3_sphere_i32 }
    new_implicit_3_sphere! { i64, new_implicit_3_sphere_i64 }
    new_implicit_3_sphere! { i128, new_implicit_3_sphere_i128 }
    new_implicit_3_sphere! { f32, new_implicit_3_sphere_f32 }
    new_implicit_3_sphere! { f64, new_implicit_3_sphere_f64 }

    macro_rules! implicit_3_sphere_test {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let center = Point3::new( 2 as $type, 2 as $type, 2 as $type );
                let radius = 2 as $type;

                let sphere = ImplicitNSphere::new(center, radius);

                assert_ne!(sphere.test(Point3::new( 2 as $type, 2 as $type, 2 as $type )), 0 as $type);
                assert_eq!(sphere.test(Point3::new( 0 as $type, 2 as $type, 2 as $type )), 0 as $type);
                assert_eq!(sphere.test(Point3::new( 4 as $type, 2 as $type, 2 as $type )), 0 as $type);
                assert_eq!(sphere.test(Point3::new( 2 as $type, 0 as $type, 2 as $type )), 0 as $type);
                assert_eq!(sphere.test(Point3::new( 2 as $type, 4 as $type, 2 as $type )), 0 as $type);
                assert_eq!(sphere.test(Point3::new( 2 as $type, 2 as $type, 0 as $type )), 0 as $type);
                assert_eq!(sphere.test(Point3::new( 2 as $type, 2 as $type, 4 as $type )), 0 as $type);
            }
        }
    }

    implicit_3_sphere_test! { i8, implicit_3_sphere_test_i8 }
    implicit_3_sphere_test! { i16, implicit_3_sphere_test_i16 }
    implicit_3_sphere_test! { i32, implicit_3_sphere_test_i32 }
    implicit_3_sphere_test! { i64, implicit_3_sphere_test_i64 }
    implicit_3_sphere_test! { i128, implicit_3_sphere_test_i128 }
    implicit_3_sphere_test! { f32, implicit_3_sphere_test_f32 }
    implicit_3_sphere_test! { f64, implicit_3_sphere_test_f64 }

    macro_rules! parametric_line_intersect_implicit_3_sphere {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let ray1 = ParametricLine::new(
                    Point3::new(4 as $type, 4 as $type, 4 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type)
                );
                
                let ray2 = ParametricLine::new(
                    Point3::new(1 as $type, 3 as $type, 4 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type)
                );
                
                let ray3 = ParametricLine::new(
                    Point3::new(1 as $type, 1 as $type, 4 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type)
                );

                let sphere = ImplicitNSphere::new(
                    Point3::new(1 as $type, 1 as $type, 1 as $type),
                    2 as $type
                );

                assert_eq!(ray1.intersect(sphere), Vec::new());
                assert_eq!(ray2.intersect(sphere), vec![3 as $type]);
                assert_eq!(ray3.intersect(sphere), vec![1 as $type, 5 as $type]);

            }
        }
    }

    parametric_line_intersect_implicit_3_sphere! { f32, parametric_line_intersect_implicit_3_sphere_f32 }
    parametric_line_intersect_implicit_3_sphere! { f64, parametric_line_intersect_implicit_3_sphere_f64 }
}

