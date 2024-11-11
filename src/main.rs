use rustracer::color::{RGB, RGBA};
use rustracer::image::converter::Converter;
use rustracer::image::farbfeld::Encoder;
use rustracer::math::{Point2, Vector2};
use rustracer::ray_casting::{RayCaster, Scene};
use rustracer::units::length::Meter;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

type FloatingPointType = f64;
type LengthType = Meter<FloatingPointType>;
type ColorType = RGB<FloatingPointType>;

struct Configuration {
    scene: Scene<LengthType, ColorType>,
    camera_name: String,
    size: Vector2<FloatingPointType>,
    output: String,
}

fn parse_configuration(mut args: impl Iterator<Item = String>) -> Result<Configuration, String> {
    _ = args.next();
    let mut size = Vector2::new(640.0, 480.0);
    let mut camera_name: String = String::from("main");
    let mut scene: Option<Scene<LengthType, ColorType>> = None;
    let mut output: String = String::from("out.ff");

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--size" => {
                let width = args.next();
                if width.is_none() {
                    return Err(String::from("Missing width for image."));
                }
                let width = width.unwrap().parse::<FloatingPointType>();
                if let Err(m) = width {
                    return Err(format!("Unable to parse width: {}", m));
                }

                let height = args.next();
                if height.is_none() {
                    return Err(String::from("Missing height for image."));
                }
                let height = height.unwrap().parse::<FloatingPointType>();
                if let Err(m) = height {
                    return Err(format!("Unable to parse height: {}", m));
                }

                size = Vector2::new(width.unwrap(), height.unwrap());
            }
            "--camera" => match args.next() {
                Some(c) => {
                    camera_name = c;
                }
                None => {
                    return Err(String::from("Missing camera name."));
                }
            },
            "-O" => match args.next() {
                Some(o) => {
                    output = o;
                }
                None => {
                    return Err(String::from("Missing output filename."));
                }
            },

            filename => match rustracer::parser::parse_scene::<LengthType>(filename) {
                Ok(s) => {
                    scene = Some(s);
                }
                Err(err) => {
                    return Err(format!(
                        "Failed to parse passed scene file. Error was: {:?}",
                        err
                    ));
                }
            },
        }
    }

    if scene.is_none() {
        return Err(String::from("No scene file was passed."));
    }

    Ok(Configuration {
        scene: scene.unwrap(),
        camera_name,
        size,
        output,
    })
}

fn main() {
    match parse_configuration(env::args()) {
        Ok(config) => {
            let mut scene = config.scene;
            let camera = scene.cameras.remove(&config.camera_name);

            if camera.is_none() {
                eprintln!("Missing camera with name {}", config.camera_name);
                return;
            }

            let raytracer = RayCaster::new(
                config.size,
                camera.unwrap(),
                scene.geometries,
                scene.lights,
                scene.ambient_light,
                scene.bg_color,
                0.0001,
            );

            let image_data = raytracer
                .clamp_color(RGB::new(0.0, 0.0, 0.0), RGB::new(1.0, 1.0, 1.0))
                .convert_color::<RGBA<f64>>()
                .convert_color::<RGBA<u16>>()
                .convert_coordinate::<Point2<usize>>()
                .encode();

            let f = File::create(config.output).unwrap();

            let mut writer = BufWriter::new(f);

            let _ = writer.write_all(image_data.as_slice());
        }
        Err(m) => {
            eprintln!("{}", m);
        }
    }
}
