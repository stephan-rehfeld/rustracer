use crate::math::Vector3;
use crate::math::Point3;
use crate::math::geometry::ParametricLine;

pub struct Orthographic<T> {
    e: Point3<T>,
    u: Vector3<T>,
    v: Vector3<T>,
    w: Vector3<T>,
    width: u32,
    height: u32,
    scale: T,
    aspectRatio: T,
}

impl Orthographic<f32> {
    pub fn new(e: Point3<f32>, g: Vector3<f32>, t: Vector3<f32>, width: u32, height: u32, scale: f32) -> Orthographic<f32> {
        let w = -g.normalized();
        let u = Vector3::cross(t, w).normalized();
        let v = Vector3::cross(w, u);

        let aspectRatio = (width as f32)/(height as f32);

        Orthographic { e, u, v, w, width, height, scale, aspectRatio }
    }

    pub fn ray_for(self, x: u32, y: u32) -> ParametricLine<f32> {
        let d = -self.w;

        let x = x as f32;
        let y = y as f32;

        let width = self.width as f32;
        let height = self.height as f32;
        
        let x_factor = x - ((width-1.0)/2.0) / (width-1.0);
        let y_factor = y - ((height-1.0)/2.0) / (height-1.0);

        let o = self.e + self.aspectRatio * self.scale * x_factor * self.u + self.scale * y_factor * self.v;

        ParametricLine::new(o, d)

    }
}

