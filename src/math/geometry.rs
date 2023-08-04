pub mod parametric_line;
pub mod implicit_plane_3;
pub mod implicit_n_sphere;
pub mod axis_aligned_box;

pub use parametric_line::ParametricLine;
pub use implicit_plane_3::ImplicitPlane3;
pub use implicit_n_sphere::ImplicitNSphere;
pub use axis_aligned_box::AxisAlignedBox;

pub trait Intersect<T> {
    type Output;

    fn intersect(self, other: T) -> Vec<Self::Output>;
}
