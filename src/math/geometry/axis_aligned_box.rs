use std::ops;

use super::Intersect;
use super::ParametricLine;
use super::ImplicitPlane3;

use crate::math::Vector3;
use crate::math::Orthonormal3;
use crate::math::Point3;

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
    T: ops::Neg<Output = T> + Copy + Clone + PartialOrd,
    Vector3<T>: Orthonormal3<T> + ops::Mul,
    <Vector3<T> as ops::Mul>::Output: ops::Div<Output = T> + PartialEq + Default,
    Point3<T>: ops::Sub<Output = Vector3<T>>,
    Vector3<T>: ops::Mul<T, Output = Vector3<T>>,
    Point3<T>: ops::Add<Vector3<T>, Output = Point3<T>>,
    {
    type Output = T;

    fn intersect(self, aab: AxisAlignedBox<Point3<T>>) -> Vec<Self::Output> {
        let left = ImplicitPlane3::new( aab.a, -Vector3::<T>::x_axis());
        let lower = ImplicitPlane3::new( aab.a, -Vector3::<T>::y_axis());
        let far = ImplicitPlane3::new( aab.a, -Vector3::<T>::z_axis());

        let right = ImplicitPlane3::new( aab.b, Vector3::<T>::x_axis());
        let upper = ImplicitPlane3::new( aab.b, Vector3::<T>::y_axis());
        let near = ImplicitPlane3::new( aab.b, Vector3::<T>::z_axis());

        let mut results: Vec<T> = Vec::new();

        let t = self.intersect(left);

        if t.len() > 0 {
            let p = self.at(t[0]);

            if p.y > aab.a.y && p.y < aab.b.y &&
               p.z > aab.a.z && p.z < aab.b.z {
                results.push(t[0]);
            }
        }
        
        let t = self.intersect(right);

        if t.len() > 0 {
            let p = self.at(t[0]);

            if p.y > aab.a.y && p.y < aab.b.y &&
               p.z > aab.a.z && p.z < aab.b.z {
                results.push(t[0]);
            }
        }

        let t = self.intersect(lower);

        if t.len() > 0 {
            let p = self.at(t[0]);

            if p.x > aab.a.x && p.x < aab.b.x &&
               p.z > aab.a.z && p.z < aab.b.z {
                results.push(t[0]);
            }
        }

        let t = self.intersect(upper);

        if t.len() > 0 {
            let p = self.at(t[0]);

            if p.x > aab.a.x && p.x < aab.b.x &&
               p.z > aab.a.z && p.z < aab.b.z {
                results.push(t[0]);
            }
        }

        let t = self.intersect(near);

        if t.len() > 0 {
            let p = self.at(t[0]);

            if p.x > aab.a.x && p.x < aab.b.x &&
               p.y > aab.a.z && p.y < aab.b.y {
                results.push(t[0]);
            }
        }

        let t = self.intersect(far);

        if t.len() > 0 {
            let p = self.at(t[0]);

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

                assert_eq!(ray1.intersect(aab), vec![3 as $type, 7 as $type]);
                assert_eq!(ray2.intersect(aab), vec![5 as $type, 9 as $type]);
                assert_eq!(ray3.intersect(aab), Vec::new());
            }
        }
    }

    parametric_line_intersect_axis_aligned_box_3! { f32, parametric_line_intersect_axis_aligned_box_3_f32 }
    parametric_line_intersect_axis_aligned_box_3! { f64, parametric_line_intersect_axis_aligned_box_3_f64 }
}
