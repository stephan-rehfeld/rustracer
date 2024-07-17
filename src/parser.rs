use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::camera::{Perspective, RaytracingCamera};
use crate::color::RGB;
use crate::image::Image;
use crate::image::SingleColorImage;
use crate::light::{Light, PointLight, SpotLight};
use crate::material::{LambertMaterial, Material, PhongMaterial, UnshadedMaterial};
use crate::math::geometry::{
    AxisAlignedBox, ImplicitNSphere, ImplicitPlane3, Intersect, ParametricLine, Triangle,
};
use crate::math::normal::Orthonormal3;
use crate::math::{Normal3, NormalizableVector, Point2, Point3, Vector2, Vector3};
use crate::ray_casting::Scene;
use crate::traits::floating_point::ToRadians;
use crate::traits::number::MultiplyStable;
use crate::traits::{Cos, FloatingPoint, Half, One, Sin, Sqrt, Tan, Zero};
use crate::units::angle::Degrees;
use crate::units::length::Length;
use crate::Renderable;
use crate::Transform;

pub struct RenderableGeometry<G, T: Length> {
    geometry: G,
    material: Box<dyn Material<T, ColorType = RGB<T::ValueType>>>,
    transform: Transform<T::ValueType>,
}

impl<G, T: Length> RenderableGeometry<G, T> {
    pub fn new(
        geometry: G,
        material: Box<dyn Material<T, ColorType = RGB<T::ValueType>>>,
        transform: Transform<T::ValueType>,
    ) -> RenderableGeometry<G, T> {
        RenderableGeometry {
            geometry,
            material,
            transform,
        }
    }
}

impl<G, T: Length> Renderable<T> for RenderableGeometry<G, T>
where
    ParametricLine<Point3<T>, Vector3<T>>: Intersect<
        G,
        Output = Vec<(
            <T as Div>::Output,
            <Vector3<T> as NormalizableVector>::NormalType,
        )>,
    >,
    T::ValueType: MultiplyStable + Mul<T, Output = T> + Sqrt<Output = T::ValueType>,
    G: Copy + Clone,
    T: Copy + Clone,
{
    type ScalarType = <T as Div>::Output;
    type LengthType = T;
    type VectorType = Vector3<T>;
    type PointType = Point3<T>;
    type NormalType = <Self::VectorType as NormalizableVector>::NormalType;
    type ColorType = RGB<<T as Length>::ValueType>;

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
        )> = transformed_ray
            .intersect(self.geometry)
            .iter()
            .map(|t| (t.0, t.1, self.material.as_ref()))
            .collect();
        let transposed_inverse = self.transform.inverse.transposed();

        hits = hits
            .iter()
            .map(|(t, n, m)| (*t, transposed_inverse * *n, *m))
            .collect();

        hits
    }
}

