use std::ops;
use crate::math::Vector3;
use crate::math::Point3;
use crate::traits;
use crate::traits::Half;
use crate::traits::Tan;
use crate::units::angle;
use crate::math::geometry::ParametricLine;

pub trait RaytracingCamera<T> {
    fn ray_for(&self, x: T, y: T) -> ParametricLine<Point3<T>, Vector3<T>>;
}

pub struct Orthographic<T> where T: ops::Div {
    e: Point3<T>,
    u: Vector3<T>,
    v: Vector3<T>,
    w: Vector3<T>,
    scale: T,
    width: T,
    height: T,
    aspect_ratio: <T as ops::Div>::Output,
}

impl<T> Orthographic<T> where
    T: ops::Div<Output = T>,
    T: ops::Mul<Output = T>,
    T: ops::Add<Output = T>,
    T: ops::Sub<Output = T>,
    T: ops::Neg<Output = T>,
    T: traits::Sqrt<Output = T>,
    T: Clone + Copy,
{
    pub fn new(e: Point3<T>, g: Vector3<T>, t: Vector3<T>, scale: T, width: T, height: T) -> Orthographic<T> 
    {
          let w = -g.normalized();
          let u = Vector3::cross(t, w).normalized();
          let v = Vector3::cross(w, u);

        let aspect_ratio = width/height;

        Orthographic { e, u, v, w, width, height, scale, aspect_ratio }
    }
}

impl<T> RaytracingCamera<T> for Orthographic<T> where
    T: ops::Div<Output = T>,
    T: ops::Mul<Vector3<T>, Output=Vector3<T>>,
    T: ops::Mul<Output = T>,
    T: ops::Add<Output = T>,
    T: ops::Sub<Output = T>,
    T: ops::Neg<Output = T>,
    T: traits::Sqrt<Output = T>,
    T: traits::Half,
    T: Clone + Copy,
{
    fn ray_for(&self, x: T, y: T) -> ParametricLine<Point3<T>, Vector3<T>> {
        let d = -self.w;

        let x_factor = x - (self.width.half() / self.width);
        let y_factor = y - (self.height.half() / self.height);

        let o = self.e + self.aspect_ratio * self.scale * x_factor * self.u + self.scale * y_factor * self.v;

        ParametricLine::new(o, d)
    }
}

pub struct Perspective<T> {
    e: Point3<T>,
    u: Vector3<T>,
    v: Vector3<T>,
    w: Vector3<T>,
    vertical_field_of_view: angle::Radians<T>,
    width: T,
    height: T 
}

impl<T> Perspective<T> where
    T: ops::Div<Output = T>,
    T: ops::Mul<Vector3<T>, Output=Vector3<T>>,
    T: ops::Mul<Output = T>,
    T: ops::Add<Output = T>,
    T: ops::Sub<Output = T>,
    T: ops::Neg<Output = T>,
    T: traits::Sqrt<Output = T>,
    T: traits::Half,
    T: Clone + Copy,
{
    pub fn new(e: Point3<T>, g: Vector3<T>, t: Vector3<T>, vertical_field_of_view: angle::Radians<T>, width: T, height: T) -> Perspective<T> {
        let w = -g.normalized();
        let u = Vector3::cross(t, w).normalized();
        let v = Vector3::cross(w, u);

        let vertical_field_of_view = vertical_field_of_view.half();

        Perspective { e, u, v, w, vertical_field_of_view, width, height }
    }
}

impl<T> RaytracingCamera<T> for Perspective<T> where
    T: ops::Add<Output = T>,
    T: ops::Sub<Output = T>,
    T: ops::Mul<Output = T>,
    T: ops::Mul<Vector3<T>, Output=Vector3<T>>,
    T: ops::Div<Output = T>,
    T: traits::Sqrt<Output = T>,
    T: traits::Half,
    T: traits::Tan<Output = T>,
    T: ops::Neg<Output = T>,
    T: Clone + Copy,
{
    fn ray_for(&self, x: T, y: T ) -> ParametricLine<Point3<T>, Vector3<T>> {
        let o = self.e;

        let a = -self.w * self.height.half()/self.vertical_field_of_view.tan(); 
        let b = (x - self.width.half()) * self.u;
        let c = (y - self.height.half()) * self.v;

        let r = a + b + c; 
        let d = r.normalized();

        ParametricLine::new(o, d)
    }
}

