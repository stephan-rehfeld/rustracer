use std::fmt::Debug;

pub trait Prefix: Debug + PartialEq + PartialOrd + Clone + Copy {
    const NUMERATOR: u64;
    const DENOMINATOR: u64;
    const PREFIX: &'static str;
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Milli;

impl Prefix for Milli {
    const NUMERATOR: u64 = 1;
    const DENOMINATOR: u64 = 1000;
    const PREFIX: &'static str = "m";
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Centi;

impl Prefix for Centi {
    const NUMERATOR: u64 = 1;
    const DENOMINATOR: u64 = 100;
    const PREFIX: &'static str = "c";
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Deci;

impl Prefix for Deci {
    const NUMERATOR: u64 = 1;
    const DENOMINATOR: u64 = 10;
    const PREFIX: &'static str = "d";
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct None;

impl Prefix for None {
    const NUMERATOR: u64 = 1;
    const DENOMINATOR: u64 = 1;
    const PREFIX: &'static str = "";
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Deca;

impl Prefix for Deca {
    const NUMERATOR: u64 = 10;
    const DENOMINATOR: u64 = 1;
    const PREFIX: &'static str = "da";
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Hecto;

impl Prefix for Hecto {
    const NUMERATOR: u64 = 100;
    const DENOMINATOR: u64 = 1;
    const PREFIX: &'static str = "h";
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Kilo;

impl Prefix for Kilo {
    const NUMERATOR: u64 = 1000;
    const DENOMINATOR: u64 = 1;
    const PREFIX: &'static str = "k";
}