#[derive(Debug)]
pub enum ParsingError {
    UnexpectedEndOfTokens,
    NumberParsingError(&'static str),
    ColorParsingError(Box<ParsingError>),
    PointParsingError(Box<ParsingError>),
    VectorParsingError(Box<ParsingError>),
    NormalParsingError(Box<ParsingError>),
    UnexpectedToken {
        expected: &'static str,
        found: String,
    },
    SingleColorTextureParsingError(Box<ParsingError>),
    UnshadedMaterialParsingError(Box<ParsingError>),
    LambertMaterialParsingError(Box<ParsingError>),
    PhongMaterialParsingError(Box<ParsingError>),
    MaterialParsingError(Box<ParsingError>),
    UnsupportedMaterial(String),
    SphereParsingError(Box<ParsingError>),
    PlaneParsingError(Box<ParsingError>),
    BoxParsingError(Box<ParsingError>),
    TriangleParsingError(Box<ParsingError>),
    PerspectiveCameraParsingError(Box<ParsingError>),
    PointLightParsingError(Box<ParsingError>),
    SpotLightParsingError(Box<ParsingError>),
    MissingElement(&'static str),
}

fn parse_next<'a, T: FromStr>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<T, ParsingError>
where
    <T as FromStr>::Err: Error,
{
    match tokens.next() {
        Some(token) => match token.parse::<T>() {
            Ok(n) => Ok(n),
            Err(_) => Err(ParsingError::NumberParsingError("Unable to parse number.")),
        },
        None => Err(ParsingError::UnexpectedEndOfTokens),
    }
}

pub trait FromTokens : Sized {
    type Err;

    fn from_tokens<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, Self::Err>; 
}

macro_rules! create_simple_token_parser {
    ($type: ident, $errorType: ident, $error: ident, [$($element: ident)+]) => {
    impl<T: FromStr> FromTokens for $type<T> where
        <T as FromStr>::Err: Error + Debug,
        {
            type Err = ParsingError;
    
            fn from_tokens<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, Self::Err> {
                $(
                    let $element = parse_next(tokens);
                    if let Err(cause) = $element {
                        return Err($errorType::$error(Box::new(cause)));
                    }
                )*
                Ok($type::new($($element.unwrap(), )*))
            }
        }
    }
}

create_simple_token_parser! { RGB, ParsingError, ColorParsingError, [red green blue] }
create_simple_token_parser! { Point3, ParsingError, PointParsingError, [x y z] }
create_simple_token_parser! { Vector3, ParsingError, VectorParsingError, [x y z] }
create_simple_token_parser! { Normal3, ParsingError, NormalParsingError, [x y z] }

fn check_next_token<'a, I: Iterator<Item = &'a str>>(
    tokens: &mut I,
    expected: &'static str,
) -> Result<(), ParsingError> {
    match tokens.next() {
        Some(token) => {
            if token != expected {
                return Err(ParsingError::UnexpectedToken {
                    expected: expected,
                    found: token.to_string(),
                });
            } else {
                return Ok(());
            }
        }
        None => Err(ParsingError::UnexpectedEndOfTokens),
    }
}

fn parse_single_color_texture<'a, T: FromStr + MultiplyStable + 'static>(
    tokens: &mut impl Iterator<Item = &'a str>,
) -> Result<Box<dyn Image<ColorType = RGB<T>, PointType = Point2<T>>>, ParsingError>
where
    <T as FromStr>::Err: Error + Debug,
{
    if let Err(cause) = check_next_token(tokens, "single_color_texture") {
        return Err(ParsingError::SingleColorTextureParsingError(Box::new(
            cause,
        )));
    }
    if let Err(cause) = check_next_token(tokens, "{") {
        return Err(ParsingError::SingleColorTextureParsingError(Box::new(
            cause,
        )));
    }
    if let Err(cause) = check_next_token(tokens, "color:") {
        return Err(ParsingError::SingleColorTextureParsingError(Box::new(
            cause,
        )));
    }

    let color = RGB::from_tokens(tokens);

    if let Err(cause) = color {
        return Err(ParsingError::SingleColorTextureParsingError(Box::new(
            cause,
        )));
    }
    if let Err(cause) = check_next_token(tokens, "}") {
        return Err(ParsingError::SingleColorTextureParsingError(Box::new(
            cause,
        )));
    }

    Ok(Box::new(SingleColorImage::new(
        color.unwrap(),
        Vector2::new(One::one(), One::one()),
    )))
}

fn parse_unshaded_material<'a, T: FromStr + MultiplyStable + 'static>(
    tokens: &mut impl Iterator<Item = &'a str>,
) -> Result<UnshadedMaterial<Box<dyn Image<ColorType=RGB<T>, PointType=Point2<T>> >>, ParsingError>
where
    <T as FromStr>::Err: Error + Debug,
{
    if let Err(cause) = check_next_token(tokens, "{") {
        return Err(ParsingError::UnshadedMaterialParsingError(Box::new(cause)));
    }
    if let Err(cause) = check_next_token(tokens, "texture:") {
        return Err(ParsingError::UnshadedMaterialParsingError(Box::new(cause)));
    }

    let texture = parse_single_color_texture(tokens);
    if let Err(cause) = texture {
        return Err(ParsingError::UnshadedMaterialParsingError(Box::new(cause)));
    }

    if let Err(cause) = check_next_token(tokens, "}") {
        return Err(ParsingError::UnshadedMaterialParsingError(Box::new(cause)));
    }

    Ok(UnshadedMaterial::new(texture.unwrap()))
}

fn parse_lambert_material<'a, T: FromStr + MultiplyStable + 'static>(
    tokens: &mut impl Iterator<Item = &'a str>,
) -> Result<LambertMaterial< Box<dyn Image<ColorType=RGB<T>, PointType=Point2<T>> >  >, ParsingError>
where
    <T as FromStr>::Err: Error + Debug,
{
    if let Err(cause) = check_next_token(tokens, "{") {
        return Err(ParsingError::LambertMaterialParsingError(Box::new(cause)));
    }
    if let Err(cause) = check_next_token(tokens, "texture:") {
        return Err(ParsingError::LambertMaterialParsingError(Box::new(cause)));
    }

    let texture = parse_single_color_texture(tokens);
    if let Err(cause) = texture {
        return Err(ParsingError::UnshadedMaterialParsingError(Box::new(cause)));
    }

    if let Err(cause) = check_next_token(tokens, "}") {
        return Err(ParsingError::LambertMaterialParsingError(Box::new(cause)));
    }

    Ok(LambertMaterial::new(texture.unwrap()))
}

fn parse_phong_material<'a, T: MultiplyStable + 'static>(
    tokens: &mut impl Iterator<Item = &'a str>,
) -> Result<PhongMaterial<   Box<dyn Image<ColorType=RGB<T>, PointType=Point2<T>> >  >, ParsingError>
where
    <T as FromStr>::Err: Error,
{
    if let Err(cause) = check_next_token(tokens, "{") {
        return Err(ParsingError::PhongMaterialParsingError(Box::new(cause)));
    }

    let mut diffuse_texture: Option<Box<dyn Image<ColorType = RGB<T>, PointType = Point2<T>>>> =
        None;
    let mut specular_texture: Option<Box<dyn Image<ColorType = RGB<T>, PointType = Point2<T>>>> =
        None;
    let mut exponent = One::one();

    while let Some(token) = tokens.next() {
        match token {
            "diffuse_texture:" => match parse_single_color_texture(tokens) {
                Ok(texture) => {
                    diffuse_texture = Some(texture);
                }
                Err(cause) => {
                    return Err(ParsingError::PhongMaterialParsingError(Box::new(cause)));
                }
            },
            "specular_texture:" => match parse_single_color_texture(tokens) {
                Ok(texture) => {
                    specular_texture = Some(texture);
                }
                Err(cause) => {
                    return Err(ParsingError::PhongMaterialParsingError(Box::new(cause)));
                }
            },
            "exponent:" => match tokens.next() {
                Some(exponent_string) => match exponent_string.parse() {
                    Ok(exp) => exponent = exp,
                    Err(_) => {
                        return Err(ParsingError::NumberParsingError(
                            "Unable to parse field of number.",
                        ));
                    }
                },
                None => {
                    return Err(ParsingError::UnexpectedEndOfTokens);
                }
            },
            "}" => {
                break;
            }
            token => {
                return Err(ParsingError::UnexpectedToken {
                    expected: "diffuse_texture:, specular_texture:, exponent:, }",
                    found: token.to_string(),
                });
            }
        }
    }

    if let None = diffuse_texture {
        return Err(ParsingError::MissingElement("diffuse_texture"));
    }

    if let None = specular_texture {
        return Err(ParsingError::MissingElement("specular_texture"));
    }

    Ok(PhongMaterial::new(
        diffuse_texture.unwrap(),
        specular_texture.unwrap(),
        exponent,
    ))
}

fn parse_material<'a, T: Length>(
    tokens: &mut impl Iterator<Item = &'a str>,
) -> Result<Box<dyn Material<T, ColorType = RGB<<T as Length>::ValueType>>>, ParsingError>
where
    <T as Length>::ValueType: FloatingPoint
        + FromStr
        + MultiplyStable
        + Sqrt<Output = <T as Length>::ValueType>
        + 'static,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
    <T as Length>::AreaType: Sqrt<Output = T>,
{
    match tokens.next() {
        Some("unshaded_material") => match parse_unshaded_material(tokens) {
            Ok(material) => Ok(Box::new(material)),
            Err(cause) => Err(ParsingError::MaterialParsingError(Box::new(cause))),
        },
        Some("lambert_material") => match parse_lambert_material(tokens) {
            Ok(material) => Ok(Box::new(material)),
            Err(cause) => Err(ParsingError::MaterialParsingError(Box::new(cause))),
        },
        Some("phong_material") => match parse_phong_material(tokens) {
            Ok(material) => Ok(Box::new(material)),
            Err(cause) => Err(ParsingError::MaterialParsingError(Box::new(cause))),
        },
        Some(material) => Err(ParsingError::UnsupportedMaterial(material.to_string())),
        None => Err(ParsingError::UnexpectedEndOfTokens),
    }
}

fn parse_triangle<'a, T: Length + Neg<Output = T>>(
    tokens: &mut impl Iterator<Item = &'a str>,
) -> Result<RenderableGeometry<Triangle<Point3<T>>, T>, ParsingError>
where
    <T as Length>::ValueType: FloatingPoint
        + FromStr
        + MultiplyStable
        + Sqrt<Output = <T as Length>::ValueType>
        + Sin<Output = T::ValueType>
        + Cos<Output = T::ValueType>
        + ToRadians<Output = T::ValueType>
        + 'static,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
    <T as Length>::AreaType: Sqrt<Output = T>,
    <T as FromStr>::Err: Error,
{
    if let Err(cause) = check_next_token(tokens, "{") {
        return Err(ParsingError::TriangleParsingError(Box::new(cause)));
    }

    let mut material: Option<Box<dyn Material<T, ColorType = RGB<<T as Length>::ValueType>>>> =
        None;
    let transform = Transform::ident();

    let mut position: Vector3<T::ValueType> =
        Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());
    let mut scale: Vector3<T::ValueType> = Vector3::new(One::one(), One::one(), One::one());
    let mut rotation: Vector3<Degrees<T::ValueType>> =
        Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());

    let mut a: Option<Point3<T>> = None;
    let mut b: Option<Point3<T>> = None;
    let mut c: Option<Point3<T>> = None;

    let mut na: Option<Normal3<<T as Length>::ValueType>> = None;
    let mut nb: Option<Normal3<<T as Length>::ValueType>> = None;
    let mut nc: Option<Normal3<<T as Length>::ValueType>> = None;

    while let Some(token) = tokens.next() {
        match token {
            "a:" => match Point3::from_tokens(tokens) {
                Ok(point) => {
                    a = Some(point);
                }
                Err(cause) => {
                    return Err(ParsingError::TriangleParsingError(Box::new(cause)));
                }
            },
            "b:" => match Point3::from_tokens(tokens) {
                Ok(point) => {
                    b = Some(point);
                }
                Err(cause) => {
                    return Err(ParsingError::TriangleParsingError(Box::new(cause)));
                }
            },
            "c:" => match Point3::from_tokens(tokens) {
                Ok(point) => {
                    c = Some(point);
                }
                Err(cause) => {
                    return Err(ParsingError::TriangleParsingError(Box::new(cause)));
                }
            },
            "na:" => match Normal3::from_tokens(tokens) {
                Ok(point) => {
                    na = Some(point);
                }
                Err(cause) => {
                    return Err(ParsingError::TriangleParsingError(Box::new(cause)));
                }
            },
            "nb:" => match Normal3::from_tokens(tokens) {
                Ok(point) => {
                    nb = Some(point);
                }
                Err(cause) => {
                    return Err(ParsingError::TriangleParsingError(Box::new(cause)));
                }
            },
            "nc:" => match Normal3::from_tokens(tokens) {
                Ok(point) => {
                    nc = Some(point);
                }
                Err(cause) => {
                    return Err(ParsingError::TriangleParsingError(Box::new(cause)));
                }
            },

            "material:" => match parse_material(tokens) {
                Ok(mat) => {
                    material = Some(mat);
                }
                Err(cause) => {
                    return Err(ParsingError::TriangleParsingError(Box::new(cause)));
                }
            },
            "position:" => match Vector3::from_tokens(tokens) {
                Ok(vec) => {
                    position = vec;
                }
                Err(cause) => {
                    return Err(ParsingError::TriangleParsingError(Box::new(cause)));
                }
            },
            "scale:" => match Vector3::from_tokens(tokens) {
                Ok(vec) => {
                    scale = vec;
                }
                Err(cause) => {
                    return Err(ParsingError::TriangleParsingError(Box::new(cause)));
                }
            },
            "rotation:" => match Vector3::from_tokens(tokens) {
                Ok(vec) => {
                    rotation = vec;
                }
                Err(cause) => {
                    return Err(ParsingError::TriangleParsingError(Box::new(cause)));
                }
            },
            "}" => {
                break;
            }
            token => {
                return Err(ParsingError::UnexpectedToken {
                    expected: "material:, position:, scale:, rotation:, }",
                    found: token.to_string(),
                });
            }
        }
    }

    if let None = material {
        return Err(ParsingError::MissingElement("material"));
    }
    if let None = a {
        return Err(ParsingError::MissingElement("a"));
    }
    if let None = b {
        return Err(ParsingError::MissingElement("b"));
    }
    if let None = c {
        return Err(ParsingError::MissingElement("c"));
    }
    if let None = na {
        return Err(ParsingError::MissingElement("na"));
    }
    if let None = nb {
        return Err(ParsingError::MissingElement("nb"));
    }
    if let None = nc {
        return Err(ParsingError::MissingElement("nc"));
    }

    let triangle = Triangle::new(
        a.unwrap(),
        b.unwrap(),
        c.unwrap(),
        na.unwrap(),
        nb.unwrap(),
        nc.unwrap(),
    );

    let triangle_geometry = RenderableGeometry::new(
        triangle,
        material.unwrap(),
        transform
            .translate(position.x, position.y, position.z)
            .rotate_z(rotation.z)
            .rotate_x(rotation.x)
            .rotate_y(rotation.y)
            .scale(scale.x, scale.y, scale.z),
    );

    Ok(triangle_geometry)
}

