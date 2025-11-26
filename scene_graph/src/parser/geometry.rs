use std::error::Error;
use std::fmt::Debug;
use std::ops::Neg;
use std::str::FromStr;

use cg_basics::{AxisAlignedBox, Cylinder, Disc, Plane, Sphere, Triangle};
use colors::RGB;
use image::Image;
use math::transform::Transform3;
use math::{Normal3, Point2, Point3, Vector3};
use random::{RandomNumberGenerator, WichmannHillPRNG};
use sampling::{PatternMapping, SamplingPattern};
use crate::{Geometry, Material, RenderableGeometry};
use traits::{ConvenientNumber, FloatingPoint, One, Sqrt, Zero};
use units::angle::Degrees;
use units::length::Length;

use crate::parser::{FromTokens, ParsingError};

use crate::parser::{material, util};

pub fn parse_triangle<'a, T: Length>(
    tokens: &mut impl Iterator<Item = &'a str>,
) -> Result<
    RenderableGeometry<
        Geometry<T>,
        Material<Box<dyn Image<ColorType = RGB<T::ValueType>, PointType = Point2<T::ValueType>>>>,
        Transform3<T::ValueType>,
    >,
    ParsingError,
>
where
    <T as Length>::ValueType: FloatingPoint + ConvenientNumber + FromStr + 'static,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
    <T as Length>::AreaType: Sqrt<Output = T>,
    <T as FromStr>::Err: Error,
    WichmannHillPRNG: RandomNumberGenerator<T::ValueType>,
    SamplingPattern<Point2<T::ValueType>>: PatternMapping<T::ValueType>,
{
    if let Err(cause) = util::check_next_token(tokens, "{") {
        return Err(ParsingError::TriangleParsingError(Box::new(cause)));
    }

    let mut material: Option<
        Material<Box<dyn Image<ColorType = RGB<T::ValueType>, PointType = Point2<T::ValueType>>>>,
    > = None;
    let transform = Transform3::ident();

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

    let mut uva: Option<Point2<<T as Length>::ValueType>> = None;
    let mut uvb: Option<Point2<<T as Length>::ValueType>> = None;
    let mut uvc: Option<Point2<<T as Length>::ValueType>> = None;

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
            "uva:" => match Point2::from_tokens(tokens) {
                Ok(point) => {
                    uva = Some(point);
                }
                Err(cause) => {
                    return Err(ParsingError::TriangleParsingError(Box::new(cause)));
                }
            },
            "uvb:" => match Point2::from_tokens(tokens) {
                Ok(point) => {
                    uvb = Some(point);
                }
                Err(cause) => {
                    return Err(ParsingError::TriangleParsingError(Box::new(cause)));
                }
            },
            "uvc:" => match Point2::from_tokens(tokens) {
                Ok(point) => {
                    uvc = Some(point);
                }
                Err(cause) => {
                    return Err(ParsingError::TriangleParsingError(Box::new(cause)));
                }
            },

            "material:" => match material::parse_material::<T>(tokens) {
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
    if let None = uva {
        return Err(ParsingError::MissingElement("uva"));
    }
    if let None = uvb {
        return Err(ParsingError::MissingElement("uvb"));
    }
    if let None = uvc {
        return Err(ParsingError::MissingElement("uvc"));
    }

    let triangle = Triangle::new(
        a.unwrap(),
        b.unwrap(),
        c.unwrap(),
        na.unwrap(),
        nb.unwrap(),
        nc.unwrap(),
        uva.unwrap(),
        uvb.unwrap(),
        uvc.unwrap(),
    );

    let triangle_geometry = RenderableGeometry::new(
        Geometry::Triangle(triangle),
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

pub fn parse_axis_aligned_box<'a, T: Length + Neg<Output = T>>(
    tokens: &mut impl Iterator<Item = &'a str>,
) -> Result<
    RenderableGeometry<
        Geometry<T>,
        Material<Box<dyn Image<ColorType = RGB<T::ValueType>, PointType = Point2<T::ValueType>>>>,
        Transform3<T::ValueType>,
    >,
    ParsingError,
>
where
    <T as Length>::ValueType: FloatingPoint + ConvenientNumber + FromStr + 'static,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
    <T as Length>::AreaType: Sqrt<Output = T>,
    WichmannHillPRNG: RandomNumberGenerator<T::ValueType>,
    SamplingPattern<Point2<T::ValueType>>: PatternMapping<T::ValueType>,
{
    if let Err(cause) = util::check_next_token(tokens, "{") {
        return Err(ParsingError::BoxParsingError(Box::new(cause)));
    }

    let mut material: Option<
        Material<Box<dyn Image<ColorType = RGB<T::ValueType>, PointType = Point2<T::ValueType>>>>,
    > = None;
    let transform = Transform3::ident();

    let mut position: Vector3<T::ValueType> =
        Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());
    let mut scale: Vector3<T::ValueType> = Vector3::new(One::one(), One::one(), One::one());
    let mut rotation: Vector3<Degrees<T::ValueType>> =
        Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());

    while let Some(token) = tokens.next() {
        match token {
            "material:" => match material::parse_material::<T>(tokens) {
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
        Geometry::AxisAlignedBox(aab),
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

pub fn parse_disc<'a, T: Length>(
    tokens: &mut impl Iterator<Item = &'a str>,
) -> Result<
    RenderableGeometry<
        Geometry<T>,
        Material<Box<dyn Image<ColorType = RGB<T::ValueType>, PointType = Point2<T::ValueType>>>>,
        Transform3<T::ValueType>,
    >,
    ParsingError,
>
where
    <T as Length>::ValueType: FloatingPoint + ConvenientNumber + FromStr + 'static,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
    <T as Length>::AreaType: Sqrt<Output = T>,
    WichmannHillPRNG: RandomNumberGenerator<T::ValueType>,
    SamplingPattern<Point2<T::ValueType>>: PatternMapping<T::ValueType>,
{
    if let Err(cause) = util::check_next_token(tokens, "{") {
        return Err(ParsingError::PlaneParsingError(Box::new(cause)));
    }

    let mut material: Option<
        Material<Box<dyn Image<ColorType = RGB<T::ValueType>, PointType = Point2<T::ValueType>>>>,
    > = None;
    let transform = Transform3::ident();

    let mut position: Vector3<T::ValueType> =
        Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());
    let mut scale: Vector3<T::ValueType> = Vector3::new(One::one(), One::one(), One::one());
    let mut rotation: Vector3<Degrees<T::ValueType>> =
        Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());

    while let Some(token) = tokens.next() {
        match token {
            "material:" => match material::parse_material::<T>(tokens) {
                Ok(mat) => {
                    material = Some(mat);
                }
                Err(cause) => {
                    return Err(ParsingError::DiscParsingError(Box::new(cause)));
                }
            },
            "position:" => match Vector3::from_tokens(tokens) {
                Ok(vec) => {
                    position = vec;
                }
                Err(cause) => {
                    return Err(ParsingError::DiscParsingError(Box::new(cause)));
                }
            },
            "scale:" => match Vector3::from_tokens(tokens) {
                Ok(vec) => {
                    scale = vec;
                }
                Err(cause) => {
                    return Err(ParsingError::DiscParsingError(Box::new(cause)));
                }
            },
            "rotation:" => match Vector3::from_tokens(tokens) {
                Ok(vec) => {
                    rotation = vec;
                }
                Err(cause) => {
                    return Err(ParsingError::DiscParsingError(Box::new(cause)));
                }
            },
            "}" => {
                break;
            }
            token => {
                return Err(ParsingError::UnexpectedToken {
                    expected: "radius:, material:, position:, scale:, rotation:, }",
                    found: token.to_string(),
                });
            }
        }
    }

    if let None = material {
        return Err(ParsingError::MissingElement("material"));
    }

    let disc = Disc::new(
        Point3::new(Zero::zero(), Zero::zero(), Zero::zero()),
        Normal3::new(Zero::zero(), One::one(), Zero::zero()),
        Vector3::new(One::one(), Zero::zero(), Zero::zero()),
        One::one(),
    );

    let disc_geometry = RenderableGeometry::new(
        Geometry::Disc(disc),
        material.unwrap(),
        transform
            .translate(position.x, position.y, position.z)
            .rotate_z(rotation.z)
            .rotate_x(rotation.x)
            .rotate_y(rotation.y)
            .scale(scale.x, scale.y, scale.z),
    );

    Ok(disc_geometry)
}

pub fn parse_plane<'a, T: Length>(
    tokens: &mut impl Iterator<Item = &'a str>,
) -> Result<
    RenderableGeometry<
        Geometry<T>,
        Material<Box<dyn Image<ColorType = RGB<T::ValueType>, PointType = Point2<T::ValueType>>>>,
        Transform3<T::ValueType>,
    >,
    ParsingError,
