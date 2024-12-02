use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::fs;
use std::ops::Div;
use std::str::FromStr;

use crate::camera::RaytracingCamera;
use crate::light::Light;
use crate::material::Material;
use crate::ray_casting::Scene;
use crate::{AxisAlignedBox, Cylinder, Disc, Plane, Renderable, Sphere, Triangle};
use cg_basics::camera::{
    FisheyeCamera, OrthographicCamera, PerspectiveCamera, PinholeCamera, SphericalCamera,
};
use cg_basics::light::{AmbientLight, PointLight, SpotLight};
use cg_basics::scene_graph::RenderableGeometry;
use colors::RGB;
use math::transform::Transform3;
use math::{Normal3, Orthonormal3};
use traits::{
    ConvenientNumber, Cos, FloatingPoint, Number, SelfMulNumber, SignedNumber, Sin, Sqrt, Zero,
};
use units::angle::{Angle, Radians};
use units::length::Length;

mod camera;
mod geometry;
mod light;
mod material;
mod misc;
mod texture;
mod util;

type MaterialType<T> = Box<dyn Material<T, ColorType = RGB<<T as Length>::ValueType>>>;

type RenderableAxisAlignedBox<T> =
    RenderableGeometry<AxisAlignedBox<T>, MaterialType<T>, Transform3<<T as Length>::ValueType>>;
type RenderableCylinder<T> =
    RenderableGeometry<Cylinder<T>, MaterialType<T>, Transform3<<T as Length>::ValueType>>;
type RenderableDisc<T> =
    RenderableGeometry<Disc<T>, MaterialType<T>, Transform3<<T as Length>::ValueType>>;
type RenderablePlane<T> =
    RenderableGeometry<Plane<T>, MaterialType<T>, Transform3<<T as Length>::ValueType>>;
type RenderableSphere<T> =
    RenderableGeometry<Sphere<T>, MaterialType<T>, Transform3<<T as Length>::ValueType>>;
type RenderableTriangle<T> =
    RenderableGeometry<Triangle<T>, MaterialType<T>, Transform3<<T as Length>::ValueType>>;