fn parse_box<'a, T: Length + Neg<Output = T>>(
    tokens: &mut impl Iterator<Item = &'a str>,
) -> Result<RenderableGeometry<AxisAlignedBox<Point3<T>>, T>, ParsingError>
where
    <T as Length>::ValueType: FloatingPoint
        + FromStr
        + MultiplyStable
        + Sqrt<Output = <T as Length>::ValueType>
        + Sin<Output = T::ValueType>
        + Cos<Output = T::ValueType>
        + ToRadians<Output = T::ValueType>
        + 'static,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
    <T as Length>::AreaType: Sqrt<Output = T>,
{
    if let Err(cause) = check_next_token(tokens, "{") {
        return Err(ParsingError::BoxParsingError(Box::new(cause)));
    }

    let mut material: Option<Box<dyn Material<T, ColorType = RGB<<T as Length>::ValueType>>>> =
        None;
    let transform = Transform::ident();

    let mut position: Vector3<T::ValueType> =
        Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());
    let mut scale: Vector3<T::ValueType> = Vector3::new(One::one(), One::one(), One::one());
    let mut rotation: Vector3<Degrees<T::ValueType>> =
        Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());

    while let Some(token) = tokens.next() {
        match token {
            "material:" => match parse_material(tokens) {
                Ok(mat) => {
                    material = Some(mat);
                }
                Err(cause) => {
                    return Err(ParsingError::BoxParsingError(Box::new(cause)));
                }
            },
            "position:" => match Vector3::from_tokens(tokens) {
                Ok(vec) => {
                    position = vec;
                }
                Err(cause) => {
                    return Err(ParsingError::BoxParsingError(Box::new(cause)));
                }
            },
            "scale:" => match Vector3::from_tokens(tokens) {
                Ok(vec) => {
                    scale = vec;
                }
                Err(cause) => {
                    return Err(ParsingError::BoxParsingError(Box::new(cause)));
                }
            },
            "rotation:" => match Vector3::from_tokens(tokens) {
                Ok(vec) => {
                    rotation = vec;
                }
                Err(cause) => {
                    return Err(ParsingError::BoxParsingError(Box::new(cause)));
                }
            },
            "}" => {
                break;
            }
            token => {
                return Err(ParsingError::UnexpectedToken {
                    expected: "material:, position:, scale:, rotation:, }",
                    found: token.to_string(),
                });
            }
        }
    }

    if let None = material {
        return Err(ParsingError::MissingElement("material"));
    }

    let aab = AxisAlignedBox::new(
        Point3::<T>::new(-T::one(), -T::one(), -T::one()),
        Point3::new(T::one(), T::one(), T::one()),
    );

    let aab_geometry = RenderableGeometry::new(
        aab,
        material.unwrap(),
        transform
            .translate(position.x, position.y, position.z)
            .rotate_z(rotation.z)
            .rotate_x(rotation.x)
            .rotate_y(rotation.y)
            .scale(scale.x, scale.y, scale.z),
    );

    Ok(aab_geometry)
}

