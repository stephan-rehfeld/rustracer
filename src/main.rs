use rustracer::math::Vector3;
use rustracer::math::Point3;
use rustracer::math::geometry::Sphere;
use rustracer::math::geometry::Plane3;
use rustracer::math::geometry::ParametricLine;
use rustracer::math::geometry::Intersect;
use rustracer::camera;

fn main() {
    let width = 640;
    let height = 480;

    let _plane = Plane3::new(
        Point3::new(0.0f32, 0.0, 0.0),
        Vector3::new(0.0f32, 1.0, 0.0)
    );

    let sphere = Sphere::new(
        Point3::new(0.0f32, 1.0, -4.0),
        1.0
    );

    let cam = camera::Perspective::new(
        Point3::new(0.0, 1.0, 0.0),
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0 / 180.0 * 3.1415,
        width,
        height
    );

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let ray = cam.ray_for(x, height - y - 1);

        let intersects = ray.intersect(sphere);
         
        if intersects.len() != 0 {
            *pixel = image::Rgb([255u8, 0, 0]);
        } else {
            *pixel = image::Rgb([0u8, 0, 0]);
        }
    }

    imgbuf.save("output.png").unwrap();

    println!("Hello, world!");
}
