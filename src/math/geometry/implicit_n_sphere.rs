use std::fmt::Debug;
use std::ops::{Mul, Sub};

use crate::math::{Point, Vector};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ImplicitNSphere<P>
where
    P: Point,
{
    pub center: P,
    pub radius: <P as Point>::ValueType,
}

impl<P> ImplicitNSphere<P>
where
    P: Point,
{
    pub fn new(center: P, radius: <P as Point>::ValueType) -> ImplicitNSphere<P> {
        ImplicitNSphere { center, radius }
    }

    pub fn test(self, point: P) -> <<P as Point>::ValueType as Mul>::Output
    where
        <<P as Point>::ValueType as Mul>::Output:
            Sub<Output = <<P as Point>::ValueType as Mul>::Output>,
    {
        let d = point - self.center;
        d.dot(d) - self.radius * self.radius
    }
}
