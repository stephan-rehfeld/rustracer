use std::error::Error;
use std::fmt::Debug;
use std::ops::Mul;
use std::str::FromStr;

use crate::camera::{
    FisheyeCamera, OrthographicCamera, PerspectiveCamera, PinholeCamera, SphericalCamera,
};
use crate::math::{Point3, Vector3};
use crate::traits::floating_point::ToRadians;
use crate::traits::number::MultiplyStable;
use crate::traits::{FloatingPoint, Half, One, SignedNumber, Sqrt, Tan, Zero};
use crate::units::angle::Degrees;
use crate::units::length::Length;

use crate::parser::util;
use crate::parser::{FromTokens, ParsingError};

impl<T: Length + SignedNumber<T::ValueType>> FromTokens for (String, PinholeCamera<T>)
where
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
    type Err = ParsingError;

    fn from_tokens<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, Self::Err> {
        if let Err(cause) = util::check_next_token(tokens, "{") {
            return Err(ParsingError::PinholeCameraParsingError(Box::new(cause)));
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
                        return Err(ParsingError::PinholeCameraParsingError(Box::new(cause)));
                    }
                },
                "gaze_direction:" => match Vector3::from_tokens(tokens) {
                    Ok(vec) => {
                        gaze_direction = vec;
                    }
                    Err(cause) => {
                        return Err(ParsingError::PinholeCameraParsingError(Box::new(cause)));
                    }
                },
                "up_vector:" => match Vector3::from_tokens(tokens) {
                    Ok(vec) => {
                        up_vector = vec;
                    }
                    Err(cause) => {
                        return Err(ParsingError::PinholeCameraParsingError(Box::new(cause)));
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
                        expected:
                            "id:, eye_position:, gaze_direction:, up_vector:, field_of_view:, }",
                        found: token.to_string(),
                    });
                }
            }
        }
        Ok((
            id.to_string(),
            PinholeCamera::new(
                eye_position,
                gaze_direction,
                up_vector,
                field_of_view.to_radians(),
            ),
        ))
    }
}

impl<T: Length + SignedNumber<T::ValueType>> FromTokens for (String, PerspectiveCamera<T>)
where
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
    type Err = ParsingError;

    fn from_tokens<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, Self::Err> {
        if let Err(cause) = util::check_next_token(tokens, "{") {
            return Err(ParsingError::PerspectiveCameraParsingError(Box::new(cause)));
        }

        let mut id = "main";
        let mut eye_position: Point3<T> = Point3::new(Zero::zero(), Zero::zero(), Zero::zero());
        let mut gaze_direction: Vector3<T> = Vector3::new(Zero::zero(), Zero::zero(), -T::one());
        let mut up_vector: Vector3<T> = Vector3::new(Zero::zero(), One::one(), Zero::zero());
        let mut field_of_view: Degrees<<T as Length>::ValueType> = Degrees::new(Zero::zero());
        let mut lens_radius = T::one();
        let mut focal_length = T::one();

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
                                "Unable to parse field of view.",
                            ));
                        }
                    },
                    None => {
                        return Err(ParsingError::UnexpectedEndOfTokens);
                    }
                },
                "lens_radius:" => match tokens.next() {
                    Some(lens_radius_string) => match lens_radius_string.parse() {
                        Ok(lr) => lens_radius = lr,
                        Err(_) => {
                            return Err(ParsingError::NumberParsingError(
                                "Unable to parse lens radius.",
                            ));
                        }
                    },
                    None => {
                        return Err(ParsingError::UnexpectedEndOfTokens);
                    }
                },
                "focal_length:" => match tokens.next() {
                    Some(focal_length_string) => match focal_length_string.parse() {
                        Ok(fl) => focal_length = fl,
                        Err(_) => {
                            return Err(ParsingError::NumberParsingError(
                                "Unable to parse folcal length.",
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
                        expected:
                            "id:, eye_position:, gaze_direction:, up_vector:, field_of_view:, lens_radius, focal_length }",
                        found: token.to_string(),
                    });
                }
            }
        }
        Ok((
            id.to_string(),
            PerspectiveCamera::new(
                eye_position,
                gaze_direction,
                up_vector,
                field_of_view.to_radians(),
                lens_radius,
                focal_length,
            ),
        ))
    }
}

