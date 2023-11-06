pub mod camera;
pub mod math;
pub mod traits;
pub mod units;
pub mod color;

use std::ops;

use math::geometry::Intersect;

pub type Ray<T> = math::geometry::ParametricLine<math::Point3<T>, math::Vector3<T>>;
pub type NormalType<T> = <math::Vector3<T> as math::NormalizableVector>::NormalType;

pub trait Renderable<T> where
    T: ops::Div + ops::Mul + Copy + Clone,
{

    fn intersect(&self, ray: Ray<T>) -> Vec<(T,NormalType<T>, color::RGB<T>)>;
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

impl<G, T> Renderable<T> for RenderableGeometry<G, T>
    where
        T: ops::Div + ops::Mul,
        Ray<T>: math::geometry::Intersect<G, Output = Vec<(T, NormalType<T>)>>,
        NormalType<T>: Copy + Clone,
        G: Copy + Clone,
        T: Copy + Clone,
{
    fn intersect(&self, ray: Ray<T>) -> Vec<(T, NormalType<T>, color::RGB<T>)> {
        ray.intersect(self.geometry).iter().map(|t| (t.0, t.1, self.color)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use math::Normal3;

    #[derive(Debug,PartialEq,Clone,Copy)]
    struct MockGeometry<T> where
        T: ops::Div + ops::Mul + Copy + Clone,
        <T as ops::Div>::Output: Copy + Clone + std::fmt::Debug + PartialEq,
    {
        t: T,
        normal: NormalType<T>
    }

    impl<T> math::geometry::Intersect<MockGeometry<T>> for Ray<T> where
        T: ops::Div + ops::Mul + Copy + Clone,
        <T as ops::Div>::Output: Copy + Clone + std::fmt::Debug + PartialEq,
    {
        type Output = Vec<(T, NormalType<T>)>;

        fn intersect(self, other: MockGeometry<T>) -> Self::Output {
            vec![(other.t, other.normal)]
        }
    }

    macro_rules! new_renderable_geometry {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let g = MockGeometry { t: 1.0 as $type, normal: Normal3::new(0 as $type, 1 as $type, 0 as $type)};
                let c = color::RGB::new(0.0 as $type, 0.5 as $type, 1.0 as $type);

                let rg = RenderableGeometry::new(g, c);

                assert_eq!(rg.geometry, g);
                assert_eq!(rg.color, c);
            }
        }
    }

    new_renderable_geometry! { f32, new_renderable_geometry_f32 }
    new_renderable_geometry! { f64, new_renderable_geometry_f64 }

    macro_rules! renderable_geometry_intersect {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let v = 25.0 as $type;
                let n = Normal3::new(0 as $type, 1 as $type, 0 as $type);
                let g = MockGeometry { t: v, normal: n };
                let c = color::RGB::new(0.0 as $type, 0.5 as $type, 1.0 as $type);

                let ray = Ray::new(
                    math::Point3::new(0 as $type, 0 as $type, 0 as $type),
                    math::Vector3::new(0 as $type, 0 as $type, -1 as $type)
                );

                let rg = RenderableGeometry::new(g, c);

                assert_eq!(rg.intersect(ray), vec![(v, n, c)]);
            }
        }
    }

    renderable_geometry_intersect! { f32, renderable_geometry_intersect_f32 }
    renderable_geometry_intersect! { f64, renderable_geometry_intersect_f64 }
}
