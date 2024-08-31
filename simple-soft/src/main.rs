use camera::mouse;
use circular_buffer::CircularBuffer;
use physics::interpolate_mouse_force;
use std::collections::HashSet;
extern crate nalgebra as na;

mod physics;

mod shapes;
use std::hint::black_box;
use std::thread::sleep;

use physics::Collision;
use renderer::render_ball;
use shapes::ball_ball_collision;
use shapes::ball_line_collision;
use shapes::ball_point_collision;
use shapes::line_line_collision;
use shapes::line_line_norm_component;
use shapes::line_norm_component;
use shapes::point_line_distance;
use shapes::{Ball, Line, Shape};

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
    let dt = 0.1;

    let mut shapes: Vec<Shape> = Vec::new();

    let mut collisions: Vec<(usize, Collision)> = Vec::new();

    let top_wall = Line::new(vector![50., 50.], vector![500., 50.]);
    let left_wall = Line::new(vector![50., 500.], vector![50., 50.]);
    let right_wall = Line::new(vector![500., 50.], vector![500., 500.]);
    let bottom_wall = Line::new(vector![500., 500.], vector![50., 500.]);

    let mut ball_1 = Ball::new_default();
    let mut ball_2 = Ball::new_default();

    shapes.push(Shape::Line(bottom_wall));
    shapes.push(Shape::Line(top_wall));
    shapes.push(Shape::Line(left_wall));
    shapes.push(Shape::Line(right_wall));
    // ball_2.velocity = vector![0., -30.];
    let ball = Shape::Ball(ball_2.translate_to(vector![100., 80.]));
    shapes.push(ball);

    let point = vector![0., 0.];
    let line = Line::new(vector![-2., -10.], vector![-2., -10.]);

    // println!("{}", point_line_distance(line, point));
    // println!("{:?}", collisions);
    let mut mouse_delta_buf = CircularBuffer::<5, Vector2<f32>>::new();
    mouse_delta_buf.fill_with(|| vector![0., 0.]);
    loop {
        // println!("{:?}", collisions);
        // TODO: have a general shapes vector (enum?) and match over the shape type to drawline drawcircle etc
        clear_background(RED);
        let mpos = input::mouse_position();
        let mpoint = vector![mpos.0, mpos.1];
        let mvel = input::mouse_delta_position() * 0.1;
        let mvel_vec = vector![mvel[0], mvel[1]];
        mouse_delta_buf.push_back(mvel_vec);
        for shape in &mut shapes {
            match shape {
                Shape::Ball(ball) => {
                    // let force = gforce(ball.mass);
                    // let force = vector![0., 0.];

                    ball.acceleration = ball.force / ball.mass;
                    ball.velocity = calc_vel(&ball.velocity, &ball.acceleration, dt);
                    ball.position = calc_pos(&ball.position, &ball.velocity, dt);
                    render_ball(ball);

                    if is_mouse_button_down(MouseButton::Right) && ball.clicked {
                        ball.position = mpoint;
                        ball.velocity = vector![0., 0.];
                        ball.force = vector![0., 0.];
                    }

                    if is_mouse_button_down(MouseButton::Left) {
                        if ball.clicked {
                            // ball.velocity = vector![0., 0.];
                            let sum_mouse_deltas: Vector2<f32> = mouse_delta_buf.iter().sum();
                            let average_mouse_delta = sum_mouse_deltas / 10.; // hardcoded buffer size since the .capacity() returns usize but need f32

                            ball.velocity = interpolate_mouse_force(
                                ball.position,
                                mpoint,
                                ball.velocity,
                                100.,
                                0.9,
                            )
                        } else if ball_point_collision(&ball, &mpoint, 20.0) {
                            ball.clicked = true;
                            ball.position = mpoint;
                            ball.color = BLACK;
                        }
                    } else {
                        ball.clicked = false;
                        ball.force = vector![0., 0.];
                        ball.color = WHITE;
                    }

                    // if ball_point_collision(&ball, &mpoint) {
                    //     if is_mouse_button_down(MouseButton::Left) {
                    //         ball.position = mpoint;
                    //     }
                    //     ball.color = BLACK;
                    // } else {
                    //     ball.color = WHITE;
                    // }
                }
                Shape::Line(line) => render_line(&line),
            }
        }

        for (i, obj1) in shapes.iter().enumerate() {
            for (j, obj2) in shapes[i + 1..].iter().enumerate() {
                // collision logic
                match (obj1, obj2) {
                    (Shape::Ball(ball1), Shape::Ball(ball2)) => {
                        if ball_ball_collision(ball1, ball2) {
                            // Handle collision
                        }
                    }
                    (Shape::Ball(ball), Shape::Line(line))
                    | (Shape::Line(line), Shape::Ball(ball)) => {
                        if ball_line_collision(ball, line) {
                            // println!("Collision detected between ball and line!");

                            let vel = &ball.velocity;
                            let perp_component = line_norm_component(vel, line);

                            let collision_depth = point_line_distance(&line, &ball.position);
                            let collision = Collision::new(line.normal(), collision_depth);
                            let collision_2 = Collision::new(line.normal(), collision_depth);
                            collisions.push((i, collision));
                            collisions.push((i + j + 1, collision_2));
                        }
                    }
                    (Shape::Line(line1), Shape::Line(line2)) => {
                        if line_line_collision(line1, line2) {
                            // Handle collision
                        }
                    }
                    _ => {}
                }
            }
        }

        for (i, collision) in &collisions {
            match (&mut shapes[*i]) {
                (Shape::Ball(ball)) => {
                    // ball.translate_by(10. * unit);
                    // if &ball.velocity.dot(&unit) < &0. {
                    //     ball.velocity = ball.velocity - 2. * *value;
                    // }

                    let normal = collision.normal;
                    let unit_normal = normal.normalize();
                    let depth = collision.depth;
                    ball.translate_by(-unit_normal * depth);
                    let delta = unit_normal * ball.velocity.dot(&unit_normal);
                    let new_velocity = ball.velocity - 2. * delta;
                    ball.velocity = new_velocity;
                }

                (Shape::Line(line)) => {
                    // println!("match line");
                }
                _ => {}
            }
        }
        collisions.clear();
        next_frame().await;
    }
}
