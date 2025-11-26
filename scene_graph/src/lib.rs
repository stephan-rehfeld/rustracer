use std::collections::HashMap;
use std::ops::Div;

use cg_basics::camera::{
    FisheyeCamera, OrthographicCamera, PerspectiveCamera, PinholeCamera, SphericalCamera,
};
use cg_basics::light::{
    AmbientLight, AmbientOcclusionLight, DirectionalLight, PointLight, SpotLight,
};
use cg_basics::material::{LambertMaterial, PhongMaterial, UnshadedMaterial};
use colors::Color;
use image::Image;
use math::geometry::{
    AxisAlignedBox, ImplicitCylinder, ImplicitDisc3, ImplicitPlane3, Sphere, Triangle3,
};
use math::transform::Transform3;
use math::{Point2, Point3};
use units::length::Length;

pub struct Scene3<T: Length, C: Color> {
    pub bg_color: C,
    pub lights: Vec<Light<T, C>>,
    pub cameras: HashMap<String, Camera<T>>,
    //pub geometries: Vec<G>,
    pub geometries: Vec<
        RenderableGeometry<
            Geometry<T>,
            Material<Box<dyn Image<ColorType = C, PointType = Point2<T::ValueType>>>>,
            Transform3<T::ValueType>,
        >,
    >,
}

pub enum Light<T: Length, C> {
    DirectionalLight(DirectionalLight<T, C>),
    PointLight(PointLight<T, C>),
    SpotLight(SpotLight<T, C>),
    AmbientLight(AmbientLight<C>),
    AmbientOcclusionLight(AmbientOcclusionLight<T, C>),
}

pub enum Camera<T: Div> {
    FisheyeCamera(FisheyeCamera<T>),
    OrthigraphicCamera(OrthographicCamera<T>),
    PerspectiveCamera(PerspectiveCamera<T>),
    PinholeCamera(PinholeCamera<T>),
    SphericalCamera(SphericalCamera<T>),
}

pub enum Material<I: Image> {
    UnshadedMaterial(UnshadedMaterial<I>),
    LambertMaterial(LambertMaterial<I>),
    PhongMaterial(PhongMaterial<I>),
}

pub enum Geometry<T: Length> {
    AxisAlignedBox(AxisAlignedBox<Point3<T>>),
    Cylinder(ImplicitCylinder<T>),
    Disc(ImplicitDisc3<T>),
    Sphere(Sphere<T>),
    Plane(ImplicitPlane3<T>),
    Triangle(Triangle3<T>),
}

impl<T: Length, C: Color> Scene3<T, C> {
    pub fn new(
        bg_color: C,
        lights: Vec<Light<T, C>>,
        cameras: HashMap<String, Camera<T>>,
        //geometries: Vec<G>,
        geometries: Vec<
            RenderableGeometry<
                Geometry<T>,
                Material<Box<dyn Image<ColorType = C, PointType = Point2<T::ValueType>>>>,
                Transform3<T::ValueType>,
            >,
        >,
    ) -> Scene3<T, C> {
        Scene3 {
            bg_color,
            lights,
            cameras,
            geometries,
        }
    }
}
pub struct RenderableGeometry<G, M, T> {
    pub geometry: G,
    pub material: M,
    pub transform: T,
}

impl<G, M, T> RenderableGeometry<G, M, T> {
    pub fn new(geometry: G, material: M, transform: T) -> RenderableGeometry<G, M, T> {
        RenderableGeometry {
            geometry,
            material,
            transform,
        }
    }
}

/*
pub struct Node<T: Length, C: Color, E> {
    pub transform: Transform3<<T as Div>::Output>,
    pub elements: Vec<
        Box<
            dyn E
/*            dyn Renderable<
                T,
                ScalarType = <T as Div>::Output,
                LengthType = T,
                PointType = Point3<T>,
                VectorType = Vector3<T>,
                NormalType = <Vector3<T> as NormalizableVector>::NormalType,
                ColorType = C,
            >,*/
        >,
    >,
}

impl<T: Length, C: Color, E> Node<T, C, E> {
    pub fn new(
        transform: Transform3<<T as Div>::Output>,
        elements: Vec<
            Box<
                dyn E
/*                dyn Renderable<
                    T,
                    ScalarType = <T as Div>::Output,
                    LengthType = T,
                    PointType = Point3<T>,
                    VectorType = Vector3<T>,
                    NormalType = <Vector3<T> as NormalizableVector>::NormalType,
                    ColorType = C,
                >,*/
            >,
        >,
    ) -> Node<T, C, E> {
        Node {
            transform,
            elements,
        }
    }
}*/
