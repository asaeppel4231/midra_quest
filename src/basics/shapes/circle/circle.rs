use crate::basics::position::Pos2;
use crate::basics::safe::float::safe_mul;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]

/* Also just 2D */
pub struct Circle {
    pub radius: f32,
    pub pos: Pos2,
    pub layer: u8,
}

impl Circle {
    pub fn new(radius: f32, pos: Pos2, layer: u8) -> Self {
        Self {
            radius: radius,
            pos: pos,
            layer: layer,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.radius > 0.0
    }

    pub fn area(&self) -> f32 {
        safe_mul(0.0, PI, safe_mul(0.0, self.radius, self.radius))
    }
}