fn parse_plane<'a, T: Length>(
    tokens: &mut impl Iterator<Item = &'a str>,
) -> Result<RenderableGeometry<ImplicitPlane3<T>, T>, ParsingError>
where
    <T as Length>::ValueType: FloatingPoint
        + FromStr
        + MultiplyStable
        + Sqrt<Output = <T as Length>::ValueType>
        + Sin<Output = T::ValueType>
        + Cos<Output = T::ValueType>
        + ToRadians<Output = T::ValueType>
        + 'static,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
    <T as Length>::AreaType: Sqrt<Output = T>,
{
    if let Err(cause) = check_next_token(tokens, "{") {
        return Err(ParsingError::PlaneParsingError(Box::new(cause)));
    }

    let mut material: Option<Box<dyn Material<T, ColorType = RGB<<T as Length>::ValueType>>>> =
        None;
    let transform = Transform::ident();

    let mut position: Vector3<T::ValueType> =
        Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());
    let mut scale: Vector3<T::ValueType> = Vector3::new(One::one(), One::one(), One::one());
    let mut rotation: Vector3<Degrees<T::ValueType>> =
        Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());

    while let Some(token) = tokens.next() {
        match token {
            "material:" => match parse_material(tokens) {
                Ok(mat) => {
                    material = Some(mat);
                }
                Err(cause) => {
                    return Err(ParsingError::PlaneParsingError(Box::new(cause)));
                }
            },
            "position:" => match Vector3::from_tokens(tokens) {
                Ok(vec) => {
                    position = vec;
                }
                Err(cause) => {
                    return Err(ParsingError::PlaneParsingError(Box::new(cause)));
                }
            },
            "scale:" => match Vector3::from_tokens(tokens) {
                Ok(vec) => {
                    scale = vec;
                }
                Err(cause) => {
                    return Err(ParsingError::PlaneParsingError(Box::new(cause)));
                }
            },
            "rotation:" => match Vector3::from_tokens(tokens) {
                Ok(vec) => {
                    rotation = vec;
                }
                Err(cause) => {
                    return Err(ParsingError::PlaneParsingError(Box::new(cause)));
                }
            },
            "}" => {
                break;
            }
            token => {
                return Err(ParsingError::UnexpectedToken {
                    expected: "material:, position:, scale:, rotation:, }",
                    found: token.to_string(),
                });
            }
        }
    }

    if let None = material {
        return Err(ParsingError::MissingElement("material"));
    }

    let plane = ImplicitPlane3::new(
        Point3::new(Zero::zero(), Zero::zero(), Zero::zero()),
        Normal3::new(Zero::zero(), One::one(), Zero::zero()),
    );

    let plane_geometry = RenderableGeometry::new(
        plane,
        material.unwrap(),
        transform
            .translate(position.x, position.y, position.z)
            .rotate_z(rotation.z)
            .rotate_x(rotation.x)
            .rotate_y(rotation.y)
            .scale(scale.x, scale.y, scale.z),
    );

    Ok(plane_geometry)
}

