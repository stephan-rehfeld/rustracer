use cg_basics::scene_graph::Scene3;
use colors::{RGB, RGBA};
use diffuseraytracer::camera::RaytracingCamera;
use diffuseraytracer::diffuse_ray_tracer::DiffuseRayTracer;
use diffuseraytracer::light::Light;
use diffuseraytracer::Renderable;
use image::converter::Converter;
use image::farbfeld::Encoder;
use math::{Point2, Vector2};
use random::{RandomNumberGenerator, WichmannHillPRNG};
use sampling::{
    HammersleyPatternGenerator, JitteredPatternGenerator, MultiJitteredPatterGenerator,
    NRooksPatternGenerator, RandomPatternGenerator, RegularPatternGenerator, SamplingPatternSet,
};
use units::length::Meter;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

type FloatingPointType = f64;
type LengthType = Meter<FloatingPointType>;
type ColorType = RGB<FloatingPointType>;

type LightContainer = Box<dyn Light<LengthType, ColorType>>;
type CameraContainer = Box<dyn RaytracingCamera<LengthType>>;
type GeometryContainer = Box<dyn Renderable<LengthType, ColorType>>;

type SceneType = Scene3<ColorType, LightContainer, CameraContainer, GeometryContainer>;

struct Configuration {
    scene: SceneType,
    camera_name: String,
    size: Vector2<usize>,
    output: String,
    sampling_patterns: SamplingPatternSet<Point2<FloatingPointType>>,
}

fn parse_next_usize(
    args: &mut impl Iterator<Item = String>,
    pattern: &str,
    parameter: &str,
) -> Result<usize, String> {
    let value = args.next();
    if value.is_none() {
        return Err(format!(
            "Parameter '{}' for {} pattern is missing.",
            parameter, pattern
        ));
    }
    let value = value.unwrap().parse::<usize>();
    if let Err(m) = value {
        return Err(format!(
            "Failed for parse parameter {} for {} pattern: {}.",
            parameter, pattern, m
        ));
    }

    Ok(value.unwrap())
}

fn parse_sampling_pattern_set(
    args: &mut impl Iterator<Item = String>,
    rnd: &mut impl RandomNumberGenerator<FloatingPointType>,
) -> Result<SamplingPatternSet<Point2<FloatingPointType>>, String> {
    match args.next() {
        Some(p) => match p.as_str() {
            "Regular" => {
                let rows = parse_next_usize(args, "Regular", "rows");
                if let Err(m) = rows {
                    return Err(m);
                }
                let columns = parse_next_usize(args, "Regular", "columns");
                if let Err(m) = columns {
                    return Err(m);
                }
                return Ok(
                    SamplingPatternSet::<Point2<FloatingPointType>>::regular_pattern(
                        rows.unwrap(),
                        columns.unwrap(),
                    ),
                );
            }
            "Random" => {
                let patterns = parse_next_usize(args, "Random", "patterns");
                if let Err(m) = patterns {
                    return Err(m);
                }
                let samples = parse_next_usize(args, "Random", "samples");
                if let Err(m) = samples {
                    return Err(m);
                }

                return Ok(
                    SamplingPatternSet::<Point2<FloatingPointType>>::random_patterns(
                        patterns.unwrap(),
                        samples.unwrap(),
                        rnd,
                    ),
                );
            }
            "Jittered" => {
                let patterns = parse_next_usize(args, "Jittered", "patterns");
                if let Err(m) = patterns {
                    return Err(m);
                }
                let rows = parse_next_usize(args, "Jittered", "rows");
                if let Err(m) = rows {
                    return Err(m);
                }
                let columns = parse_next_usize(args, "Jittered", "columns");
                if let Err(m) = columns {
                    return Err(m);
                }
                return Ok(
                    SamplingPatternSet::<Point2<FloatingPointType>>::jittered_patterns(
                        patterns.unwrap(),
                        rows.unwrap(),
                        columns.unwrap(),
                        rnd,
                    ),
                );
            }
            "NRooks" => {
                let patterns = parse_next_usize(args, "NRooks", "patterns");
                if let Err(m) = patterns {
                    return Err(m);
                }
                let samples = parse_next_usize(args, "NRooks", "samples");
                if let Err(m) = samples {
                    return Err(m);
                }

                return Ok(
                    SamplingPatternSet::<Point2<FloatingPointType>>::n_rooks_patterns(
                        patterns.unwrap(),
                        samples.unwrap(),
                        rnd,
                    ),
                );
            }
            "MultiJittered" => {
                let patterns = parse_next_usize(args, "MultiJittered", "patterns");
                if let Err(m) = patterns {
                    return Err(m);
                }
                let rows = parse_next_usize(args, "MultiJittered", "rows");
                if let Err(m) = rows {
                    return Err(m);
                }
                let columns = parse_next_usize(args, "MultiJittered", "columns");
                if let Err(m) = columns {
                    return Err(m);
                }
                return Ok(
                    SamplingPatternSet::<Point2<FloatingPointType>>::multi_jittered_patterns(
                        patterns.unwrap(),
                        rows.unwrap(),
                        columns.unwrap(),
                        rnd,
                    ),
                );
            }
            "Hammersley" => {
                let samples = parse_next_usize(args, "NRooks", "samples");
                if let Err(m) = samples {
                    return Err(m);
                }
                return Ok(
                    SamplingPatternSet::<Point2<FloatingPointType>>::hammersley_pattern(
                        samples.unwrap(),
                    ),
                );
            }
            &_ => {
                return Err(String::from("Unknown sampling pattern."));
            }
        },
        None => {
            return Err(String::from("Missing pattern name for anti-aliasing."));
        }
    }
}

fn parse_configuration(mut args: impl Iterator<Item = String>) -> Result<Configuration, String> {
    _ = args.next();
    let mut size = Vector2::new(640, 480);
    let mut camera_name: String = String::from("main");
    let mut scene: Option<SceneType> = None;
    let mut output: String = String::from("out.ff");
    let mut rnd = WichmannHillPRNG::new_random();
    let mut sampling_patterns =
        SamplingPatternSet::<Point2<FloatingPointType>>::regular_pattern(1, 1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--sampling" => match parse_sampling_pattern_set(&mut args, &mut rnd) {
                Ok(patterns) => {
                    sampling_patterns = patterns;
                }
                Err(m) => {
                    return Err(m);
                }
            },
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

            filename => match diffuseraytracer::parser::parse_scene::<LengthType>(filename) {
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
        sampling_patterns,
    })
}

fn main() {
    match parse_configuration(env::args()) {
        Ok(config) => {
            let diffuse_ray_tracer =
                DiffuseRayTracer::<LengthType>::new(config.sampling_patterns, 0.0001);

            let rnd = WichmannHillPRNG::new_random();

            let rendered_image =
                diffuse_ray_tracer.render(config.scene, &config.camera_name, config.size, rnd);

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
