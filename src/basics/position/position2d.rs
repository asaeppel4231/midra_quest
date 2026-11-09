use crate::basics::safe::float::{safe_div, safe_sub};
use crate::basics::vector::Vect2;

pub type Pos2 = Vect2;

#[derive(Debug, Clone, Copy)]

/* Tile Position 3D makes no sense (tilemaps are 2D only), so we don't need the suffix */
pub struct TPos {
    pub x: u32,
    pub y: u32,
}

impl Pos2 {
    pub fn move_pos(&mut self, mx: f32, my: f32) -> () {
        self.add(Vect2 { x: mx, y: my });
    }

    pub fn x_distance(&self, pos: Pos2) -> f32 {
        safe_sub(0.0, self.x, pos.x)
    }

    pub fn y_distance(&self, pos: Pos2) -> f32 {
        safe_sub(0.0, self.y, pos.y)
    }
}

impl TPos {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn from_pos(tile_width: u16, tile_height: u16, pos: Pos2) -> Self {
        Self {
            x: safe_div(0.0, pos.x, tile_width as f32) as u32,
            y: safe_div(0.0, pos.y, tile_height as f32) as u32,
        }
    }
}