fn parse_sphere<'a, T: Length>(
    tokens: &mut impl Iterator<Item = &'a str>,
) -> Result<RenderableGeometry<ImplicitNSphere<Point3<T>>, T>, ParsingError>
where
    <T as Length>::ValueType: FloatingPoint
        + FromStr
        + MultiplyStable
        + Sqrt<Output = <T as Length>::ValueType>
        + Sin<Output = T::ValueType>
        + Cos<Output = T::ValueType>
        + ToRadians<Output = T::ValueType>
        + 'static,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
    <T as Length>::AreaType: Sqrt<Output = T>,
{
    if let Err(cause) = check_next_token(tokens, "{") {
        return Err(ParsingError::SphereParsingError(Box::new(cause)));
    }

    let mut material: Option<Box<dyn Material<T, ColorType = RGB<<T as Length>::ValueType>>>> =
        None;
    let transform = Transform::ident();

    let mut position: Vector3<T::ValueType> =
        Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());
    let mut scale: Vector3<T::ValueType> = Vector3::new(One::one(), One::one(), One::one());
    let mut rotation: Vector3<Degrees<T::ValueType>> =
        Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());

    while let Some(token) = tokens.next() {
        match token {
            "material:" => match parse_material(tokens) {
                Ok(mat) => {
                    material = Some(mat);
                }
                Err(cause) => {
                    return Err(ParsingError::SphereParsingError(Box::new(cause)));
                }
            },
            "position:" => match Vector3::from_tokens(tokens) {
                Ok(vec) => {
                    position = vec;
                }
                Err(cause) => {
                    return Err(ParsingError::SphereParsingError(Box::new(cause)));
                }
            },
            "scale:" => match Vector3::from_tokens(tokens) {
                Ok(vec) => {
                    scale = vec;
                }
                Err(cause) => {
                    return Err(ParsingError::SphereParsingError(Box::new(cause)));
                }
            },
            "rotation:" => match Vector3::from_tokens(tokens) {
                Ok(vec) => {
                    rotation = vec;
                }
                Err(cause) => {
                    return Err(ParsingError::SphereParsingError(Box::new(cause)));
                }
            },
            "}" => {
                break;
            }
            token => {
                return Err(ParsingError::UnexpectedToken {
                    expected: "material:, position:, scale:, rotation:, }",
                    found: token.to_string(),
                });
            }
        }
    }

    if let None = material {
        return Err(ParsingError::MissingElement("material"));
    }

    let sphere = ImplicitNSphere::new(
        Point3::new(Zero::zero(), Zero::zero(), Zero::zero()),
        One::one(),
    );
    let sphere_geometry = RenderableGeometry::new(
        sphere,
        material.unwrap(),
        transform
            .translate(position.x, position.y, position.z)
            .rotate_z(rotation.z)
            .rotate_x(rotation.x)
            .rotate_y(rotation.y)
            .scale(scale.x, scale.y, scale.z),
    );

    Ok(sphere_geometry)
}

