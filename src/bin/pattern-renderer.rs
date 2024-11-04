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

fn main() {
    let mut rnd = WichmannHillPRNG::new(876, 1239, 24986);
    //let patterns = SamplingPatternSet::regular_pattern(5, 5);
    //let patterns = SamplingPatternSet::random_patterns(1, 16, &mut rnd);
    //let patterns = SamplingPatternSet::jittered_patterns(1, 8, 8, &mut rnd);
    //let patterns = SamplingPatternSet::n_rooks_patterns(20, 32, &mut rnd);
    let patterns = SamplingPatternSet::multi_jittered_patterns(20, 5, 5, &mut rnd);
    //let patterns = SamplingPatternSet::hammersley_pattern(256);

    for i in 0..patterns.len() {
        let pattern = &patterns[i];
        let mut image_buffer = ImageBuffer::new(Vector2::new(600, 600), RGB::new(1.0, 1.0, 1.0));

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

        let rows = 25;
        let columns = 25;

        let pixel_per_row = 500 / rows;
        let pixel_per_column = 500 / columns;

        for row in 0..rows {
            for column in 0..columns {
                image_buffer.draw_rectangle(
                    Rectangle2::new(
                        Point2::new(50 + column * pixel_per_column, 50 + row * pixel_per_column),
                        Vector2::new(pixel_per_column, pixel_per_row),
                    ),
                    RGB::new(0.8, 0.8, 0.8),
                );
            }
        }

        let rows = 5;
        let columns = 5;

        let pixel_per_row = 500 / rows;
        let pixel_per_column = 500 / columns;

        for row in 0..rows {
            for column in 0..columns {
                image_buffer.draw_rectangle(
                    Rectangle2::new(
                        Point2::new(50 + column * pixel_per_column, 50 + row * pixel_per_column),
                        Vector2::new(pixel_per_column, pixel_per_row),
                    ),
                    RGB::new(0.0, 0.0, 0.0),
                );
            }
        }

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

        let f = File::create(format!("pattern-{:04}.ff", i)).unwrap();

        let mut writer = BufWriter::new(f);

        let _ = writer.write_all(converted_image_data.as_slice());
        println!("Writing pattern #{}.", (i + 1));
    }
}
