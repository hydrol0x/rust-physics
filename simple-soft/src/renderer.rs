use crate::{
    physics::PointForceGenerator,
    shapes::{Ball, Line},
};
use macroquad::prelude::*;

pub fn render_line(line: &Line) {
    draw_line(
        line.start_point[0],
        line.start_point[1],
        line.end_point[0],
        line.end_point[1],
        5.,
        line.color,
    );
}

pub fn render_ball(ball: &Ball) {
    draw_circle(ball.position[0], ball.position[1], ball.radius, ball.color);
}
pub fn render_point_force_generator(generator: &PointForceGenerator) {
    draw_circle(
        generator.position[0],
        generator.position[1],
        15.,
        Color {
            r: 1.,
            g: 1.,
            b: 1.,
            a: 0.5,
        },
    );
    draw_circle(
        generator.position[0],
        generator.position[1],
        10.,
        Color {
            r: 1.,
            g: 1.,
            b: 1.,
            a: 0.2,
        },
    );
    draw_circle(
        generator.position[0],
        generator.position[1],
        8.,
        Color {
            r: 0.9,
            g: 0.9,
            b: 1.,
            a: 0.4,
        },
    );
    draw_circle(
        generator.position[0],
        generator.position[1],
        5.,
        Color {
            r: 0.2,
            g: 0.2,
            b: 1.,
            a: 0.5,
        },
    );
}
