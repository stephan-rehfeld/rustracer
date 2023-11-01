use std::ops;

use super::Intersect;
use super::ParametricLine;

use crate::math::vector::DotProduct;
use crate::math::Point3;
use crate::math::Vector3;
use crate::traits::Zero;

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct ImplicitPlane3<T> {
    anchor: Point3<T>,
    normal: Vector3<T>
}

impl<T> ImplicitPlane3<T> {
    pub fn new(anchor: Point3<T>, normal: Vector3<T>) -> ImplicitPlane3<T> {
        ImplicitPlane3 { anchor, normal }
    }

    pub fn test<U>(self, p: Point3<U>) -> <<U as ops::Sub<T>>::Output as ops::Mul<T>>::Output where
        U: ops::Sub<T>,
        <U as ops::Sub<T>>::Output: ops::Mul<T> + Copy + Clone,
        <<U as ops::Sub<T>>::Output as ops::Mul<T>>::Output: ops::Add<Output=<<U as ops::Sub<T>>::Output as ops::Mul<T>>::Output> + Zero,
    {
        (p - self.anchor).dot(self.normal)
    }
}

impl<T> Intersect<ImplicitPlane3<T>> for ParametricLine<Point3<T>, Vector3<T>> where
    T: ops::Mul + Clone + Copy,
    <T as ops::Mul>::Output: ops::Add<Output=<T as ops::Mul>::Output> + ops::Div +  PartialEq + Zero,
    Point3<T>: ops::Sub<Output=Vector3<T>>,
{
    type Output = Vec<<<T as ops::Mul>::Output as ops::Div>::Output>;

    fn intersect(self, plane: ImplicitPlane3<T>) -> Self::Output {
        if self.direction.dot(plane.normal) == Zero::zero() {
            Vec::new()
        } else {
            vec![  (plane.anchor - self.origin).dot(plane.normal) / self.direction.dot(plane.normal)]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
