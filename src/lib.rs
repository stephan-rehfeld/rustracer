pub mod camera;
pub mod math;
pub mod traits;
pub mod units;
pub mod color;

use math::geometry::Intersect;

pub type Ray<T> = math::geometry::ParametricLine<math::Point3<T>, math::Vector3<T>>;

pub trait Geometry<T> {
    fn intersect(&self, ray: Ray<T>) -> Vec<(T,color::RGB<T>)>;
}

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct RenderableGeometry<G, T> {
    geometry: G,
    color: color::RGB<T>,
}

impl<G, T> RenderableGeometry<G, T> {
    pub fn new(geometry: G, color: color::RGB<T>) -> RenderableGeometry<G, T> {
        RenderableGeometry { geometry, color }
    }
}

impl<G, T> Geometry<T> for RenderableGeometry<G, T>
    where
        Ray<T>: math::geometry::Intersect<G, Output = Vec<T>>,
        G: Copy + Clone,
        T: Copy + Clone,
{
    fn intersect(&self, ray: Ray<T>) -> Vec<(T, color::RGB<T>)> {
        ray.intersect(self.geometry).iter().map(|t| (*t, self.color)).collect()
    }
}
