use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use super::{Intersect, ParametricLine, SurfacePoint};

use crate::math::geometry::Rectangle2;
use crate::math::vector::{DotProduct, NormalizableVector};
use crate::math::{Point, Point2, Point3, Vector2, Vector3};
use crate::traits::floating_point::Pi;
use crate::traits::{Acos, Atan2, Half, One, Sqrt, Zero};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ImplicitNSphere<P>
where
    P: Point,
    <P as Point>::ValueType: Copy + Clone + PartialEq + Debug,
{
    pub center: P,
    pub radius: <P as Point>::ValueType,
}

impl<P> ImplicitNSphere<P>
where
    P: Point,
    <P as Point>::ValueType: Copy + Clone + PartialEq + Debug,
{
    pub fn new(center: P, radius: <P as Point>::ValueType) -> ImplicitNSphere<P> {
        ImplicitNSphere { center, radius }
    }

    pub fn test(self, point: P) -> <<P as Point>::ValueType as Mul>::Output
    where
        P: Sub,
        <P as Sub>::Output:
            DotProduct<Output = <<P as Point>::ValueType as Mul>::Output> + Copy + Clone,
        <P as Point>::ValueType: Mul,
        <<P as Point>::ValueType as Mul>::Output:
            Sub<Output = <<P as Point>::ValueType as Mul>::Output>,
    {
        let d = point - self.center;
        d.dot(d) - self.radius * self.radius
    }
}

impl<T> ImplicitNSphere<Point2<T>>
where
    T: Add<Output = T> + Sub<Output = T> + PartialEq + Copy + Debug,
{
    pub fn bound(self) -> Rectangle2<T> {
        let point = Point2::new(self.center.x - self.radius, self.center.y - self.radius);
        let dimension = Vector2::new(
            self.center.x + self.radius - point.x,
            self.center.y + self.radius - point.y,
        );

        Rectangle2::new(point, dimension)
    }
}

