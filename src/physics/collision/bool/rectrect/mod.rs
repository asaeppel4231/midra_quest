use crate::basics::safe::float::{safe_add, safe_div};
use crate::basics::shapes::Rectangle;
use crate::basics::vector::Vect2;
use crate::basics::vector::distance2;

fn radius_coarse_rectrect_collision(width: f32, height: f32) -> f32 {
    let temp: Vect2;

    temp = Vect2 {
        x: safe_div(0.0, width, 2.0),
        y: safe_div(0.0, height, 2.0),
    };

    temp.length()
}

pub fn coarse_rect_rect(rect1: Rectangle, rect2: Rectangle) -> bool {
    debug_assert!(rect1.layer != rect2.layer);
    debug_assert!(rect1.is_valid());
    debug_assert!(rect2.is_valid());
    let radius_1: f32 = radius_coarse_rectrect_collision(rect1.width, rect1.height);
    let radius_2: f32 = radius_coarse_rectrect_collision(rect2.width, rect2.height);

    let distance: f32 = distance2(rect1.pos, rect2.pos);

    if safe_add(0.0, radius_1, radius_2) > distance {
        return true;
    }
    false
}
