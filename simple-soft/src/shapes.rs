extern crate nalgebra as na;
use macroquad::{color::BLACK, prelude::Color};
use na::Vector2;

pub struct Ball {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub mass: f32,
    pub radius: f32,
}

impl Ball {
    pub fn new(position: Vector2<f32>, velocity: Vector2<f32>, mass: f32, radius: f32) -> Self {
        Self {
            position,
            velocity,
            mass,
            radius,
        }
    }

    pub fn new_default() -> Self {
        Self {
            position: Vector2::new(0.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
            mass: 1.0,
            radius: 10.0,
        }
    }

    pub fn translate_to(&mut self, position: Vector2<f32>) {
        self.position = position;
    }

    pub fn translate_by(&mut self, delta: Vector2<f32>) {
        self.position += delta;
    }
}

pub struct Line {
    pub start_point: Vector2<f32>,
    pub end_point: Vector2<f32>,
    pub color: Color,
}

impl Line {
    pub fn new(start: Vector2<f32>, end: Vector2<f32>) -> Self {
        Self {
            start_point: start,
            end_point: end,
            color: BLACK,
        }
    }

    pub fn translate_by(&mut self, delta: Vector2<f32>) {
        self.start_point += delta;
        self.end_point += delta;
    }

    pub fn translate_to(&mut self, position: Vector2<f32>) {
        let d = self.end_point - self.start_point;
        self.start_point = position;
        self.end_point = position + d;
    }
}
