use std::fmt::Debug;
use std::ops::{Div, Mul};

use math::geometry::{ParametricLine, SurfacePoint};
use math::{Point3, Vector3};
use random::{RandomNumberGenerator, WichmannHillPRNG};
use sampling::SamplingPatternSet;
use traits::{Cos, FloatingPoint, SignedNumber, Sqrt, Zero};
use units::angle::Radians;
use units::length::Length;

pub trait Light<T, C>
where
    T: Div + Copy + Debug,
    <T as Div>::Output: Copy + Debug + PartialEq,
{
    fn direction_from(&self, sp: SurfacePoint<T>) -> Vector3<<T as Div>::Output>;
    fn get_color(&self) -> C;
    // Change parameter to SurfacePoint
    fn illuminates(
        &self,
        sp: SurfacePoint<T>,
        shadow_check: &dyn Fn(
            ParametricLine<Point3<T>, Vector3<T>>,
            Option<T>,
        ) -> Option<<T as Div>::Output>,
    ) -> bool;
}

pub struct DirectionalLight<T, C>
where
    T: Div,
{
    color: C,
    direction: Vector3<<T as Div>::Output>,
}

impl<T, C> DirectionalLight<T, C>
where
    T: Div,
{
    pub fn new(color: C, direction: Vector3<<T as Div>::Output>) -> DirectionalLight<T, C> {
        DirectionalLight { color, direction }
    }
}

