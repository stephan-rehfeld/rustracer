use std::ops::{Add, Div, Mul, Sub};

use math::{Point3, Vector3};
use traits::{ConvenientNumber, FloatingPoint, Half, Number, SelfMulNumber, Sqrt, Zero};
use units::angle::Radians;

pub struct SphericalCamera<T>
where
    T: Div,
{
    pub e: Point3<T>,
    pub u: Vector3<<T as Div>::Output>,
    pub v: Vector3<<T as Div>::Output>,
    pub w: Vector3<<T as Div>::Output>,
    pub vertical_field_of_view: Radians<<T as Div>::Output>,
}

impl<T> SphericalCamera<T>
where
    T: Number<<T as Div>::Output> + SelfMulNumber<<T as Div>::Output>,
    <T as Div>::Output: FloatingPoint<<T as Div>::Output> + ConvenientNumber,
    <T as Mul>::Output: Add<Output = <T as Mul>::Output>
        + Sub<Output = <T as Mul>::Output>
        + Sqrt<Output = T>
        + Zero,
{
    pub fn new(
        e: Point3<T>,
        g: Vector3<T>,
        t: Vector3<T>,
        vertical_field_of_view: Radians<<T as Div>::Output>,
    ) -> SphericalCamera<T> {
        let w = -g.normalized();
        let u = Vector3::cross(t, w).normalized();
        let v = Vector3::cross(w, u).normalized();

        let vertical_field_of_view = vertical_field_of_view.half();

        SphericalCamera {
            e,
            u,
            v,
            w,
            vertical_field_of_view,
        }
    }
}