impl<T: Length + SignedNumber<T::ValueType>> FromTokens for (String, OrthographicCamera<T>)
where
    <T as Length>::AreaType: Sqrt<Output = T>,
    <T as Length>::ValueType: FloatingPoint + Sqrt<Output = <T as Length>::ValueType>,
    <T as FromStr>::Err: Error + Debug,
{
    type Err = ParsingError;

    fn from_tokens<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, Self::Err> {
        if let Err(cause) = util::check_next_token(tokens, "{") {
            return Err(ParsingError::OrthographicCameraParsingError(Box::new(
                cause,
            )));
        }

        let mut id = "main";
        let mut eye_position: Point3<T> = Point3::new(Zero::zero(), Zero::zero(), Zero::zero());
        let mut gaze_direction: Vector3<T> = Vector3::new(Zero::zero(), Zero::zero(), -T::one());
        let mut up_vector: Vector3<T> = Vector3::new(Zero::zero(), One::one(), Zero::zero());
        let mut scale: <T as Length>::ValueType = One::one();

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
                        return Err(ParsingError::OrthographicCameraParsingError(Box::new(
                            cause,
                        )));
                    }
                },
                "gaze_direction:" => match Vector3::from_tokens(tokens) {
                    Ok(vec) => {
                        gaze_direction = vec;
                    }
                    Err(cause) => {
                        return Err(ParsingError::OrthographicCameraParsingError(Box::new(
                            cause,
                        )));
                    }
                },
                "up_vector:" => match Vector3::from_tokens(tokens) {
                    Ok(vec) => {
                        up_vector = vec;
                    }
                    Err(cause) => {
                        return Err(ParsingError::OrthographicCameraParsingError(Box::new(
                            cause,
                        )));
                    }
                },
                "scale:" => match tokens.next() {
                    Some(fov_string) => match fov_string.parse() {
                        Ok(s) => scale = s,
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
                        expected:
                            "id:, eye_position:, gaze_direction:, up_vector:, field_of_view:, }",
                        found: token.to_string(),
                    });
                }
            }
        }
        Ok((
            id.to_string(),
            OrthographicCamera::new(eye_position, gaze_direction, up_vector, scale),
        ))
    }
}

impl<T: Length + SignedNumber<T::ValueType>> FromTokens for (String, FisheyeCamera<T>)
where
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
    type Err = ParsingError;

    fn from_tokens<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, Self::Err> {
        if let Err(cause) = util::check_next_token(tokens, "{") {
            return Err(ParsingError::FisheyeCameraParsingError(Box::new(cause)));
        }

        let mut id = "main";
        let mut eye_position: Point3<T> = Point3::new(Zero::zero(), Zero::zero(), Zero::zero());
        let mut gaze_direction: Vector3<T> = Vector3::new(Zero::zero(), Zero::zero(), -T::one());
        let mut up_vector: Vector3<T> = Vector3::new(Zero::zero(), One::one(), Zero::zero());
        let mut psi: Degrees<<T as Length>::ValueType> = Degrees::new(Zero::zero());

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
                        return Err(ParsingError::FisheyeCameraParsingError(Box::new(cause)));
                    }
                },
                "gaze_direction:" => match Vector3::from_tokens(tokens) {
                    Ok(vec) => {
                        gaze_direction = vec;
                    }
                    Err(cause) => {
                        return Err(ParsingError::FisheyeCameraParsingError(Box::new(cause)));
                    }
                },
                "up_vector:" => match Vector3::from_tokens(tokens) {
                    Ok(vec) => {
                        up_vector = vec;
                    }
                    Err(cause) => {
                        return Err(ParsingError::FisheyeCameraParsingError(Box::new(cause)));
                    }
                },
                "psi:" => match tokens.next() {
                    Some(psi_string) => match psi_string.parse() {
                        Ok(p) => psi = p,
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
                        expected: "id:, eye_position:, gaze_direction:, up_vector:, psi:, }",
                        found: token.to_string(),
                    });
                }
            }
        }
        Ok((
            id.to_string(),
            FisheyeCamera::new(eye_position, gaze_direction, up_vector, psi.to_radians()),
        ))
    }
}

impl<T: Length + SignedNumber<T::ValueType>> FromTokens for (String, SphericalCamera<T>)
where
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
    type Err = ParsingError;

    fn from_tokens<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, Self::Err> {
        if let Err(cause) = util::check_next_token(tokens, "{") {
            return Err(ParsingError::SphericalCameraParsingError(Box::new(cause)));
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
                        return Err(ParsingError::SphericalCameraParsingError(Box::new(cause)));
                    }
                },
                "gaze_direction:" => match Vector3::from_tokens(tokens) {
                    Ok(vec) => {
                        gaze_direction = vec;
                    }
                    Err(cause) => {
                        return Err(ParsingError::SphericalCameraParsingError(Box::new(cause)));
                    }
                },
                "up_vector:" => match Vector3::from_tokens(tokens) {
                    Ok(vec) => {
                        up_vector = vec;
                    }
                    Err(cause) => {
                        return Err(ParsingError::SphericalCameraParsingError(Box::new(cause)));
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
                        expected:
                            "id:, eye_position:, gaze_direction:, up_vector:, field_of_view:, }",
                        found: token.to_string(),
                    });
                }
            }
        }
        Ok((
            id.to_string(),
            SphericalCamera::new(
                eye_position,
                gaze_direction,
                up_vector,
                field_of_view.to_radians(),
            ),
        ))
    }
}
