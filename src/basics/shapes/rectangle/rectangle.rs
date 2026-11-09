use crate::basics::position::Pos2;
use crate::basics::safe::float::safe_mul;

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub pos: Pos2,
    pub layer: u8,
}

impl Rectangle {
    pub fn new(width: f32, height: f32, rotation: f32, pos: Pos2, layer: u8) -> Self {
        Self {
            width: width,
            height: height,
            rotation: rotation,
            pos: pos,
            layer: layer,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.width > 0.0 && self.height > 0.0
    }

    pub fn area(&self) -> f32 {
        safe_mul(0.0, self.width, self.height)
    }
}
