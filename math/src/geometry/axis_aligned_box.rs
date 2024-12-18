use std::fmt::Debug;
use std::ops::{Div, Mul, Sub};

use super::{ImplicitPlane3, Intersect, ParametricLine, SurfacePoint};

use crate::{Normal3, Orthonormal3, Point3, Vector3};
use traits::{FloatingPoint, Number, SelfMulNumber};

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
    T: SelfMulNumber<<T as Div>::Output>,
    <T as Div>::Output: FloatingPoint,
    <T as Mul>::Output: Number<<T as Div>::Output>,
    Normal3<<T as Div>::Output>: Orthonormal3,
    Point3<T>: Sub<Output = Vector3<T>>,
{
    type Output = Vec<(
        <<T as Mul<<T as Div>::Output>>::Output as Div>::Output,
        SurfacePoint<T>,
    )>;

    fn intersect(self, aab: AxisAlignedBox<Point3<T>>) -> Self::Output {
        let left = ImplicitPlane3::new(aab.a, -Normal3::x_axis(), Normal3::z_axis().as_vector());
        let lower = ImplicitPlane3::new(aab.a, -Normal3::y_axis(), Normal3::x_axis().as_vector());
        let far = ImplicitPlane3::new(aab.a, -Normal3::z_axis(), -Normal3::x_axis().as_vector());

        let right = ImplicitPlane3::new(aab.b, Normal3::x_axis(), -Normal3::z_axis().as_vector());
        let upper = ImplicitPlane3::new(aab.b, Normal3::y_axis(), -Normal3::x_axis().as_vector());
        let near = ImplicitPlane3::new(aab.b, Normal3::z_axis(), Normal3::x_axis().as_vector());

        let mut results: Vec<(
            <<T as Mul<<T as Div>::Output>>::Output as Div>::Output,
            SurfacePoint<T>,
        )> = Vec::new();

        let mut t = self.intersect(left);

        if t.len() > 0 {
            let p = self.at(t[0].0);

            if p.y > aab.a.y && p.y < aab.b.y && p.z > aab.a.z && p.z < aab.b.z {
                results.push(t.remove(0));
            }
        }

        let mut t = self.intersect(right);

        if t.len() > 0 {
            let p = self.at(t[0].0);

            if p.y > aab.a.y && p.y < aab.b.y && p.z > aab.a.z && p.z < aab.b.z {
                results.push(t.remove(0));
            }
        }

        let mut t = self.intersect(lower);

        if t.len() > 0 {
            let p = self.at(t[0].0);

            if p.x > aab.a.x && p.x < aab.b.x && p.z > aab.a.z && p.z < aab.b.z {
                results.push(t.remove(0));
            }
        }

        let mut t = self.intersect(upper);

        if t.len() > 0 {
            let p = self.at(t[0].0);

            if p.x > aab.a.x && p.x < aab.b.x && p.z > aab.a.z && p.z < aab.b.z {
                results.push(t.remove(0));
            }
        }

        let mut t = self.intersect(near);

        if t.len() > 0 {
            let p = self.at(t[0].0);

            if p.x > aab.a.x && p.x < aab.b.x && p.y > aab.a.y && p.y < aab.b.y {
                results.push(t.remove(0));
            }
        }

        let mut t = self.intersect(far);

        if t.len() > 0 {
            let p = self.at(t[0].0);

            if p.x > aab.a.x && p.x < aab.b.x && p.y > aab.a.y && p.y < aab.b.y {
                results.push(t.remove(0));
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Point2;

    macro_rules! new_axis_aligned_box3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let a = Point3::new(1 as $type, 2 as $type, 3 as $type);
                let b = Point3::new(4 as $type, 5 as $type, 6 as $type);

                let aab = AxisAlignedBox::new(a, b);

                assert_eq!(aab.a, a);
                assert_eq!(aab.b, b);
            }
        };
    }

    new_axis_aligned_box3! { u8, new_axis_aligned_box3_u8 }
    new_axis_aligned_box3! { u16, new_axis_aligned_box3_u16 }
    new_axis_aligned_box3! { u32, new_axis_aligned_box3_u32 }
    new_axis_aligned_box3! { u64, new_axis_aligned_box3_u64 }
    new_axis_aligned_box3! { u128, new_axis_aligned_box3_u128 }
    new_axis_aligned_box3! { i8, new_axis_aligned_box3_i8 }
    new_axis_aligned_box3! { i16, new_axis_aligned_box3_i16 }
    new_axis_aligned_box3! { i32, new_axis_aligned_box3_i32 }
    new_axis_aligned_box3! { i64, new_axis_aligned_box3_i64 }
    new_axis_aligned_box3! { i128, new_axis_aligned_box3_i128 }
    new_axis_aligned_box3! { f32, new_axis_aligned_box3_f32 }
    new_axis_aligned_box3! { f64, new_axis_aligned_box3_f64 }

    macro_rules! parametric_line_intersect_axis_aligned_box_3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let ray1 = ParametricLine::new(
                    Point3::new(0 as $type, 0 as $type, 5 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type),
                );

                let ray2 = ParametricLine::new(
                    Point3::new(1 as $type, 1 as $type, 7 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type),
                );

                let ray3 = ParametricLine::new(
                    Point3::new(6 as $type, 0 as $type, 5 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type),
                );

                let aab = AxisAlignedBox::new(
                    Point3::new(-2 as $type, -2 as $type, -2 as $type),
                    Point3::new(2 as $type, 2 as $type, 2 as $type),
                );

                assert_eq!(
                    ray1.intersect(aab),
                    vec![
                        (
                            3 as $type,
                            SurfacePoint::new(
                                Point3::new(0 as $type, 0 as $type, 2 as $type),
                                Normal3::<$type>::z_axis(),
                                Point2::new(0 as $type, 0 as $type)
                            )
                        ),
                        (
                            7 as $type,
                            SurfacePoint::new(
                                Point3::new(0 as $type, 0 as $type, -2 as $type),
                                -Normal3::<$type>::z_axis(),
                                Point2::new(0 as $type, 0 as $type)
                            )
                        ),
                    ]
                );
                assert_eq!(
                    ray2.intersect(aab),
                    vec![
                        (
                            5 as $type,
                            SurfacePoint::new(
                                Point3::new(1 as $type, 1 as $type, 2 as $type),
                                Normal3::<$type>::z_axis(),
                                Point2::new(0 as $type, 0 as $type)
                            )
                        ),
                        (
                            9 as $type,
                            SurfacePoint::new(
                                Point3::new(1 as $type, 1 as $type, -2 as $type),
                                -Normal3::<$type>::z_axis(),
                                Point2::new(0 as $type, 0 as $type)
                            )
                        ),
                    ]
                );
                assert_eq!(ray3.intersect(aab), Vec::new());
            }
        };
    }

    parametric_line_intersect_axis_aligned_box_3! { f32, parametric_line_intersect_axis_aligned_box_3_f32 }
    parametric_line_intersect_axis_aligned_box_3! { f64, parametric_line_intersect_axis_aligned_box_3_f64 }
}
