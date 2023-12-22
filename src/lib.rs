use std::ops::{Div, Mul, Sub};

use camera::RaytracingCamera;
use color::Color;
use image::Image;
use math::{Normal, Normal3, NormalizableVector, Point, Point2, Point3, Vector, Vector2, Vector3};
use math::geometry::{Intersect, ParametricLine};
use traits::{One, Zero};

pub mod camera;
pub mod color;
pub mod image;
pub mod math;
pub mod traits;
pub mod units;

pub trait Renderable<T> where
    T: Div + Mul + Copy + Clone,
{
    type ScalarType;
    type LengthType;
    type VectorType: Vector<ValueType = Self::LengthType>;
    type PointType: Point<ValueType = Self::LengthType>;
    type NormalType: Normal<ValueType = Self::ScalarType>;
    type ColorType: Color<ChannelType = Self::ScalarType>;

    fn intersect(&self, ray: ParametricLine<Self::PointType, Self::VectorType>) -> Vec<(Self::ScalarType, Self::NormalType, Self::ColorType)>;
}

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct RenderableGeometry<G, C> {
    geometry: G,
    color: C,
}

impl<G, C> RenderableGeometry<G, C> {
    pub fn new(geometry: G, color: C) -> RenderableGeometry<G, C> {
        RenderableGeometry { geometry, color }
    }
}

impl<G, T, C> Renderable<T> for RenderableGeometry<G, C>
    where
        T: Div + Mul,
        <T as Div>::Output: Copy,
        ParametricLine<Point3<T>, Vector3<T>>: Intersect<G, Output = Vec<(<T as Div>::Output, <Vector3<T> as NormalizableVector>::NormalType)>>,
        G: Copy + Clone,
        T: Copy + Clone,
        C: Color<ChannelType = <T as Div>::Output> 
{
    type ScalarType = <T as Div>::Output;
    type LengthType = T;
    type VectorType = Vector3<T>;
    type PointType = Point3<T>;
    type NormalType = <Self::VectorType as NormalizableVector>::NormalType;
    type ColorType = C;

    fn intersect(&self, ray: ParametricLine<Self::PointType, Self::VectorType>) -> Vec<(Self::ScalarType, Self::NormalType, Self::ColorType)> {
        ray.intersect(self.geometry).iter().map(|t| (t.0, t.1, self.color)).collect()
    }
}

pub trait Raytracer : Image {
    type ScalarType;
    type LengthType: Mul + Div<Output = Self::ScalarType> + Copy;
    type PointType: Point<ValueType = Self::LengthType>;
    type VectorType: Vector<ValueType = Self::LengthType> + NormalizableVector<NormalType = Self::NormalType>;
    type NormalType: Normal<ValueType = Self::ScalarType>;
    type ColorType: Color<ChannelType = Self::ScalarType>;

    type Ray;

    type RenderableTraitType: ?Sized;
}

pub struct ClassicRaytracer<T, C> where
    T: Div + Mul + Copy + One + Default + Zero + Sub<Output=T> + PartialOrd,
    <T as Div>::Output: Copy + Default + PartialEq + PartialOrd + Zero,
    C: Color<ChannelType = <T as Div>::Output>
{
    camera: Box<dyn RaytracingCamera<T>>,
    scene: Vec<Box< <Self as Raytracer>::RenderableTraitType  >>,
    bg_color: C,
}

impl<T, C> ClassicRaytracer<T, C> where
    T: Div + Mul + Copy + One + Default + Zero + Sub<Output=T> + PartialOrd,
    <T as Div>::Output: Copy + Default + PartialEq + PartialOrd + Zero,
    C: Color<ChannelType = <T as Div>::Output>
{
    pub fn new(camera: Box<dyn RaytracingCamera<T>>, scene: Vec<Box< <Self as Raytracer>::RenderableTraitType>>, bg_color: C) -> ClassicRaytracer<T, C> {
        ClassicRaytracer { camera, scene, bg_color }
    }
}

