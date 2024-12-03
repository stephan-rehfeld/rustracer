use std::ops::Div;

use math::{Point3, Vector3};
use units::angle::Radians;
use units::length::Length;

pub struct DirectionalLight<T, C>
where
    T: Div,
{
    pub color: C,
    pub direction: Vector3<<T as Div>::Output>,
}

impl<T, C> DirectionalLight<T, C>
where
    T: Div,
{
    pub fn new(color: C, direction: Vector3<<T as Div>::Output>) -> DirectionalLight<T, C> {
        DirectionalLight { color, direction }
    }
}

pub struct PointLight<T, C> {
    pub color: C,
    pub position: Point3<T>,
}

impl<T, C> PointLight<T, C> {
    pub fn new(color: C, position: Point3<T>) -> PointLight<T, C> {
        PointLight { color, position }
    }
}

pub struct SpotLight<T, C>
where
    T: Div,
{
    pub color: C,
    pub position: Point3<T>,
    pub direction: Vector3<<T as Div>::Output>,
    pub angle: Radians<<T as Div>::Output>,
}

impl<T, C> SpotLight<T, C>
where
    T: Div,
{
    pub fn new(
        color: C,
        position: Point3<T>,
        direction: Vector3<<T as Div>::Output>,
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

pub struct AmbientLight<C> {
    pub color: C,
}

impl<C> AmbientLight<C> {
    pub fn new(color: C) -> AmbientLight<C> {
        AmbientLight { color }
    }
}

pub struct AmbientOcclusionLight<T: Length, C> {
    pub color: C,
    pub e: T::ValueType,
    pub distance: T,
}

impl<T: Length, C> AmbientOcclusionLight<T, C> {
    pub fn new(color: C, e: T::ValueType, distance: T) -> AmbientOcclusionLight<T, C> {
        AmbientOcclusionLight { color, e, distance }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use colors::RGB;
    use math::Vector3;
    use units::length::Meter;

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

    macro_rules! new_spot_light {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let color = RGB::new(0.0, 0.5, 1.0);
                let position = Point3::new(
                    Meter::<$type>::new(1.0),
                    Meter::<$type>::new(-2.0),
                    Meter::<$type>::new(3.0),
                );
                let direction = Vector3::<Meter<$type>>::new(
                    Meter::<$type>::new(1.0),
                    -Meter::<$type>::new(2.0),
                    Meter::<$type>::new(3.0),
                )
                .normalized();
                let angle = Radians::new(1.23);

                let light =
                    SpotLight::<Meter<$type>, RGB<$type>>::new(color, position, direction, angle);

                assert_eq!(color, light.color);
                assert_eq!(position, light.position);
                assert_eq!(direction, light.direction);
                assert_eq!(angle, light.angle);
            }
        };
    }

    new_spot_light! { f32, new_spot_light_f32 }
    new_spot_light! { f64, new_spot_light_f64 }
}
