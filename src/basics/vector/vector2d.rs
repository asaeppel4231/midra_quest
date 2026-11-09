use crate::basics::safe::float::{safe_add, safe_div, safe_mul, safe_sub};

#[derive(Debug, Clone, Copy)]
pub struct Vect2 {
    pub x: f32,
    pub y: f32,
}

impl Vect2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn add(&mut self, vect: Vect2) -> () {
        self.x = safe_add(self.x, self.x, vect.x);
        self.y = safe_add(self.y, self.y, vect.y);
    }

    pub fn sub(&mut self, vect: Vect2) -> () {
        self.x = safe_sub(self.x, self.x, vect.x);
        self.y = safe_sub(self.y, self.y, vect.y);
    }

    pub fn length(&self) -> f32 {
        let quad_x = safe_mul(0.0, self.x, self.x);
        let quad_y = safe_mul(0.0, self.y, self.y);

        let result = safe_add(0.0, quad_x, quad_y);

        result.sqrt()
    }

    pub fn scale(&mut self, factor: f32) -> () {
        self.x = safe_mul(self.x, self.x, factor);
        self.y = safe_mul(self.y, self.y, factor);
    }

    pub fn normalize(&mut self) -> () {
        let length: f32;

        length = self.length();

        *self = Vect2 {
            x: safe_div(self.x, self.x, length),
            y: safe_div(self.y, self.y, length),
        };
    }

    pub fn dot_product(&self, vect: Vect2) -> f32 {
        safe_add(
            0.0,
            safe_mul(0.0, self.x, vect.x),
            safe_mul(0.0, self.y, vect.y),
        )
    }

    pub fn cross_product(&self, vect: Vect2) -> f32 {
        safe_sub(
            0.0,
            safe_mul(0.0, self.x, vect.y),
            safe_mul(0.0, self.y, vect.x),
        )
    }

    pub fn reflect(&mut self, normal: Vect2, restitution: f32) -> () {
        let dot_product: f32 = self.dot_product(normal);

        let temp = safe_add(0.0, 1.0, restitution);

        let sub_1 = safe_sub(0.0, self.x, temp);
        let sub_2 = safe_sub(0.0, self.y, temp);

        let mul_1 = safe_mul(0.0, sub_1, dot_product);
        let mul_2 = safe_mul(0.0, sub_2, dot_product);

        let mul_3 = safe_mul(0.0, mul_1, normal.x);
        let mul_4 = safe_mul(0.0, mul_2, normal.y);

        *self = Vect2 { x: mul_3, y: mul_4 }
    }
}

pub fn distance2(start: Vect2, end: Vect2) -> f32 {
    let mut temp: Vect2 = start;

    temp.sub(end);

    temp.length()
}