impl<T: Default + PartialEq + Copy + Zero + Div + Sub<Output=T> + One + Mul + PartialOrd, C> Image for ClassicRaytracer<T, C>  where
    <T as Div>::Output: Default + PartialEq + Copy + PartialOrd + Zero,
    C: Color<ChannelType = <T as Div>::Output>
{
    type ColorType = C;
    type PointType = Point2<T>;

    fn size(&self) -> Vector2<T> {
        self.camera.size()
    }

    fn get(&self, p: Self::PointType) -> Self::ColorType {
        let p = Point2::new(p.x, self.size().y - p.y - One::one());
        let ray = self.camera.ray_for(p);

        let mut hits : Vec<(<Self as Raytracer>::ScalarType, <Self as Raytracer>::NormalType, Self::ColorType)> = self.scene.iter().flat_map(|g| g.intersect(ray)).filter(|(t,_,_)| *t > Zero::zero()).collect();
        hits.sort_by( |(t1,_,_), (t2,_,_)| t1.partial_cmp( t2 ).unwrap() );

        if hits.is_empty() {
            return self.bg_color;
        } else {
            let (_, _, color) = hits[0];
            return color;
        }
    }
}

impl<T, C> Raytracer for ClassicRaytracer<T, C> where
    T: Default + PartialEq + Copy + Zero + Div + Sub<Output=T> + One + Mul + PartialOrd,
    <T as Div>::Output: Copy + Default + PartialEq + PartialOrd + Zero, 
    C: Color<ChannelType = <T as Div>::Output>
{
    type ScalarType = <T as Div>::Output;
    type LengthType = T;
    type PointType = Point3<T>;
    type VectorType = Vector3<T>;
    type NormalType = Normal3<Self::ScalarType>;
    type ColorType = C;

    type RenderableTraitType = dyn Renderable<Self::LengthType, ScalarType = Self::ScalarType, LengthType = Self::LengthType, VectorType = Self::VectorType, PointType = <Self as Raytracer>::PointType, NormalType = Self::NormalType, ColorType = <Self as Raytracer>::ColorType>;

    type Ray = ParametricLine<<Self as Raytracer>::PointType, <Self as Raytracer>::VectorType>;
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fmt::Debug;

    use color::RGB;
    use math::Normal3;

    #[derive(Debug,PartialEq,Clone,Copy)]
    struct MockGeometry<T> where
        T: Div + Mul + Copy + Clone,
        <T as Div>::Output: Copy + Clone + Debug + PartialEq,
    {
        t: T,
        normal: <Vector3<T> as NormalizableVector>::NormalType,
    }

    impl<T> Intersect<MockGeometry<T>> for ParametricLine<Point3<T>, Vector3<T>> where
        T: Div + Mul + Copy + Clone,
        <T as Div>::Output: Copy + Clone + Debug + PartialEq,
    {
        type Output = Vec<(T, <Vector3<T> as NormalizableVector>::NormalType)>;

        fn intersect(self, other: MockGeometry<T>) -> Self::Output {
            vec![(other.t, other.normal)]
        }
    }

    macro_rules! new_renderable_geometry {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let g = MockGeometry { t: 1.0 as $type, normal: Normal3::new(0 as $type, 1 as $type, 0 as $type)};
                let c = RGB::new(0.0 as $type, 0.5 as $type, 1.0 as $type);

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
                let c = RGB::new(0.0 as $type, 0.5 as $type, 1.0 as $type);

                let ray = ParametricLine::new(
                    Point3::new(0 as $type, 0 as $type, 0 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type)
                );

                let rg = RenderableGeometry::new(g, c);

                assert_eq!(rg.intersect(ray), vec![(v, n, c)]);
            }
        }
    }

    renderable_geometry_intersect! { f32, renderable_geometry_intersect_f32 }
    renderable_geometry_intersect! { f64, renderable_geometry_intersect_f64 }
}
