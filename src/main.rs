use rustracer::math::Vector3;
use rustracer::math::Point3;
use rustracer::math::geometry::ImplicitNSphere;
use rustracer::math::geometry::ImplicitPlane3;
use rustracer::math::geometry::AxisAlignedBox;
use rustracer::math::geometry::Triangle;
use rustracer::units::angle;
use rustracer::camera;
use rustracer::camera::RaytracingCamera;
use rustracer::traits::ToRadians;
use rustracer::color;


use rustracer::units::length::Meter;

fn main() {
    let width = 640;
    let height = 480;

    let plane = ImplicitPlane3::new(
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0)
    );

    let sphere = ImplicitNSphere::new(
        Point3::new(0.0, 2.0, -4.0),
        1.0
    );

    let aab = AxisAlignedBox::new(
        Point3::new(-0.5, -0.5, -0.5),
        Point3::new(0.5, 0.5, 0.5)
    );

    let triangle = Triangle::new(
        Point3::new(-3.0, 3.0, -3.0),
        Point3::new(-1.0, 3.0, -3.0),
        Point3::new(-1.0, 1.0, -3.0)
    );

    let plane_geometry = Box::new(rustracer::RenderableGeometry::new(plane, color::RGB::new(1.0, 0.0, 0.0)));
    let sphere_geometry = Box::new(rustracer::RenderableGeometry::new(sphere, color::RGB::new(0.0, 1.0, 0.0)));
    let aab_geometry = Box::new(rustracer::RenderableGeometry::new(aab, color::RGB::new(0.0, 0.0, 1.0)));
    let triangle_geometry = Box::new(rustracer::RenderableGeometry::new(triangle, color::RGB::new(1.0, 1.0, 0.0)));

    let geometries : Vec<Box<dyn rustracer::Renderable<f64>>> = vec![plane_geometry, aab_geometry, sphere_geometry, triangle_geometry];

    let cam = camera::Perspective::new(
        Point3::new(0.0, 2.0, 5.0),
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
        angle::Degrees::<f64>::new(90.0).to_radians(),
        width as f64,
        height as f64
    );

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let ray = cam.ray_for(x as f64, (height - y - 1) as f64);

        let mut hits : Vec<(f64, color::RGB<f64>)> = geometries.iter().flat_map(|g| g.intersect(ray)).filter(|(t,_)| *t > 0.0).collect();
        hits.sort_by( |(t1,_), (t2, _)| t1.partial_cmp( t2 ).unwrap() );

        if hits.is_empty() {
            *pixel = image::Rgb([0u8, 0, 0]);
        } else {
            let (_, color) = hits[0];
            *pixel = image::Rgb([(color.red * 255.0) as u8, (color.green * 255.0) as u8, (color.blue * 255.0) as u8,]);
        }
    }

    imgbuf.save("output.png").unwrap();
    println!("Hello, world!");

    let a = Meter::new( 2.0 );
    let b = Meter::new( 3.0 );
    let c = Meter::new( 4.0 );

    let area = a * b;
    let volume = area * c;

    let area2 = volume / a;
    let length2 = area2 / b;

    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
    println!("area = {}", area);
    println!("volume = {}", volume);
    println!("area2 = {}", area2);
    println!("length2 = {}", length2);
}
