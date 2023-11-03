use super::ValueWithPrefixAndUnit;
use super::prefix;

use crate::units::area::SquareMeter;

use std::ops;

#[derive(Debug,PartialEq,PartialOrd,Clone,Copy)]
pub struct MeterUnit;

impl super::Unit for MeterUnit {
    const UNIT: &'static str = "m"; 
}

pub type Meter<T> = ValueWithPrefixAndUnit<T, prefix::None, MeterUnit>;

impl<T: ops::Mul> ops::Mul for Meter<T> {

    type Output = SquareMeter< <T as ops::Mul >::Output>;

    fn mul(self, rhs: Meter<T>) -> Self::Output {
        SquareMeter::new(self.value * rhs.value)
    }
}
