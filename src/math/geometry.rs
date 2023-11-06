pub mod axis_aligned_box;
pub mod implicit_n_sphere;
pub mod implicit_plane_3;
pub mod parametric_line;
pub mod triangle;

pub use axis_aligned_box::AxisAlignedBox;
//pub use implicit_n_sphere::ImplicitNSphere;
pub use implicit_plane_3::ImplicitPlane3;
pub use parametric_line::ParametricLine;
//pub use triangle::Triangle;

pub trait Intersect<T> {
    type Output;

    fn intersect(self, other: T) -> Self::Output;
}
