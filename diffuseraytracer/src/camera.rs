use std::ops::{Div, Mul, Neg};

use math::geometry::ParametricLine;
use math::{Point2, Point3, Vector2, Vector3};

use sampling::SamplingPattern;
use scene_graph::Camera;
use traits::{ConvenientNumber, Cos, FloatingPoint, Number, SelfMulNumber, Sin, Sqrt};
use units::angle::{Angle, Radians};

pub trait RaytracingCamera<T>
where
    T: Div,
{
    fn ray_for(
        &self,
        size: Vector2<<T as Div>::Output>,
        p: Point2<<T as Div>::Output>,
        pattern: &SamplingPattern<Point2<<T as Div>::Output>>,
    ) -> Option<ParametricLine<Point3<T>, Vector3<T>>>;
}

impl<T> RaytracingCamera<T> for Camera<T>
where
    T: ConvenientNumber + Neg<Output = T> + SelfMulNumber<<T as Div>::Output>,
    <T as Div>::Output: FloatingPoint + ConvenientNumber,
    <T as Mul>::Output: Number<<T as Div>::Output> + ConvenientNumber + Sqrt<Output = T>,
    T: Number<<T as Div>::Output> + ConvenientNumber,
    <T as Div>::Output: FloatingPoint + ConvenientNumber + Mul<T, Output = T>,
    Radians<<T as Div>::Output>:
        Angle + Cos<Output = <T as Div>::Output> + Sin<Output = <T as Div>::Output>,
{
    fn ray_for(
        &self,
        size: Vector2<<T as Div>::Output>,
        p: Point2<<T as Div>::Output>,
        pattern: &SamplingPattern<Point2<<T as Div>::Output>>,
    ) -> Option<ParametricLine<Point3<T>, Vector3<T>>> {
        match self {
            Camera::FisheyeCamera(cam) => cam.ray_for(size, p, pattern),
            Camera::OrthigraphicCamera(cam) => cam.ray_for(size, p, pattern),
            Camera::PerspectiveCamera(cam) => cam.ray_for(size, p, pattern),
            Camera::PinholeCamera(cam) => cam.ray_for(size, p, pattern),
            Camera::SphericalCamera(cam) => cam.ray_for(size, p, pattern),
        }
    }
}

mod fisheye_camera;
mod orthographic_camera;
mod perspective_camera;
mod pinhole_camera;
mod spherical_camera;
