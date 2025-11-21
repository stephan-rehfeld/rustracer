use std::collections::HashMap;
use std::ops::Div;

use cg_basics::camera::{FisheyeCamera, OrthographicCamera, PerspectiveCamera, PinholeCamera, SphericalCamera};
use cg_basics::light::{
    AmbientLight, AmbientOcclusionLight, DirectionalLight, PointLight, SpotLight,
};
use cg_basics::material::{
    UnshadedMaterial, LambertMaterial, PhongMaterial,
};
use image::Image;
use units::length::Length;

pub struct Scene3<T: Length, C, G> {
    pub bg_color: C,
    pub lights: Vec<Light<T, C>>,
    pub cameras: HashMap<String, Camera<T>>,
    pub geometries: Vec<G>,
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

impl<T: Length, C, G> Scene3<T, C, G> {
    pub fn new(
        bg_color: C,
        lights: Vec<Light<T, C>>,
        cameras: HashMap<String, Camera<T>>,
        geometries: Vec<G>,
    ) -> Scene3<T, C, G> {
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
