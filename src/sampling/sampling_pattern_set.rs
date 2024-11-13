use std::collections::HashMap;
use std::ops::Index;

use crate::math::{Point2, Point3};
use crate::random::RandomNumberGenerator;

use super::SamplingPattern;

pub struct SamplingPatternSet<T> {
    patterns: Vec<SamplingPattern<T>>,
}

impl<T> SamplingPatternSet<T> {
    pub fn new(patterns: Vec<SamplingPattern<T>>) -> SamplingPatternSet<T> {
        SamplingPatternSet { patterns }
    }

    pub fn len(&self) -> usize {
        self.patterns.len()
    }

    pub fn draw_pattern(&self, rnd: &mut impl RandomNumberGenerator<u8>) -> &SamplingPattern<T> {
        let index = (rnd.next_random() as usize) % self.patterns.len();
        &self.patterns[index]
    }
}

impl<T> Index<usize> for SamplingPatternSet<T> {
    type Output = SamplingPattern<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.patterns[index]
    }
}

impl SamplingPatternSet<Point2<f32>> {
    pub fn mapped_to_disc(&self) -> SamplingPatternSet<Point2<f32>> {
        let patterns = self
            .patterns
            .iter()
            .map(|pattern| pattern.mapped_to_disc())
            .collect();

        SamplingPatternSet::new(patterns)
    }

    pub fn mapped_to_hemisphere(&self, e: f32) -> SamplingPatternSet<Point3<f32>> {
        let patterns = self
            .patterns
            .iter()
            .map(|pattern| pattern.mapped_to_hemisphere(e))
            .collect();

        SamplingPatternSet::new(patterns)
    }
}

impl SamplingPatternSet<Point2<f64>> {
    pub fn mapped_to_disc(&self) -> SamplingPatternSet<Point2<f64>> {
        let patterns = self
            .patterns
            .iter()
            .map(|pattern| pattern.mapped_to_disc())
            .collect();

        SamplingPatternSet::new(patterns)
    }

    pub fn mapped_to_hemisphere(&self, e: f64) -> SamplingPatternSet<Point3<f64>> {
        let patterns = self
            .patterns
            .iter()
            .map(|pattern| pattern.mapped_to_hemisphere(e))
            .collect();

        SamplingPatternSet::new(patterns)
    }
}

impl SamplingPatternSet<Point2<f32>> {
    pub fn regular_pattern(rows: usize, columns: usize) -> SamplingPatternSet<Point2<f32>> {
        let mut points = Vec::new();
        let x_step = ((columns + 1) as f32).recip();
        let y_step = ((rows + 1) as f32).recip();

        for row in 1..=rows {
            let row = row as f32;
            for column in 1..=columns {
                let column = column as f32;

                let point = Point2::new(column * x_step, row * y_step);
                points.push(point);
            }
        }

        SamplingPatternSet::new(vec![SamplingPattern::new(points)])
    }
}

impl SamplingPatternSet<Point2<f64>> {
    pub fn regular_pattern(rows: usize, columns: usize) -> SamplingPatternSet<Point2<f64>> {
        let mut points = Vec::new();
        let x_step = ((columns + 1) as f64).recip();
        let y_step = ((rows + 1) as f64).recip();

        for row in 1..=rows {
            let row = row as f64;
            for column in 1..=columns {
                let column = column as f64;

                let point = Point2::new(column * x_step, row * y_step);
                points.push(point);
            }
        }

        SamplingPatternSet::new(vec![SamplingPattern::new(points)])
    }
}

impl SamplingPatternSet<Point2<f32>> {
    pub fn random_patterns(
        patterns: usize,
        samples: usize,
        rnd: &mut impl RandomNumberGenerator<f32>,
    ) -> SamplingPatternSet<Point2<f32>> {
        let mut sampling_patterns = Vec::new();
        for _ in 1..=patterns {
            let mut points = Vec::new();
            for _ in 1..=samples {
                let point = Point2::new(rnd.next_random(), rnd.next_random());
                points.push(point);
            }
            sampling_patterns.push(SamplingPattern::new(points));
        }

        SamplingPatternSet::new(sampling_patterns)
    }
}

impl SamplingPatternSet<Point2<f64>> {
    pub fn random_patterns(
        patterns: usize,
        samples: usize,
        rnd: &mut impl RandomNumberGenerator<f64>,
    ) -> SamplingPatternSet<Point2<f64>> {
        let mut sampling_patterns = Vec::new();
        for _ in 1..=patterns {
            let mut points = Vec::new();
            for _ in 1..=samples {
                let point = Point2::new(rnd.next_random(), rnd.next_random());
                points.push(point);
            }
            sampling_patterns.push(SamplingPattern::new(points));
        }

        SamplingPatternSet::new(sampling_patterns)
    }
}

