use std::ops::{Add, Div, Mul, Neg};

use color::Color;
use image::Image;
use material::Material;
use math::geometry::{Intersect, ParametricLine};
use math::{Mat4x4, Normal, NormalizableVector, Point, Point3, Vector, Vector3};
use traits::{Cos, One, Recip, Sin, Sqrt, Zero};
use units::length::Length;

pub mod camera;
pub mod color;
pub mod image;
pub mod light;
pub mod material;
pub mod math;
pub mod parser;
pub mod ray_casting;
pub mod traits;
pub mod units;

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
        Self::NormalType,
        &dyn Material<Self::LengthType, ColorType = Self::ColorType>,
    )>;
}

pub struct RenderableGeometry<G, M> {
    geometry: G,
    material: M,
}

impl<G, M> RenderableGeometry<G, M> {
    pub fn new(geometry: G, material: M) -> RenderableGeometry<G, M> {
        RenderableGeometry { geometry, material }
    }
}

impl<G, T: Length, M> Renderable<T> for RenderableGeometry<G, M>
where
    ParametricLine<Point3<T>, Vector3<T>>: Intersect<
        G,
        Output = Vec<(
            <T as Div>::Output,
            <Vector3<T> as NormalizableVector>::NormalType,
        )>,
    >,
    G: Copy + Clone,
    T: Copy + Clone,
    M: Material<T>,
    <M as Material<T>>::ColorType: Color<ChannelType = <T as Div>::Output>,
{
    type ScalarType = <T as Div>::Output;
    type LengthType = T;
    type VectorType = Vector3<T>;
    type PointType = Point3<T>;
    type NormalType = <Self::VectorType as NormalizableVector>::NormalType;
    type ColorType = <M as Material<T>>::ColorType;

    fn intersect(
        &self,
        ray: ParametricLine<Self::PointType, Self::VectorType>,
    ) -> Vec<(
        Self::ScalarType,
        Self::NormalType,
        &dyn Material<T, ColorType = Self::ColorType>,
    )> {
        ray.intersect(self.geometry)
            .iter()
            .map(|t| {
                (
                    t.0,
                    t.1,
                    &self.material as &dyn Material<T, ColorType = Self::ColorType>,
                )
            })
            .collect()
    }
}

pub struct Node<T: Length, C: Color> {
    transform: Transform<<T as Div>::Output>,
    elements: Vec<
        Box<
            dyn Renderable<
                T,
                ScalarType = <T as Div>::Output,
                LengthType = T,
                PointType = Point3<T>,
                VectorType = Vector3<T>,
                NormalType = <Vector3<T> as NormalizableVector>::NormalType,
                ColorType = C,
            >,
        >,
    >,
}

impl<T: Length, C: Color> Node<T, C> {
    pub fn new(
        transform: Transform<<T as Div>::Output>,
        elements: Vec<
            Box<
                dyn Renderable<
                    T,
                    ScalarType = <T as Div>::Output,
                    LengthType = T,
                    PointType = Point3<T>,
                    VectorType = Vector3<T>,
                    NormalType = <Vector3<T> as NormalizableVector>::NormalType,
                    ColorType = C,
                >,
            >,
        >,
    ) -> Node<T, C> {
        Node {
            transform,
            elements,
        }
    }
}

impl<T: Length, C: Color<ChannelType = <T as Length>::ValueType>> Renderable<T> for Node<T, C>
where
    <T as Length>::ValueType: Mul<T, Output = T> + Sqrt<Output = <T as Length>::ValueType>,
{
    type ScalarType = <T as Div>::Output;
    type LengthType = T;
    type VectorType = Vector3<T>;
    type PointType = Point3<T>;
    type NormalType = <Self::VectorType as NormalizableVector>::NormalType;
    type ColorType = C;

    fn intersect(
        &self,
        ray: ParametricLine<Self::PointType, Self::VectorType>,
    ) -> Vec<(
        Self::ScalarType,
        Self::NormalType,
        &dyn Material<T, ColorType = Self::ColorType>,
    )> {
        let transformed_ray = ParametricLine::new(
            self.transform.inverse * ray.origin,
            self.transform.inverse * ray.direction,
        );

        let mut hits: Vec<(
            Self::ScalarType,
            Self::NormalType,
            &dyn Material<T, ColorType = Self::ColorType>,
        )> = self
            .elements
            .iter()
            .flat_map(|g| g.intersect(transformed_ray))
            .filter(|(t, _, _)| *t > Zero::zero())
            .collect();
        hits.sort_by(|(t1, _, _), (t2, _, _)| t1.partial_cmp(t2).unwrap());

        let transposed_inverse = self.transform.inverse.transposed();

        hits = hits
            .iter()
            .map(|(t, n, m)| (*t, transposed_inverse * *n, *m))
            .collect();

        hits
    }
}

pub struct Transform<T> {
    matrix: Mat4x4<T>,
    inverse: Mat4x4<T>,
}

