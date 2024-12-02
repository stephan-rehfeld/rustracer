use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};

use super::{Intersect, ParametricLine, SurfacePoint};

use crate::{Mat3x3, Normal3, Point2, Point3, Vector3};
use traits::{FloatingPoint, Number, One, SelfMulNumber, Zero};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ImplicitPlane3<T>
where
    T: Div,
    <T as Div>::Output: Debug + PartialEq + Copy,
{
    anchor: Point3<T>,
    normal: Normal3<<T as Div>::Output>,
    right: Vector3<<T as Div>::Output>,
}

impl<T> ImplicitPlane3<T>
where
    T: Div + Copy,
    <T as Div>::Output: Debug + PartialEq + Copy,
{
    pub fn new(
        anchor: Point3<T>,
        normal: Normal3<<T as Div>::Output>,
        right: Vector3<<T as Div>::Output>,
    ) -> ImplicitPlane3<T> {
        ImplicitPlane3 {
            anchor,
            normal,
            right,
        }
    }

    pub fn test(self, p: Point3<T>) -> T
    where
        T: Add<Output = T> + Sub<Output = T> + Mul<<T as Div>::Output, Output = T> + Zero,
    {
        (p - self.anchor).dot(self.normal.as_vector())
    }
}

impl<T> Intersect<ImplicitPlane3<T>> for ParametricLine<Point3<T>, Vector3<T>>
where
    T: SelfMulNumber<<T as Div>::Output>,
    <T as Mul>::Output: Number<<T as Div>::Output>,
    <T as Div>::Output: FloatingPoint,
    Point3<T>: Sub<Output = Vector3<T>>,
{
    type Output = Vec<(
        <<T as Mul<<T as Div>::Output>>::Output as Div>::Output,
        SurfacePoint<T>,
    )>;

    fn intersect(self, plane: ImplicitPlane3<T>) -> Self::Output {
        if self.direction.dot(plane.normal.as_vector()) == Zero::zero() {
            Vec::new()
        } else {
            let t = (plane.anchor - self.origin).dot(plane.normal.as_vector())
                / self.direction.dot(plane.normal.as_vector());

            let p = self.at(t);
            let n = plane.normal;

            let u_vector = plane.right;
            let v_vector = plane.normal.as_vector();
            let w_vector = Vector3::cross(v_vector, u_vector);

            let m = Mat3x3::from_vector3s(u_vector, v_vector, w_vector);

            let m_determinante = m.determinant();

            let m1 = m.change_column_1(p.as_vector() / T::one());

            let u = m1.determinant() / m_determinante;

            let m3 = m.change_column_3(p.as_vector() / T::one());

            let v = -m3.determinant() / m_determinante;

            let uv: Point2<<T as Div>::Output> = Point2::new(
                (u % <T as Div>::Output::one() + <T as Div>::Output::one())
                    % <T as Div>::Output::one(),
                (v % <T as Div>::Output::one() + <T as Div>::Output::one())
                    % <T as Div>::Output::one(),
            );

            vec![(t, SurfacePoint::new(p, n, uv))]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Normal3;

    macro_rules! new_implicit_plane3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let anchor = Point3::new(1 as $type, 2 as $type, 3 as $type);
                let normal = Normal3::new(4 as $type, 5 as $type, 6 as $type);
                let right = Vector3::new(1 as $type, 0 as $type, 0 as $type);

                let plane = ImplicitPlane3::new(anchor, normal, right);

                assert_eq!(plane.anchor, anchor);
                assert_eq!(plane.normal, normal);
                assert_eq!(plane.right, right);
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
                    Vector3::new(1 as $type, 0 as $type, 0 as $type),
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
                let right = Vector3::new(1 as $type, 0 as $type, 0 as $type);

                let ray1 = ParametricLine::new(
                    Point3::new(0 as $type, 1 as $type, 0 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type),
                );

                let plane =
                    ImplicitPlane3::new(Point3::new(0 as $type, 0 as $type, 0 as $type), n, right);

                assert_eq!(ray1.intersect(plane), Vec::new());

                let ray2 = ParametricLine::new(
                    Point3::new(0 as $type, 1 as $type, 0 as $type),
                    Vector3::new(0 as $type, -1 as $type, 0 as $type),
                );

                assert_eq!(
                    ray2.intersect(plane),
                    vec![(
                        1 as $type,
                        SurfacePoint::new(
                            Point3::new(0 as $type, 0 as $type, 0 as $type),
                            n,
                            Point2::new(0 as $type, 0 as $type)
                        )
                    )]
                );
            }
        };
    }

    parametric_line_intersect_implicit_plane3! { f32, parametric_line_intersect_implicit_plane3_f32 }
    parametric_line_intersect_implicit_plane3! { f64, parametric_line_intersect_implicit_plane3_f64 }
}
