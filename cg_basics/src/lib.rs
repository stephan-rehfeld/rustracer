pub mod camera;
pub mod light;
pub mod material;

use math::Point3;

pub type Cylinder<T> = math::geometry::ImplicitCylinder<T>;
pub type Disc<T> = math::geometry::ImplicitDisc3<T>;
pub type Plane<T> = math::geometry::ImplicitPlane3<T>;
pub type Sphere<T> = math::geometry::Sphere<T>;
pub type AxisAlignedBox<T> = math::geometry::AxisAlignedBox<Point3<T>>;
pub type Triangle<T> = math::geometry::Triangle3<T>;