fn parse_perspective_camera<'a, T: Length + 'static>(
    tokens: &mut impl Iterator<Item = &'a str>,
    screen_size: Vector2<<T as Length>::ValueType>,
) -> Result<(String, Perspective<T>), ParsingError>
where
    T: Neg<Output = T>,
    <T as Length>::AreaType: Sqrt<Output = T>,
    <T as Length>::ValueType: Mul<T, Output = T>
        + Tan<Output = <T as Length>::ValueType>
        + FloatingPoint
        + Half
        + MultiplyStable
        + Sqrt<Output = <T as Length>::ValueType>
        + ToRadians<Output = <T as Length>::ValueType>,
    <T as FromStr>::Err: Error + Debug,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
{
    if let Err(cause) = check_next_token(tokens, "{") {
        return Err(ParsingError::PerspectiveCameraParsingError(Box::new(cause)));
    }

    let mut id = "main";
    let mut eye_position: Point3<T> = Point3::new(Zero::zero(), Zero::zero(), Zero::zero());
    let mut gaze_direction: Vector3<T> = Vector3::new(Zero::zero(), Zero::zero(), -T::one());
    let mut up_vector: Vector3<T> = Vector3::new(Zero::zero(), One::one(), Zero::zero());
    let mut field_of_view: Degrees<<T as Length>::ValueType> = Degrees::new(Zero::zero());

    while let Some(token) = tokens.next() {
        match token {
            "id:" => match tokens.next() {
                Some(parsed_id) => {
                    id = parsed_id;
                }
                None => {
                    return Err(ParsingError::UnexpectedEndOfTokens);
                }
            },
            "eye_position:" => match Point3::from_tokens(tokens) {
                Ok(pos) => {
                    eye_position = pos;
                }
                Err(cause) => {
                    return Err(ParsingError::PerspectiveCameraParsingError(Box::new(cause)));
                }
            },
            "gaze_direction:" => match Vector3::from_tokens(tokens) {
                Ok(vec) => {
                    gaze_direction = vec;
                }
                Err(cause) => {
                    return Err(ParsingError::PerspectiveCameraParsingError(Box::new(cause)));
                }
            },
            "up_vector:" => match Vector3::from_tokens(tokens) {
                Ok(vec) => {
                    up_vector = vec;
                }
                Err(cause) => {
                    return Err(ParsingError::PerspectiveCameraParsingError(Box::new(cause)));
                }
            },
            "field_of_view:" => match tokens.next() {
                Some(fov_string) => match fov_string.parse() {
                    Ok(fov) => field_of_view = fov,
                    Err(_) => {
                        return Err(ParsingError::NumberParsingError(
                            "Unable to parse field of number.",
                        ));
                    }
                },
                None => {
                    return Err(ParsingError::UnexpectedEndOfTokens);
                }
            },
            "}" => {
                break;
            }
            token => {
                return Err(ParsingError::UnexpectedToken {
                    expected: "id:, eye_position:, gaze_direction:, up_vector:, field_of_view:, }",
                    found: token.to_string(),
                });
            }
        }
    }
    Ok((
        id.to_string(),
        Perspective::new(
            eye_position,
            gaze_direction,
            up_vector,
            field_of_view.to_radians(),
            screen_size,
        ),
    ))
}

