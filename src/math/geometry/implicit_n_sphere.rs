use std::fmt::Debug;
use std::ops::{Mul, Sub};

use crate::math::{Point, Vector};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ImplicitNSphere<P>
where
    P: Point,
    <P as Point>::ValueType: Copy + Clone + PartialEq + Debug,
{
    pub center: P,
    pub radius: <P as Point>::ValueType,
}

impl<P> ImplicitNSphere<P>
where
    P: Point,
    <P as Point>::ValueType: Copy + Clone + PartialEq + Debug,
{
    pub fn new(center: P, radius: <P as Point>::ValueType) -> ImplicitNSphere<P> {
        ImplicitNSphere { center, radius }
    }

    pub fn test(self, point: P) -> <<P as Point>::ValueType as Mul>::Output
    where
        P: Sub,
        <P as Sub>::Output: Vector + Copy + Clone,
        <P as Point>::ValueType: Mul,
        <<P as Point>::ValueType as Mul>::Output:
            Sub<Output = <<P as Point>::ValueType as Mul>::Output>,
        <<<P as Sub>::Output as Vector>::ValueType as Mul>::Output: Sub<
            <<P as Point>::ValueType as Mul>::Output,
            Output = <<P as Point>::ValueType as Mul>::Output,
        >,
    {
        let d = point - self.center;
        d.dot(d) - self.radius * self.radius
    }
}
