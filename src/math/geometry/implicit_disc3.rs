use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use super::{Intersect, ParametricLine, SurfacePoint};

use crate::math::{Mat3x3, Normal3, Point2, Point3, Vector3};
use crate::traits::{Atan2, One, Sqrt, Zero};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ImplicitDisc3<T>
where
    T: Mul + Div + Copy + Clone,
    <T as Div>::Output: std::fmt::Debug + PartialEq + Clone + Copy,
{
    anchor: Point3<T>,
    normal: Normal3<<T as Div>::Output>,
    right: Vector3<<T as Div>::Output>,
    radius: T,
}

impl<T> ImplicitDisc3<T>
where
    T: Mul + Div + Copy + Clone,
    <T as Div>::Output: std::fmt::Debug + PartialEq + Clone + Copy,
{
    pub fn new(
        anchor: Point3<T>,
        normal: Normal3<<T as Div>::Output>,
        right: Vector3<<T as Div>::Output>,
        radius: T,
    ) -> ImplicitDisc3<T> {
        ImplicitDisc3 {
            anchor,
            normal,
            right,
            radius,
        }
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

impl<T> Intersect<ImplicitDisc3<T>> for ParametricLine<Point3<T>, Vector3<T>>
where
    T: One + Mul + Div + Add<Output = T> + Zero + Copy + Clone + PartialOrd,
    <T as Mul>::Output:
        Add<Output = <T as Mul>::Output> + Div + PartialEq + Sqrt<Output = T> + Zero,
    <T as Div>::Output: Add<Output = <T as Div>::Output>
        + Atan2<Output = <T as Div>::Output>
        + Div<Output = <T as Div>::Output>
        + Mul<Output = <T as Div>::Output>
        + Neg<Output = <T as Div>::Output>
        + One
        + Rem<Output = <T as Div>::Output>
        + Sub<Output = <T as Div>::Output>
        + std::fmt::Debug
        + Zero
        + PartialEq
        + Clone
        + Copy,
    Point3<T>: Sub<Output = Vector3<T>>,
    <T as Mul<<T as Div>::Output>>::Output: PartialEq,
    T: Mul<<T as Div>::Output, Output = T>,
{
    type Output = Vec<(
        <<T as Mul<<T as Div>::Output>>::Output as Div>::Output,
        SurfacePoint<T>,
    )>;

    fn intersect(self, disc: ImplicitDisc3<T>) -> Self::Output {
        if self.direction.dot(disc.normal.as_vector()) == Zero::zero() {
            Vec::new()
        } else {
            let t = (disc.anchor - self.origin).dot(disc.normal.as_vector())
                / self.direction.dot(disc.normal.as_vector());

            let p = self.at(t);

            if (p - disc.anchor).magnitude() > disc.radius {
                return Vec::new();
            }

            let n = disc.normal;

            let u_vector = disc.right;
            let v_vector = disc.normal.as_vector();
            let w_vector = Vector3::cross(v_vector, u_vector);

            let m = Mat3x3::from_vector3s(u_vector, v_vector, w_vector);

            let m_determinante = m.determinant();

            let m1 = m.change_column_1(p.as_vector() / T::one());

            let x = m1.determinant() / m_determinante;

            let m3 = m.change_column_3(p.as_vector() / T::one());

            let z = -m3.determinant() / m_determinante;

            let u = (x * x + z * z) / (disc.radius / One::one());
            let v = x.atan2(z);

            let uv: Point2<<T as Div>::Output> =
                Point2::new(u, (v % One::one() + One::one()) % One::one());

            vec![(t, SurfacePoint::new(p, n, uv))]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::math::Normal3;

    macro_rules! new_implicit_disc3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let anchor = Point3::new(1 as $type, 2 as $type, 3 as $type);
                let normal = Normal3::new(4 as $type, 5 as $type, 6 as $type);
                let right = Vector3::new(1 as $type, 0 as $type, 0 as $type);
                let radius = 3 as $type;

                let disc = ImplicitDisc3::new(anchor, normal, right, radius);

                assert_eq!(disc.anchor, anchor);
                assert_eq!(disc.normal, normal);
                assert_eq!(disc.right, right);
                assert_eq!(disc.radius, radius);
            }
        };
    }

    new_implicit_disc3! { u8, new_implicit_disc3_u8 }
    new_implicit_disc3! { u16, new_implicit_disc3_u16 }
    new_implicit_disc3! { u32, new_implicit_disc3_u32 }
    new_implicit_disc3! { u64, new_implicit_disc3_u64 }
    new_implicit_disc3! { u128, new_implicit_disc3_u128 }
    new_implicit_disc3! { i8, new_implicit_disc3_i8 }
    new_implicit_disc3! { i16, new_implicit_disc3_i16 }
    new_implicit_disc3! { i32, new_implicit_disc3_i32 }
    new_implicit_disc3! { i64, new_implicit_disc3_i64 }
    new_implicit_disc3! { i128, new_implicit_disc3_i128 }
    new_implicit_disc3! { f32, new_implicit_disc3_f32 }
    new_implicit_disc3! { f64, new_implicit_disc3_f64 }

    macro_rules! implicit_disc3_test_point {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let disc = ImplicitDisc3::new(
                    Point3::new(1 as $type, 1 as $type, 1 as $type),
                    Normal3::new(0 as $type, 1 as $type, 0 as $type),
                    Vector3::new(1 as $type, 0 as $type, 0 as $type),
                    3 as $type,
                );

                assert_eq!(
                    disc.test(Point3::new(5 as $type, 1 as $type, 10 as $type)),
                    0 as $type
                );
                assert_ne!(
                    disc.test(Point3::new(1 as $type, 2 as $type, 1 as $type)),
                    0 as $type
                );
            }
        };
    }

    implicit_disc3_test_point! { u8, implicit_disc3_test_point_u8 }
    implicit_disc3_test_point! { u16, implicit_disc3_test_point_u16 }
    implicit_disc3_test_point! { u32, implicit_disc3_test_point_u32 }
    implicit_disc3_test_point! { u64, implicit_disc3_test_point_u64 }
    implicit_disc3_test_point! { u128, implicit_disc3_test_point_u128 }
    implicit_disc3_test_point! { i8, implicit_disc3_test_point_i8 }
    implicit_disc3_test_point! { i16, implicit_disc3_test_point_i16 }
    implicit_disc3_test_point! { i32, implicit_disc3_test_point_i32 }
    implicit_disc3_test_point! { i64, implicit_disc3_test_point_i64 }
    implicit_disc3_test_point! { i128, implicit_disc3_test_point_i128 }
    implicit_disc3_test_point! { f32, implicit_disc3_test_point_f32 }
    implicit_disc3_test_point! { f64, implicit_disc3_test_point_f64 }

    macro_rules! parametric_line_intersect_implicit_disc3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let n = Normal3::new(0 as $type, 1 as $type, 0 as $type);
                let right = Vector3::new(1 as $type, 0 as $type, 0 as $type);

                let ray1 = ParametricLine::new(
                    Point3::new(0 as $type, 1 as $type, 0 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type),
                );

                let disc = ImplicitDisc3::new(
                    Point3::new(0 as $type, 0 as $type, 0 as $type),
                    n,
                    right,
                    2 as $type,
                );

                assert_eq!(ray1.intersect(disc), Vec::new());

                let ray2 = ParametricLine::new(
                    Point3::new(0 as $type, 1 as $type, 0 as $type),
                    Vector3::new(0 as $type, -1 as $type, 0 as $type),
                );

                assert_eq!(
                    ray2.intersect(disc),
                    vec![(
                        1 as $type,
                        SurfacePoint::new(
                            Point3::new(0 as $type, 0 as $type, 0 as $type),
                            n,
                            Point2::new(0 as $type, 0 as $type)
                        )
                    )]
                );

                let ray3 = ParametricLine::new(
                    Point3::new(2 as $type, 1 as $type, 2 as $type),
                    Vector3::new(0 as $type, -1 as $type, 0 as $type),
                );

                assert_eq!(ray3.intersect(disc), Vec::new());
            }
        };
    }

    parametric_line_intersect_implicit_disc3! { f32, parametric_line_intersect_implicit_disc3_f32 }
    parametric_line_intersect_implicit_disc3! { f64, parametric_line_intersect_implicit_disc3_f64 }
}
