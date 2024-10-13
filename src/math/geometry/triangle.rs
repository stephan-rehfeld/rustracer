use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};

use super::{Intersect, ParametricLine, SurfacePoint};

use crate::math::{Mat3x3, Normal3, NormalizableVector, Point, Point2, Point3, Vector3};
use crate::traits::{One, Sqrt, Zero};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Triangle3<T: Div>
where
    <T as Div>::Output: Copy + Debug + PartialEq,
{
    a: Point3<T>,
    b: Point3<T>,
    c: Point3<T>,
    na: Normal3<<T as Div>::Output>,
    nb: Normal3<<T as Div>::Output>,
    nc: Normal3<<T as Div>::Output>,
    uva: Point2<<T as Div>::Output>,
    uvb: Point2<<T as Div>::Output>,
    uvc: Point2<<T as Div>::Output>,
}

impl<T: Div> Triangle3<T>
where
    <T as Div>::Output: Copy + Debug + PartialEq,
{
    pub fn new(
        a: Point3<T>,
        b: Point3<T>,
        c: Point3<T>,
        na: Normal3<<T as Div>::Output>,
        nb: Normal3<<T as Div>::Output>,
        nc: Normal3<<T as Div>::Output>,
        uva: Point2<<T as Div>::Output>,
        uvb: Point2<<T as Div>::Output>,
        uvc: Point2<<T as Div>::Output>,
    ) -> Triangle3<T> {
        Triangle3 {
            a,
            b,
            c,
            na,
            nb,
            nc,
            uva,
            uvb,
            uvc,
        }
    }
}

