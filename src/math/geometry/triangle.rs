use std::ops;

use super::Intersect;
use super::ParametricLine;

use crate::math::Mat3x3;
use crate::math::Point;
use crate::math::Point3;
use crate::math::NormalizableVector;
use crate::math::Vector3;
use crate::traits::One;
use crate::traits::Zero;
use crate::traits::Sqrt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Triangle<T: Point> where
    <T as Point>::VectorType: NormalizableVector,
    <<T as Point>::VectorType as NormalizableVector>::NormalType: PartialEq + Copy + std::fmt::Debug,
{
    a: T,
    b: T,
    c: T,
    na: <<T as Point>::VectorType as NormalizableVector>::NormalType,
    nb: <<T as Point>::VectorType as NormalizableVector>::NormalType,
    nc: <<T as Point>::VectorType as NormalizableVector>::NormalType,
}

impl<T: Point> Triangle<T> where
    <T as Point>::VectorType: NormalizableVector,
    <<T as Point>::VectorType as NormalizableVector>::NormalType: PartialEq + Copy + std::fmt::Debug,
{
    pub fn new(a: T, b: T, c: T, na: <<T as Point>::VectorType as NormalizableVector>::NormalType, nb: <<T as Point>::VectorType as NormalizableVector>::NormalType, nc: <<T as Point>::VectorType as NormalizableVector>::NormalType ) -> Triangle<T> {
        Triangle { a, b, c, na, nb, nc }
    }
}

impl<T> Intersect<Triangle<Point3<T>>> for ParametricLine<Point3<T>, Vector3<T>> where
        <T as ops::Div>::Output: ops::Add<Output=<T as ops::Div>::Output>
                               + ops::Sub<Output=<T as ops::Div>::Output>
                               + ops::Mul<Output=<T as ops::Div>::Output>
                               + ops::Div<Output=<T as ops::Div>::Output>
                               + ops::Neg<Output=<T as ops::Div>::Output>
                               + Sqrt<Output=<T as ops::Div>::Output>
                               + PartialOrd
                               + One
                               + Zero
                               + Copy
                               + std::fmt::Debug
                               + PartialEq,
        T: ops::Sub< Output = T> + ops::Mul + ops::Div + Copy ,
        <T as ops::Mul>::Output: ops::Mul<T>,
        <<T as ops::Mul>::Output as ops::Mul<T>>::Output: ops::Add<Output=<<T as ops::Mul>::Output as ops::Mul<T>>::Output>
                                                        + ops::Sub<Output=<<T as ops::Mul>::Output as ops::Mul<T>>::Output>
                                                        + ops::Div<Output=<T as ops::Div>::Output>,
/*        <<<T as ops::Mul>::Output as ops::Mul<T>>::Output as ops::Div>::Output: ops::Add<Output=<<<T as ops::Mul>::Output as ops::Mul<T>>::Output as ops::Div>::Output>
                                                                              + ops::Sub<Output=<<<T as ops::Mul>::Output as ops::Mul<T>>::Output as ops::Div>::Output>
                                                                              + ops::Neg<Output=<<<T as ops::Mul>::Output as ops::Mul<T>>::Output as ops::Div>::Output>

                                                                              + PartialOrd
                                                                              + One
                                                                              + Zero
                                                                              + Copy,*/
        <<T as ops::Mul>::Output as ops::Mul<T>>::Output: Zero 
                                                        + PartialEq 
                                                        + Copy, 
{
    type Output = Vec<(<T as ops::Div>::Output, <Vector3<T> as NormalizableVector>::NormalType)>; 

    fn intersect(self, triangle: Triangle<Point3<T>>) -> Self::Output 
        {
        let m = Mat3x3::from_vector3s( triangle.a - triangle.b, triangle.a - triangle.c, self.direction );
        let v = triangle.a - self.origin;

        let m_determinante = m.determinant();
        
        if m_determinante == Zero::zero() {
            return vec! { }
        }

        let m1 = m.change_column_1(v);

        let beta = m1.determinant() / m_determinante;

        if beta < Zero::zero() || beta > One::one() {
            return vec! { }
        }

        let m2 = m.change_column_2(v);

        let gamma = m2.determinant() / m_determinante;

        if gamma < Zero::zero() || gamma > One::one() {
            return vec! { }
        }

        if beta + gamma < Zero::zero() || beta + gamma > One::one() {
            return vec! { }
        }

        let m3 = m.change_column_3(v);

        let t = m3.determinant() / m_determinante;
        let alpha = -beta - gamma + One::one();
        
        let n = (triangle.na * alpha + triangle.nb * beta + triangle.nc * gamma).normalized();

        vec! { (t, n) }
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
                let a = Point3::new( 1 as $type, 2 as $type, 3 as $type ); 
                let b = Point3::new( 4 as $type, 5 as $type, 6 as $type ); 
                let c = Point3::new( 7 as $type, 8 as $type, 9 as $type ); 

                let na = Normal3::new(1 as $type, 0 as $type, 0 as $type);
                let nb = Normal3::new(0 as $type, 1 as $type, 0 as $type);
                let nc = Normal3::new(0 as $type, 0 as $type, 1 as $type);

                let triangle = Triangle::new(a, b, c, na, nb, nc);

                assert_eq!(triangle.a, a);
                assert_eq!(triangle.b, b);
                assert_eq!(triangle.c, c);

                assert_eq!(triangle.na, na);
                assert_eq!(triangle.nb, nb);
                assert_eq!(triangle.nc, nc);
            }
        }
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
                let triangle = Triangle::new(
                    Point3::new(-1 as $type, 1 as $type, -2 as $type),
                    Point3::new(1 as $type, 1 as $type, -2 as $type),
                    Point3::new(1 as $type, -1 as $type, -2 as $type),
                    n,
                    n,
                    n
                );

                let line1 = ParametricLine::new(
                    Point3::new(0 as $type, 0 as $type, 0 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type)
                );


                let line2 = ParametricLine::new(
                    Point3::new(-1 as $type, 1 as $type, 0 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type)
                );

                let line3 = ParametricLine::new(
                    Point3::new(1 as $type, 1 as $type, 0 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type)
                );

                let line4 = ParametricLine::new(
                    Point3::new(1 as $type, -1 as $type, 0 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type)
                );

                let line5 = ParametricLine::new(
                    Point3::new(-1 as $type, -1 as $type, 0 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type)
                );

                assert_eq!(line1.intersect(triangle),vec![(2 as $type, n)]);
                assert_eq!(line2.intersect(triangle),vec![(2 as $type, n)]);
                assert_eq!(line3.intersect(triangle),vec![(2 as $type, n)]);
                assert_eq!(line4.intersect(triangle),vec![(2 as $type, n)]);
                assert_eq!(line5.intersect(triangle),Vec::new());
            }
        }
    }

    parametric_line_intersect_triangle! { f32, parametric_line_intersect_triangle_f32 }
    parametric_line_intersect_triangle! { f64, parametric_line_intersect_triangle_f64 }
}
