use std::error::Error;
use std::fmt::Debug;
use std::str::FromStr;

use colors::RGB;
use math::{Normal3, Point2, Point3, Vector3};

use crate::parser::{FromTokens, ParsingError};

pub fn parse_next<'a, T: FromStr>(
    tokens: &mut impl Iterator<Item = &'a str>,
) -> Result<T, ParsingError>
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
create_simple_token_parser! { Point2, ParsingError, Point2ParsingError, [x y] }
create_simple_token_parser! { Point3, ParsingError, Point3ParsingError, [x y z] }
create_simple_token_parser! { Vector3, ParsingError, VectorParsingError, [x y z] }
create_simple_token_parser! { Normal3, ParsingError, NormalParsingError, [x y z] }
