use std::fmt::Debug;
use std::ops::{Mul, Sub};

use crate::math::vector::DotProduct;
use crate::math::Point;

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
        <P as Sub>::Output:
            DotProduct<Output = <<P as Point>::ValueType as Mul>::Output> + Copy + Clone,
        <P as Point>::ValueType: Mul,
        <<P as Point>::ValueType as Mul>::Output:
            Sub<Output = <<P as Point>::ValueType as Mul>::Output>,
    {
        let d = point - self.center;
        d.dot(d) - self.radius * self.radius
    }
}
