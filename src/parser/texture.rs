use std::error::Error;
use std::fmt::Debug;
use std::str::FromStr;

use crate::color::RGB;
use crate::image::generator::{Checkerboard, Grid};
use crate::image::{Image, SingleColorImage};
use crate::math::{Point2, Vector2};
use crate::traits::number::MultiplyStable;
use crate::traits::{Half, One};

use crate::parser::util;
use crate::parser::{FromTokens, ParsingError};

pub fn parse_texture<'a, T: FromStr + MultiplyStable + Half + 'static>(
    tokens: &mut impl Iterator<Item = &'a str>,
) -> Result<Box<dyn Image<ColorType = RGB<T>, PointType = Point2<T>>>, ParsingError>
where
    <T as FromStr>::Err: Error + Debug,
{
    match tokens.next() {
        Some("single_color_texture") => match SingleColorImage::from_tokens(tokens) {
            Ok(tex) => Ok(Box::new(tex)),
            Err(cause) => Err(ParsingError::TextureParsingError(Box::new(cause))),
        },
        Some("checkerboard_texture") => match Checkerboard::from_tokens(tokens) {
            Ok(tex) => Ok(Box::new(tex)),
            Err(cause) => Err(ParsingError::TextureParsingError(Box::new(cause))),
        },
        Some("grid_texture") => match Grid::from_tokens(tokens) {
            Ok(tex) => Ok(Box::new(tex)),
            Err(cause) => Err(ParsingError::TextureParsingError(Box::new(cause))),
        },

        Some(texture) => Err(ParsingError::UnsupportedTexture(texture.to_string())),
        None => Err(ParsingError::UnexpectedEndOfTokens),
    }
}

impl<T: FromStr + MultiplyStable> FromTokens for SingleColorImage<RGB<T>, Vector2<T>>
where
    <T as FromStr>::Err: Error + Debug,
{
    type Err = ParsingError;

    fn from_tokens<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, Self::Err> {
        if let Err(cause) = util::check_next_token(tokens, "{") {
            return Err(ParsingError::SingleColorTextureParsingError(Box::new(
                cause,
            )));
        }
        if let Err(cause) = util::check_next_token(tokens, "color:") {
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
        if let Err(cause) = util::check_next_token(tokens, "}") {
            return Err(ParsingError::SingleColorTextureParsingError(Box::new(
                cause,
            )));
        }

        Ok(SingleColorImage::new(
            color.unwrap(),
            Vector2::new(One::one(), One::one()),
        ))
    }
}

impl<T: FromStr + MultiplyStable + Half> FromTokens for Checkerboard<RGB<T>>
where
    <T as FromStr>::Err: Error + Debug,
{
    type Err = ParsingError;

    fn from_tokens<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, Self::Err> {
        if let Err(cause) = util::check_next_token(tokens, "{") {
            return Err(ParsingError::CheckerboardTextureParsingError(Box::new(
                cause,
            )));
        }

        let mut a: Option<RGB<T>> = None;
        let mut b: Option<RGB<T>> = None;

        while let Some(token) = tokens.next() {
            match token {
                "a:" => match RGB::from_tokens(tokens) {
                    Ok(color) => {
                        a = Some(color);
                    }
                    Err(cause) => {
                        return Err(ParsingError::CheckerboardTextureParsingError(Box::new(
                            cause,
                        )));
                    }
                },
                "b:" => match RGB::from_tokens(tokens) {
                    Ok(color) => {
                        b = Some(color);
                    }
                    Err(cause) => {
                        return Err(ParsingError::CheckerboardTextureParsingError(Box::new(
                            cause,
                        )));
                    }
                },
                "}" => {
                    break;
                }
                token => {
                    return Err(ParsingError::UnexpectedToken {
                        expected: "a:, b:, }",
                        found: token.to_string(),
                    });
                }
            }
        }

        if let None = a {
            return Err(ParsingError::MissingElement("a"));
        }
        if let None = b {
            return Err(ParsingError::MissingElement("b"));
        }

        Ok(Checkerboard::generate(a.unwrap(), b.unwrap()))
    }
}

impl<T: FromStr + MultiplyStable + Half> FromTokens for Grid<RGB<T>>
where
    <T as FromStr>::Err: Error + Debug,
{
    type Err = ParsingError;

    fn from_tokens<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, Self::Err> {
        if let Err(cause) = util::check_next_token(tokens, "{") {
            return Err(ParsingError::GridTextureParsingError(Box::new(cause)));
        }

        let mut border: Option<RGB<T>> = None;
        let mut face: Option<RGB<T>> = None;
        let mut width: Option<T> = None;

        while let Some(token) = tokens.next() {
            match token {
                "border:" => match RGB::from_tokens(tokens) {
                    Ok(color) => {
                        border = Some(color);
                    }
                    Err(cause) => {
                        return Err(ParsingError::GridTextureParsingError(Box::new(cause)));
                    }
                },
                "face:" => match RGB::from_tokens(tokens) {
                    Ok(color) => {
                        face = Some(color);
                    }
                    Err(cause) => {
                        return Err(ParsingError::GridTextureParsingError(Box::new(cause)));
                    }
                },
                "width:" => match tokens.next() {
                    Some(width_string) => match width_string.parse() {
                        Ok(w) => width = Some(w),
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
                        expected: "border:, face:, width:, }",
                        found: token.to_string(),
                    });
                }
            }
        }

        if let None = border {
            return Err(ParsingError::MissingElement("border"));
        }
        if let None = face {
            return Err(ParsingError::MissingElement("face"));
        }
        if let None = width {
            return Err(ParsingError::MissingElement("width"));
        }

        Ok(Grid::generate(
            border.unwrap(),
            face.unwrap(),
            width.unwrap(),
        ))
    }
}