impl SamplingPatternSet<Point2<f32>> {
    pub fn jittered_patterns(
        patterns: usize,
        rows: usize,
        columns: usize,
        rnd: &mut impl RandomNumberGenerator<f32>,
    ) -> SamplingPatternSet<Point2<f32>> {
        let mut sampling_patterns = Vec::new();

        let x_step = (columns as f32).recip();
        let y_step = (rows as f32).recip();

        for _ in 1..=patterns {
            let mut points = Vec::new();

            for row in 0..rows {
                let row = row as f32;
                for column in 0..columns {
                    let column = column as f32;
                    let point = Point2::new(
                        column * x_step + rnd.next_random() * x_step,
                        row * y_step + rnd.next_random() * y_step,
                    );
                    points.push(point);
                }
            }

            sampling_patterns.push(SamplingPattern::new(points));
        }

        SamplingPatternSet::new(sampling_patterns)
    }
}

impl SamplingPatternSet<Point2<f64>> {
    pub fn jittered_patterns(
        patterns: usize,
        rows: usize,
        columns: usize,
        rnd: &mut impl RandomNumberGenerator<f64>,
    ) -> SamplingPatternSet<Point2<f64>> {
        let mut sampling_patterns = Vec::new();

        let x_step = (columns as f64).recip();
        let y_step = (rows as f64).recip();

        for _ in 1..=patterns {
            let mut points = Vec::new();

            for row in 0..rows {
                let row = row as f64;
                for column in 0..columns {
                    let column = column as f64;
                    let point = Point2::new(
                        column * x_step + rnd.next_random() * x_step,
                        row * y_step + rnd.next_random() * y_step,
                    );
                    points.push(point);
                }
            }

            sampling_patterns.push(SamplingPattern::new(points));
        }

        SamplingPatternSet::new(sampling_patterns)
    }
}

impl SamplingPatternSet<Point2<f32>> {
    pub fn n_rooks_patterns(
        patterns: usize,
        samples: usize,
        rnd: &mut impl RandomNumberGenerator<f32>,
    ) -> SamplingPatternSet<Point2<f32>> {
        let mut sampling_patterns = Vec::new();

        let step_size = (samples as f32).recip();

        for _ in 1..=patterns {
            let mut points = Vec::new();

            for sample in 0..samples {
                let sample = sample as f32;
                let point = Point2::new(
                    sample * step_size + rnd.next_random() * step_size,
                    sample * step_size + rnd.next_random() * step_size,
                );
                points.push(point);
            }

            for i in 0..samples {
                let target = (rnd.next_random() * (samples as f32)) as usize;
                let new_source_point = Point2::new(points[target].x, points[i].y);
                let new_target_point = Point2::new(points[i].x, points[target].y);

                points[i] = new_source_point;
                points[target] = new_target_point;
            }

            for i in 0..samples {
                let target = (rnd.next_random() * (samples as f32)) as usize;

                let new_source_point = Point2::new(points[i].x, points[target].y);
                let new_target_point = Point2::new(points[target].x, points[i].y);

                points[i] = new_source_point;
                points[target] = new_target_point;
            }

            sampling_patterns.push(SamplingPattern::new(points));
        }
        SamplingPatternSet::new(sampling_patterns)
    }
}

impl SamplingPatternSet<Point2<f64>> {
    pub fn n_rooks_patterns(
        patterns: usize,
        samples: usize,
        rnd: &mut impl RandomNumberGenerator<f64>,
    ) -> SamplingPatternSet<Point2<f64>> {
        let mut sampling_patterns = Vec::new();

        let step_size = (samples as f64).recip();

        for _ in 1..=patterns {
            let mut points = Vec::new();

            for sample in 0..samples {
                let sample = sample as f64;
                let point = Point2::new(
                    sample * step_size + rnd.next_random() * step_size,
                    sample * step_size + rnd.next_random() * step_size,
                );
                points.push(point);
            }

            for i in 0..samples {
                let target = (rnd.next_random() * (samples as f64)) as usize;
                let new_source_point = Point2::new(points[target].x, points[i].y);
                let new_target_point = Point2::new(points[i].x, points[target].y);

                points[i] = new_source_point;
                points[target] = new_target_point;
            }

            for i in 0..samples {
                let target = (rnd.next_random() * (samples as f64)) as usize;

                let new_source_point = Point2::new(points[i].x, points[target].y);
                let new_target_point = Point2::new(points[target].x, points[i].y);

                points[i] = new_source_point;
                points[target] = new_target_point;
            }

            sampling_patterns.push(SamplingPattern::new(points));
        }
        SamplingPatternSet::new(sampling_patterns)
    }
}

