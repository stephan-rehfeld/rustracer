use std::ops::{Div, Mul};

use color::Color;
use image::Image;
use material::Material;
use math::geometry::{Intersect, ParametricLine, SurfacePoint};
use math::transform::Transform3;
use math::{Normal, Normal3, Point, Point3, Vector, Vector3};
use traits::{MultiplyStable, Sqrt};
use units::length::Length;

use crate::scene_graph::RenderableGeometry;

pub mod camera;
pub mod color;
pub mod image;
pub mod light;
pub mod material;
pub mod math;
pub mod parser;
pub mod ray_casting;
pub mod scene_graph;
pub mod traits;
pub mod units;

type Cylinder<T> = math::geometry::ImplicitCylinder<T>;
type Plane<T> = math::geometry::ImplicitPlane3<T>;
type Sphere<T> = math::geometry::ImplicitNSphere<Point3<T>>;
type AxisAlignedBox<T> = math::geometry::AxisAlignedBox<Point3<T>>;
type Triangle<T> = math::geometry::Triangle3<T>;

pub trait Renderable<T: Length> {
    type ScalarType;
    type LengthType;
    type VectorType: Vector<ValueType = Self::LengthType>;
    type PointType: Point<ValueType = Self::LengthType>;
    type NormalType: Normal<ValueType = Self::ScalarType>;
    type ColorType: Color<ChannelType = Self::ScalarType>;

    fn intersect(
        &self,
        ray: ParametricLine<Self::PointType, Self::VectorType>,
    ) -> Vec<(
        Self::ScalarType,
        SurfacePoint<T>,
        &dyn Material<Self::LengthType, ColorType = Self::ColorType>,
    )>;
}

impl<G, T: Length, M> Renderable<T> for RenderableGeometry<G, M, Transform3<T::ValueType>>
where
    ParametricLine<Point3<T>, Vector3<T>>:
        Intersect<G, Output = Vec<(<T as Div>::Output, SurfacePoint<T>)>>,
    G: Copy + Clone,
    T: Copy + Clone,
    T::ValueType: MultiplyStable + Mul<T, Output = T> + Sqrt<Output = T::ValueType>,
    M: Material<T>,
    <M as Material<T>>::ColorType: Color<ChannelType = <T as Div>::Output>,
{
    type ScalarType = <T as Div>::Output;
    type LengthType = T;
    type VectorType = Vector3<T>;
    type PointType = Point3<T>;
    type NormalType = Normal3<<T as Div>::Output>;
    type ColorType = <M as Material<T>>::ColorType;

    fn intersect(
        &self,
        ray: ParametricLine<Self::PointType, Self::VectorType>,
    ) -> Vec<(
        Self::ScalarType,
        SurfacePoint<T>,
        &dyn Material<T, ColorType = Self::ColorType>,
    )> {
        let transformed_ray = ParametricLine::new(
            self.transform.inverse * ray.origin,
            self.transform.inverse * ray.direction,
        );

        let mut hits: Vec<(
            Self::ScalarType,
            SurfacePoint<T>,
            &dyn Material<T, ColorType = Self::ColorType>,
        )> = transformed_ray
            .intersect(self.geometry)
            .iter()
            .map(|t| {
                (
                    t.0,
                    t.1,
                    &self.material as &dyn Material<T, ColorType = Self::ColorType>,
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

pub trait Raytracer: Image {
    type ScalarType;
    type LengthType: Length;
    type PointType: Point<ValueType = Self::LengthType>;
    type VectorType: Vector<ValueType = Self::LengthType>;
    type NormalType: Normal<ValueType = Self::ScalarType>;
    type ColorType: Color<ChannelType = Self::ScalarType>;

    type Ray;

    type RenderableTraitType: ?Sized;
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fmt::Debug;

    use color::RGB;
    use math::{Normal3, Point2};
    use traits::number::MultiplyStable;
    use traits::Zero;

    use crate::light::Light;
    use crate::units::length::Meter;

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
        <T as Length>::ValueType: MultiplyStable,
    {
        type ColorType = RGB<<T as Length>::ValueType>;

        fn color_for(
            &self,
            _sp: SurfacePoint<T>,
            _d: Vector3<T>,
            _lights: Vec<&Box<dyn Light<T, RGB<<T as Length>::ValueType>>>>,
            _ambient_light: RGB<<T as Length>::ValueType>,
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