impl<T> Intersect<ImplicitNSphere<Point3<T>>> for ParametricLine<Point3<T>, Vector3<T>>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul
        + Mul<<T as Div>::Output, Output = T>
        + Div
        + Copy
        + Debug
        + PartialEq,
    <T as Div>::Output: Acos<Output = <T as Div>::Output>
        + Add<Output = <T as Div>::Output>
        + Atan2<Output = <T as Div>::Output>
        + Debug
        + Div<Output = <T as Div>::Output>
        + Half
        + Neg<Output = <T as Div>::Output>
        + Rem<Output = <T as Div>::Output>
        + Pi
        + One
        + PartialEq
        + Copy,
    <T as Mul>::Output: Add<Output = <T as Mul>::Output>
        + Sub<Output = <T as Mul>::Output>
        + Mul
        + Div<Output = <T as Div>::Output>
        + Sqrt<Output = T>
        + Neg<Output = <T as Mul>::Output>
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

    fn intersect(self, sphere: ImplicitNSphere<Point3<T>>) -> Self::Output {
        let a = self.direction.dot(self.direction);
        let b = self
            .direction
            .dot((self.origin - sphere.center) + (self.origin - sphere.center));

        let c = (self.origin - sphere.center).dot(self.origin - sphere.center)
            - sphere.radius * sphere.radius;

        let helper = b * b - (a * c + a * c + a * c + a * c);

        if helper < Zero::zero() {
            Vec::new()
        } else if helper == Zero::zero() {
            let t = -b / (a + a);
            let p = self.at(t);
            let v = (p - sphere.center).normalized();
            let n = v.as_normal();

            let theta = v.y.acos();
            let phi = v.x.atan2(v.z);

            let u = (phi / Pi::PI).half();
            let v = -(theta / Pi::PI);

            let uv: Point2<<T as Div>::Output> = Point2::new(u, v);
            vec![(t, SurfacePoint::new(p, n, uv))]
        } else {
            let helper = helper.sqrt();

            let t1 = (-b - helper) / (a + a);
            let t2 = (-b + helper) / (a + a);

            let p1 = self.at(t1);
            let p2 = self.at(t2);

            let v1 = (p1 - sphere.center).normalized();
            let v2 = (p2 - sphere.center).normalized();

            let n1 = v1.as_normal();
            let n2 = v2.as_normal();

            let theta1 = v1.y.acos();
            let theta2 = v2.y.acos();

            let phi1 = v1.x.atan2(v1.z);
            let phi2 = v2.x.atan2(v2.z);

            let u1 = (phi1 / Pi::PI).half();
            let u2 = (phi2 / Pi::PI).half();
            let v1 = -(theta1 / Pi::PI);
            let v2 = -(theta2 / Pi::PI);

            let uv1: Point2<<T as Div>::Output> = Point2::new(
                (u1 + One::one()) % One::one(),
                (v1 + One::one()) % One::one(),
            );
            let uv2: Point2<<T as Div>::Output> = Point2::new(
                (u2 + One::one()) % One::one(),
                (v2 + One::one()) % One::one(),
            );

            vec![
                (t1, SurfacePoint::new(p1, n1, uv1)),
                (t2, SurfacePoint::new(p2, n2, uv2)),
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::math::{Normal3, Point3, Vector3};

    macro_rules! new_implicit_3_sphere {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let center = Point3::new(1 as $type, 2 as $type, 3 as $type);
                let radius = 4 as $type;

                let sphere = ImplicitNSphere::new(center, radius);

                assert_eq!(sphere.center, center);
                assert_eq!(sphere.radius, radius);
            }
        };
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
                let center = Point3::new(2 as $type, 2 as $type, 2 as $type);
                let radius = 2 as $type;

                let sphere = ImplicitNSphere::new(center, radius);

                assert_ne!(
                    sphere.test(Point3::new(2 as $type, 2 as $type, 2 as $type)),
                    0 as $type
                );
                assert_eq!(
                    sphere.test(Point3::new(0 as $type, 2 as $type, 2 as $type)),
                    0 as $type
                );
                assert_eq!(
                    sphere.test(Point3::new(4 as $type, 2 as $type, 2 as $type)),
                    0 as $type
                );
                assert_eq!(
                    sphere.test(Point3::new(2 as $type, 0 as $type, 2 as $type)),
                    0 as $type
                );
                assert_eq!(
                    sphere.test(Point3::new(2 as $type, 4 as $type, 2 as $type)),
                    0 as $type
                );
                assert_eq!(
                    sphere.test(Point3::new(2 as $type, 2 as $type, 0 as $type)),
                    0 as $type
                );
                assert_eq!(
                    sphere.test(Point3::new(2 as $type, 2 as $type, 4 as $type)),
                    0 as $type
                );
            }
        };
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
                    Vector3::new(0 as $type, 0 as $type, -1 as $type),
                );

                let ray2 = ParametricLine::new(
                    Point3::new(1 as $type, 3 as $type, 4 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type),
                );

                let ray3 = ParametricLine::new(
                    Point3::new(1 as $type, 1 as $type, 4 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type),
                );

                let sphere = ImplicitNSphere::new(
                    Point3::new(1 as $type, 1 as $type, 1 as $type),
                    2 as $type,
                );

                assert_eq!(ray1.intersect(sphere), Vec::new());
                assert_eq!(
                    ray2.intersect(sphere),
                    vec![(
                        3 as $type,
                        SurfacePoint::new(
                            Point3::new(1 as $type, 3 as $type, 1 as $type),
                            Normal3::new(0 as $type, 1 as $type, 0 as $type),
                            Point2::new(0 as $type, 0 as $type)
                        )
                    )]
                );
                assert_eq!(
                    ray3.intersect(sphere),
                    vec![
                        (
                            1 as $type,
                            SurfacePoint::new(
                                Point3::new(1 as $type, 1 as $type, 3 as $type),
                                Normal3::new(0 as $type, 0 as $type, 1 as $type),
                                Point2::new(0 as $type, 0.5 as $type)
                            )
                        ),
                        (
                            5 as $type,
                            SurfacePoint::new(
                                Point3::new(1 as $type, 1 as $type, -1 as $type),
                                Normal3::new(0 as $type, 0 as $type, -1 as $type),
                                Point2::new(0.5 as $type, 0.5 as $type)
                            )
                        )
                    ]
                );
            }
        };
    }

    parametric_line_intersect_implicit_3_sphere! { f32, parametric_line_intersect_implicit_3_sphere_f32 }
    parametric_line_intersect_implicit_3_sphere! { f64, parametric_line_intersect_implicit_3_sphere_f64 }
}
