use std::ops::{Div, Mul, Neg};

use crate::math::geometry::ParametricLine;
use crate::math::{Normal3, NormalizableVector, Point3, Vector3};
use crate::traits::{Cos, MultiplyStable, Sqrt, Zero};
use crate::units::angle::Radians;
use crate::units::length::Length;

pub trait Light<T, C>
where
    T: Div,
{
    fn direction_from(&self, p: Point3<T>) -> Normal3<<T as Div>::Output>;
    fn get_color(&self) -> C;
    fn illuminates(
        &self,
        p: Point3<T>,
        n: Normal3<<T as Div>::Output>,
        shadow_check: &dyn Fn(ParametricLine<Point3<T>, Vector3<T>>) -> Option<<T as Div>::Output>,
    ) -> bool;
}

pub struct DirectionalLight<T, C>
where
    T: Div,
{
    color: C,
    direction: Normal3<<T as Div>::Output>,
}

impl<T, C> DirectionalLight<T, C>
where
    T: Div,
{
    pub fn new(color: C, direction: Normal3<<T as Div>::Output>) -> DirectionalLight<T, C> {
        DirectionalLight { color, direction }
    }
}

impl<T, C> Light<T, C> for DirectionalLight<T, C>
where
    C: Copy,
    T: Length,
    <T as Length>::ValueType:
        MultiplyStable + Mul<T, Output = T> + Neg<Output = <T as Length>::ValueType>,
{
    fn direction_from(&self, _p: Point3<T>) -> Normal3<<T as Div>::Output> {
        -self.direction
    }

    fn get_color(&self) -> C {
        self.color
    }

    fn illuminates(
        &self,
        p: Point3<T>,
        n: Normal3<<T as Div>::Output>,
        shadow_check: &dyn Fn(ParametricLine<Point3<T>, Vector3<T>>) -> Option<<T as Div>::Output>,
    ) -> bool {
        Normal3::dot(self.direction, n) > Zero::zero()
            && shadow_check(ParametricLine::new(p, self.direction_from(p) * T::one())).is_none()
    }
}

pub struct PointLight<T, C> {
    color: C,
    position: Point3<T>,
}

impl<T, C> PointLight<T, C> {
    pub fn new(color: C, position: Point3<T>) -> PointLight<T, C> {
        PointLight { color, position }
    }
}

impl<T, C> Light<T, C> for PointLight<T, C>
where
    C: Copy,
    T: Length,
    <T as Length>::ValueType:
        MultiplyStable + Mul<T, Output = T> + Neg<Output = <T as Length>::ValueType>,
    <T as Length>::AreaType: Sqrt<Output = T>,
{
    fn direction_from(&self, p: Point3<T>) -> Normal3<<T as Div>::Output> {
        (self.position - p).normalized()
    }

    fn get_color(&self) -> C {
        self.color
    }

    fn illuminates(
        &self,
        p: Point3<T>,
        n: Normal3<<T as Div>::Output>,
        shadow_check: &dyn Fn(ParametricLine<Point3<T>, Vector3<T>>) -> Option<<T as Div>::Output>,
    ) -> bool {
        if Normal3::dot(self.direction_from(p), n) > Zero::zero() {
            let ot = shadow_check(ParametricLine::new(p, self.direction_from(p) * T::one()));
            match ot {
                Some(t) => t > ((self.position - p).magnitude() / T::one()),
                None => true,
            }
        } else {
            false
        }
    }
}

pub struct SpotLight<T, C>
where
    T: Div,
{
    color: C,
    position: Point3<T>,
    direction: Normal3<<T as Div>::Output>,
    angle: Radians<<T as Div>::Output>,
}

impl<T, C> SpotLight<T, C>
where
    T: Div,
{
    pub fn new(
        color: C,
        position: Point3<T>,
        direction: Normal3<<T as Div>::Output>,
        angle: Radians<<T as Div>::Output>,
    ) -> SpotLight<T, C> {
        SpotLight {
            color,
            position,
            direction,
            angle,
        }
    }
}

