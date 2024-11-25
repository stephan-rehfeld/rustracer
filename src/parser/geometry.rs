use std::error::Error;
use std::fmt::Debug;
use std::str::FromStr;

use crate::color::RGB;
use crate::material::Material;
use crate::math::transform::Transform3;
use crate::math::{Normal3, Point2, Point3, Vector3};
use crate::scene_graph::RenderableGeometry;
use crate::traits::{ConvenientNumber, FloatingPoint, One, SignedNumber, Sqrt, Zero};
use crate::units::angle::Degrees;
use crate::units::length::Length;
use crate::{AxisAlignedBox, Cylinder, Disc, Plane, Sphere, Triangle};

use crate::parser::{
    FromTokens, ParsingError, RenderableAxisAlignedBox, RenderableCylinder, RenderableDisc,
    RenderablePlane, RenderableSphere, RenderableTriangle,
};

use crate::parser::{material, util};

impl<T: Length> FromTokens for RenderableTriangle<T>
where
    <T as Length>::ValueType: FloatingPoint + ConvenientNumber + FromStr + 'static,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
    <T as Length>::AreaType: Sqrt<Output = T>,
    <T as FromStr>::Err: Error,
{
    type Err = ParsingError;

    fn from_tokens<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, Self::Err> {
        if let Err(cause) = util::check_next_token(tokens, "{") {
            return Err(ParsingError::TriangleParsingError(Box::new(cause)));
        }

        let mut material: Option<Box<dyn Material<T, ColorType = RGB<<T as Length>::ValueType>>>> =
            None;
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

                "material:" => match material::parse_material(tokens) {
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
}

impl<T: Length + SignedNumber<T::ValueType>> FromTokens for RenderableAxisAlignedBox<T>
where
    <T as Length>::ValueType: FloatingPoint + ConvenientNumber + FromStr + 'static,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
    <T as Length>::AreaType: Sqrt<Output = T>,
{
    type Err = ParsingError;

    fn from_tokens<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, Self::Err> {
        if let Err(cause) = util::check_next_token(tokens, "{") {
            return Err(ParsingError::BoxParsingError(Box::new(cause)));
        }

        let mut material: Option<Box<dyn Material<T, ColorType = RGB<<T as Length>::ValueType>>>> =
            None;
        let transform = Transform3::ident();

        let mut position: Vector3<T::ValueType> =
            Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());
        let mut scale: Vector3<T::ValueType> = Vector3::new(One::one(), One::one(), One::one());
        let mut rotation: Vector3<Degrees<T::ValueType>> =
            Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());

        while let Some(token) = tokens.next() {
            match token {
                "material:" => match material::parse_material(tokens) {
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
}

impl<T: Length> FromTokens for RenderableDisc<T>
where
    <T as Length>::ValueType: FloatingPoint + ConvenientNumber + FromStr + 'static,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
    <T as Length>::AreaType: Sqrt<Output = T>,
{
    type Err = ParsingError;

    fn from_tokens<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, Self::Err> {
        if let Err(cause) = util::check_next_token(tokens, "{") {
            return Err(ParsingError::PlaneParsingError(Box::new(cause)));
        }

        let mut material: Option<Box<dyn Material<T, ColorType = RGB<<T as Length>::ValueType>>>> =
            None;
        let transform = Transform3::ident();

        let mut position: Vector3<T::ValueType> =
            Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());
        let mut scale: Vector3<T::ValueType> = Vector3::new(One::one(), One::one(), One::one());
        let mut rotation: Vector3<Degrees<T::ValueType>> =
            Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());

        while let Some(token) = tokens.next() {
            match token {
                "material:" => match material::parse_material(tokens) {
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
            disc,
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
}

impl<T: Length> FromTokens for RenderablePlane<T>
where
    <T as Length>::ValueType: FloatingPoint + ConvenientNumber + FromStr + 'static,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
    <T as Length>::AreaType: Sqrt<Output = T>,
{
    type Err = ParsingError;

    fn from_tokens<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, Self::Err> {
        if let Err(cause) = util::check_next_token(tokens, "{") {
            return Err(ParsingError::PlaneParsingError(Box::new(cause)));
        }

        let mut material: Option<Box<dyn Material<T, ColorType = RGB<<T as Length>::ValueType>>>> =
            None;
        let transform = Transform3::ident();

        let mut position: Vector3<T::ValueType> =
            Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());
        let mut scale: Vector3<T::ValueType> = Vector3::new(One::one(), One::one(), One::one());
        let mut rotation: Vector3<Degrees<T::ValueType>> =
            Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());

        while let Some(token) = tokens.next() {
            match token {
                "material:" => match material::parse_material(tokens) {
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
}

impl<T: Length> FromTokens for RenderableSphere<T>
where
    <T as Length>::ValueType: FloatingPoint + ConvenientNumber + FromStr + 'static,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
    <T as Length>::AreaType: Sqrt<Output = T>,
{
    type Err = ParsingError;

    fn from_tokens<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, Self::Err> {
        if let Err(cause) = util::check_next_token(tokens, "{") {
            return Err(ParsingError::SphereParsingError(Box::new(cause)));
        }

        let mut material: Option<Box<dyn Material<T, ColorType = RGB<<T as Length>::ValueType>>>> =
            None;
        let transform = Transform3::ident();

        let mut position: Vector3<T::ValueType> =
            Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());
        let mut scale: Vector3<T::ValueType> = Vector3::new(One::one(), One::one(), One::one());
        let mut rotation: Vector3<Degrees<T::ValueType>> =
            Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());

        while let Some(token) = tokens.next() {
            match token {
                "material:" => match material::parse_material(tokens) {
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
}

impl<T: Length> FromTokens for RenderableCylinder<T>
where
    <T as Length>::ValueType: FloatingPoint + ConvenientNumber + FromStr + 'static,
    <<T as Length>::ValueType as FromStr>::Err: Error + Debug,
    <T as Length>::AreaType: Sqrt<Output = T>,
{
    type Err = ParsingError;

    fn from_tokens<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, Self::Err> {
        if let Err(cause) = util::check_next_token(tokens, "{") {
            return Err(ParsingError::SphereParsingError(Box::new(cause)));
        }

        let mut material: Option<Box<dyn Material<T, ColorType = RGB<<T as Length>::ValueType>>>> =
            None;
        let transform = Transform3::ident();

        let mut position: Vector3<T::ValueType> =
            Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());
        let mut scale: Vector3<T::ValueType> = Vector3::new(One::one(), One::one(), One::one());
        let mut rotation: Vector3<Degrees<T::ValueType>> =
            Vector3::new(Zero::zero(), Zero::zero(), Zero::zero());

        while let Some(token) = tokens.next() {
            match token {
                "material:" => match material::parse_material(tokens) {
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
            cylinder,
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
}
