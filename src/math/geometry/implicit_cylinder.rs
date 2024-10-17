use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use super::{Intersect, ParametricLine, SurfacePoint};

use crate::math::vector::NormalizableVector;
use crate::math::{Point2, Point3, Vector3};
use crate::traits::{Atan2, Half, One, Pi, Sqrt, Zero};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ImplicitCylinder<T> {
    center: Point3<T>,
    height: T,
    radius: T,
}

impl<T> ImplicitCylinder<T> {
    pub fn new(center: Point3<T>, height: T, radius: T) -> ImplicitCylinder<T> {
        ImplicitCylinder {
            center,
            height,
            radius,
        }
    }

    pub fn test(self, point: Point3<T>) -> <T as Mul>::Output
    where
        T: Mul + Sub<Output = T> + Copy,
        <T as Mul>::Output: Add<Output = <T as Mul>::Output> + Sub<Output = <T as Mul>::Output>,
    {
        let v = point - self.center;

        v.x * v.x + v.z * v.z - self.radius * self.radius
    }
}

impl<T> Intersect<ImplicitCylinder<T>> for ParametricLine<Point3<T>, Vector3<T>>
where
    T: Add<Output = T>
        + Div
        + Half
        + Mul
        + Mul<<T as Div>::Output, Output = T>
        + Neg<Output = T>
        + Sub<Output = T>
        + Zero
        + Copy
        + PartialOrd,
    <T as Div>::Output: Add<Output = <T as Div>::Output>
        + Atan2<Output = <T as Div>::Output>
        + Copy
        + Div<Output = <T as Div>::Output>
        + Debug
        + Half
        + One
        + PartialEq
        + Pi
        + Rem<Output = <T as Div>::Output>,
    <T as Mul>::Output: Add<Output = <T as Mul>::Output>
        + Div<Output = <T as Div>::Output>
        + Mul
        + Neg<Output = <T as Mul>::Output>
        + Sub<Output = <T as Mul>::Output>
        + Sqrt<Output = T>
        + Zero
        + Copy,
    <<T as Mul>::Output as Mul>::Output: Add<Output = <<T as Mul>::Output as Mul>::Output>
        + Sub<Output = <<T as Mul>::Output as Mul>::Output>
        + Sqrt<Output = <T as Mul>::Output>
        + Zero
        + PartialEq
        + PartialOrd,
{
    type Output = Vec<(<T as Div>::Output, SurfacePoint<T>)>;

    fn intersect(self, cylinder: ImplicitCylinder<T>) -> Self::Output {
        let a = self.direction.x * self.direction.x + self.direction.z * self.direction.z;
        let v = self.origin - cylinder.center;
        let v2 = v + v;

        let b = self.direction.x * v2.x + self.direction.z * v2.z;

        let c = v.x * v.x + v.z * v.z - cylinder.radius * cylinder.radius;

        let helper = b * b - (a * c + a * c + a * c + a * c);

        if helper < Zero::zero() {
            Vec::new()
        } else if helper == Zero::zero() {
            let t = -b / (a + a);
            let p = self.at(t);

            let vec = p - cylinder.center;

            if vec.y < -cylinder.height.half() || vec.y > cylinder.height.half() {
                return Vec::new();
            }

            let n = Vector3::new(vec.x, Zero::zero(), vec.z)
                .normalized()
                .as_normal();

            let phi = n.x.atan2(n.z);

            let u = (phi / Pi::PI).half();
            let v = (vec.y + cylinder.height.half()) / cylinder.height;

            let uv: Point2<<T as Div>::Output> = Point2::new(u, v);
            vec![(t, SurfacePoint::new(p, n, uv))]
        } else {
            let helper = helper.sqrt();

            let t1 = (-b - helper) / (a + a);
            let t2 = (-b + helper) / (a + a);

            let p1 = self.at(t1);
            let p2 = self.at(t2);

            let vec1 = p1 - cylinder.center;
            let vec2 = p2 - cylinder.center;

            let mut hits: Self::Output = Vec::new();

            if vec1.y >= -cylinder.height.half() && vec1.y <= cylinder.height.half() {
                let n1 = Vector3::new(vec1.x, Zero::zero(), vec1.z)
                    .normalized()
                    .as_normal();
                let phi1 = n1.x.atan2(n1.z);

                let u1 = (phi1 / Pi::PI).half();
                let v1 = (vec1.y + cylinder.height.half()) / cylinder.height;

                let uv1: Point2<<T as Div>::Output> = Point2::new(
                    (u1 + One::one()) % One::one(),
                    (v1 + One::one()) % One::one(),
                );

                hits.push((t1, SurfacePoint::new(p1, n1, uv1)));
            }

            if vec2.y >= -cylinder.height.half() && vec2.y <= cylinder.height.half() {
                let n2 = Vector3::new(vec2.x, Zero::zero(), vec2.z)
                    .normalized()
                    .as_normal();
                let phi2 = n2.x.atan2(n2.z);

                let u2 = (phi2 / Pi::PI).half();
                let v2 = (vec2.y + cylinder.height.half()) / cylinder.height;

                let uv2: Point2<<T as Div>::Output> = Point2::new(
                    (u2 + One::one()) % One::one(),
                    (v2 + One::one()) % One::one(),
                );

                hits.push((t2, SurfacePoint::new(p2, n2, uv2)));
            }

            hits
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::math::Normal3;

    macro_rules! new_implicit_cylinder {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let center = Point3::new(1 as $type, 2 as $type, 3 as $type);
                let height = 5 as $type;
                let radius = 6 as $type;

                let cylinder = ImplicitCylinder::new(center, height, radius);

                assert_eq!(cylinder.center, center);
                assert_eq!(cylinder.height, height);
                assert_eq!(cylinder.radius, radius);
            }
        };
    }

    new_implicit_cylinder! { u8, new_implicit_cylinder_u8 }
    new_implicit_cylinder! { u16, new_implicit_cylinder_u16 }
    new_implicit_cylinder! { u32, new_implicit_cylinder_u32 }
    new_implicit_cylinder! { u64, new_implicit_cylinder_u64 }
    new_implicit_cylinder! { u128, new_implicit_cylinder_u128 }
    new_implicit_cylinder! { i8, new_implicit_cylinder_i8 }
    new_implicit_cylinder! { i16, new_implicit_cylinder_i16 }
    new_implicit_cylinder! { i32, new_implicit_cylinder_i32 }
    new_implicit_cylinder! { i64, new_implicit_cylinder_i64 }
    new_implicit_cylinder! { i128, new_implicit_cylinder_i128 }
    new_implicit_cylinder! { f32, new_implicit_cylinder_f32 }
    new_implicit_cylinder! { f64, new_implicit_cylinder_f64 }

    macro_rules! implicit_cylinder_test {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let center = Point3::new(2 as $type, 2 as $type, 2 as $type);
                let height = 3 as $type;
                let radius = 2 as $type;

                let cylinder = ImplicitCylinder::new(center, height, radius);

                assert_ne!(
                    cylinder.test(Point3::new(2 as $type, 2 as $type, 2 as $type)),
                    0 as $type
                );
                assert_ne!(
                    cylinder.test(Point3::new(3 as $type, 2 as $type, 2 as $type)),
                    0 as $type
                );
                assert_eq!(
                    cylinder.test(Point3::new(4 as $type, 2 as $type, 2 as $type)),
                    0 as $type
                );
                assert_eq!(
                    cylinder.test(Point3::new(2 as $type, 2 as $type, 4 as $type)),
                    0 as $type
                );
                assert_eq!(
                    cylinder.test(Point3::new(0 as $type, 2 as $type, 2 as $type)),
                    0 as $type
                );
                assert_eq!(
                    cylinder.test(Point3::new(2 as $type, 2 as $type, 0 as $type)),
                    0 as $type
                );
            }
        };
    }

    implicit_cylinder_test! { i8, implicit_cylinder_test_i8 }
    implicit_cylinder_test! { i16, implicit_cylinder_test_i16 }
    implicit_cylinder_test! { i32, implicit_cylinder_test_i32 }
    implicit_cylinder_test! { i64, implicit_cylinder_test_i64 }
    implicit_cylinder_test! { i128, implicit_cylinder_test_i128 }
    implicit_cylinder_test! { f32, implicit_cylinder_test_f32 }
    implicit_cylinder_test! { f64, implicit_cylinder_test_f64 }

    macro_rules! parametric_line_intersect_implicit_cylinder {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let ray1 = ParametricLine::new(
                    Point3::new(1 as $type, 6 as $type, 5 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type),
                );

                let ray2 = ParametricLine::new(
                    Point3::new(8 as $type, 0 as $type, 5 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type),
                );

                let ray3 = ParametricLine::new(
                    Point3::new(2 as $type, 1 as $type, 5 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type),
                );

                let ray4 = ParametricLine::new(
                    Point3::new(1 as $type, 1 as $type, 5 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type),
                );

                let cylinder = ImplicitCylinder::new(
                    Point3::new(1 as $type, 1 as $type, 1 as $type),
                    4 as $type,
                    1 as $type,
                );

                assert_eq!(ray1.intersect(cylinder), Vec::new());
                assert_eq!(ray2.intersect(cylinder), Vec::new());
                assert_eq!(
                    ray3.intersect(cylinder),
                    vec![(
                        4 as $type,
                        SurfacePoint::new(
                            Point3::new(2 as $type, 1 as $type, 1 as $type),
                            Normal3::new(1 as $type, 0 as $type, 0 as $type),
                            Point2::new(0.25 as $type, 0.5 as $type)
                        )
                    )]
                );
                assert_eq!(
                    ray4.intersect(cylinder),
                    vec![
                        (
                            3 as $type,
                            SurfacePoint::new(
                                Point3::new(1 as $type, 1 as $type, 2 as $type),
                                Normal3::new(0 as $type, 0 as $type, 1 as $type),
                                Point2::new(0 as $type, 0.5 as $type)
                            )
                        ),
                        (
                            5 as $type,
                            SurfacePoint::new(
                                Point3::new(1 as $type, 1 as $type, 0 as $type),
                                Normal3::new(0 as $type, 0 as $type, -1 as $type),
                                Point2::new(0.5 as $type, 0.5 as $type)
                            )
                        )
                    ]
                );
            }
        };
    }

    parametric_line_intersect_implicit_cylinder! { f32, parametric_line_intersect_implicit_cylinder_f32 }
    parametric_line_intersect_implicit_cylinder! { f64, parametric_line_intersect_implicit_cylinder_f64 }
}
