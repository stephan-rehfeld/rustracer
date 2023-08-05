use std::marker::PhantomData;

trait Angle<T> {
    fn to_degrees(self) -> Degrees<T>;
    fn to_radians(self) -> Radians<T>;
}

// Add
// AddAsign
// Sub
// SubAssign
// Mul T
// MulAssign T
// Div T
// DivAssign T
// Div -> T
// Neg
// Default
// Display

// acos
// acosh
// asin
// asinh
// atan
// atan2
// atanh
// cos
// cosh
// sin
// sin_cos
// sinh
// tan
// tanh
// to_degrees
// to_radians

type Degrees<T> = ValueWithPrefixAndUnit<T, None, DegreesUnit>;
type Radians<T> = ValueWithPrefixAndUnit<T, None, RadiansUnit>;

trait Unit {
    const UNIT: &'static str; 
}

struct DegreesUnit;

impl Unit for DegreesUnit {
    const UNIT: &'static str = "°"; 
}

struct RadiansUnit;

impl Unit for RadiansUnit {
    const UNIT: &'static str = "rad"; 
}

#[derive(Debug,PartialEq,PartialOrd,Clone,Copy)]
struct ValueWithPrefixAndUnit<T, P: Prefix, U: Unit> {
    value: T,
    _prefix: PhantomData<P>,
    _unit: PhantomData<U>,
} 

impl<T, P: Prefix, U: Unit> ValueWithPrefixAndUnit<T, P, U> {
    fn new(value: T) -> ValueWithPrefixAndUnit<T, P, U> {
        ValueWithPrefixAndUnit { value: value, _prefix: PhantomData, _unit: PhantomData }
    }
}

trait Prefix {
    const NUMERATOR: u64;
    const DENOMINATOR: u64;
    const PREFIX: &'static str;
}

struct Milli;

impl Prefix for Milli {
    const NUMERATOR: u64 = 1;
    const DENOMINATOR: u64 = 1000;
    const PREFIX: &'static str = "m";
}


struct Centi;

impl Prefix for Centi {
    const NUMERATOR: u64 = 1;
    const DENOMINATOR: u64 = 100;
    const PREFIX: &'static str = "c";

}

struct Deci;

impl Prefix for Deci {
    const NUMERATOR: u64 = 1;
    const DENOMINATOR: u64 = 10;
    const PREFIX: &'static str = "d";

}

struct None;

impl Prefix for None {
    const NUMERATOR: u64 = 1;
    const DENOMINATOR: u64 = 1;
    const PREFIX: &'static str = "";
}

struct Deca;

impl Prefix for Deca {
    const NUMERATOR: u64 = 10;
    const DENOMINATOR: u64 = 1;
    const PREFIX: &'static str = "da";
}

struct Hecto;

impl Prefix for Hecto {
    const NUMERATOR: u64 = 100;
    const DENOMINATOR: u64 = 1;
    const PREFIX: &'static str = "h";
}

struct Kilo;

impl Prefix for Kilo {
    const NUMERATOR: u64 = 1000;
    const DENOMINATOR: u64 = 1;
    const PREFIX: &'static str = "k";
}