impl<T: Div> Intersect<Triangle3<T>> for ParametricLine<Point3<T>, Vector3<T>>
where
    <T as Div>::Output: Add<Output = <T as Div>::Output>
        + Sub<Output = <T as Div>::Output>
        + Mul<Output = <T as Div>::Output>
        + Div<Output = <T as Div>::Output>
        + Neg<Output = <T as Div>::Output>
        + Sqrt<Output = <T as Div>::Output>
        + One
        + Zero
        + Debug
        + PartialOrd
        + Copy
        + PartialEq,
    T: Add<Output = T> + Mul<<T as Div>::Output, Output = T> + Sub<Output = T> + Mul + Div + Copy,
    <T as Mul>::Output: Mul<T>,
    <<T as Mul>::Output as Mul<T>>::Output: Add<Output = <<T as Mul>::Output as Mul<T>>::Output>
        + Sub<Output = <<T as Mul>::Output as Mul<T>>::Output>
        + Div<Output = <T as Div>::Output>,
    <<T as Mul>::Output as Mul<T>>::Output: Zero + PartialEq + Copy,
{
    type Output = Vec<(<T as Div>::Output, SurfacePoint<T>)>;

    fn intersect(self, triangle: Triangle3<T>) -> Self::Output {
        let m = Mat3x3::from_vector3s(
            triangle.a - triangle.b,
            triangle.a - triangle.c,
            self.direction,
        );
        let v = triangle.a - self.origin;

        let m_determinante = m.determinant();

        if m_determinante == Zero::zero() {
            return vec![];
        }

        let m1 = m.change_column_1(v);

        let beta = m1.determinant() / m_determinante;

        if beta < Zero::zero() || beta > One::one() {
            return vec![];
        }

        let m2 = m.change_column_2(v);

        let gamma = m2.determinant() / m_determinante;

        if gamma < Zero::zero() || gamma > One::one() {
            return vec![];
        }

        if beta + gamma < Zero::zero() || beta + gamma > One::one() {
            return vec![];
        }

        let m3 = m.change_column_3(v);

        let t = m3.determinant() / m_determinante;
        let alpha = -beta - gamma + One::one();

        let p = self.at(t);
        let n = (triangle.na * alpha + triangle.nb * beta + triangle.nc * gamma)
            .normalized()
            .as_normal();
        let uv = triangle.uva.as_vector() * alpha
            + triangle.uvb.as_vector() * beta
            + triangle.uvc.as_vector() * gamma;

        vec![(t, SurfacePoint::new(p, n, uv.as_point()))]
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::math::Normal3;

    macro_rules! new_triangle {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let a = Point3::new(1 as $type, 2 as $type, 3 as $type);
                let b = Point3::new(4 as $type, 5 as $type, 6 as $type);
                let c = Point3::new(7 as $type, 8 as $type, 9 as $type);

                let na = Normal3::new(1 as $type, 0 as $type, 0 as $type);
                let nb = Normal3::new(0 as $type, 1 as $type, 0 as $type);
                let nc = Normal3::new(0 as $type, 0 as $type, 1 as $type);

                let uva = Point2::new(0 as $type, 0 as $type);
                let uvb = Point2::new(1 as $type, 0 as $type);
                let uvc = Point2::new(0 as $type, 1 as $type);

                let triangle = Triangle3::new(a, b, c, na, nb, nc, uva, uvb, uvc);

                assert_eq!(triangle.a, a);
                assert_eq!(triangle.b, b);
                assert_eq!(triangle.c, c);

                assert_eq!(triangle.na, na);
                assert_eq!(triangle.nb, nb);
                assert_eq!(triangle.nc, nc);

                assert_eq!(triangle.uva, uva);
                assert_eq!(triangle.uvb, uvb);
                assert_eq!(triangle.uvc, uvc);
            }
        };
    }

    new_triangle! { u8, new_triangle_u8 }
    new_triangle! { u16, new_triangle_u16 }
    new_triangle! { u32, new_triangle_u32 }
    new_triangle! { u64, new_triangle_u64 }
    new_triangle! { u128, new_triangle_u128 }
    new_triangle! { i8, new_triangle_i8 }
    new_triangle! { i16, new_triangle_i16 }
    new_triangle! { i32, new_triangle_i32 }
    new_triangle! { i64, new_triangle_i64 }
    new_triangle! { i128, new_triangle_i128 }
    new_triangle! { f32, new_triangle_f32 }
    new_triangle! { f64, new_triangle_f64 }

    macro_rules! parametric_line_intersect_triangle {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let n = Normal3::new(0 as $type, 0 as $type, -1 as $type);
                let triangle = Triangle3::new(
                    Point3::new(-1 as $type, 1 as $type, -2 as $type),
                    Point3::new(1 as $type, 1 as $type, -2 as $type),
                    Point3::new(1 as $type, -1 as $type, -2 as $type),
                    n,
                    n,
                    n,
                    Point2::new(0 as $type, 0 as $type),
                    Point2::new(1 as $type, 0 as $type),
                    Point2::new(0 as $type, 1 as $type),
                );

                let line1 = ParametricLine::new(
                    Point3::new(0 as $type, 0 as $type, 0 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type),
                );

                let line2 = ParametricLine::new(
                    Point3::new(-1 as $type, 1 as $type, 0 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type),
                );

                let line3 = ParametricLine::new(
                    Point3::new(1 as $type, 1 as $type, 0 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type),
                );

                let line4 = ParametricLine::new(
                    Point3::new(1 as $type, -1 as $type, 0 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type),
                );

                let line5 = ParametricLine::new(
                    Point3::new(-1 as $type, -1 as $type, 0 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type),
                );

                assert_eq!(
                    line1.intersect(triangle),
                    vec![(
                        2 as $type,
                        SurfacePoint::new(
                            Point3::new(0 as $type, 0 as $type, -2 as $type),
                            n,
                            Point2::new(0 as $type, 0.5 as $type)
                        )
                    )]
                );
                assert_eq!(
                    line2.intersect(triangle),
                    vec![(
                        2 as $type,
                        SurfacePoint::new(
                            Point3::new(-1 as $type, 1 as $type, -2 as $type),
                            n,
                            Point2::new(0 as $type, 0 as $type)
                        )
                    )]
                );
                assert_eq!(
                    line3.intersect(triangle),
                    vec![(
                        2 as $type,
                        SurfacePoint::new(
                            Point3::new(1 as $type, 1 as $type, -2 as $type),
                            n,
                            Point2::new(1.0 as $type, 0 as $type)
                        )
                    )]
                );
                assert_eq!(
                    line4.intersect(triangle),
                    vec![(
                        2 as $type,
                        SurfacePoint::new(
                            Point3::new(1 as $type, -1 as $type, -2 as $type),
                            n,
                            Point2::new(0 as $type, 1.0 as $type)
                        )
                    )]
                );
                assert_eq!(line5.intersect(triangle), Vec::new());
            }
        };
    }

    parametric_line_intersect_triangle! { f32, parametric_line_intersect_triangle_f32 }
    parametric_line_intersect_triangle! { f64, parametric_line_intersect_triangle_f64 }
}
