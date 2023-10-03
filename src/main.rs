use rustracer::math::Vector3;
use rustracer::math::Point3;
use rustracer::math::geometry::ImplicitNSphere;
use rustracer::math::geometry::ImplicitPlane3;
use rustracer::math::geometry::AxisAlignedBox;
use rustracer::math::geometry::Triangle;
use rustracer::math::geometry::Intersect;
use rustracer::units::angle;
use rustracer::camera;
use rustracer::camera::RaytracingCamera;
use rustracer::traits::ToRadians;

fn main() {
    let width = 640;
    let height = 480;

    let _plane = ImplicitPlane3::new(
        Point3::new(0.0f32, 0.0, 0.0),
        Vector3::new(0.0f32, 1.0, 0.0)
    );

    let _sphere = ImplicitNSphere::new(
        Point3::new(0.0f32, 1.0, -4.0),
        1.0
    );

    let _aab = AxisAlignedBox::new(
        Point3::new(-0.5, -0.5, -0.5),
        Point3::new(0.5, 0.5, 0.5)
    );

    let triangle = Triangle::new(
        Point3::new(-1.0, 1.0, -3.0),
        Point3::new(1.0, 1.0, -3.0),
        Point3::new(1.0, -1.0, -3.0)
    );

    let cam = camera::Perspective::new(
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
        angle::Degrees::<f32>::new(90.0).to_radians(),
        width as f32,
        height as f32
    );

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let ray = cam.ray_for(x as f32, (height - y - 1) as f32);

        let intersects = ray.intersect(triangle);
         
        if intersects.len() != 0 {
            *pixel = image::Rgb([255u8, 0, 0]);
        } else {
            *pixel = image::Rgb([0u8, 0, 0]);
        }
    }

    imgbuf.save("output.png").unwrap();

    println!("Hello, world!");
}