#[derive(Debug)]
pub enum ParsingError {
    UnexpectedEndOfTokens,
    NumberParsingError(&'static str),

    ColorParsingError(Box<ParsingError>),
    Point2ParsingError(Box<ParsingError>),
    Point3ParsingError(Box<ParsingError>),
    VectorParsingError(Box<ParsingError>),
    NormalParsingError(Box<ParsingError>),

    UnexpectedToken {
        expected: &'static str,
        found: String,
    },
    TextureParsingError(Box<ParsingError>),
    UnsupportedTexture(String),
    SingleColorTextureParsingError(Box<ParsingError>),
    CheckerboardTextureParsingError(Box<ParsingError>),
    GridTextureParsingError(Box<ParsingError>),

    UnshadedMaterialParsingError(Box<ParsingError>),
    LambertMaterialParsingError(Box<ParsingError>),
    PhongMaterialParsingError(Box<ParsingError>),
    MaterialParsingError(Box<ParsingError>),
    UnsupportedMaterial(String),

    DiscParsingError(Box<ParsingError>),
    SphereParsingError(Box<ParsingError>),
    CylinderParsingError(Box<ParsingError>),
    PlaneParsingError(Box<ParsingError>),
    BoxParsingError(Box<ParsingError>),
    TriangleParsingError(Box<ParsingError>),

    PinholeCameraParsingError(Box<ParsingError>),
    PerspectiveCameraParsingError(Box<ParsingError>),
    FisheyeCameraParsingError(Box<ParsingError>),
    OrthographicCameraParsingError(Box<ParsingError>),
    SphericalCameraParsingError(Box<ParsingError>),

    PointLightParsingError(Box<ParsingError>),
    SpotLightParsingError(Box<ParsingError>),
    AmbientOcclusionLightParsingError(Box<ParsingError>),

    MissingElement(&'static str),
    UnsupportedElement(String),
    SceneParsingError(Box<ParsingError>),
}

trait FromTokens: Sized {
    type Err;

    fn from_tokens<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, Self::Err>;
}

pub fn parse_scene<T: Length + SignedNumber<T::ValueType> + ConvenientNumber + 'static>(
    filename: &str,
) -> Result<
    Scene<
        T,
        RGB<<T as Length>::ValueType>,
        Box<dyn Light<T, RGB<<T as Length>::ValueType>>>,
        Box<dyn Renderable<T, RGB<T::ValueType>>>,
    >,
    ParsingError,
>
where
    <T as Length>::ValueType: FloatingPoint + ConvenientNumber,
    <<T as Length>::ValueType as FromStr>::Err: Error,
    <T as Length>::AreaType: Sqrt<Output = T>
        + SelfMulNumber<T::ValueType>
        + SignedNumber<T::ValueType>
        + ConvenientNumber,
    <T as Length>::SecondMomentOfAreaType:
        Number<T::ValueType> + Sqrt<Output = <T as Length>::AreaType> + ConvenientNumber,
    <T as FromStr>::Err: Error,
    Normal3<<T as Length>::ValueType>: Orthonormal3,
    Radians<<T as Div>::Output>:
        Angle + Cos<Output = <T as Div>::Output> + Sin<Output = <T as Div>::Output>,
{
    let file_content = fs::read_to_string(filename).expect("Unable to read file");

    let mut tokens = file_content
        .split(&[' ', '\t', '\n'])
        .filter(|token| !token.is_empty());

    let mut geometries: Vec<Box<dyn Renderable<T, RGB<T::ValueType>>>> = Vec::new();
    let mut lights: Vec<Box<dyn Light<T, RGB<<T as Length>::ValueType>>>> = Vec::new();
    let mut cameras: HashMap<String, Box<dyn RaytracingCamera<T>>> = HashMap::new();
    let mut background_color: RGB<<T as Length>::ValueType> =
        RGB::new(Zero::zero(), Zero::zero(), Zero::zero());

    while let Some(token) = tokens.next() {
        match token {
            "sphere" => match RenderableSphere::<T>::from_tokens(&mut tokens) {
                Ok(sphere) => {
                    geometries.push(Box::new(sphere));
                }
                Err(cause) => {
                    return Err(ParsingError::SceneParsingError(Box::new(cause)));
                }
            },
            "cylinder" => match RenderableCylinder::<T>::from_tokens(&mut tokens) {
                Ok(cylinder) => {
                    geometries.push(Box::new(cylinder));
                }
                Err(cause) => {
                    return Err(ParsingError::SceneParsingError(Box::new(cause)));
                }
            },
            "disc" => match RenderableDisc::<T>::from_tokens(&mut tokens) {
                Ok(disc) => {
                    geometries.push(Box::new(disc));
                }
                Err(cause) => {
                    return Err(ParsingError::SceneParsingError(Box::new(cause)));
                }
            },

            "plane" => match RenderablePlane::<T>::from_tokens(&mut tokens) {
                Ok(plane) => {
                    geometries.push(Box::new(plane));
                }
                Err(cause) => {
                    return Err(ParsingError::SceneParsingError(Box::new(cause)));
                }
            },
            "box" => match RenderableAxisAlignedBox::<T>::from_tokens(&mut tokens) {
                Ok(aab) => {
                    geometries.push(Box::new(aab));
                }
                Err(cause) => {
                    return Err(ParsingError::SceneParsingError(Box::new(cause)));
                }
            },
            "triangle" => match RenderableTriangle::<T>::from_tokens(&mut tokens) {
                Ok(triangle) => {
                    geometries.push(Box::new(triangle));
                }
                Err(cause) => {
                    return Err(ParsingError::SceneParsingError(Box::new(cause)));
                }
            },
            "pinhole_camera" => match <(String, PinholeCamera<T>)>::from_tokens(&mut tokens) {
                Ok((id, camera)) => {
                    cameras.insert(id, Box::new(camera));
                }
                Err(cause) => {
                    return Err(ParsingError::SceneParsingError(Box::new(cause)));
                }
            },
            "perspective_camera" => {
                match <(String, PerspectiveCamera<T>)>::from_tokens(&mut tokens) {
                    Ok((id, camera)) => {
                        cameras.insert(id, Box::new(camera));
                    }
                    Err(cause) => {
                        return Err(ParsingError::SceneParsingError(Box::new(cause)));
                    }
                }
            }
            "orthographic_camera" => {
                match <(String, OrthographicCamera<T>)>::from_tokens(&mut tokens) {
                    Ok((id, camera)) => {
                        cameras.insert(id, Box::new(camera));
                    }
                    Err(cause) => {
                        return Err(ParsingError::SceneParsingError(Box::new(cause)));
                    }
                }
            }
            "fisheye_camera" => match <(String, FisheyeCamera<T>)>::from_tokens(&mut tokens) {
                Ok((id, camera)) => {
                    cameras.insert(id, Box::new(camera));
                }
                Err(cause) => {
                    return Err(ParsingError::SceneParsingError(Box::new(cause)));
                }
            },
            "spherical_camera" => match <(String, SphericalCamera<T>)>::from_tokens(&mut tokens) {
                Ok((id, camera)) => {
                    cameras.insert(id, Box::new(camera));
                }
                Err(cause) => {
                    return Err(ParsingError::SceneParsingError(Box::new(cause)));
                }
            },
            "point_light" => match PointLight::from_tokens(&mut tokens) {
                Ok(point_light) => {
                    lights.push(Box::new(point_light));
                }
                Err(cause) => {
                    return Err(ParsingError::SceneParsingError(Box::new(cause)));
                }
            },
            "spot_light" => match SpotLight::from_tokens(&mut tokens) {
                Ok(spot_light) => {
                    lights.push(Box::new(spot_light));
                }
                Err(cause) => {
                    return Err(ParsingError::SceneParsingError(Box::new(cause)));
                }
            },
            "background_color:" => match RGB::from_tokens(&mut tokens) {
                Ok(bg) => {
                    background_color = bg;
                }
                Err(cause) => {
                    return Err(ParsingError::SceneParsingError(Box::new(cause)));
                }
            },
            "ambient_light:" => match RGB::from_tokens(&mut tokens) {
                Ok(ambient) => {
                    lights.push(Box::new(AmbientLight::new(ambient)));
                }
                Err(cause) => {
                    return Err(ParsingError::SceneParsingError(Box::new(cause)));
                }
            },
            &_ => {
                return Err(ParsingError::UnsupportedElement(token.to_string()));
            }
        }
    }

    Ok(Scene::new(background_color, lights, cameras, geometries))
}
