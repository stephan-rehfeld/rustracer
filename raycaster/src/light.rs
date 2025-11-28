
use cg_basics::light::{
    AmbientLight, AmbientOcclusionLight, DirectionalLight, PointLight, SpotLight,
};
use colors::Color;
use math::geometry::{ParametricLine, SurfacePoint};
use math::{Point3, Vector3};
use scene_graph::Light;
use traits::{Cos, FloatingPoint, SignedNumber, Zero};
use units::length::Length;

pub trait RaycasterLight<T: Length, C: Color<ChannelType = T::ValueType>>
{
    fn direction_from(&self, sp: SurfacePoint<T>) -> Vector3<T::ValueType>;
    fn get_color(&self) -> C;

    fn illuminates(
        &self,
        sp: SurfacePoint<T>,
        shadow_check: &dyn Fn(
            ParametricLine<Point3<T>, Vector3<T>>,
            Option<T>,
        ) -> Option<T::ValueType>,
    ) -> bool;
}

impl<T: Length, C: Color<ChannelType = T::ValueType>> RaycasterLight<T, C> for Light<T, C>
where
    T::ValueType: FloatingPoint,
{
    fn direction_from(&self, sp: SurfacePoint<T>) -> Vector3<T::ValueType> {
        match self {
            Light::AmbientLight(l) => l.direction_from(sp),
            Light::AmbientOcclusionLight(l) => l.direction_from(sp),
            Light::DirectionalLight(l) => l.direction_from(sp),
            Light::PointLight(l) => l.direction_from(sp),
            Light::SpotLight(l) => l.direction_from(sp),
        }
    }

    fn get_color(&self) -> C {
        match self {
            Light::AmbientLight(l) => {
                <cg_basics::light::AmbientLight<C> as RaycasterLight<T, C>>::get_color(l)
            }
            Light::AmbientOcclusionLight(l) => l.get_color(),
            Light::DirectionalLight(l) => l.get_color(),
            Light::PointLight(l) => l.get_color(),
            Light::SpotLight(l) => l.get_color(),
        }
    }

    fn illuminates(
        &self,
        sp: SurfacePoint<T>,
        shadow_check: &dyn Fn(
            ParametricLine<Point3<T>, Vector3<T>>,
            Option<T>,
        ) -> Option<T::ValueType>
    ) -> bool {
        match self {
            Light::AmbientLight(l) => l.illuminates(sp, shadow_check),
            Light::AmbientOcclusionLight(l) => l.illuminates(sp, shadow_check),
            Light::DirectionalLight(l) => l.illuminates(sp, shadow_check),
            Light::PointLight(l) => l.illuminates(sp, shadow_check),
            Light::SpotLight(l) => l.illuminates(sp, shadow_check),
        }
    }
}

impl<T: Length, C: Color<ChannelType = T::ValueType>> RaycasterLight<T, C> for DirectionalLight<T, C>
where
    T::ValueType: SignedNumber,
{
    fn direction_from(&self, _sp: SurfacePoint<T>) -> Vector3<T::ValueType> {
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
        ) -> Option<T::ValueType>,
    ) -> bool {
        self.direction.dot(sp.n.as_vector()) > Zero::zero()
            && shadow_check(
                ParametricLine::new(sp.p, self.direction_from(sp) * T::one()),
                None,
            )
            .is_none()
    }
}

impl<T: Length, C: Color<ChannelType = T::ValueType>> RaycasterLight<T, C> for PointLight<T, C>
where
    T::ValueType: SignedNumber,
{
    fn direction_from(&self, sp: SurfacePoint<T>) -> Vector3<T::ValueType> {
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
        ) -> Option<T::ValueType>,
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

impl<T: Length, C: Color<ChannelType = T::ValueType>> RaycasterLight<T, C> for SpotLight<T, C>
where
    T::ValueType: FloatingPoint,
{
    fn direction_from(&self, sp: SurfacePoint<T>) -> Vector3<T::ValueType> {
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
        ) -> Option<T::ValueType>,
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

impl<T: Length, C: Color<ChannelType = T::ValueType>> RaycasterLight<T, C> for AmbientLight<C>
where
    T::ValueType: FloatingPoint,
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
        ) -> Option<T::ValueType>,
    ) -> bool {
        true
    }

    fn direction_from(&self, sp: SurfacePoint<T>) -> Vector3<T::ValueType> {
        sp.n.as_vector().normalized()
    }
}

impl<T: Length, C: Color<ChannelType = T::ValueType>> RaycasterLight<T, C> for AmbientOcclusionLight<T, C>
where
    <T as Length>::ValueType: FloatingPoint,
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
        ) -> Option<T::ValueType>
    ) -> bool {
        false
    }

    fn direction_from(&self, sp: SurfacePoint<T>) -> Vector3<T::ValueType> {
        sp.n.as_vector().normalized()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use colors::RGB;
    use math::{Normal3, Point2, Vector3};
    use units::angle::Radians;
    use units::length::Meter;

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