pub fn parse_spot_light<'a, T: Length + 'static>(
    tokens: &mut impl Iterator<Item = &'a str>,
) -> Result<SpotLight<T, RGB<<T as Length>::ValueType>>, ParsingError>
where
    <T as Length>::AreaType: Sqrt<Output = T>,
    <T as Length>::ValueType: ToRadians<Output=<T as Length>::ValueType> + 
        Neg<Output = <T as Length>::ValueType> + MultiplyStable + Mul<T, Output = T>,
    <T as FromStr>::Err: Error + Debug,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
{
    if let Err(cause) = check_next_token(tokens, "{") {
        return Err(ParsingError::SpotLightParsingError(Box::new(cause)));
    }

    let mut color = RGB::new(Zero::zero(), Zero::zero(), Zero::zero());
    let mut position: Point3<T> = Point3::new(Zero::zero(), Zero::zero(), Zero::zero());
    let mut direction: Option<Normal3<<T as Length>::ValueType>> = None;
    let mut angle: Option<Degrees<<T as Length>::ValueType>> = None;

    while let Some(token) = tokens.next() {
        match token {
            "color:" => match RGB::from_tokens(tokens) {
                Ok(col) => {
                    color = col;
                }
                Err(cause) => {
                    return Err(ParsingError::SpotLightParsingError(Box::new(cause)));
                }
            },

            "position:" => match Point3::from_tokens(tokens) {
                Ok(pos) => {
                    position = pos;
                }
                Err(cause) => {
                    return Err(ParsingError::SpotLightParsingError(Box::new(cause)));
                }
            },
            "direction:" => match Vector3::<T>::from_tokens(tokens) {
                Ok(vec) => {
                    direction = Some(vec.normalized());
                }
                Err(cause) => {
                    return Err(ParsingError::SpotLightParsingError(Box::new(cause)));
                }
            }
            "angle:" => match tokens.next() {
                Some(angle_string) => match angle_string.parse() {
                    Ok(a) => angle = Some(a),
                    Err(_) => {
                        return Err(ParsingError::NumberParsingError(
                            "Unable to parse field of number.",
                        ));
                    }
                },
                None => {
                    return Err(ParsingError::UnexpectedEndOfTokens);
                }
            },
            "}" => {
                break;
            }
            token => {
                return Err(ParsingError::UnexpectedToken {
                    expected: "color:, position:, }",
                    found: token.to_string(),
                });
            }
        }
    }
    let spot_light = SpotLight::new(
        color,
        position,
        direction.unwrap(),
        angle.unwrap().to_radians(),
    );


    Ok(spot_light)
}


pub fn parse_point_light<'a, T: Length + 'static>(
    tokens: &mut impl Iterator<Item = &'a str>,
) -> Result<PointLight<T, RGB<<T as Length>::ValueType>>, ParsingError>
where
    <T as Length>::AreaType: Sqrt<Output = T>,
    <T as Length>::ValueType:
        Neg<Output = <T as Length>::ValueType> + MultiplyStable + Mul<T, Output = T>,
    <T as FromStr>::Err: Error + Debug,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
{
    if let Err(cause) = check_next_token(tokens, "{") {
        return Err(ParsingError::PointLightParsingError(Box::new(cause)));
    }

    let mut color = RGB::new(Zero::zero(), Zero::zero(), Zero::zero());
    let mut position: Point3<T> = Point3::new(Zero::zero(), Zero::zero(), Zero::zero());

    while let Some(token) = tokens.next() {
        match token {
            "color:" => match RGB::from_tokens(tokens) {
                Ok(col) => {
                    color = col;
                }
                Err(cause) => {
                    return Err(ParsingError::PointLightParsingError(Box::new(cause)));
                }
            },

            "position:" => match Point3::from_tokens(tokens) {
                Ok(pos) => {
                    position = pos;
                }
                Err(cause) => {
                    return Err(ParsingError::PointLightParsingError(Box::new(cause)));
                }
            },
            "}" => {
                break;
            }
            token => {
                return Err(ParsingError::UnexpectedToken {
                    expected: "color:, position:, }",
                    found: token.to_string(),
                });
            }
        }
    }

    Ok(PointLight::new(color, position))
}

