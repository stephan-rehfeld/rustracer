use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

use rustracer::color::{RGB, RGBA};
use rustracer::image::converter::Converter;
use rustracer::image::farbfeld::Encoder;
use rustracer::image::ImageBuffer;
use rustracer::math::geometry::Rectangle2;
use rustracer::math::{Point2, Vector2};
use rustracer::random::WichmannHillPRNG;
use rustracer::sampling::SamplingPatternSet;

use rustracer::image::image_buffer::Circle;

type FloatingPointType = f64;
type ColorType = RGB<FloatingPointType>;

enum Pattern {
    Regular(usize, usize),
    Random(usize, usize),
    Jittered(usize, usize, usize),
    NRooks(usize, usize),
    MultiJittered(usize, usize, usize),
    Hammersley(usize),
}

enum Mode {
    Square,
    Disc,
}

struct Configuration {
    pattern: Pattern,
    mode: Mode,
    seed: Option<u64>,
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

fn parse_configuration(mut args: impl Iterator<Item = String>) -> Result<Configuration, String> {
    let mut pattern: Option<Pattern> = None;
    let mut mode = Mode::Square;
    let mut seed: Option<u64> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "Regular" => {
                let rows = parse_next_usize(&mut args, "Regular", "rows");
                if let Err(m) = rows {
                    return Err(m);
                }
                let columns = parse_next_usize(&mut args, "Regular", "columns");
                if let Err(m) = columns {
                    return Err(m);
                }

                pattern = Some(Pattern::Regular(rows.unwrap(), columns.unwrap()));
            }
            "Random" => {
                let patterns = parse_next_usize(&mut args, "Random", "patterns");
                if let Err(m) = patterns {
                    return Err(m);
                }
                let samples = parse_next_usize(&mut args, "Random", "samples");
                if let Err(m) = samples {
                    return Err(m);
                }

                pattern = Some(Pattern::Random(patterns.unwrap(), samples.unwrap()));
            }
            "Jittered" => {
                let patterns = parse_next_usize(&mut args, "Jittered", "patterns");
                if let Err(m) = patterns {
                    return Err(m);
                }
                let rows = parse_next_usize(&mut args, "Jittered", "rows");
                if let Err(m) = rows {
                    return Err(m);
                }
                let columns = parse_next_usize(&mut args, "Jittered", "columns");
                if let Err(m) = columns {
                    return Err(m);
                }
                pattern = Some(Pattern::Jittered(
                    patterns.unwrap(),
                    rows.unwrap(),
                    columns.unwrap(),
                ));
            }
            "NRooks" => {
                let patterns = parse_next_usize(&mut args, "NRooks", "patterns");
                if let Err(m) = patterns {
                    return Err(m);
                }
                let samples = parse_next_usize(&mut args, "NRooks", "samples");
                if let Err(m) = samples {
                    return Err(m);
                }

                pattern = Some(Pattern::NRooks(patterns.unwrap(), samples.unwrap()));
            }
            "MultiJittered" => {
                let patterns = parse_next_usize(&mut args, "MultiJittered", "patterns");
                if let Err(m) = patterns {
                    return Err(m);
                }
                let rows = parse_next_usize(&mut args, "MultiJittered", "rows");
                if let Err(m) = rows {
                    return Err(m);
                }
                let columns = parse_next_usize(&mut args, "MultiJittered", "columns");
                if let Err(m) = columns {
                    return Err(m);
                }
                pattern = Some(Pattern::MultiJittered(
                    patterns.unwrap(),
                    rows.unwrap(),
                    columns.unwrap(),
                ));
            }
            "Hammersley" => {
                let samples = parse_next_usize(&mut args, "NRooks", "samples");
                if let Err(m) = samples {
                    return Err(m);
                }
                pattern = Some(Pattern::Hammersley(samples.unwrap()));
            }
            "--seed" => match args.next() {
                Some(s) => match s.parse::<u64>() {
                    Ok(s) => {
                        seed = Some(s);
                    }
                    Err(m) => {
                        return Err(format!("Failed to parse seed: {}.", m));
                    }
                },
                None => {
                    return Err(String::from("Missing Seed"));
                }
            },
            "--mode" => match args.next() {
                Some(m) => match m.as_str() {
                    "Square" => {
                        mode = Mode::Square;
                    }
                    "Disc" => {
                        mode = Mode::Disc;
                    }
                    m => {
                        return Err(format!("Unknown mode: {},", m));
                    }
                },
                None => {
                    return Err(String::from("Missing mode"));
                }
            },
            _ => {}
        }
    }

    if pattern.is_none() {
        return Err(String::from("No pattern selected."));
    }

    Ok(Configuration {
        pattern: pattern.unwrap(),
        mode,
        seed,
    })
}

