use std::error::Error;
use std::fmt::Debug;
use std::str::FromStr;

use crate::light::{AmbientOcclusionLight, PointLight, SpotLight};
use colors::RGB;
use math::{Point2, Point3, Vector3};
use random::{RandomNumberGenerator, WichmannHillPRNG};
use sampling::{MultiJitteredPatterGenerator, PatternMapping, SamplingPatternSet};
use traits::floating_point::ToRadians;
use traits::{FloatingPoint, SignedNumber, Sqrt, Zero};
use units::angle::Degrees;
use units::length::Length;

use crate::parser::util;
use crate::parser::{FromTokens, ParsingError};

impl<T: Length> FromTokens for SpotLight<T, RGB<<T as Length>::ValueType>>
where
    <T as Length>::AreaType: Sqrt<Output = T>,
    <T as Length>::ValueType: FloatingPoint,
    <T as FromStr>::Err: Error + Debug,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
{
    type Err = ParsingError;

    fn from_tokens<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, Self::Err> {
        if let Err(cause) = util::check_next_token(tokens, "{") {
            return Err(ParsingError::SpotLightParsingError(Box::new(cause)));
        }

        let mut color = RGB::new(Zero::zero(), Zero::zero(), Zero::zero());
        let mut position: Point3<T> = Point3::new(Zero::zero(), Zero::zero(), Zero::zero());
        let mut direction: Option<Vector3<<T as Length>::ValueType>> = None;
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
                },
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
}

impl<T: Length> FromTokens for PointLight<T, RGB<<T as Length>::ValueType>>
where
    <T as Length>::AreaType: Sqrt<Output = T>,
    <T as Length>::ValueType: SignedNumber,
    <T as FromStr>::Err: Error + Debug,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
{
    type Err = ParsingError;

    fn from_tokens<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, Self::Err> {
        if let Err(cause) = util::check_next_token(tokens, "{") {
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
}

impl<T: Length> FromTokens for AmbientOcclusionLight<T, RGB<<T as Length>::ValueType>>
where
    <T as Length>::AreaType: Sqrt<Output = T>,
    <T as Length>::ValueType: SignedNumber,
    <T as FromStr>::Err: Error + Debug,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
    SamplingPatternSet<Point2<T::ValueType>>:
        MultiJitteredPatterGenerator<T::ValueType> + PatternMapping<T::ValueType>,
    WichmannHillPRNG: RandomNumberGenerator<T::ValueType>,
{
    type Err = ParsingError;

    fn from_tokens<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, Self::Err> {
        if let Err(cause) = util::check_next_token(tokens, "{") {
            return Err(ParsingError::AmbientOcclusionLightParsingError(Box::new(
                cause,
            )));
        }

        let mut color = RGB::new(Zero::zero(), Zero::zero(), Zero::zero());
        let mut mapping_exponent: T::ValueType = T::ValueType::zero();
        let mut distance: Option<T> = None;

        while let Some(token) = tokens.next() {
            match token {
                "color:" => match RGB::from_tokens(tokens) {
                    Ok(col) => {
                        color = col;
                    }
                    Err(cause) => {
                        return Err(ParsingError::AmbientOcclusionLightParsingError(Box::new(
                            cause,
                        )));
                    }
                },

                "distance:" => match tokens.next() {
                    Some(distance_string) => match distance_string.parse() {
                        Ok(d) => distance = Some(d),
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

                "mapping_exponent:" => match tokens.next() {
                    Some(mapping_exponent_string) => match mapping_exponent_string.parse() {
                        Ok(mp) => mapping_exponent = mp,
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
                        expected: "color:, distance:, mapping_exponent, }",
                        found: token.to_string(),
                    });
                }
            }
        }

        let mut rnd = WichmannHillPRNG::new_random();

        Ok(AmbientOcclusionLight::new(
            color,
            SamplingPatternSet::<Point2<T::ValueType>>::multi_jittered_patterns(10, 5, 5, &mut rnd)
                .mapped_to_hemisphere(mapping_exponent),
            distance.unwrap(),
        ))
    }
}
