use std::error::Error;
use std::fmt::Debug;
use std::str::FromStr;

use crate::color::RGB;
use crate::image::{Image, SingleColorImage};
use crate::math::{Point2, Vector2};
use crate::traits::number::MultiplyStable;
use crate::traits::One;

use crate::parser::util;
use crate::parser::{FromTokens, ParsingError};

pub fn parse_texture<'a, T: FromStr + MultiplyStable + 'static>(
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
