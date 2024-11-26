use std::ops::{Div, Mul};

use super::{ImplicitNSphere, Intersect, ParametricLine, SurfacePoint};

use crate::math::{Point2, Point3, Vector3};
use crate::traits::floating_point::Pi;
use crate::traits::{
    Acos, Atan2, ConvenientNumber, FloatingPoint, Half, Number, One, SelfMulNumber, SignedNumber,
    Sqrt, Zero,
};

pub type Sphere<T> = ImplicitNSphere<Point3<T>>;

impl<T> Intersect<Sphere<T>> for ParametricLine<Point3<T>, Vector3<T>>
where
    T: SelfMulNumber<<T as Div>::Output>,
    <T as Div>::Output: FloatingPoint + ConvenientNumber + Pi,
    <T as Mul>::Output: SelfMulNumber<<T as Div>::Output>
        + SignedNumber<<T as Div>::Output>
        + Div<Output = <T as Div>::Output>
        + Sqrt<Output = T>,
    <<T as Mul>::Output as Mul>::Output:
        Number<<T as Div>::Output> + Sqrt<Output = <T as Mul>::Output>,
{
    type Output = Vec<(<T as Div>::Output, SurfacePoint<T>)>;

    fn intersect(self, sphere: Sphere<T>) -> Self::Output {
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

            let u = (phi / <T as Div>::Output::PI).half();
            let v = -(theta / <T as Div>::Output::PI);

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

            let u1 = (phi1 / <T as Div>::Output::PI).half();
            let u2 = (phi2 / <T as Div>::Output::PI).half();
            let v1 = -(theta1 / <T as Div>::Output::PI);
            let v2 = -(theta2 / <T as Div>::Output::PI);

            let uv1: Point2<<T as Div>::Output> = Point2::new(
                (u1 + <T as Div>::Output::one()) % <T as Div>::Output::one(),
                (v1 + <T as Div>::Output::one()) % <T as Div>::Output::one(),
            );
            let uv2: Point2<<T as Div>::Output> = Point2::new(
                (u2 + <T as Div>::Output::one()) % <T as Div>::Output::one(),
                (v2 + <T as Div>::Output::one()) % <T as Div>::Output::one(),
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

    macro_rules! new_sphere {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let center = Point3::new(1 as $type, 2 as $type, 3 as $type);
                let radius = 4 as $type;

                let sphere = Sphere::new(center, radius);

                assert_eq!(sphere.center, center);
                assert_eq!(sphere.radius, radius);
            }
        };
    }

    new_sphere! { u8, new_sphere_u8 }
    new_sphere! { u16, new_sphere_u16 }
    new_sphere! { u32, new_sphere_u32 }
    new_sphere! { u64, new_sphere_u64 }
    new_sphere! { u128, new_sphere_u128 }
    new_sphere! { i8, new_sphere_i8 }
    new_sphere! { i16, new_sphere_i16 }
    new_sphere! { i32, new_sphere_i32 }
    new_sphere! { i64, new_sphere_i64 }
    new_sphere! { i128, new_sphere_i128 }
    new_sphere! { f32, new_sphere_f32 }
    new_sphere! { f64, new_sphere_f64 }

    macro_rules! sphere_test {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let center = Point3::new(2 as $type, 2 as $type, 2 as $type);
                let radius = 2 as $type;

                let sphere = Sphere::new(center, radius);

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

    sphere_test! { i8, sphere_test_i8 }
    sphere_test! { i16, sphere_test_i16 }
    sphere_test! { i32, sphere_test_i32 }
    sphere_test! { i64, sphere_test_i64 }
    sphere_test! { i128, sphere_test_i128 }
    sphere_test! { f32, sphere_test_f32 }
    sphere_test! { f64, sphere_test_f64 }

    macro_rules! parametric_line_intersect_sphere {
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

                let sphere =
                    Sphere::new(Point3::new(1 as $type, 1 as $type, 1 as $type), 2 as $type);

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

    parametric_line_intersect_sphere! { f32, parametric_line_intersect_sphere_f32 }
    parametric_line_intersect_sphere! { f64, parametric_line_intersect_sphere_f64 }
}