pub fn parse_scene<T: Length + Neg<Output = T> + 'static>(
    filename: &str,
    screen_size: Vector2<<T as Length>::ValueType>,
) -> Result<Scene<T, RGB<<T as Length>::ValueType>>, ParsingError>
where
    <T as Length>::ValueType: Sin<Output = T::ValueType>
        + Cos<Output = T::ValueType>
        + FloatingPoint
        + Mul<T, Output = T>
        + MultiplyStable
        + Tan<Output = <T as Length>::ValueType>
        + Half
        + Sqrt<Output = <T as Length>::ValueType>
        + ToRadians<Output = <T as Length>::ValueType>,
    <<T as Length>::ValueType as FromStr>::Err: Error,
    <T as Length>::AreaType: Mul<T>
        + Mul
        + Div<Output = <T as Length>::ValueType>
        + Sqrt<Output = T>
        + Neg<Output = <T as Length>::AreaType>,
    <<T as Length>::AreaType as Mul>::Output: Add<Output = <<T as Length>::AreaType as Mul>::Output>
        + Sub<Output = <<T as Length>::AreaType as Mul>::Output>
        + Sqrt<Output = <T as Length>::AreaType>
        + Zero
        + PartialOrd
        + Copy,
    <T as FromStr>::Err: Error,
    Normal3<<T as Length>::ValueType>: Orthonormal3<<T as Length>::ValueType>,
    <<T as Length>::AreaType as Mul<T>>::Output: PartialEq
        + Copy
        + Zero
        + Div<Output = <T as Length>::ValueType>
        + Sub<Output = <<T as Length>::AreaType as Mul<T>>::Output>
        + Add<Output = <<T as Length>::AreaType as Mul<T>>::Output>,
{
    let file_content = fs::read_to_string(filename).expect("Unable to read file");

    let mut tokens = file_content
        .split(&[' ', '\t', '\n'])
        .filter(|token| !token.is_empty());

    let mut geometries: Vec<
        Box<
            dyn Renderable<
                T,
                ScalarType = <T as Length>::ValueType,
                ColorType = RGB<<T as Length>::ValueType>,
                LengthType = T,
                VectorType = Vector3<T>,
                PointType = Point3<T>,
                NormalType = Normal3<<T as Length>::ValueType>,
            >,
        >,
    > = Vec::new();
    let mut lights: Vec<Box<dyn Light<T, RGB<<T as Length>::ValueType>>>> = Vec::new();
    let mut cameras: HashMap<String, Box<dyn RaytracingCamera<T>>> = HashMap::new();
    let mut background_color: RGB<<T as Length>::ValueType> =
        RGB::new(Zero::zero(), Zero::zero(), Zero::zero());
    let mut ambient_light: RGB<<T as Length>::ValueType> =
        RGB::new(Zero::zero(), Zero::zero(), Zero::zero());

    while let Some(token) = tokens.next() {
        match token {
            "sphere" => {
                geometries.push(Box::new(parse_sphere::<T>(&mut tokens).unwrap()));
            }
            "plane" => {
                geometries.push(Box::new(parse_plane::<T>(&mut tokens).unwrap()));
            }
            "box" => {
                geometries.push(Box::new(parse_box::<T>(&mut tokens).unwrap()));
            }
            "triangle" => {
                geometries.push(Box::new(parse_triangle::<T>(&mut tokens).unwrap()));
            }
            "perspective_camera" => {
                let (id, camera) = parse_perspective_camera::<T>(&mut tokens, screen_size).unwrap();
                cameras.insert(id, Box::new(camera));
            }
            "point_light" => {
                lights.push(Box::new(parse_point_light(&mut tokens).unwrap()));
            }
            "spot_light" => {
                lights.push(Box::new(parse_spot_light(&mut tokens).unwrap()));
            }
            "background_color:" => {
                background_color = RGB::from_tokens(&mut tokens).unwrap();
            }
            "ambient_light:" => {
                ambient_light = RGB::from_tokens(&mut tokens).unwrap();
            }
            &_ => {
                println!("Unexpected token while parsing root of scene: {}", token);
                break;
            }
        }
    }

    Ok(Scene::new(
        background_color,
        ambient_light,
        lights,
        cameras,
        geometries,
    ))
}
