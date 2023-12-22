use std::fmt::Debug;
use std::ops::{Add, Div, Neg, Mul, Sub};

use super::{ImplicitPlane3, Intersect, ParametricLine};

use crate::math::{Normal3, NormalizableVector, Point3, Vector3};
use crate::math::normal::Orthonormal3;
use crate::traits::Zero;

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

impl<T> Intersect<AxisAlignedBox<Point3<T>>> for ParametricLine<Point3<T>, Vector3<T>> where
    T: Mul + Mul<<T as Div>::Output, Output=T> + Div + Add<Output=T> + Copy + Clone + PartialOrd,
    T: Mul<<T as Div>::Output, Output=T>,
    <T as Div>::Output: Neg<Output=<T as Div>::Output> + Debug + PartialEq + Clone + Copy,
    <T as Mul>::Output: Add<Output=<T as Mul>::Output> + Div +  PartialEq + Zero,
    <T as Mul<<T as Div>::Output>>::Output: PartialEq,
    <T as Mul< <T as Div>::Output >>::Output: Add<Output=<T as Mul< <T as Div>::Output  >>::Output> + Zero,
    Normal3<<T as Div>::Output>: Orthonormal3<<T as Div>::Output>,
    Point3<T>: Sub<Output=Vector3<T>>,
    
{
    type Output = Vec<(<<T as Mul<<T as Div>::Output>>::Output as Div>::Output, <Vector3<T> as NormalizableVector>::NormalType)>;

    fn intersect(self, aab: AxisAlignedBox<Point3<T>>) -> Self::Output {
        let left = ImplicitPlane3::new( aab.a, -Normal3::x_axis());
        let lower = ImplicitPlane3::new( aab.a, -Normal3::y_axis());
        let far = ImplicitPlane3::new( aab.a, -Normal3::z_axis());

        let right = ImplicitPlane3::new( aab.b, Normal3::x_axis());
        let upper = ImplicitPlane3::new( aab.b, Normal3::y_axis());
        let near = ImplicitPlane3::new( aab.b, Normal3::z_axis());


        let mut results: Vec<(<<T as Mul<<T as Div>::Output>>::Output as Div>::Output, <Vector3<T> as NormalizableVector>::NormalType)> = Vec::new();

        let t = self.intersect(left);

        if t.len() > 0 {
            let p = self.at(t[0].0);

            if p.y > aab.a.y && p.y < aab.b.y &&
               p.z > aab.a.z && p.z < aab.b.z {
                results.push(t[0]);
            }
        }
        
        let t = self.intersect(right);

        if t.len() > 0 {
            let p = self.at(t[0].0);

            if p.y > aab.a.y && p.y < aab.b.y &&
               p.z > aab.a.z && p.z < aab.b.z {
                results.push(t[0]);
            }
        }

        let t = self.intersect(lower);

        if t.len() > 0 {
            let p = self.at(t[0].0);

            if p.x > aab.a.x && p.x < aab.b.x &&
               p.z > aab.a.z && p.z < aab.b.z {
                results.push(t[0]);
            }
        }

        let t = self.intersect(upper);

        if t.len() > 0 {
            let p = self.at(t[0].0);

            if p.x > aab.a.x && p.x < aab.b.x &&
               p.z > aab.a.z && p.z < aab.b.z {
                results.push(t[0]);
            }
        }

        let t = self.intersect(near);

        if t.len() > 0 {
            let p = self.at(t[0].0);

            if p.x > aab.a.x && p.x < aab.b.x &&
               p.y > aab.a.z && p.y < aab.b.y {
                results.push(t[0]);
            }
        }

        let t = self.intersect(far);

        if t.len() > 0 {
            let p = self.at(t[0].0);

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

    macro_rules! new_axis_aligned_box3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let a = Point3::new( 1 as $type, 2 as $type, 3 as $type );
                let b = Point3::new( 4 as $type, 5 as $type, 6 as $type );

                let aab = AxisAlignedBox::new(a, b);

                assert_eq!(aab.a, a);
                assert_eq!(aab.b, b);
            }
        }
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
                    Vector3::new(0 as $type, 0 as $type, -1 as $type)
                );
                
                let ray2 = ParametricLine::new(
                    Point3::new(1 as $type, 1 as $type, 7 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type)
                );
                
                let ray3 = ParametricLine::new(
                    Point3::new(6 as $type, 0 as $type, 5 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type)
                );

                let aab = AxisAlignedBox::new(
                    Point3::new(-2 as $type, -2 as $type, -2 as $type),
                    Point3::new(2 as $type, 2 as $type, 2 as $type)
                );

                assert_eq!(ray1.intersect(aab), vec![(3 as $type, Normal3::z_axis()), (7 as $type, -Normal3::<$type>::z_axis())]);
                assert_eq!(ray2.intersect(aab), vec![(5 as $type, Normal3::z_axis()), (9 as $type, -Normal3::<$type>::z_axis())]);
                assert_eq!(ray3.intersect(aab), Vec::new());
            }
        }
    }

    parametric_line_intersect_axis_aligned_box_3! { f32, parametric_line_intersect_axis_aligned_box_3_f32 }
    parametric_line_intersect_axis_aligned_box_3! { f64, parametric_line_intersect_axis_aligned_box_3_f64 }
}
