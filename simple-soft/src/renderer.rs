use crate::shapes::{Ball, Line};
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
