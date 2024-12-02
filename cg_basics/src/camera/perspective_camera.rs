use std::ops::{Div, Mul};

use math::{Point3, Vector3};
use traits::{ConvenientNumber, FloatingPoint, Half, Number, SelfMulNumber, Sqrt};
use units::angle::Radians;

pub struct PerspectiveCamera<T>
where
    T: Div,
{
    pub e: Point3<T>,
    pub u: Vector3<<T as Div>::Output>,
    pub v: Vector3<<T as Div>::Output>,
    pub w: Vector3<<T as Div>::Output>,
    pub vertical_field_of_view: Radians<<T as Div>::Output>,
    pub lens_radius: T,
    pub focal_length: T,
}

impl<T> PerspectiveCamera<T>
where
    T: SelfMulNumber<<T as Div>::Output>,
    <T as Div>::Output: FloatingPoint + ConvenientNumber,
    <T as Mul>::Output: Number<<T as Div>::Output> + ConvenientNumber + Sqrt<Output = T>,
{
    pub fn new(
        e: Point3<T>,
        g: Vector3<T>,
        t: Vector3<T>,
        vertical_field_of_view: Radians<<T as Div>::Output>,
        lens_radius: T,
        focal_length: T,
    ) -> PerspectiveCamera<T> {
        let w = -g.normalized();
        let u = Vector3::cross(t, w).normalized();
        let v = Vector3::cross(w, u).normalized();

        let vertical_field_of_view = vertical_field_of_view.half();

        PerspectiveCamera {
            e,
            u,
            v,
            w,
            vertical_field_of_view,
            lens_radius,
            focal_length,
        }
    }
}
