use crate::basics::position::Pos2;
use crate::basics::vector::distance2;

#[derive(Debug, Clone, Copy)]

pub struct Line2 {
    pub start: Pos2,
    pub end: Pos2,
    pub length: f32,
    pub layer: u8,
}

impl Line2 {
    pub fn new(start: Pos2, end: Pos2, length: f32, layer: u8) -> Self {
        Self {
            start: start,
            end: end,
            length: length,
            layer: layer,
        }
    }

    pub fn update_length(&mut self) -> () {
        self.length = distance2(self.start, self.end);
    }
}
