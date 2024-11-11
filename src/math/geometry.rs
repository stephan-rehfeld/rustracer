pub mod axis_aligned_box;
pub mod circle;
pub mod implicit_cylinder;
pub mod implicit_disc3;
pub mod implicit_n_sphere;
pub mod implicit_plane_3;
pub mod parametric_line;
pub mod rectangle;
pub mod sphere;
pub mod triangle;

pub use axis_aligned_box::AxisAlignedBox;
pub use circle::Circle;
pub use implicit_cylinder::ImplicitCylinder;
pub use implicit_disc3::ImplicitDisc3;
pub use implicit_n_sphere::ImplicitNSphere;
pub use implicit_plane_3::ImplicitPlane3;
pub use parametric_line::ParametricLine;
pub use rectangle::Rectangle2;
pub use sphere::Sphere;
pub use triangle::Triangle3;

pub trait Intersect<T> {
    type Output;

    fn intersect(self, other: T) -> Self::Output;
}

use crate::math::{Normal3, Point2, Point3};
use std::fmt::Debug;
use std::ops::Div;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SurfacePoint<T: Div + Copy>
where
    <T as Div>::Output: Debug + Copy + PartialEq,
{
    pub p: Point3<T>,
    pub n: Normal3<<T as Div>::Output>,
    pub uv: Point2<<T as Div>::Output>,
    // Parametric partial derivate for point in u direction
    // Parametric partial derivate for point in v direction
    // Partial derivate for normal in u direction
    // Partial derivate for normal in v direction
}

impl<T: Div + Copy> SurfacePoint<T>
where
    <T as Div>::Output: Debug + Copy + PartialEq,
{
    pub fn new(
        p: Point3<T>,
        n: Normal3<<T as Div>::Output>,
        uv: Point2<<T as Div>::Output>,
    ) -> SurfacePoint<T> {
        SurfacePoint { p, n, uv }
    }
}
