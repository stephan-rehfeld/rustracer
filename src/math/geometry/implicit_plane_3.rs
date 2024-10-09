use std::ops::{Div, Add, Mul, Sub};

use super::{Intersect, ParametricLine, SurfacePoint};

use crate::math::vector::DotProduct;
use crate::math::NormalizableVector;
use crate::math::Point2;
use crate::math::Point3;
use crate::math::Vector3;
use crate::traits::Zero;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ImplicitPlane3<T>
where
    T: Mul + Div + Copy + Clone,
    <T as Div>::Output: std::fmt::Debug + PartialEq + Clone + Copy,
{
    anchor: Point3<T>,
    normal: <Vector3<T> as NormalizableVector>::NormalType,
}

impl<T> ImplicitPlane3<T>
where
    T: Mul + Div + Copy + Clone,
    <T as Div>::Output: std::fmt::Debug + PartialEq + Clone + Copy,
{
    pub fn new(
        anchor: Point3<T>,
        normal: <Vector3<T> as NormalizableVector>::NormalType,
    ) -> ImplicitPlane3<T> {
        ImplicitPlane3 { anchor, normal }
    }

    pub fn test(self, p: Point3<T>) -> <T as Mul<<T as Div>::Output>>::Output
    where
        T: Mul<<T as Div>::Output>,
        T: Sub<Output = T>,
        <T as Mul<<T as Div>::Output>>::Output:
            Add<Output = <T as Mul<<T as Div>::Output>>::Output> + Zero,
    {
        (p - self.anchor).dot(self.normal.as_vector())
    }
}

impl<T> Intersect<ImplicitPlane3<T>> for ParametricLine<Point3<T>, Vector3<T>>
where
    T: Mul + Div + Add<Output = T> + Zero + Copy + Clone,
    <T as Mul>::Output:
        Add<Output = <T as Mul>::Output> + Div + PartialEq + Zero,
    <T as Div>::Output: std::fmt::Debug + Zero + PartialEq + Clone + Copy,
    Point3<T>: Sub<Output = Vector3<T>>,
    <T as Mul<<T as Div>::Output>>::Output: PartialEq,
    T: Mul<<T as Div>::Output, Output = T>,
{
    type Output = Vec<(
        <<T as Mul<<T as Div>::Output>>::Output as Div>::Output,
        SurfacePoint<T>
    )>;

    fn intersect(self, plane: ImplicitPlane3<T>) -> Self::Output {
        if self.direction.dot(plane.normal.as_vector()) == Zero::zero() {
            Vec::new()
        } else {
            let t = (plane.anchor - self.origin).dot(plane.normal.as_vector())
                / self.direction.dot(plane.normal.as_vector());

            let p = self.at(t);
            let n = plane.normal;
            let uv: Point2<<T as Div>::Output> = Point2::new(Zero::zero(), Zero::zero());

            vec![(t, SurfacePoint::new(p, n, uv))]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::math::Normal3;

    macro_rules! new_implicit_plane3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let anchor = Point3::new(1 as $type, 2 as $type, 3 as $type);
                let normal = Normal3::new(4 as $type, 5 as $type, 6 as $type);

                let plane = ImplicitPlane3::new(anchor, normal);

                assert_eq!(plane.anchor, anchor);
                assert_eq!(plane.normal, normal);
            }
        };
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
                    Normal3::new(0 as $type, 1 as $type, 0 as $type),
                );

                assert_eq!(
                    plane.test(Point3::new(5 as $type, 1 as $type, 10 as $type)),
                    0 as $type
                );
                assert_ne!(
                    plane.test(Point3::new(1 as $type, 2 as $type, 1 as $type)),
                    0 as $type
                );
            }
        };
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
                let n = Normal3::new(0 as $type, 1 as $type, 0 as $type);

                let ray1 = ParametricLine::new(
                    Point3::new(0 as $type, 1 as $type, 0 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type),
                );

                let plane = ImplicitPlane3::new(Point3::new(0 as $type, 0 as $type, 0 as $type), n);

                assert_eq!(ray1.intersect(plane), Vec::new());

                let ray2 = ParametricLine::new(
                    Point3::new(0 as $type, 1 as $type, 0 as $type),
                    Vector3::new(0 as $type, -1 as $type, 0 as $type),
                );

                assert_eq!(ray2.intersect(plane), vec![(1 as $type, SurfacePoint::new(Point3::new(0 as $type, 0 as $type, 0 as $type), n, Point2::new(0 as $type, 0 as $type ) ))]);
            }
        };
    }

    parametric_line_intersect_implicit_plane3! { i8, parametric_line_intersect_implicit_plane3_i8 }
    parametric_line_intersect_implicit_plane3! { i16, parametric_line_intersect_implicit_plane3_i16 }
    parametric_line_intersect_implicit_plane3! { i32, parametric_line_intersect_implicit_plane3_i32 }
    parametric_line_intersect_implicit_plane3! { i64, parametric_line_intersect_implicit_plane3_i64 }
    parametric_line_intersect_implicit_plane3! { i128, parametric_line_intersect_implicit_plane3_i128 }
    parametric_line_intersect_implicit_plane3! { f32, parametric_line_intersect_implicit_plane3_f32 }
    parametric_line_intersect_implicit_plane3! { f64, parametric_line_intersect_implicit_plane3_f64 }
}
