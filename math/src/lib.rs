pub mod geometry;
mod mat;
pub mod normal;
mod point;
pub mod transform;
mod vector;

pub use mat::Mat3x3;
pub use mat::Mat4x4;
pub use normal::Normal;
pub use normal::Normal2;
pub use normal::Normal3;
pub use point::Point;
pub use point::Point2;
pub use point::Point3;
pub use vector::Vector;
pub use vector::Vector2;
pub use vector::Vector3;

pub trait Orthonormal2 {
    fn x_axis() -> Self;
    fn y_axis() -> Self;
}

pub trait Orthonormal3 {
    fn x_axis() -> Self;
    fn y_axis() -> Self;
    fn z_axis() -> Self;
}