impl<T, C> Light<T, C> for SpotLight<T, C>
where
    C: Copy,
    T: Length,
    <T as Length>::ValueType: Cos<Output = <T as Length>::ValueType>
        + MultiplyStable
        + Mul<T, Output = T>
        + Neg<Output = <T as Length>::ValueType>,
    <T as Length>::AreaType: Sqrt<Output = T>,
{
    fn direction_from(&self, p: Point3<T>) -> Normal3<<T as Div>::Output> {
        (self.position - p).normalized()
    }

    fn get_color(&self) -> C {
        self.color
    }

    fn illuminates(
        &self,
        p: Point3<T>,
        n: Normal3<<T as Div>::Output>,
        shadow_check: &dyn Fn(ParametricLine<Point3<T>, Vector3<T>>) -> Option<<T as Div>::Output>,
    ) -> bool {
        let direction = self.direction_from(p);

        if Normal3::dot(direction, n) > Zero::zero()
            && Normal3::dot(-direction, self.direction) > self.angle.cos()
        {
            let ot = shadow_check(ParametricLine::new(p, self.direction_from(p) * T::one()));
            match ot {
                Some(t) => t > ((self.position - p).magnitude() / T::one()),
                None => true,
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::color::RGB;
    use crate::math::Vector3;

    macro_rules! new_directional_light {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let color = RGB::new(0.0, 0.5, 1.0);
                let direction = Vector3::<$type>::new(1.0, -2.0, 3.0).normalized();

                let light = DirectionalLight::<$type, RGB<$type>>::new(color, direction);

                assert_eq!(color, light.color);
                assert_eq!(direction, light.direction);
            }
        };
    }

    new_directional_light! { f32, new_directional_light_f32 }
    new_directional_light! { f64, new_directional_light_f64 }

    macro_rules! directional_light_direction_from {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let color = RGB::new(0.0, 0.5, 1.0);
                let direction = Vector3::<$type>::new(1.0, -2.0, 3.0).normalized();

                let light = DirectionalLight::<$type, RGB<$type>>::new(color, direction);

                let p1 = Point3::new(0.0, 0.0, 0.0);
                let p2 = Point3::new(1.0, -1.0, 2.0);
                let p3 = Point3::new(21341.0, 11234.0, 20989.0);

                assert_eq!(-direction, light.direction_from(p1));
                assert_eq!(-direction, light.direction_from(p2));
                assert_eq!(-direction, light.direction_from(p3));
            }
        };
    }

    directional_light_direction_from! { f32, directional_light_direction_from_f32 }
    directional_light_direction_from! { f64, directional_light_direction_from_f64 }

    macro_rules! directional_light_get_color {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let color = RGB::new(0.0, 0.5, 1.0);
                let direction = Vector3::<$type>::new(1.0, -2.0, 3.0).normalized();

                let light = DirectionalLight::<$type, RGB<$type>>::new(color, direction);

                assert_eq!(color, light.get_color());
            }
        };
    }

    directional_light_get_color! { f32, directional_light_get_color_f32 }
    directional_light_get_color! { f64, directional_light_get_color_f64 }

    macro_rules! new_point_light {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let color = RGB::new(0.0, 0.5, 1.0);
                let position = Point3::new(1.0, -2.0, 3.0);

                let light = PointLight::<$type, RGB<$type>>::new(color, position);

                assert_eq!(color, light.color);
                assert_eq!(position, light.position);
            }
        };
    }

    new_point_light! { f32, new_point_light_f32 }
    new_point_light! { f64, new_point_light_f64 }

    macro_rules! point_light_direction_from {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let color = RGB::new(0.0, 0.5, 1.0);
                let position = Point3::new(0.0, 1.0, 0.0);

                let light = PointLight::<$type, RGB<$type>>::new(color, position);

                assert_eq!(
                    Normal3::new(0.0, 1.0, 0.0),
                    light.direction_from(Point3::new(0.0, -1.0, 0.0))
                );
                assert_eq!(
                    Normal3::new(0.0, -1.0, 0.0),
                    light.direction_from(Point3::new(0.0, 10.0, 0.0))
                );

                assert_eq!(
                    Normal3::new(-1.0, 0.0, 0.0),
                    light.direction_from(Point3::new(123.0, 1.0, 0.0))
                );
                assert_eq!(
                    Normal3::new(1.0, 0.0, 0.0),
                    light.direction_from(Point3::new(-5234.0, 1.0, 0.0))
                );

                assert_eq!(
                    Normal3::new(0.0, 0.0, -1.0),
                    light.direction_from(Point3::new(0.0, 1.0, 53737.0))
                );
                assert_eq!(
                    Normal3::new(0.0, 0.0, 1.0),
                    light.direction_from(Point3::new(0.0, 1.0, -236.0))
                );
            }
        };
    }

    point_light_direction_from! { f32, point_light_direction_from_f32 }
    point_light_direction_from! { f64, point_light_direction_from_f64 }

    macro_rules! point_light_get_color {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let color = RGB::new(0.0, 0.5, 1.0);
                let position = Point3::new(0.0, 1.0, 0.0);

                let light = PointLight::<$type, RGB<$type>>::new(color, position);

                assert_eq!(color, light.get_color());
            }
        };
    }

    point_light_get_color! { f32, point_light_get_color_f32 }
    point_light_get_color! { f64, point_light_get_color_f64 }

    macro_rules! new_spot_light {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let color = RGB::new(0.0, 0.5, 1.0);
                let position = Point3::new(1.0, -2.0, 3.0);
                let direction = Vector3::<$type>::new(1.0, -2.0, 3.0).normalized();
                let angle = Radians::new(1.23);

                let light = SpotLight::<$type, RGB<$type>>::new(color, position, direction, angle);

                assert_eq!(color, light.color);
                assert_eq!(position, light.position);
                assert_eq!(direction, light.direction);
                assert_eq!(angle, light.angle);
            }
        };
    }

    new_spot_light! { f32, new_spot_light_f32 }
    new_spot_light! { f64, new_spot_light_f64 }

    macro_rules! spot_light_direction_from {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let color = RGB::new(0.0, 0.5, 1.0);
                let position = Point3::new(0.0, 1.0, 0.0);
                let direction = Vector3::<$type>::new(1.0, -2.0, 3.0).normalized();
                let angle = Radians::new(1.23);

                let light = SpotLight::<$type, RGB<$type>>::new(color, position, direction, angle);

                assert_eq!(
                    Normal3::new(0.0, 1.0, 0.0),
                    light.direction_from(Point3::new(0.0, -1.0, 0.0))
                );
                assert_eq!(
                    Normal3::new(0.0, -1.0, 0.0),
                    light.direction_from(Point3::new(0.0, 10.0, 0.0))
                );

                assert_eq!(
                    Normal3::new(-1.0, 0.0, 0.0),
                    light.direction_from(Point3::new(123.0, 1.0, 0.0))
                );
                assert_eq!(
                    Normal3::new(1.0, 0.0, 0.0),
                    light.direction_from(Point3::new(-5234.0, 1.0, 0.0))
                );

                assert_eq!(
                    Normal3::new(0.0, 0.0, -1.0),
                    light.direction_from(Point3::new(0.0, 1.0, 53737.0))
                );
                assert_eq!(
                    Normal3::new(0.0, 0.0, 1.0),
                    light.direction_from(Point3::new(0.0, 1.0, -236.0))
                );
            }
        };
    }

    spot_light_direction_from! { f32, spot_light_direction_from_f32 }
    spot_light_direction_from! { f64, spot_light_direction_from_f64 }

    macro_rules! spot_light_get_color {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let color = RGB::new(0.0, 0.5, 1.0);
                let position = Point3::new(0.0, 1.0, 0.0);
                let direction = Vector3::<$type>::new(1.0, -2.0, 3.0).normalized();
                let angle = Radians::new(1.23);

                let light = SpotLight::<$type, RGB<$type>>::new(color, position, direction, angle);

                assert_eq!(color, light.get_color());
            }
        };
    }

    spot_light_get_color! { f32, spot_light_get_color_f32 }
    spot_light_get_color! { f64, spot_light_get_color_f64 }
}
