use std::ops::{Add, Mul, Neg};

use crate::math::Mat4x4;
use crate::traits::{Cos, One, Recip, Sin, Zero};

pub struct Transform3<T> {
    pub matrix: Mat4x4<T>,
    pub inverse: Mat4x4<T>,
}

impl<T: One + Zero> Transform3<T> {
    pub fn ident() -> Transform3<T> {
        Transform3 {
            matrix: Mat4x4::ident(),
            inverse: Mat4x4::ident(),
        }
    }
}

impl<T> Transform3<T>
where
    T: Add<Output = T> + Neg<Output = T> + Mul<Output = T> + Recip + Zero + One + Copy,
{
    pub fn scale(self, x: T, y: T, z: T) -> Transform3<T> {
        let matrix = Mat4x4::new(
            x,
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            y,
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            z,
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        );

        let inverse = Mat4x4::new(
            x.recip(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            y.recip(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            z.recip(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        );

        Transform3 {
            matrix: self.matrix * matrix,
            inverse: inverse * self.inverse,
        }
    }

    pub fn translate(self, x: T, y: T, z: T) -> Transform3<T> {
        let matrix = Mat4x4::new(
            One::one(),
            Zero::zero(),
            Zero::zero(),
            x,
            Zero::zero(),
            One::one(),
            Zero::zero(),
            y,
            Zero::zero(),
            Zero::zero(),
            One::one(),
            z,
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        );

        let inverse = Mat4x4::new(
            One::one(),
            Zero::zero(),
            Zero::zero(),
            -x,
            Zero::zero(),
            One::one(),
            Zero::zero(),
            -y,
            Zero::zero(),
            Zero::zero(),
            One::one(),
            -z,
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        );

        Transform3 {
            matrix: self.matrix * matrix,
            inverse: inverse * self.inverse,
        }
    }

    pub fn rotate_x<A: Cos<Output = T> + Sin<Output = T> + Copy>(self, angle: A) -> Transform3<T> {
        let matrix = Mat4x4::new(
            One::one(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            angle.cos(),
            -angle.sin(),
            Zero::zero(),
            Zero::zero(),
            angle.sin(),
            angle.cos(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        );

        let inverse = Mat4x4::new(
            One::one(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            angle.cos(),
            angle.sin(),
            Zero::zero(),
            Zero::zero(),
            -angle.sin(),
            angle.cos(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        );

        Transform3 {
            matrix: self.matrix * matrix,
            inverse: inverse * self.inverse,
        }
    }

    pub fn rotate_y<A: Cos<Output = T> + Sin<Output = T> + Copy>(self, angle: A) -> Transform3<T> {
        let matrix = Mat4x4::new(
            angle.cos(),
            Zero::zero(),
            -angle.sin(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
            Zero::zero(),
            Zero::zero(),
            angle.sin(),
            Zero::zero(),
            angle.cos(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        );

        let inverse = Mat4x4::new(
            angle.cos(),
            Zero::zero(),
            angle.sin(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
            Zero::zero(),
            Zero::zero(),
            -angle.sin(),
            Zero::zero(),
            angle.cos(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        );

        Transform3 {
            matrix: self.matrix * matrix,
            inverse: inverse * self.inverse,
        }
    }

    pub fn rotate_z<A: Cos<Output = T> + Sin<Output = T> + Copy>(self, angle: A) -> Transform3<T> {
        let matrix = Mat4x4::new(
            angle.cos(),
            -angle.sin(),
            Zero::zero(),
            Zero::zero(),
            angle.sin(),
            angle.cos(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        );

        let inverse = Mat4x4::new(
            angle.cos(),
            angle.sin(),
            Zero::zero(),
            Zero::zero(),
            -angle.sin(),
            angle.cos(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        );

        Transform3 {
            matrix: self.matrix * matrix,
            inverse: inverse * self.inverse,
        }
    }
}
