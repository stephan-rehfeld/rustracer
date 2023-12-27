use std::fs::File;
use std::io::{BufWriter, Write};
use rustracer::{ClassicRaytracer, Raytracer, RenderableGeometry};
use rustracer::camera::Perspective;
use rustracer::color::{RGB, RGBA};
use rustracer::image::converter::Converter;
use rustracer::image::farbfeld::Encoder;
use rustracer::math::{Normal3, Point2, Point3, Vector2, Vector3};
use rustracer::math::geometry::{AxisAlignedBox, ImplicitNSphere, ImplicitPlane3, Triangle};
use rustracer::traits::ToRadians;
use rustracer::units::angle::Degrees;
use rustracer::units::length::Meter;

fn main() {
    let size = Vector2::new( 640.0, 480.0 );

    let plane = ImplicitPlane3::new(
        Point3::new(Meter::new(0.0), Meter::new(0.0), Meter::new(0.0)),
        Normal3::new(0.0, 1.0, 0.0)
    );

    let sphere = ImplicitNSphere::new(
        Point3::new(Meter::new(0.0), Meter::new(2.0), Meter::new(-4.0)),
        Meter::new(1.0)
    );

    let aab = AxisAlignedBox::new(
        Point3::new(Meter::new(-0.5), Meter::new(-0.5), Meter::new(-0.5)),
        Point3::new(Meter::new(0.5), Meter::new(0.5), Meter::new(0.5))
    );

    let n = Normal3::new(0.0, 0.0, -1.0);

    let triangle = Triangle::new(
        Point3::new(Meter::new(-3.0), Meter::new(3.0), Meter::new(-3.0)),
        Point3::new(Meter::new(-1.0), Meter::new(3.0), Meter::new(-3.0)),
        Point3::new(Meter::new(-1.0), Meter::new(1.0), Meter::new(-3.0)),
        n,
        n,
        n
    );

    let plane_geometry = Box::new(RenderableGeometry::new(plane, RGB::new(1.0, 0.0, 0.0)));
    let sphere_geometry = Box::new(RenderableGeometry::new(sphere, RGB::new(0.0, 1.0, 0.0)));
    let aab_geometry = Box::new(RenderableGeometry::new(aab, RGB::new(0.0, 0.0, 1.0)));
    let triangle_geometry = Box::new(RenderableGeometry::new(triangle, RGB::new(1.0, 1.0, 0.0)));

    let geometries : Vec<Box< <ClassicRaytracer<Meter<f64>, RGB<f64>> as Raytracer>::RenderableTraitType>> = vec![plane_geometry, aab_geometry, sphere_geometry, triangle_geometry];

    let cam = Box::new(Perspective::new(
        Point3::new(Meter::new(0.0), Meter::new(2.0), Meter::new(5.0)),
        Vector3::new(Meter::new(0.0), Meter::new(0.0), Meter::new(-1.0)),
        Vector3::new(Meter::new(0.0), Meter::new(1.0), Meter::new(0.0)),
        Degrees::<f64>::new(90.0).to_radians(),
        size
    ));

    let raytracer = ClassicRaytracer::new(cam, geometries, RGB::new(0.0, 0.0, 0.0));

    let image_data = raytracer
        .clamp_color(RGB::new(0.0, 0.0, 0.0), RGB::new(1.0, 1.0, 1.0))
        .convert_color::<RGBA<f64>>()
        .convert_color::<RGBA<u16>>()
        .convert_coordinate::<Point2<usize>>()
        .encode();

    let f = File::create("output.ff").unwrap();

    let mut writer = BufWriter::new(f);

    let _ = writer.write_all(image_data.as_slice());
}
