use std::ops::{Div, Mul};

use math::{Point3, Vector3};
use traits::{ConvenientNumber, FloatingPoint, Half, Number, SelfMulNumber, Sqrt};
use units::angle::Radians;

pub struct FisheyeCamera<T>
where
    T: Div,
{
    pub e: Point3<T>,
    pub u: Vector3<<T as Div>::Output>,
    pub v: Vector3<<T as Div>::Output>,
    pub w: Vector3<<T as Div>::Output>,
    pub psi: Radians<<T as Div>::Output>,
}

impl<T> FisheyeCamera<T>
where
    T: Number<<T as Div>::Output> + SelfMulNumber<<T as Div>::Output>,
    <T as Div>::Output: FloatingPoint + ConvenientNumber,
    <T as Mul>::Output: Number<<T as Div>::Output> + ConvenientNumber + Sqrt<Output = T>,
{
    pub fn new(
        e: Point3<T>,
        g: Vector3<T>,
        t: Vector3<T>,
        psi: Radians<<T as Div>::Output>,
    ) -> FisheyeCamera<T> {
        let w = -g.normalized();
        let u = Vector3::cross(t, w).normalized();
        let v = Vector3::cross(w, u).normalized();

        FisheyeCamera {
            e,
            u,
            v,
            w,
            psi: psi.half(),
        }
    }
}
