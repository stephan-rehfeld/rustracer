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