impl<T, C> Light<T, C> for DirectionalLight<T, C>
where
    C: Copy,
    T: Length,
    <T as Length>::ValueType: SignedNumber + Mul<T, Output = T>,
{
    fn direction_from(&self, _sp: SurfacePoint<T>) -> Vector3<<T as Div>::Output> {
        -self.direction
    }

    fn get_color(&self) -> C {
        self.color
    }

    fn illuminates(
        &self,
        sp: SurfacePoint<T>,
        shadow_check: &dyn Fn(
            ParametricLine<Point3<T>, Vector3<T>>,
            Option<T>,
        ) -> Option<<T as Div>::Output>,
    ) -> bool {
        self.direction.dot(sp.n.as_vector()) > Zero::zero()
            && shadow_check(
                ParametricLine::new(sp.p, self.direction_from(sp) * T::one()),
                None,
            )
            .is_none()
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
    <T as Length>::ValueType: SignedNumber + Mul<T, Output = T>,
    <T as Length>::AreaType: Sqrt<Output = T>,
{
    fn direction_from(&self, sp: SurfacePoint<T>) -> Vector3<<T as Div>::Output> {
        (self.position - sp.p).normalized()
    }

    fn get_color(&self) -> C {
        self.color
    }

    fn illuminates(
        &self,
        sp: SurfacePoint<T>,
        shadow_check: &dyn Fn(
            ParametricLine<Point3<T>, Vector3<T>>,
            Option<T>,
        ) -> Option<<T as Div>::Output>,
    ) -> bool {
        if self.direction_from(sp).dot(sp.n.as_vector()) > Zero::zero() {
            let ot = shadow_check(
                ParametricLine::new(sp.p, self.direction_from(sp) * T::one()),
                None,
            );
            match ot {
                Some(t) => t > ((self.position - sp.p).magnitude() / T::one()),
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
    direction: Vector3<<T as Div>::Output>,
    angle: Radians<<T as Div>::Output>,
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

impl<T, C> Light<T, C> for SpotLight<T, C>
where
    C: Copy,
    T: Length,
    <T as Length>::ValueType: FloatingPoint + Mul<T, Output = T>,
    <T as Length>::AreaType: Sqrt<Output = T>,
{
    fn direction_from(&self, sp: SurfacePoint<T>) -> Vector3<<T as Div>::Output> {
        (self.position - sp.p).normalized()
    }

    fn get_color(&self) -> C {
        self.color
    }

    fn illuminates(
        &self,
        sp: SurfacePoint<T>,
        shadow_check: &dyn Fn(
            ParametricLine<Point3<T>, Vector3<T>>,
            Option<T>,
        ) -> Option<<T as Div>::Output>,
    ) -> bool {
        let direction = self.direction_from(sp);

        if direction.dot(sp.n.as_vector()) > Zero::zero()
            && (-direction).dot(self.direction) > self.angle.cos()
        {
            let ot = shadow_check(ParametricLine::new(sp.p, direction * T::one()), None);
            match ot {
                Some(t) => t > ((self.position - sp.p).magnitude() / T::one()),
                None => true,
            }
        } else {
            false
        }
    }
}

pub struct AmbientLight<C> {
    color: C,
}

impl<C> AmbientLight<C> {
    pub fn new(color: C) -> AmbientLight<C> {
        AmbientLight { color }
    }
}

impl<T, C> Light<T, C> for AmbientLight<C>
where
    C: Copy,
    T: Length,
    <T as Length>::ValueType: FloatingPoint + Mul<T, Output = T>,
    <T as Length>::AreaType: Sqrt<Output = T>,
{
    fn get_color(&self) -> C {
        self.color
    }

    fn illuminates(
        &self,
        _sp: SurfacePoint<T>,
        _shadow_check: &dyn Fn(
            ParametricLine<Point3<T>, Vector3<T>>,
            Option<T>,
        ) -> Option<<T as Div>::Output>,
    ) -> bool {
        true
    }

    fn direction_from(&self, sp: SurfacePoint<T>) -> Vector3<<T as Div>::Output> {
        sp.n.as_vector().normalized()
    }
}

pub struct AmbientOcclusionLight<T: Length, C> {
    color: C,
    patterns: SamplingPatternSet<Point3<T::ValueType>>,
    distance: T,
}

impl<T: Length, C> AmbientOcclusionLight<T, C> {
    pub fn new(
        color: C,
        patterns: SamplingPatternSet<Point3<T::ValueType>>,
        distance: T,
    ) -> AmbientOcclusionLight<T, C> {
        AmbientOcclusionLight {
            color,
            patterns,
            distance,
        }
    }
}

impl<T: Length, C> Light<T, C> for AmbientOcclusionLight<T, C>
where
    C: Copy,
    T: Length,
    <T as Length>::ValueType: FloatingPoint + Mul<T, Output = T>,
    <T as Length>::AreaType: Sqrt<Output = T>,
    WichmannHillPRNG: RandomNumberGenerator<T::ValueType>,
{
    fn get_color(&self) -> C {
        self.color
    }

    fn illuminates(
        &self,
        sp: SurfacePoint<T>,
        shadow_check: &dyn Fn(
            ParametricLine<Point3<T>, Vector3<T>>,
            Option<T>,
        ) -> Option<<T as Div>::Output>,
    ) -> bool {
        let mut rnd = WichmannHillPRNG::new_random();

        let w = sp.n.as_vector();
        let rnd_vector: Vector3<T::ValueType> =
            Vector3::new(rnd.next_random(), rnd.next_random(), rnd.next_random()).normalized();
        let v = Vector3::cross(w, rnd_vector).normalized();
        let u = Vector3::cross(v, w);

        let pattern = self.patterns.draw_pattern(&mut rnd);

        let sample = pattern.draw_point(&mut rnd);
        let direction = (u * sample.x + v * sample.y + w * sample.z).normalized() * T::one();

        let shadow_ray = ParametricLine::new(sp.p, direction);

        shadow_check(shadow_ray, Some(self.distance)).is_none()
    }

    fn direction_from(&self, sp: SurfacePoint<T>) -> Vector3<<T as Div>::Output> {
        sp.n.as_vector().normalized()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use colors::RGB;
    use math::{Normal3, Point2, Vector3};
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

    macro_rules! directional_light_direction_from {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let color = RGB::new(0.0, 0.5, 1.0);
                let direction = Vector3::<$type>::new(1.0, -2.0, 3.0).normalized();

                let light = DirectionalLight::<Meter<$type>, RGB<$type>>::new(color, direction);

                let p1 = Point3::new(
                    Meter::<$type>::new(0.0),
                    Meter::<$type>::new(0.0),
                    Meter::<$type>::new(0.0),
                );
                let p2 = Point3::new(
                    Meter::<$type>::new(1.0),
                    Meter::<$type>::new(-1.0),
                    Meter::<$type>::new(2.0),
                );
                let p3 = Point3::new(
                    Meter::<$type>::new(21341.0),
                    Meter::<$type>::new(11234.0),
                    Meter::<$type>::new(20989.0),
                );

                let sp1 = SurfacePoint::new(
                    p1,
                    Normal3::new(0 as $type, 1 as $type, 0 as $type),
                    Point2::new(0 as $type, 0 as $type),
                );

                let sp2 = SurfacePoint::new(
                    p2,
                    Normal3::new(0 as $type, 1 as $type, 0 as $type),
                    Point2::new(0 as $type, 0 as $type),
                );

                let sp3 = SurfacePoint::new(
                    p3,
                    Normal3::new(0 as $type, 1 as $type, 0 as $type),
                    Point2::new(0 as $type, 0 as $type),
                );

                assert_eq!(-direction, light.direction_from(sp1));
                assert_eq!(-direction, light.direction_from(sp2));
                assert_eq!(-direction, light.direction_from(sp3));
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

                let light = DirectionalLight::<Meter<$type>, RGB<$type>>::new(color, direction);

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
                let position = Point3::new(
                    Meter::<$type>::new(0.0),
                    Meter::<$type>::new(1.0),
                    Meter::<$type>::new(0.0),
                );

                let light = PointLight::<Meter<$type>, RGB<$type>>::new(color, position);

                assert_eq!(
                    Vector3::new(0.0, 1.0, 0.0),
                    light.direction_from(SurfacePoint::new(
                        Point3::new(
                            Meter::<$type>::new(0.0),
                            Meter::<$type>::new(-1.0),
                            Meter::<$type>::new(0.0)
                        ),
                        Normal3::new(0 as $type, 1 as $type, 0 as $type),
                        Point2::new(0 as $type, 0 as $type),
                    ))
                );
                assert_eq!(
                    Vector3::new(0.0, -1.0, 0.0),
                    light.direction_from(SurfacePoint::new(
                        Point3::new(
                            Meter::<$type>::new(0.0),
                            Meter::<$type>::new(10.0),
                            Meter::<$type>::new(0.0)
                        ),
                        Normal3::new(0 as $type, 1 as $type, 0 as $type),
                        Point2::new(0 as $type, 0 as $type),
                    ))
                );

                assert_eq!(
                    Vector3::new(-1.0, 0.0, 0.0),
                    light.direction_from(SurfacePoint::new(
                        Point3::new(
                            Meter::<$type>::new(123.0),
                            Meter::<$type>::new(1.0),
                            Meter::<$type>::new(0.0)
                        ),
                        Normal3::new(0 as $type, 1 as $type, 0 as $type),
                        Point2::new(0 as $type, 0 as $type),
                    ))
                );
                assert_eq!(
                    Vector3::new(1.0, 0.0, 0.0),
                    light.direction_from(SurfacePoint::new(
                        Point3::new(
                            Meter::<$type>::new(-5234.0),
                            Meter::<$type>::new(1.0),
                            Meter::<$type>::new(0.0)
                        ),
                        Normal3::new(0 as $type, 1 as $type, 0 as $type),
                        Point2::new(0 as $type, 0 as $type),
                    ))
                );

                assert_eq!(
                    Vector3::new(0.0, 0.0, -1.0),
                    light.direction_from(SurfacePoint::new(
                        Point3::new(
                            Meter::<$type>::new(0.0),
                            Meter::<$type>::new(1.0),
                            Meter::<$type>::new(53737.0)
                        ),
                        Normal3::new(0 as $type, 1 as $type, 0 as $type),
                        Point2::new(0 as $type, 0 as $type),
                    ))
                );
                assert_eq!(
                    Vector3::new(0.0, 0.0, 1.0),
                    light.direction_from(SurfacePoint::new(
                        Point3::new(
                            Meter::<$type>::new(0.0),
                            Meter::<$type>::new(1.0),
                            Meter::<$type>::new(-236.0)
                        ),
                        Normal3::new(0 as $type, 1 as $type, 0 as $type),
                        Point2::new(0 as $type, 0 as $type),
                    ))
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
                let position = Point3::new(
                    Meter::<$type>::new(0.0),
                    Meter::<$type>::new(1.0),
                    Meter::<$type>::new(0.0),
                );

                let light = PointLight::<Meter<$type>, RGB<$type>>::new(color, position);

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

    macro_rules! spot_light_direction_from {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let color = RGB::new(0.0, 0.5, 1.0);
                let position = Point3::new(
                    Meter::<$type>::new(0.0),
                    Meter::<$type>::new(1.0),
                    Meter::<$type>::new(0.0),
                );
                let direction = Vector3::<Meter<$type>>::new(
                    Meter::<$type>::new(1.0),
                    Meter::<$type>::new(-2.0),
                    Meter::<$type>::new(3.0),
                )
                .normalized();
                let angle = Radians::new(1.23);

                let light =
                    SpotLight::<Meter<$type>, RGB<$type>>::new(color, position, direction, angle);

                assert_eq!(
                    Vector3::new(0.0, 1.0, 0.0),
                    light.direction_from(SurfacePoint::new(
                        Point3::new(
                            Meter::<$type>::new(0.0),
                            Meter::<$type>::new(-1.0),
                            Meter::<$type>::new(0.0)
                        ),
                        Normal3::new(0 as $type, 1 as $type, 0 as $type),
                        Point2::new(0 as $type, 0 as $type),
                    ))
                );
                assert_eq!(
                    Vector3::new(0.0, -1.0, 0.0),
                    light.direction_from(SurfacePoint::new(
                        Point3::new(
                            Meter::<$type>::new(0.0),
                            Meter::<$type>::new(10.0),
                            Meter::<$type>::new(0.0)
                        ),
                        Normal3::new(0 as $type, 1 as $type, 0 as $type),
                        Point2::new(0 as $type, 0 as $type),
                    ))
                );

                assert_eq!(
                    Vector3::new(-1.0, 0.0, 0.0),
                    light.direction_from(SurfacePoint::new(
                        Point3::new(
                            Meter::<$type>::new(123.0),
                            Meter::<$type>::new(1.0),
                            Meter::<$type>::new(0.0)
                        ),
                        Normal3::new(0 as $type, 1 as $type, 0 as $type),
                        Point2::new(0 as $type, 0 as $type),
                    ))
                );
                assert_eq!(
                    Vector3::new(1.0, 0.0, 0.0),
                    light.direction_from(SurfacePoint::new(
                        Point3::new(
                            Meter::<$type>::new(-5234.0),
                            Meter::<$type>::new(1.0),
                            Meter::<$type>::new(0.0)
                        ),
                        Normal3::new(0 as $type, 1 as $type, 0 as $type),
                        Point2::new(0 as $type, 0 as $type),
                    ))
                );

                assert_eq!(
                    Vector3::new(0.0, 0.0, -1.0),
                    light.direction_from(SurfacePoint::new(
                        Point3::new(
                            Meter::<$type>::new(0.0),
                            Meter::<$type>::new(1.0),
                            Meter::<$type>::new(53737.0)
                        ),
                        Normal3::new(0 as $type, 1 as $type, 0 as $type),
                        Point2::new(0 as $type, 0 as $type),
                    ))
                );
                assert_eq!(
                    Vector3::new(0.0, 0.0, 1.0),
                    light.direction_from(SurfacePoint::new(
                        Point3::new(
                            Meter::<$type>::new(0.0),
                            Meter::<$type>::new(1.0),
                            Meter::<$type>::new(-236.0)
                        ),
                        Normal3::new(0 as $type, 1 as $type, 0 as $type),
                        Point2::new(0 as $type, 0 as $type),
                    ))
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
                let position = Point3::new(
                    Meter::<$type>::new(0.0),
                    Meter::<$type>::new(1.0),
                    Meter::<$type>::new(0.0),
                );
                let direction = Vector3::<Meter<$type>>::new(
                    Meter::<$type>::new(1.0),
                    Meter::<$type>::new(-2.0),
                    Meter::<$type>::new(3.0),
                )
                .normalized();
                let angle = Radians::new(1.23);

                let light =
                    SpotLight::<Meter<$type>, RGB<$type>>::new(color, position, direction, angle);

                assert_eq!(color, light.get_color());
            }
        };
    }

    spot_light_get_color! { f32, spot_light_get_color_f32 }
    spot_light_get_color! { f64, spot_light_get_color_f64 }
}
