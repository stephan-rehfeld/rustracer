use std::ops::{Div, Mul};

use colors::Color;
use material::Material;
use math::geometry::{Intersect, ParametricLine, SurfacePoint};
use math::transform::Transform3;
use math::{Point3, Vector3};
use traits::{Number, Sqrt};
use units::length::Length;

use cg_basics::scene_graph::RenderableGeometry;

pub mod camera;
pub mod diffuse_ray_tracer;
pub mod light;
pub mod material;
pub mod parser;
pub mod ray_casting;

type Cylinder<T> = math::geometry::ImplicitCylinder<T>;
type Disc<T> = math::geometry::ImplicitDisc3<T>;
type Plane<T> = math::geometry::ImplicitPlane3<T>;
type Sphere<T> = math::geometry::Sphere<T>;
type AxisAlignedBox<T> = math::geometry::AxisAlignedBox<Point3<T>>;
type Triangle<T> = math::geometry::Triangle3<T>;

pub trait Renderable<T: Length, C: Color<ChannelType = T::ValueType>> {
    fn intersect(
        &self,
        ray: ParametricLine<Point3<T>, Vector3<T>>,
    ) -> Vec<(
        T::ValueType,
        SurfacePoint<T>,
        &dyn Material<T, ColorType = C>,
    )>;
}

impl<G, T: Length, M> Renderable<T, <M as Material<T>>::ColorType>
    for RenderableGeometry<G, M, Transform3<T::ValueType>>
where
    ParametricLine<Point3<T>, Vector3<T>>:
        Intersect<G, Output = Vec<(<T as Div>::Output, SurfacePoint<T>)>>,
    G: Copy + Clone,
    T: Copy + Clone,
    T::ValueType: Number + Mul<T, Output = T> + Sqrt<Output = T::ValueType>,
    M: Material<T>,
    <M as Material<T>>::ColorType: Color<ChannelType = <T as Div>::Output>,
{
    fn intersect(
        &self,
        ray: ParametricLine<Point3<T>, Vector3<T>>,
    ) -> Vec<(
        T::ValueType,
        SurfacePoint<T>,
        &dyn Material<T, ColorType = <M as Material<T>>::ColorType>,
    )> {
        let transformed_ray = ParametricLine::new(
            self.transform.inverse * ray.origin,
            self.transform.inverse * ray.direction,
        );

        let mut hits: Vec<(
            T::ValueType,
            SurfacePoint<T>,
            &dyn Material<T, ColorType = <M as Material<T>>::ColorType>,
        )> = transformed_ray
            .intersect(self.geometry)
            .iter()
            .map(|t| {
                (
                    t.0,
                    t.1,
                    &self.material as &dyn Material<T, ColorType = <M as Material<T>>::ColorType>,
                )
            })
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::fmt::Debug;

    use colors::RGB;
    use math::{Normal3, Point2};
    use traits::Zero;
    use units::length::Meter;

    use crate::light::Light;

    #[derive(Debug, PartialEq, Clone, Copy)]
    struct MockGeometry<T>
    where
        T: Length,
    {
        t: <T as Length>::ValueType,
        normal: Normal3<<T as Length>::ValueType>,
    }

    impl<T> Intersect<MockGeometry<T>> for ParametricLine<Point3<T>, Vector3<T>>
    where
        T: Length,
    {
        type Output = Vec<(<T as Length>::ValueType, SurfacePoint<T>)>;

        fn intersect(self, other: MockGeometry<T>) -> Self::Output {
            vec![(
                other.t,
                SurfacePoint::new(
                    Point3::new(Zero::zero(), Zero::zero(), Zero::zero()),
                    other.normal,
                    Point2::new(Zero::zero(), Zero::zero()),
                ),
            )]
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    struct MockMaterial<T: Length> {
        color: RGB<<T as Length>::ValueType>,
    }

    impl<T: Length> Material<T> for MockMaterial<T>
    where
        <T as Length>::ValueType: Number,
    {
        type ColorType = RGB<<T as Length>::ValueType>;

        fn color_for(
            &self,
            _sp: SurfacePoint<T>,
            _d: Vector3<T>,
            _lights: Vec<&Box<dyn Light<T, RGB<<T as Length>::ValueType>>>>,
        ) -> RGB<<T as Length>::ValueType> {
            self.color
        }
    }

    macro_rules! new_renderable_geometry {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let g: MockGeometry<Meter<$type>> = MockGeometry {
                    t: 1.0 as $type,
                    normal: Normal3::new(0 as $type, 1 as $type, 0 as $type),
                };
                let m: MockMaterial<Meter<$type>> = MockMaterial {
                    color: RGB::new(0.0 as $type, 0.5 as $type, 1.0 as $type),
                };

                let rg = RenderableGeometry::new(g, m, Transform3::<$type>::ident());

                assert_eq!(rg.geometry, g);
                assert_eq!(rg.material, m);
            }
        };
    }

    new_renderable_geometry! { f32, new_renderable_geometry_f32 }
    new_renderable_geometry! { f64, new_renderable_geometry_f64 }

    macro_rules! renderable_geometry_intersect {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let v = 25.0 as $type;
                let n = Normal3::new(0 as $type, 1 as $type, 0 as $type);
                let g: MockGeometry<Meter<$type>> = MockGeometry { t: v, normal: n };
                let m: MockMaterial<Meter<$type>> = MockMaterial {
                    color: RGB::new(0.0 as $type, 0.5 as $type, 1.0 as $type),
                };

                let ray = ParametricLine::new(
                    Point3::new(
                        Meter::new(0 as $type),
                        Meter::new(0 as $type),
                        Meter::new(0 as $type),
                    ),
                    Vector3::new(
                        Meter::new(0 as $type),
                        Meter::new(0 as $type),
                        Meter::new(-1 as $type),
                    ),
                );

                let rg = RenderableGeometry::new(g, m, Transform3::<$type>::ident());

                let intersections = rg.intersect(ray);
                assert_eq!(1, intersections.len());
                assert_eq!(v, intersections[0].0);
                assert_eq!(n, intersections[0].1.n);

                // Cast to *const () is required to prevent fat pointer from being used.
                let rg_mat_pointer = &rg.material
                    as *const dyn Material<Meter<$type>, ColorType = RGB<$type>>
                    as *const ();
                let intersect_mat_pointer = intersections[0].2
                    as *const dyn Material<Meter<$type>, ColorType = RGB<$type>>
                    as *const ();

                assert_eq!(rg_mat_pointer, intersect_mat_pointer);
            }
        };
    }

    renderable_geometry_intersect! { f32, renderable_geometry_intersect_f32 }
    renderable_geometry_intersect! { f64, renderable_geometry_intersect_f64 }
}
