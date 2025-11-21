use std::collections::HashMap;

use cg_basics::light::{
    AmbientLight, AmbientOcclusionLight, DirectionalLight, PointLight, SpotLight,
};
use units::length::Length;

pub struct Scene3<T: Length, C, CAM, G> {
    pub bg_color: C,
    pub lights: Vec<Light<T, C>>,
    pub cameras: HashMap<String, CAM>,
    pub geometries: Vec<G>,
}

pub enum Light<T: Length, C> {
    DirectionalLight(DirectionalLight<T, C>),
    PointLight(PointLight<T, C>),
    SpotLight(SpotLight<T, C>),
    AmbientLight(AmbientLight<C>),
    AmbientOcclusionLight(AmbientOcclusionLight<T, C>),
}

impl<T: Length, C, CAM, G> Scene3<T, C, CAM, G> {
    pub fn new(
        bg_color: C,
        lights: Vec<Light<T, C>>,
        cameras: HashMap<String, CAM>,
        geometries: Vec<G>,
    ) -> Scene3<T, C, CAM, G> {
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
