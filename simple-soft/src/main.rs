extern crate nalgebra as na;

mod physics;

mod shapes;
use physics::ball_line_collision;
use physics::line_line_norm_component;
use physics::line_norm_component;
use shapes::{Ball, Line};

use macroquad::input;
use macroquad::prelude::*;

mod renderer;
use renderer::render_line;

use na::{vector, Vector2};
use physics::{calc_acceleration, calc_pos, calc_vel, gforce};

#[macroquad::main("MyGame")]

// fn create_balls() {
//     Ball()
// }
async fn main() {
    fn in_bounds(ball: &Ball, min: &Vector2<f32>, max: &Vector2<f32>) -> bool {
        ball.position[0] >= min[0]
            && ball.position[0] <= max[0]
            && ball.position[1] >= min[1]
            && ball.position[1] <= max[1]
    }

    let min = Vector2::new(0.0, 0.0); // Bottom-left corner of the bounding box
    let max = Vector2::new(100.0, 100.0); // Top-right corner of the bounding box

    let dt = 0.01;

    let mut i = 1.0;

    let mut ball_1 = Ball::new_default();
    let mut ball_2 = Ball::new_default();

    ball_1.translate_by(vector![150., 300.]);
    ball_2.translate_by(vector![500., 400.]);
    let mut balls: Vec<Ball> = vec![ball_1, ball_2];

    let mut line = Line::new(vector![0., 0.], vector![100., 50.]);
    let mut line_2 = Line::new(vector![200., 200.], vector![300., 500.]);

    loop {
        // TODO: have a general shapes vector (enum?) and match over the shape type to drawline drawcircle etc
        clear_background(RED);
        let mut collided = false;
        for ball in &mut balls {
            let force = gforce(ball.mass);

            ball.velocity = calc_vel(&ball.velocity, &calc_acceleration(&force, ball.mass), dt);
            ball.position = calc_pos(&ball.position, &ball.velocity, dt);
            draw_circle(ball.position[0], ball.position[1], ball.radius, WHITE);

            // Calculate if at edge
            if !in_bounds(&ball, &min, &max) {}
            if ball_line_collision(&ball, &line) {
                collided = true
            }
        }

        if collided {
            line.color = WHITE;
        } else {
            line.color = BLACK;
        }
        let mpos = input::mouse_position();
        line.translate_to(vector![mpos.0, mpos.1]);
        render_line(&line);

        render_line(&line_2);

        let mut norm = Line::from_vector(-line_line_norm_component(&line, &line_2));
        norm.translate_to(line_2.start_point);
        render_line(&norm);
        line_2.end_point[1] += 0.5;

        i += 1.;

        next_frame().await
    }
}
