use super::ValueWithPrefixAndUnit;
use super::prefix::None;

use crate::units::area::SquareMeter;

use std::ops::Mul;

#[derive(Debug,PartialEq,PartialOrd,Clone,Copy)]
pub struct MeterUnit;

impl super::Unit for MeterUnit {
    const UNIT: &'static str = "m"; 
}

pub type Meter<T> = ValueWithPrefixAndUnit<T, None, MeterUnit>;

impl<T: Mul> Mul for Meter<T> {

    type Output = SquareMeter< <T as Mul >::Output>;

    fn mul(self, rhs: Meter<T>) -> Self::Output {
        SquareMeter::new(self.value * rhs.value)
    }
}
