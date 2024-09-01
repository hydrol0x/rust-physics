use std::default;

use na::constraint;
use physics::interpolate_mouse_force;
use physics::point_force;
use physics::wall_collision_position_delta;
use physics::wall_collision_velocity;
extern crate nalgebra as na;

mod physics;

mod shapes;

use physics::collision_force;
use physics::Collision;
use physics::PointForceGenerator;
use renderer::render_ball;
use renderer::render_point_force_generator;
use shapes::ball_ball_collision;
use shapes::ball_line_collision;
use shapes::ball_point_collision;
use shapes::line_line_collision;
use shapes::point_line_distance;
use shapes::{Ball, Line, Shape};

use macroquad::input;
use macroquad::prelude::*;

mod renderer;
use renderer::render_line;

use na::{vector, Vector2};
use physics::{calc_pos, calc_vel, gforce};

use ::rand::Rng;

mod constraints;
use constraints::DistanceConstraint;

#[macroquad::main("MyGame")]

// fn create_balls() {
//     Ball()
// }

async fn main() {
    fn generate_balls(n: u32, shapes: &mut Vec<Shape>) {
        let mut rng = ::rand::thread_rng();
        for i in 0..n {
            let mut ball = Ball::new_default();
            let x: f32 = rng.gen_range(55..=400) as f32;
            let y: f32 = rng.gen_range(55..=400) as f32;
            let vx: f32 = rng.gen_range(-0..=0) as f32;
            let vy: f32 = rng.gen_range(-0..=0) as f32;
            let position = vector![x, y];
            let velocity = vector![vx, vy];
            ball.velocity = velocity;
            ball.elasticity = 0.98;
            shapes.push(Shape::Ball(ball.translate_to(position)));
        }
    }
    let dt = 0.1;

    let mut constraints: Vec<DistanceConstraint> = Vec::new();

    let mut shapes: Vec<Shape> = Vec::new();
    generate_balls(3, &mut shapes);
    shapes.push(Shape::Ball(
        Ball::new_default().translate_to(vector![100., 100.]),
    ));
    shapes.push(Shape::Ball(
        Ball::new_default().translate_to(vector![110., 100.]),
    ));
    let mut collisions: Vec<(usize, Collision)> = Vec::new();

    let constraint = DistanceConstraint {
        index_0: 0,
        index_1: 1,
        distance: 40.,
    };
    let constraint2 = DistanceConstraint {
        index_0: 1,
        index_1: 2,
        distance: 40.,
    };
    let constraint3 = DistanceConstraint {
        index_0: 2,
        index_1: 3,
        distance: 40.,
    };
    let constraint4 = DistanceConstraint {
        index_0: 3,
        index_1: 0,
        distance: 40.,
    };
    constraints.push(constraint);
    constraints.push(constraint2);

    let top_wall = Line::new(vector![50., 50.], vector![500., 50.]);
    let left_wall = Line::new(vector![50., 500.], vector![50., 50.]);
    let right_wall = Line::new(vector![500., 50.], vector![500., 500.]);
    let bottom_wall = Line::new(vector![500., 500.], vector![50., 500.]);

    shapes.push(Shape::Line(bottom_wall));
    shapes.push(Shape::Line(top_wall));
    shapes.push(Shape::Line(left_wall));
    shapes.push(Shape::Line(right_wall));
    // ball_2.velocity = vector![0., -30.];

    // println!("{}", point_line_distance(line, point));
    // println!("{:?}", collisions);
    let mut ball_focused = false;

    let mut mouse_point_force_generator = PointForceGenerator::new(10., vector![0., 0.]);
    loop {
        // println!("{:?}", collisions);
        // TODO: have a general shapes vector (enum?) and match over the shape type to drawline drawcircle etc
        clear_background(RED);
        let mpos = input::mouse_position();
        let mpoint = vector![mpos.0, mpos.1];
        mouse_point_force_generator.position = mpoint;
        for shape in &mut shapes {
            match shape {
                Shape::Ball(ball) => {
                    ball.force = vector![0., 0.];
                    let mut mouse_point_force = vector![0., 0.];
                    if is_mouse_button_down(MouseButton::Right) {
                        mouse_point_force =
                            point_force(&ball.position, &mouse_point_force_generator);
                        render_point_force_generator(&mouse_point_force_generator);
                    }

                    if input::is_key_down(KeyCode::Space) {
                        ball.force = vector![0., 0.];
                    } else {
                        // ball.force += gforce(ball.mass) + mouse_point_force;
                        ball.force += mouse_point_force;
                    }
                    ball.acceleration = ball.force / ball.mass;
                    let mut vel = calc_vel(&ball.velocity, &ball.acceleration, dt);
                    if vel.magnitude() < 0.5 {
                        vel = vector![0., 0.];
                    }
                    ball.velocity = vel;
                    ball.position = calc_pos(&ball.position, &ball.velocity, dt);
                    render_ball(ball);

                    // if is_mouse_button_down(MouseButton::Right) && ball.clicked {
                    //     ball.position = mpoint;
                    //     ball.velocity = vector![0., 0.];
                    //     ball.force = vector![0., 0.];
                    // }

                    if is_mouse_button_down(MouseButton::Left) {
                        if ball.clicked {
                            // ball.velocity = vector![0., 0.];

                            ball.velocity = interpolate_mouse_force(
                                ball.position,
                                mpoint,
                                ball.velocity,
                                300.,
                                0.9,
                            )
                        } else if !ball_focused && ball_point_collision(&ball, &mpoint, 20.0) {
                            ball.clicked = true;
                            ball_focused = true;
                            ball.position = mpoint;
                            ball.color = BLACK;
                        }
                    } else {
                        ball.clicked = false;
                        ball_focused = false;
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
                            let d = ball2.position - ball1.position;
                            let total_radius = ball1.radius + ball2.radius;
                            let collision_depth = d.magnitude() / 2.;
                            let normal = d.normalize();
                            let collision_1 =
                                Collision::new(normal, collision_depth / 2., ball1.elasticity);
                            let collision_2 =
                                Collision::new(-normal, collision_depth / 2., ball1.elasticity);
                            collisions.push((i, collision_1));
                            collisions.push((i + j + 1, collision_2));
                        }
                    }
                    (Shape::Ball(ball), Shape::Line(line))
                    | (Shape::Line(line), Shape::Ball(ball)) => {
                        if ball_line_collision(ball, line) {
                            // println!("Collision detected between ball and line!");

                            let vel = &ball.velocity;

                            let collision_depth =
                                ball.radius - point_line_distance(&line, &ball.position);
                            let collision =
                                Collision::new(line.normal(), collision_depth, ball.elasticity);
                            let collision_2 =
                                Collision::new(-line.normal(), collision_depth, ball.elasticity);
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

                    // TODO: add properties to each object for elasticity and friction coeff. so that this can be updated, and add these to Collision
                    // TODO: change these function names to not be wall but just general collision since its not only wall
                    let pos_delta = wall_collision_position_delta(&collision);

                    // TODO: remove elasticity from collision, should just be property of lines and balls
                    ball.translate_by(pos_delta);
                    let ball_vel = collision.elasticity * wall_collision_velocity(&collision, ball);

                    ball.velocity = ball_vel;
                }

                (Shape::Line(line)) => {
                    // println!("match line");
                }
                _ => {}
            }
        }
        collisions.clear();

        let mut offsets: Vec<(usize, Vector2<f32>)> = Vec::new();
        for constraint in &constraints {
            let p0 = &shapes[constraint.index_0];
            let p1 = &shapes[constraint.index_1];
            match (p0, p1) {
                (Shape::Ball(ball1), Shape::Ball(ball2)) => {
                    let delta = ball2.position - ball1.position;
                    let total_correction = delta.magnitude() - constraint.distance;
                    let norm = delta.normalize();
                    let offset = norm * total_correction;
                    // let distance = delta.magnitude();

                    // let required_delta = delta * (constraint.distance / distance);
                    // let offset = delta - required_delta;
                    // println!(
                    //     "delta {} \n distance {} \n constraint distance {}",
                    //     delta, distance, constraint.distance
                    // );
                    offsets.push((constraint.index_0, offset / 2.));
                    offsets.push((constraint.index_1, -offset / 2.));
                }
                _ => {}
            }
        }

        for (i, offset) in &offsets {
            let shape = &mut shapes[*i];
            match shape {
                Shape::Ball(ball) => {
                    // println!("{}", offset);
                    ball.position += offset;
                }
                _ => {}
            }
        }
        offsets.clear();

        next_frame().await;
    }
}
