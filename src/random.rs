use std::time::{SystemTime, UNIX_EPOCH};

pub trait RandomNumberGenerator<T> {
    fn next_random(&mut self) -> T;
}

pub struct WichmannHillPRNG {
    s1: u32,
    s2: u32,
    s3: u32,
}

impl WichmannHillPRNG {
    pub fn new(s1: u32, s2: u32, s3: u32) -> WichmannHillPRNG {
        WichmannHillPRNG { s1, s2, s3 }
    }

    pub fn from_seed(seed: u64) -> WichmannHillPRNG {
        let s1 = seed % 30296;
        let s2 = seed % 30307;
        let s3 = seed % 30323;

        WichmannHillPRNG::new(s1 as u32, s2 as u32, s3 as u32)
    }

    pub fn new_random() -> WichmannHillPRNG {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        WichmannHillPRNG::from_seed(current_time)
    }
}

impl RandomNumberGenerator<f32> for WichmannHillPRNG {
    fn next_random(&mut self) -> f32 {
        let s1 = self.s1 as f32;
        let s2 = self.s2 as f32;
        let s3 = self.s3 as f32;
        let random_number = (s1 / 30296.0 + s2 / 30307.0 + s3 / 30323.0) % 1.0;

        self.s1 = (171 * self.s1) % 30269;
        self.s2 = (172 * self.s1) % 30307;
        self.s3 = (170 * self.s1) % 30323;

        random_number
    }
}

impl RandomNumberGenerator<f64> for WichmannHillPRNG {
    fn next_random(&mut self) -> f64 {
        let s1 = self.s1 as f64;
        let s2 = self.s2 as f64;
        let s3 = self.s3 as f64;
        let random_number = (s1 / 30296.0 + s2 / 30307.0 + s3 / 30323.0) % 1.0;

        self.s1 = (171 * self.s1) % 30269;
        self.s2 = (172 * self.s1) % 30307;
        self.s3 = (170 * self.s1) % 30323;

        random_number
    }
}

impl RandomNumberGenerator<usize> for WichmannHillPRNG {
    fn next_random(&mut self) -> usize {
        (<WichmannHillPRNG as RandomNumberGenerator<f64>>::next_random(self) * (usize::MAX as f64))
            as usize
    }
}

// usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128