impl<T: One + Zero> Transform<T> {
    pub fn ident() -> Transform<T> {
        Transform {
            matrix: Mat4x4::ident(),
            inverse: Mat4x4::ident(),
        }
    }
}

impl<T> Transform<T>
where
    T: Add<Output = T> + Neg<Output = T> + Mul<Output = T> + Recip + Zero + One + Copy,
{
    pub fn scale(self, x: T, y: T, z: T) -> Transform<T> {
        let matrix = Mat4x4::new(
            x,
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            y,
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            z,
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        );

        let inverse = Mat4x4::new(
            x.recip(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            y.recip(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            z.recip(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        );

        Transform {
            matrix: self.matrix * matrix,
            inverse: inverse * self.inverse,
        }
    }

    pub fn translate(self, x: T, y: T, z: T) -> Transform<T> {
        let matrix = Mat4x4::new(
            One::one(),
            Zero::zero(),
            Zero::zero(),
            x,
            Zero::zero(),
            One::one(),
            Zero::zero(),
            y,
            Zero::zero(),
            Zero::zero(),
            One::one(),
            z,
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        );

        let inverse = Mat4x4::new(
            One::one(),
            Zero::zero(),
            Zero::zero(),
            -x,
            Zero::zero(),
            One::one(),
            Zero::zero(),
            -y,
            Zero::zero(),
            Zero::zero(),
            One::one(),
            -z,
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        );

        Transform {
            matrix: self.matrix * matrix,
            inverse: inverse * self.inverse,
        }
    }

    pub fn rotate_x<A: Cos<Output = T> + Sin<Output = T> + Copy>(self, angle: A) -> Transform<T> {
        let matrix = Mat4x4::new(
            One::one(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            angle.cos(),
            -angle.sin(),
            Zero::zero(),
            Zero::zero(),
            angle.sin(),
            angle.cos(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        );

        let inverse = Mat4x4::new(
            One::one(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            angle.cos(),
            angle.sin(),
            Zero::zero(),
            Zero::zero(),
            -angle.sin(),
            angle.cos(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        );

        Transform {
            matrix: self.matrix * matrix,
            inverse: inverse * self.inverse,
        }
    }

    pub fn rotate_y<A: Cos<Output = T> + Sin<Output = T> + Copy>(self, angle: A) -> Transform<T> {
        let matrix = Mat4x4::new(
            angle.cos(),
            Zero::zero(),
            -angle.sin(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
            Zero::zero(),
            Zero::zero(),
            angle.sin(),
            Zero::zero(),
            angle.cos(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        );

        let inverse = Mat4x4::new(
            angle.cos(),
            Zero::zero(),
            angle.sin(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
            Zero::zero(),
            Zero::zero(),
            -angle.sin(),
            Zero::zero(),
            angle.cos(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        );

        Transform {
            matrix: self.matrix * matrix,
            inverse: inverse * self.inverse,
        }
    }

    pub fn rotate_z<A: Cos<Output = T> + Sin<Output = T> + Copy>(self, angle: A) -> Transform<T> {
        let matrix = Mat4x4::new(
            angle.cos(),
            -angle.sin(),
            Zero::zero(),
            Zero::zero(),
            angle.sin(),
            angle.cos(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        );

        let inverse = Mat4x4::new(
            angle.cos(),
            angle.sin(),
            Zero::zero(),
            Zero::zero(),
            -angle.sin(),
            angle.cos(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        );

        Transform {
            matrix: self.matrix * matrix,
            inverse: inverse * self.inverse,
        }
    }
}

pub trait Raytracer: Image {
    type ScalarType;
    type LengthType: Length;
    type PointType: Point<ValueType = Self::LengthType>;
    type VectorType: Vector<ValueType = Self::LengthType>
        + NormalizableVector<NormalType = Self::NormalType>;
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

    use crate::light::Light;
    use crate::units::length::Meter;

    #[derive(Debug, PartialEq, Clone, Copy)]
    struct MockGeometry<T>
    where
        T: Length,
    {
        t: <T as Length>::ValueType,
        normal: <Vector3<T> as NormalizableVector>::NormalType,
    }

    impl<T> Intersect<MockGeometry<T>> for ParametricLine<Point3<T>, Vector3<T>>
    where
        T: Length,
    {
        type Output = Vec<(
            <T as Length>::ValueType,
            <Vector3<T> as NormalizableVector>::NormalType,
        )>;

        fn intersect(self, other: MockGeometry<T>) -> Self::Output {
            vec![(other.t, other.normal)]
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
            _p: Point3<T>,
            _n: Normal3<<T as Length>::ValueType>,
            _tex: Point2<<T as Length>::ValueType>,
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

                let rg = RenderableGeometry::new(g, m);

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

                let rg = RenderableGeometry::new(g, m);

                let intersections = rg.intersect(ray);
                assert_eq!(1, intersections.len());
                assert_eq!(v, intersections[0].0);
                assert_eq!(n, intersections[0].1);

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