fn main() {
    match parse_configuration(env::args()) {
        Ok(configuration) => {
            let mut rnd = if let Some(seed) = configuration.seed {
                WichmannHillPRNG::from_seed(seed)
            } else {
                WichmannHillPRNG::new_random()
            };

            let patterns = match configuration.pattern {
                Pattern::Regular(rows, columns) => {
                    SamplingPatternSet::<FloatingPointType>::regular_pattern(rows, columns)
                }
                Pattern::Random(patterns, samples) => {
                    SamplingPatternSet::<FloatingPointType>::random_patterns(
                        patterns, samples, &mut rnd,
                    )
                }
                Pattern::Jittered(patterns, rows, columns) => {
                    SamplingPatternSet::<FloatingPointType>::jittered_patterns(
                        patterns, rows, columns, &mut rnd,
                    )
                }
                Pattern::NRooks(patterns, samples) => {
                    SamplingPatternSet::<FloatingPointType>::n_rooks_patterns(
                        patterns, samples, &mut rnd,
                    )
                }
                Pattern::MultiJittered(patterns, rows, columns) => {
                    SamplingPatternSet::<FloatingPointType>::multi_jittered_patterns(
                        patterns, rows, columns, &mut rnd,
                    )
                }
                Pattern::Hammersley(samples) => {
                    SamplingPatternSet::<FloatingPointType>::hammersley_pattern(samples)
                }
            };

            match configuration.mode {
                Mode::Square => {
                    render_square_patterns(patterns);
                }
                Mode::Disc => {
                    render_disc_patterns(patterns.mapped_to_disc());
                }
            }
        }
        Err(m) => {
            eprintln!("{}", m);
        }
    }
}

fn render_disc_patterns(patterns: SamplingPatternSet<FloatingPointType>) {
    for i in 0..patterns.len() {
        let pattern = &patterns[i];
        let mut image_buffer = ImageBuffer::new(Vector2::new(600, 600), RGB::new(1.0, 1.0, 1.0));

        image_buffer.draw_circle(
            Circle::new(Point2::new(300, 300), 250),
            RGB::new(0.8, 0.8, 1.0),
        );

        for j in 0..pattern.len() {
            let point = &pattern[j];
            let transformed_point = Point2::new(
                300 + (point.x * 250.0) as isize,
                300 - (point.y * 250.0) as isize,
            );
            image_buffer.draw_circle(Circle::new(transformed_point, 5), RGB::new(1.0, 0.0, 0.0));
        }

        let converted_image_data = image_buffer
            .convert_color::<RGBA<f64>>()
            .convert_color::<RGBA<u16>>()
            .encode();

        let filename = format!("disc-pattern-{:04}.ff", i);
        let f = File::create(filename.clone()).unwrap();

        let mut writer = BufWriter::new(f);

        let _ = writer.write_all(converted_image_data.as_slice());
        println!("Writing file:: {}.", filename);
    }
}

fn render_square_patterns(patterns: SamplingPatternSet<FloatingPointType>) {
    for i in 0..patterns.len() {
        let pattern = &patterns[i];
        let mut image_buffer = ImageBuffer::new(Vector2::new(600, 600), RGB::new(1.0, 1.0, 1.0));

        draw_square_pattern_outline(&mut image_buffer);
        draw_helper_grid(&mut image_buffer, 25, 25, RGB::new(0.8, 0.8, 0.8));
        draw_helper_grid(&mut image_buffer, 5, 5, RGB::new(0.0, 0.0, 0.0));

        for j in 0..pattern.len() {
            let point = &pattern[j];
            let transformed_point = Point2::new(
                50 + (point.x * 500.0) as isize,
                550 - (point.y * 500.0) as isize,
            );
            image_buffer.draw_circle(Circle::new(transformed_point, 5), RGB::new(1.0, 0.0, 0.0));
            image_buffer.draw_circle(
                Circle::new(Point2::new(transformed_point.x, 575), 5),
                RGB::new(1.0, 0.0, 0.0),
            );
            image_buffer.draw_circle(
                Circle::new(Point2::new(25, transformed_point.y), 5),
                RGB::new(1.0, 0.0, 0.0),
            );
        }
        let converted_image_data = image_buffer
            .convert_color::<RGBA<f64>>()
            .convert_color::<RGBA<u16>>()
            .encode();

        let filename = format!("square-pattern-{:04}.ff", i);
        let f = File::create(filename.clone()).unwrap();

        let mut writer = BufWriter::new(f);

        let _ = writer.write_all(converted_image_data.as_slice());
        println!("Writing file: {}.", filename);
    }
}

fn draw_square_pattern_outline(image_buffer: &mut ImageBuffer<ColorType>) {
    image_buffer.draw_rectangle(
        Rectangle2::new(Point2::new(50, 50), Vector2::new(500, 500)),
        RGB::new(0.0, 0.0, 0.0),
    );

    image_buffer.draw_rectangle(
        Rectangle2::new(Point2::new(10, 50), Vector2::new(30, 500)),
        RGB::new(0.0, 0.0, 0.0),
    );

    image_buffer.draw_rectangle(
        Rectangle2::new(Point2::new(50, 560), Vector2::new(500, 30)),
        RGB::new(0.0, 0.0, 0.0),
    );
}

fn draw_helper_grid(
    image_buffer: &mut ImageBuffer<ColorType>,
    rows: usize,
    columns: usize,
    color: ColorType,
) {
    let pixel_per_row = 500 / rows;
    let pixel_per_column = 500 / columns;

    for row in 0..rows {
        for column in 0..columns {
            image_buffer.draw_rectangle(
                Rectangle2::new(
                    Point2::new(50 + column * pixel_per_column, 50 + row * pixel_per_column),
                    Vector2::new(pixel_per_column, pixel_per_row),
                ),
                color,
            );
        }
    }
}
