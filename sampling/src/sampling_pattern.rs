use std::ops::Index;

use math::{Point2, Point3};
use random::RandomNumberGenerator;

pub struct SamplingPattern<T> {
    points: Vec<T>,
}

impl<T> SamplingPattern<T> {
    pub fn new(points: Vec<T>) -> SamplingPattern<T> {
        SamplingPattern { points }
    }

    pub fn shuffle(&mut self, rnd: &mut impl RandomNumberGenerator<usize>) {
        let mut points = Vec::new();

        while self.points.len() > 0 {
            points.push(self.points.remove(rnd.next_random() % self.points.len()));
        }

        self.points = points;
    }

    pub fn shuffled(&self, rnd: &mut impl RandomNumberGenerator<usize>) -> Self
    where
        T: Clone,
    {
        let mut points = self.points.clone();
        let mut shuffled_points = Vec::new();

        while self.points.len() > 0 {
            shuffled_points.push(points.remove(rnd.next_random() % points.len()));
        }

        Self::new(shuffled_points)
    }

    pub fn draw_point(&self, rnd: &mut impl RandomNumberGenerator<usize>) -> &T {
        &self.points[rnd.next_random() % self.points.len()]
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }
}

impl<T> Index<usize> for SamplingPattern<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.points[index]
    }
}

pub trait PatternMapping<T> {
    fn mapped_to_disc(&self) -> SamplingPattern<Point2<T>>;
    fn mapped_to_hemisphere(&self, e: T) -> SamplingPattern<Point3<T>>;
}

impl PatternMapping<f32> for SamplingPattern<Point2<f32>> {
    fn mapped_to_disc(&self) -> SamplingPattern<Point2<f32>> {
        let points = self
            .points
            .iter()
            .map(|point| {
                let x = 2.0 * point.x - 1.0;
                let y = 2.0 * point.y - 1.0;

                let r: f32;
                let mut phi: f32 = 0.0;

                if x > -y {
                    if x > y {
                        r = x;
                        phi = y / x;
                    } else {
                        r = y;
                        phi = 2.0 - x / y;
                    }
                } else {
                    if x < y {
                        r = -x;
                        phi = 4.0 + y / x;
                    } else {
                        r = -y;
                        if y != 0.0 {
                            phi = 6.0 - x / y;
                        }
                    }
                }

                phi *= std::f32::consts::PI / 4.0;

                Point2::new(r * phi.cos(), r * phi.sin())
            })
            .collect();

        SamplingPattern::new(points)
    }

    fn mapped_to_hemisphere(&self, e: f32) -> SamplingPattern<Point3<f32>> {
        let points = self
            .points
            .iter()
            .map(|point| {
                let cos_phi = (2.0 * std::f32::consts::PI * point.x).cos();
                let sin_phi = (2.0 * std::f32::consts::PI * point.x).sin();

                let cos_theta = (1.0 - point.y).powf(1.0 / (e + 1.0));
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let x = sin_theta * cos_phi;
                let y = sin_theta * sin_phi;
                let z = cos_theta;

                Point3::new(x, y, z)
            })
            .collect();

        SamplingPattern::new(points)
    }
}

impl PatternMapping<f64> for SamplingPattern<Point2<f64>> {
    fn mapped_to_disc(&self) -> SamplingPattern<Point2<f64>> {
        let points = self
            .points
            .iter()
            .map(|point| {
                let x = 2.0 * point.x - 1.0;
                let y = 2.0 * point.y - 1.0;

                let r: f64;
                let mut phi: f64 = 0.0;

                if x > -y {
                    if x > y {
                        r = x;
                        phi = y / x;
                    } else {
                        r = y;
                        phi = 2.0 - x / y;
                    }
                } else {
                    if x < y {
                        r = -x;
                        phi = 4.0 + y / x;
                    } else {
                        r = -y;
                        if y != 0.0 {
                            phi = 6.0 - x / y;
                        }
                    }
                }

                phi *= std::f64::consts::PI / 4.0;

                Point2::new(r * phi.cos(), r * phi.sin())
            })
            .collect();

        SamplingPattern::new(points)
    }

    fn mapped_to_hemisphere(&self, e: f64) -> SamplingPattern<Point3<f64>> {
        let points = self
            .points
            .iter()
            .map(|point| {
                let cos_phi = (2.0 * std::f64::consts::PI * point.x).cos();
                let sin_phi = (2.0 * std::f64::consts::PI * point.x).sin();

                let cos_theta = (1.0 - point.y).powf(1.0 / (e + 1.0));
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let x = sin_theta * cos_phi;
                let y = sin_theta * sin_phi;
                let z = cos_theta;

                Point3::new(x, y, z)
            })
            .collect();

        SamplingPattern::new(points)
    }
}
