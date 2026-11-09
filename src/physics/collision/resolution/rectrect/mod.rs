use crate::basics::safe::float::{safe_add, safe_div, safe_mul, safe_sub};
use crate::basics::shapes::Rectangle;
use crate::basics::vector::Vect2;

/* Rust port of rectrect.c's narrow_rect_rect_collision. */

pub fn narrow_rect_rect_collision(rect1: Rectangle, rect2: Rectangle) -> Option<Vect2> {
    let mut new_axes = [Vect2::new(0.0, 0.0); 4];

    new_axes[0] = Vect2 {
        x: rect1.rotation.cos(),
        y: rect1.rotation.sin(),
    };

    new_axes[1] = Vect2 {
        x: rect2.rotation.cos(),
        y: rect2.rotation.sin(),
    };

    new_axes[2] = Vect2 {
        x: -new_axes[0].y,
        y: new_axes[0].x,
    };

    new_axes[3] = Vect2 {
        x: -new_axes[1].y,
        y: new_axes[1].x,
    };

    for i in 0..4 {
        new_axes[i].normalize();
    }

    let mut mtv: Vect2 = Vect2 { x: 0.0, y: 0.0 };
    let mut min_overlap: f32 = f32::MAX;
    let mut center_distance: Vect2;

    center_distance = rect2.pos;
    center_distance.sub(rect1.pos);

    for i in 0..4 {
        let r1_x_dot_prod: f32 = new_axes[0].dot_product(new_axes[i]);
        let r1_y_dot_prod: f32 = new_axes[2].dot_product(new_axes[i]);

        let r1_x_proj: f32 = safe_mul(0.0, r1_x_dot_prod, safe_div(0.0, rect1.width, 2.0)).abs();
        let r1_y_proj: f32 = safe_mul(0.0, r1_y_dot_prod, safe_div(0.0, rect1.height, 2.0)).abs();

        let r2_x_dot_prod: f32 = new_axes[1].dot_product(new_axes[i]);
        let r2_y_dot_prod: f32 = new_axes[3].dot_product(new_axes[i]);

        let r2_x_proj: f32 = safe_mul(0.0, r2_x_dot_prod, safe_div(0.0, rect2.width, 2.0));
        let r2_y_proj: f32 = safe_mul(0.0, r2_y_dot_prod, safe_div(0.0, rect2.height, 2.0));

        let half_rect1_region: f32 = safe_add(0.0, r1_x_proj, r1_y_proj);
        let half_rect2_region: f32 = safe_add(0.0, r2_x_proj, r2_y_proj);

        let distance = center_distance.dot_product(new_axes[i]);

        if distance >= safe_add(0.0, half_rect1_region, half_rect2_region) {
            /* No collision possible */
            return None;
        }

        let overlap = safe_sub(
            0.0,
            safe_add(0.0, half_rect1_region, half_rect2_region),
            distance,
        );

        if overlap < min_overlap {
            min_overlap = overlap;
            mtv = new_axes[i];
        }
    }

    /* If we are here, there IS an collision. */
    if center_distance.dot_product(mtv) < 0.0 {
        mtv.scale(-1.0);
    }

    mtv.scale(min_overlap);

    Some(mtv)
}
