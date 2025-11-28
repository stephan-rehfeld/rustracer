use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

use raycaster::raycaster::Raycaster;

use colors::{RGB, RGBA};
use image::converter::Converter;
use image::farbfeld::Encoder;
use math::Vector2;
use scene_graph::Scene3;
use units::length::Meter;

type FloatingPointType = f64;
type LengthType = Meter<FloatingPointType>;
type ColorType = RGB<FloatingPointType>;

type SceneType = Scene3<LengthType, ColorType>;

struct Configuration {
    scene: SceneType,
    camera_name: String,
    size: Vector2<usize>,
    output: String,
}

fn parse_configuration(mut args: impl Iterator<Item = String>) -> Result<Configuration, String> {
    _ = args.next();
    let mut size = Vector2::new(640, 480);
    let mut camera_name: String = String::from("main");
    let mut scene: Option<SceneType> = None;
    let mut output: String = String::from("out.ff");

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--size" => {
                let width = args.next();
                if width.is_none() {
                    return Err(String::from("Missing width for image."));
                }
                let width = width.unwrap().parse::<usize>();
                if let Err(m) = width {
                    return Err(format!("Unable to parse width: {}", m));
                }

                let height = args.next();
                if height.is_none() {
                    return Err(String::from("Missing height for image."));
                }
                let height = height.unwrap().parse::<usize>();
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

            filename => match scene_graph::parser::parse_scene::<LengthType>(filename) {
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
            let raycaster = Raycaster::<LengthType>::new(0.0001);

            let rendered_image = raycaster.render(config.scene, &config.camera_name, config.size);

            let image_data = rendered_image
                .clamp_color(RGB::new(0.0, 0.0, 0.0), RGB::new(1.0, 1.0, 1.0))
                .convert_color::<RGBA<FloatingPointType>>()
                .convert_color::<RGBA<u16>>()
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