// Multijittered Generator
impl SamplingPatternSet<Point2<f32>> {
    pub fn multi_jittered_patterns(
        patterns: usize,
        rows: usize,
        columns: usize,
        rnd: &mut impl RandomNumberGenerator<f32>,
    ) -> SamplingPatternSet<Point2<f32>> {
        let mut sampling_patterns = Vec::new();

        let x_step = ((columns) as f32).recip();
        let y_step = ((rows) as f32).recip();

        let x_sub_step = x_step / (columns as f32);
        let y_sub_step = y_step / (rows as f32);

        for _ in 1..=patterns {
            let mut points = Vec::new();

            let mut available_rows_map: HashMap<usize, Vec<usize>> = HashMap::new();
            let mut available_columns_map: HashMap<usize, Vec<usize>> = HashMap::new();

            for row in 0..rows {
                available_rows_map.insert(row, (0..rows).collect());
            }

            for column in 0..columns {
                available_columns_map.insert(column, (0..columns).collect());
            }

            for row in 0..rows {
                let available_rows = available_rows_map.get_mut(&row).expect(
                    "A row that should exist in the map is missing. This should not happen.",
                );

                for column in 0..columns {
                    let available_columns = available_columns_map.get_mut(&column).expect(
                        "A column that should exist in the map is missing. This should not happen.",
                    );

                    let sub_row = available_rows
                        .remove((rnd.next_random() * (available_rows.len() as f32)) as usize)
                        as f32;
                    let sub_column = available_columns
                        .remove((rnd.next_random() * (available_columns.len() as f32)) as usize)
                        as f32;

                    let column = column as f32;
                    let row = row as f32;

                    let point = Point2::new(
                        column * x_step + sub_column * x_sub_step + rnd.next_random() * x_sub_step,
                        row * y_step + sub_row * y_sub_step + rnd.next_random() * y_sub_step,
                    );

                    points.push(point);
                }
            }

            sampling_patterns.push(SamplingPattern::new(points));
        }

        SamplingPatternSet::new(sampling_patterns)
    }
}

impl SamplingPatternSet<Point2<f64>> {
    pub fn multi_jittered_patterns(
        patterns: usize,
        rows: usize,
        columns: usize,
        rnd: &mut impl RandomNumberGenerator<f64>,
    ) -> SamplingPatternSet<Point2<f64>> {
        let mut sampling_patterns = Vec::new();

        let x_step = ((columns) as f64).recip();
        let y_step = ((rows) as f64).recip();

        let x_sub_step = x_step / (columns as f64);
        let y_sub_step = y_step / (rows as f64);

        for _ in 1..=patterns {
            let mut points = Vec::new();

            let mut available_rows_map: HashMap<usize, Vec<usize>> = HashMap::new();
            let mut available_columns_map: HashMap<usize, Vec<usize>> = HashMap::new();

            for row in 0..rows {
                available_rows_map.insert(row, (0..rows).collect());
            }

            for column in 0..columns {
                available_columns_map.insert(column, (0..columns).collect());
            }

            for row in 0..rows {
                let available_rows = available_rows_map.get_mut(&row).expect(
                    "A row that should exist in the map is missing. This should not happen.",
                );

                for column in 0..columns {
                    let available_columns = available_columns_map.get_mut(&column).expect(
                        "A column that should exist in the map is missing. This should not happen.",
                    );

                    let sub_row = available_rows
                        .remove((rnd.next_random() * (available_rows.len() as f64)) as usize)
                        as f64;
                    let sub_column = available_columns
                        .remove((rnd.next_random() * (available_columns.len() as f64)) as usize)
                        as f64;

                    let column = column as f64;
                    let row = row as f64;

                    let point = Point2::new(
                        column * x_step + sub_column * x_sub_step + rnd.next_random() * x_sub_step,
                        row * y_step + sub_row * y_sub_step + rnd.next_random() * y_sub_step,
                    );

                    points.push(point);
                }
            }

            sampling_patterns.push(SamplingPattern::new(points));
        }

        SamplingPatternSet::new(sampling_patterns)
    }
}

impl SamplingPatternSet<Point2<f32>> {
    pub fn hammersley_pattern(num_points: usize) -> SamplingPatternSet<Point2<f32>> {
        let mut points = vec![Point2::new(0.0, 0.0)];

        let x_step = ((num_points - 1) as f32).recip();

        for p in 1..num_points {
            let point = Point2::new((p as f32) * x_step, Self::radical_inverse(p));
            points.push(point);
        }

        SamplingPatternSet::new(vec![SamplingPattern::new(points)])
    }

    fn radical_inverse(j: usize) -> f32 {
        let mut j = j;
        let mut x = 0.0;
        let mut f = 0.5;

        loop {
            if j % 2 != 0 {
                x += f;
            }
            j /= 2;
            f *= 0.5;

            if j == 0 {
                break;
            }
        }

        x
    }
}

impl SamplingPatternSet<Point2<f64>> {
    pub fn hammersley_pattern(num_points: usize) -> SamplingPatternSet<Point2<f64>> {
        let mut points = vec![Point2::new(0.0, 0.0)];

        let x_step = ((num_points - 1) as f64).recip();

        for p in 1..num_points {
            let point = Point2::new((p as f64) * x_step, Self::radical_inverse(p));
            points.push(point);
        }

        SamplingPatternSet::new(vec![SamplingPattern::new(points)])
    }

    fn radical_inverse(j: usize) -> f64 {
        let mut j = j;
        let mut x = 0.0;
        let mut f = 0.5;

        loop {
            if j % 2 != 0 {
                x += f;
            }
            j /= 2;
            f *= 0.5;

            if j == 0 {
                break;
            }
        }

        x
    }
}

// Holton generator
// Sobol generator
