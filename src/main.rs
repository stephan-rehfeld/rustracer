use rustracer::color::{RGB, RGBA};
use rustracer::image::converter::Converter;
use rustracer::image::farbfeld::Encoder;
use rustracer::image::sampler::Sampler;
use rustracer::math::{Point2, Vector2};
use rustracer::random::{RandomNumberGenerator, WichmannHillPRNG};
use rustracer::ray_casting::{RayCaster, Scene};
use rustracer::sampling::{
    HammersleyPatternGenerator, JitteredPatternGenerator, MultiJitteredPatterGenerator,
    NRooksPatternGenerator, PatternMapping, RandomPatternGenerator, RegularPatternGenerator,
    SamplingPatternSet,
};
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
    anti_aliasing_patterns: SamplingPatternSet<Point2<FloatingPointType>>,
    camera_sampling_patterns: SamplingPatternSet<Point2<FloatingPointType>>,
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
    let mut size = Vector2::new(640.0, 480.0);
    let mut camera_name: String = String::from("main");
    let mut scene: Option<Scene<LengthType, ColorType>> = None;
    let mut output: String = String::from("out.ff");
    let mut rnd = WichmannHillPRNG::new_random();
    let mut anti_aliasing_patterns =
        SamplingPatternSet::<Point2<FloatingPointType>>::regular_pattern(1, 1);

    let mut camera_sampling_patterns =
        SamplingPatternSet::<Point2<FloatingPointType>>::regular_pattern(5, 5);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--anti-aliasing" => match parse_sampling_pattern_set(&mut args, &mut rnd) {
                Ok(patterns) => {
                    anti_aliasing_patterns = patterns;
                }
                Err(m) => {
                    return Err(m);
                }
            },
            "--camera-sampling" => match parse_sampling_pattern_set(&mut args, &mut rnd) {
                Ok(patterns) => {
                    camera_sampling_patterns = patterns;
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
        anti_aliasing_patterns,
        camera_sampling_patterns,
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
                config.camera_sampling_patterns.mapped_to_disc(),
                scene.geometries,
                scene.lights,
                scene.bg_color,
                0.0001,
            );

            let image_data = raytracer
                .sample(config.anti_aliasing_patterns)
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
