pub mod camera;
pub mod math;
pub mod traits;
pub mod units;
pub mod color;

use math::geometry::Intersect;

pub type Ray<T> = math::geometry::ParametricLine<math::Point3<T>, math::Vector3<T>>;

pub trait Renderable<T> {
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

impl<G, T> Renderable<T> for RenderableGeometry<G, T>
    where
        Ray<T>: math::geometry::Intersect<G, Output = Vec<T>>,
        G: Copy + Clone,
        T: Copy + Clone,
{
    fn intersect(&self, ray: Ray<T>) -> Vec<(T, color::RGB<T>)> {
        ray.intersect(self.geometry).iter().map(|t| (*t, self.color)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug,PartialEq,Clone,Copy)]
    struct MockGeometry<T>
    {
        intersect_result: T,
    }

    impl<T> math::geometry::Intersect<MockGeometry<T>> for Ray<T> {
        type Output = Vec<T>;

        fn intersect(self, other: MockGeometry<T>) -> Self::Output {
            vec![other.intersect_result]
        }
    }

    macro_rules! new_renderable_geometry {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let g = MockGeometry { intersect_result: 1.0 as $type };
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
                let g = MockGeometry { intersect_result: v };
                let c = color::RGB::new(0.0 as $type, 0.5 as $type, 1.0 as $type);

                let ray = Ray::new(
                    math::Point3::new(0 as $type, 0 as $type, 0 as $type),
                    math::Vector3::new(0 as $type, 0 as $type, -1 as $type)
                );

                let rg = RenderableGeometry::new(g, c);

                assert_eq!(rg.intersect(ray), vec![(v, c)]);
            }
        }
    }

    renderable_geometry_intersect! { f32, renderable_geometry_intersect_f32 }
    renderable_geometry_intersect! { f64, renderable_geometry_intersect_f64 }
}