>
where
    <T as Length>::ValueType: FloatingPoint + ConvenientNumber + FromStr + 'static,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
    <T as Length>::AreaType: Sqrt<Output = T>,
    WichmannHillPRNG: RandomNumberGenerator<T::ValueType>,
    SamplingPattern<Point2<T::ValueType>>: PatternMapping<T::ValueType>,
{
    if let Err(cause) = util::check_next_token(tokens, "{") {
        return Err(ParsingError::PlaneParsingError(Box::new(cause)));
    }

    let mut material: Option<
        Material<Box<dyn Image<ColorType = RGB<T::ValueType>, PointType = Point2<T::ValueType>>>>,
    > = None;
    let transform = Transform3::ident();

    let mut position: Vector3<T::ValueType> =
        Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());
    let mut scale: Vector3<T::ValueType> = Vector3::new(One::one(), One::one(), One::one());
    let mut rotation: Vector3<Degrees<T::ValueType>> =
        Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());

    while let Some(token) = tokens.next() {
        match token {
            "material:" => match material::parse_material::<T>(tokens) {
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

    let plane = Plane::new(
        Point3::new(Zero::zero(), Zero::zero(), Zero::zero()),
        Normal3::new(Zero::zero(), One::one(), Zero::zero()),
        Vector3::new(One::one(), Zero::zero(), Zero::zero()),
    );

    let plane_geometry = RenderableGeometry::new(
        Geometry::Plane(plane),
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

pub fn parse_sphere<'a, T: Length>(
    tokens: &mut impl Iterator<Item = &'a str>,
) -> Result<
    RenderableGeometry<
        Geometry<T>,
        Material<Box<dyn Image<ColorType = RGB<T::ValueType>, PointType = Point2<T::ValueType>>>>,
        Transform3<T::ValueType>,
    >,
    ParsingError,
>
where
    <T as Length>::ValueType: FloatingPoint + ConvenientNumber + FromStr + 'static,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
    <T as Length>::AreaType: Sqrt<Output = T>,
    WichmannHillPRNG: RandomNumberGenerator<T::ValueType>,
    SamplingPattern<Point2<T::ValueType>>: PatternMapping<T::ValueType>,
{
    if let Err(cause) = util::check_next_token(tokens, "{") {
        return Err(ParsingError::SphereParsingError(Box::new(cause)));
    }

    let mut material: Option<
        Material<Box<dyn Image<ColorType = RGB<T::ValueType>, PointType = Point2<T::ValueType>>>>,
    > = None;
    let transform = Transform3::ident();

    let mut position: Vector3<T::ValueType> =
        Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());
    let mut scale: Vector3<T::ValueType> = Vector3::new(One::one(), One::one(), One::one());
    let mut rotation: Vector3<Degrees<T::ValueType>> =
        Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());

    while let Some(token) = tokens.next() {
        match token {
            "material:" => match material::parse_material::<T>(tokens) {
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

    let sphere = Sphere::new(
        Point3::new(Zero::zero(), Zero::zero(), Zero::zero()),
        One::one(),
    );
    let sphere_geometry = RenderableGeometry::new(
        Geometry::Sphere(sphere),
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

pub fn parse_cylinder<'a, T: Length>(
    tokens: &mut impl Iterator<Item = &'a str>,
) -> Result<
    RenderableGeometry<
        Geometry<T>,
        Material<Box<dyn Image<ColorType = RGB<T::ValueType>, PointType = Point2<T::ValueType>>>>,
        Transform3<T::ValueType>,
    >,
    ParsingError,
>
where
    <T as Length>::ValueType: FloatingPoint + ConvenientNumber + FromStr + 'static,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
    <T as Length>::AreaType: Sqrt<Output = T>,
    WichmannHillPRNG: RandomNumberGenerator<T::ValueType>,
    SamplingPattern<Point2<T::ValueType>>: PatternMapping<T::ValueType>,
{
    if let Err(cause) = util::check_next_token(tokens, "{") {
        return Err(ParsingError::SphereParsingError(Box::new(cause)));
    }

    let mut material: Option<
        Material<Box<dyn Image<ColorType = RGB<T::ValueType>, PointType = Point2<T::ValueType>>>>,
    > = None;
    let transform = Transform3::ident();

    let mut position: Vector3<T::ValueType> =
        Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());
    let mut scale: Vector3<T::ValueType> = Vector3::new(One::one(), One::one(), One::one());
    let mut rotation: Vector3<Degrees<T::ValueType>> =
        Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());

    while let Some(token) = tokens.next() {
        match token {
            "material:" => match material::parse_material::<T>(tokens) {
                Ok(mat) => {
                    material = Some(mat);
                }
                Err(cause) => {
                    return Err(ParsingError::CylinderParsingError(Box::new(cause)));
                }
            },
            "position:" => match Vector3::from_tokens(tokens) {
                Ok(vec) => {
                    position = vec;
                }
                Err(cause) => {
                    return Err(ParsingError::CylinderParsingError(Box::new(cause)));
                }
            },
            "scale:" => match Vector3::from_tokens(tokens) {
                Ok(vec) => {
                    scale = vec;
                }
                Err(cause) => {
                    return Err(ParsingError::CylinderParsingError(Box::new(cause)));
                }
            },
            "rotation:" => match Vector3::from_tokens(tokens) {
                Ok(vec) => {
                    rotation = vec;
                }
                Err(cause) => {
                    return Err(ParsingError::CylinderParsingError(Box::new(cause)));
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

    let cylinder = Cylinder::new(
        Point3::new(Zero::zero(), Zero::zero(), Zero::zero()),
        One::one(),
        One::one(),
    );
    let cylinder_geometry = RenderableGeometry::new(
        Geometry::Cylinder(cylinder),
        material.unwrap(),
        transform
            .translate(position.x, position.y, position.z)
            .rotate_z(rotation.z)
            .rotate_x(rotation.x)
            .rotate_y(rotation.y)
            .scale(scale.x, scale.y, scale.z),
    );

    Ok(cylinder_geometry)
}
