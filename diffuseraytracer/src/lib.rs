use std::ops::Mul;

use cg_basics::{AxisAlignedBox, Cylinder, Disc, Plane, Sphere, Triangle};
use colors::Color;
use image::Image;
use math::geometry::{Intersect, ParametricLine, SurfacePoint};
use math::transform::Transform3;
use math::{Point2, Point3, Vector3};
use scene_graph::Geometry;
use traits::{Number, Sqrt};
use units::length::Length;

use scene_graph::RenderableGeometry;

pub mod camera;
pub mod diffuse_ray_tracer;
pub mod light;
pub mod material;

pub trait Renderable<T: Length, C: Color<ChannelType = T::ValueType>> {
    fn intersect(
        &self,
        ray: ParametricLine<Point3<T>, Vector3<T>>,
    ) -> Vec<(
        T::ValueType,
        SurfacePoint<T>,
        &scene_graph::Material<Box<dyn Image<ColorType = C, PointType = Point2<T::ValueType>>>>,
    )>;
}

impl<T: Length, C: Color<ChannelType = T::ValueType>> Renderable<T, C>
    for RenderableGeometry<
        Geometry<T>,
        scene_graph::Material<Box<dyn Image<ColorType = C, PointType = Point2<T::ValueType>>>>,
        Transform3<T::ValueType>,
    >
where
    T: Copy + Clone,
    T::ValueType: Number + Mul<T, Output = T> + Sqrt<Output = T::ValueType>,
    ParametricLine<Point3<T>, Vector3<T>>: Intersect<AxisAlignedBox<T>, Output = Vec<(T::ValueType, SurfacePoint<T>)>>
        + Intersect<Cylinder<T>, Output = Vec<(T::ValueType, SurfacePoint<T>)>>
        + Intersect<Disc<T>, Output = Vec<(T::ValueType, SurfacePoint<T>)>>
        + Intersect<Plane<T>, Output = Vec<(T::ValueType, SurfacePoint<T>)>>
        + Intersect<Sphere<T>, Output = Vec<(T::ValueType, SurfacePoint<T>)>>
        + Intersect<Triangle<T>, Output = Vec<(T::ValueType, SurfacePoint<T>)>>,
{
    fn intersect(
        &self,
        ray: ParametricLine<Point3<T>, Vector3<T>>,
    ) -> Vec<(
        T::ValueType,
        SurfacePoint<T>,
        &scene_graph::Material<Box<dyn Image<ColorType = C, PointType = Point2<T::ValueType>>>>,
    )> {
        let transformed_ray = ParametricLine::new(
            self.transform.inverse * ray.origin,
            self.transform.inverse * ray.direction,
        );

        let mut hits: Vec<(
            T::ValueType,
            SurfacePoint<T>,
            &scene_graph::Material<Box<dyn Image<ColorType = C, PointType = Point2<T::ValueType>>>>,
        )> = match self.geometry {
            Geometry::AxisAlignedBox(g) => transformed_ray.intersect(g),
            Geometry::Cylinder(g) => transformed_ray.intersect(g),
            Geometry::Disc(g) => transformed_ray.intersect(g),
            Geometry::Plane(g) => transformed_ray.intersect(g),
            Geometry::Sphere(g) => transformed_ray.intersect(g),
            Geometry::Triangle(g) => transformed_ray.intersect(g),
        }
        .iter()
        .map(|t| (t.0, t.1, &self.material))
        .collect();
        let transposed_inverse = self.transform.inverse.transposed();

        hits = hits
            .iter()
            .map(|(t, sp, m)| {
                (
                    *t,
                    SurfacePoint::new(
                        self.transform.matrix * sp.p,
                        transposed_inverse * sp.n,
                        sp.uv,
                    ),
                    *m,
                )
            })
            .collect();

        hits
    }
}
