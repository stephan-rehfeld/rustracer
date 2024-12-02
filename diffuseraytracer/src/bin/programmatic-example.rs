use cg_basics::camera::{PinholeCamera, RaytracingCamera};
use cg_basics::light::{AmbientLight, Light, PointLight, SpotLight};
use cg_basics::material::{LambertMaterial, PhongMaterial};
use cg_basics::scene_graph::RenderableGeometry;
use colors::{RGB, RGBA};
use diffuseraytracer::diffuse_ray_tracer::DiffuseRayTracer;
use diffuseraytracer::ray_casting::{NewScene, Scene};
use diffuseraytracer::Renderable;
use image::converter::Converter;
use image::farbfeld::Encoder;
use image::SingleColorImage;
use math::geometry::{AxisAlignedBox, ImplicitNSphere, ImplicitPlane3, Triangle3};
use math::transform::Transform3;
use math::{Normal3, Point2, Point3, Vector2, Vector3};
use random::WichmannHillPRNG;
use sampling::{RegularPatternGenerator, SamplingPatternSet};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use traits::ToRadians;
use units::angle::Degrees;
use units::length::Meter;

fn main() {
    let size = Vector2::new(640, 480);

    let plane = ImplicitPlane3::new(
        Point3::new(Meter::new(0.0), Meter::new(0.0), Meter::new(0.0)),
        Normal3::new(0.0, 1.0, 0.0),
        Vector3::new(1.0, 0.0, 0.0),
    );

    let sphere = ImplicitNSphere::new(
        Point3::new(Meter::new(0.0), Meter::new(2.0), Meter::new(-4.0)),
        Meter::new(1.0),
    );

    let aab = AxisAlignedBox::new(
        Point3::new(Meter::new(-0.5), Meter::new(0.5), Meter::new(-0.5)),
        Point3::new(Meter::new(0.5), Meter::new(1.5), Meter::new(0.5)),
    );

    let n = Normal3::new(0.0, 0.0, 1.0);

    let triangle = Triangle3::new(
        Point3::new(Meter::new(-3.0), Meter::new(3.0), Meter::new(-3.0)),
        Point3::new(Meter::new(-1.0), Meter::new(3.0), Meter::new(-3.0)),
        Point3::new(Meter::new(-1.0), Meter::new(1.0), Meter::new(-3.0)),
        n,
        n,
        n,
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(0.0, 1.0),
    );

    let plane_geometry = Box::new(RenderableGeometry::new(
        plane,
        LambertMaterial::new(SingleColorImage::new(
            RGB::new(1.0, 0.0, 0.0),
            Vector2::new(1.0, 1.0),
        )),
        Transform3::<f64>::ident(),
    ));
    let sphere_geometry = Box::new(RenderableGeometry::new(
        sphere,
        PhongMaterial::new(
            SingleColorImage::new(RGB::new(0.0, 1.0, 0.0), Vector2::new(1.0, 1.0)),
            SingleColorImage::new(RGB::new(1.0, 1.0, 1.0), Vector2::new(1.0, 1.0)),
            64.0,
        ),
        Transform3::<f64>::ident(),
    ));
    let aab_geometry = Box::new(RenderableGeometry::new(
        aab,
        LambertMaterial::new(SingleColorImage::new(
            RGB::new(0.0, 0.0, 1.0),
            Vector2::new(1.0, 1.0),
        )),
        Transform3::<f64>::ident(),
    ));
    let triangle_geometry = Box::new(RenderableGeometry::new(
        triangle,
        LambertMaterial::new(SingleColorImage::new(
            RGB::new(1.0, 1.0, 0.0),
            Vector2::new(1.0, 1.0),
        )),
        Transform3::<f64>::ident(),
    ));

    let geometries: Vec<
        Box<
            dyn Renderable<
                Meter<f64>,
                ScalarType = f64,
                ColorType = RGB<f64>,
                LengthType = Meter<f64>,
                VectorType = Vector3<Meter<f64>>,
                PointType = Point3<Meter<f64>>,
                NormalType = Normal3<f64>,
            >,
        >,
    > = vec![
        plane_geometry,
        aab_geometry,
        sphere_geometry,
        triangle_geometry,
    ];

    let ambient_light = Box::new(AmbientLight::new(RGB::new(0.1, 0.1, 0.1)));

    let point_light = Box::new(PointLight::new(
        RGB::new(0.8, 0.8, 0.8),
        Point3::new(Meter::new(0.0), Meter::new(5.0), Meter::new(0.0)),
    ));

    let spot_light = Box::new(SpotLight::new(
        RGB::new(0.5, 0.5, 0.5),
        Point3::new(Meter::new(0.0), Meter::new(4.0), Meter::new(0.0)),
        Vector3::new(0.0, -1.0, 0.0),
        Degrees::new(30.0).to_radians(),
    ));

    let lights: Vec<Box<dyn Light<Meter<f64>, RGB<f64>>>> =
        vec![ambient_light, point_light, spot_light];

    let cam = Box::new(PinholeCamera::new(
        Point3::new(Meter::new(0.0), Meter::new(2.0), Meter::new(5.0)),
        Vector3::new(Meter::new(0.0), Meter::new(0.0), Meter::new(-1.0)),
        Vector3::new(Meter::new(0.0), Meter::new(1.0), Meter::new(0.0)),
        Degrees::<f64>::new(90.0).to_radians(),
    ));

    let mut cameras: HashMap<String, Box<dyn RaytracingCamera<Meter<f64>>>> = HashMap::new();
    cameras.insert(String::from("main"), cam);

    let diffuse_ray_tracer =
        DiffuseRayTracer::new(SamplingPatternSet::regular_pattern(1, 1), 0.0001);

    let scene = Scene::new(RGB::new(0.0, 0.0, 0.0), lights, cameras, geometries);

    let rnd = WichmannHillPRNG::new_random();

    let rendered_image = diffuse_ray_tracer.render(scene, "main", size, rnd);

    let image_data = rendered_image
        .clamp_color(RGB::new(0.0, 0.0, 0.0), RGB::new(1.0, 1.0, 1.0))
        .convert_color::<RGBA<f64>>()
        .convert_color::<RGBA<u16>>()
        .encode();

    let f = File::create("output.ff").unwrap();

    let mut writer = BufWriter::new(f);

    let _ = writer.write_all(image_data.as_slice());
}
