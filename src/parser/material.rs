use std::error::Error;
use std::fmt::Debug;
use std::str::FromStr;

use crate::color::RGB;
use crate::image::Image;
use crate::material::{LambertMaterial, Material, PhongMaterial, UnshadedMaterial};
use crate::math::Point2;
use crate::traits::number::MultiplyStable;
use crate::traits::{FloatingPoint, Half, One, Sqrt};
use crate::units::length::Length;

use crate::parser::texture;
use crate::parser::util;
use crate::parser::{FromTokens, ParsingError};

pub fn parse_material<'a, T: Length>(
    tokens: &mut impl Iterator<Item = &'a str>,
) -> Result<Box<dyn Material<T, ColorType = RGB<<T as Length>::ValueType>>>, ParsingError>
where
    <T as Length>::ValueType: FloatingPoint + Half + FromStr + MultiplyStable + 'static,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
    <T as Length>::AreaType: Sqrt<Output = T>,
{
    match tokens.next() {
        Some("unshaded_material") => match UnshadedMaterial::from_tokens(tokens) {
            Ok(material) => Ok(Box::new(material)),
            Err(cause) => Err(ParsingError::MaterialParsingError(Box::new(cause))),
        },
        Some("lambert_material") => match LambertMaterial::from_tokens(tokens) {
            Ok(material) => Ok(Box::new(material)),
            Err(cause) => Err(ParsingError::MaterialParsingError(Box::new(cause))),
        },
        Some("phong_material") => match PhongMaterial::from_tokens(tokens) {
            Ok(material) => Ok(Box::new(material)),
            Err(cause) => Err(ParsingError::MaterialParsingError(Box::new(cause))),
        },
        Some(material) => Err(ParsingError::UnsupportedMaterial(material.to_string())),
        None => Err(ParsingError::UnexpectedEndOfTokens),
    }
}

impl<T: FromStr + Half + MultiplyStable + 'static> FromTokens
    for UnshadedMaterial<Box<dyn Image<ColorType = RGB<T>, PointType = Point2<T>>>>
where
    <T as FromStr>::Err: Error + Debug,
{
    type Err = ParsingError;

    fn from_tokens<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, Self::Err> {
        if let Err(cause) = util::check_next_token(tokens, "{") {
            return Err(ParsingError::UnshadedMaterialParsingError(Box::new(cause)));
        }
        if let Err(cause) = util::check_next_token(tokens, "texture:") {
            return Err(ParsingError::UnshadedMaterialParsingError(Box::new(cause)));
        }

        let texture = texture::parse_texture(tokens);
        if let Err(cause) = texture {
            return Err(ParsingError::UnshadedMaterialParsingError(Box::new(cause)));
        }

        if let Err(cause) = util::check_next_token(tokens, "}") {
            return Err(ParsingError::UnshadedMaterialParsingError(Box::new(cause)));
        }

        Ok(UnshadedMaterial::new(texture.unwrap()))
    }
}

impl<T: FromStr + Half + MultiplyStable + 'static> FromTokens
    for LambertMaterial<Box<dyn Image<ColorType = RGB<T>, PointType = Point2<T>>>>
where
    <T as FromStr>::Err: Error + Debug,
{
    type Err = ParsingError;

    fn from_tokens<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, Self::Err> {
        if let Err(cause) = util::check_next_token(tokens, "{") {
            return Err(ParsingError::LambertMaterialParsingError(Box::new(cause)));
        }
        if let Err(cause) = util::check_next_token(tokens, "texture:") {
            return Err(ParsingError::LambertMaterialParsingError(Box::new(cause)));
        }

        let texture = texture::parse_texture(tokens);
        if let Err(cause) = texture {
            return Err(ParsingError::LambertMaterialParsingError(Box::new(cause)));
        }

        if let Err(cause) = util::check_next_token(tokens, "}") {
            return Err(ParsingError::LambertMaterialParsingError(Box::new(cause)));
        }

        Ok(LambertMaterial::new(texture.unwrap()))
    }
}

impl<T: FromStr + Half + MultiplyStable + 'static> FromTokens
    for PhongMaterial<Box<dyn Image<ColorType = RGB<T>, PointType = Point2<T>>>>
where
    <T as FromStr>::Err: Error + Debug,
{
    type Err = ParsingError;

    fn from_tokens<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, Self::Err> {
        if let Err(cause) = util::check_next_token(tokens, "{") {
            return Err(ParsingError::PhongMaterialParsingError(Box::new(cause)));
        }

        let mut diffuse_texture: Option<Box<dyn Image<ColorType = RGB<T>, PointType = Point2<T>>>> =
            None;
        let mut specular_texture: Option<
            Box<dyn Image<ColorType = RGB<T>, PointType = Point2<T>>>,
        > = None;
        let mut exponent = One::one();

        while let Some(token) = tokens.next() {
            match token {
                "diffuse_texture:" => match texture::parse_texture(tokens) {
                    Ok(texture) => {
                        diffuse_texture = Some(texture);
                    }
                    Err(cause) => {
                        return Err(ParsingError::PhongMaterialParsingError(Box::new(cause)));
                    }
                },
                "specular_texture:" => match texture::parse_texture(tokens) {
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
}
