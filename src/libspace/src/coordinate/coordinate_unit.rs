pub use glam::f64::*;
use std::ops::Mul;

#[derive(Copy, Clone, Debug)]
pub enum CoordinateUnit {
    Meter,
    Mile,
    KiloMeter,
    MegaMeter,
    Au,
    Parsec,
}

impl CoordinateUnit {
    pub fn factor_from_base(&self) -> f64 {
        match self {
            CoordinateUnit::Meter => 1.0,
            CoordinateUnit::Mile => 1609.344,
            CoordinateUnit::KiloMeter => 1000.0,
            CoordinateUnit::MegaMeter => 1e6,
            CoordinateUnit::Au => 149597900000.0,
            CoordinateUnit::Parsec => 30856780000000000.0,
        }
    }

    pub fn factor_to(&self, other: Self) -> f64 {
        let mine = self.factor_from_base();
        let other = other.factor_from_base();
        mine / other
    }

    pub fn to<T>(&self, other: Self, value: &T) -> T
    where
        T: Mul<f64, Output = T> + Copy,
    {
        let factor = self.factor_to(other);
        *value * factor
    }
}
