use std::default;
use std::f32::EPSILON;

use constraints::Constraint;
use constraints::ConstraintUpdate;
use constraints::FixedPointConstraint;
use constraints::SpringConstraint;
use physics::collision_position_delta;
use physics::elastic_collision_velocity;
use physics::interpolate_mouse_force;
use physics::SpringForceGenerator;
use solver::EntityState;
use solver::RungeKuttaIntegrator;
extern crate generational_arena;
extern crate nalgebra as na;

mod physics;

mod shapes;

mod solver;

use physics::wall_collision_velocity;
use physics::Collision;
use physics::ForceGenerator;
use physics::ObjectForceGenerator;
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
use physics::{calc_pos, calc_vel};

use ::rand::Rng;

mod constraints;
use constraints::DistanceConstraint;

#[macroquad::main("MyGame")]

// fn create_balls() {
//     Ball()
// }

async fn main() {
    let N = 7; // number of balls
    let MAX_VELOCITY = 1000.;
    let mut dt = 0.1;
    let mut FPS = false;
    let mut GRAVITY = false;

    fn generate_balls(n: u32, shapes: &mut Vec<Shape>) {
        let mut rng = ::rand::thread_rng();
        for i in 0..n {
            let mut ball = Ball::new_default();
            let x: f32 = rng.gen_range(55..=900) as f32;
            let y: f32 = rng.gen_range(55..=900) as f32;
            let vx: f32 = rng.gen_range(-0..=0) as f32;
            let vy: f32 = rng.gen_range(-0..=0) as f32;
            let position = vector![x, y];
            let velocity = vector![vx, vy];
            ball.velocity = velocity;
            ball.elasticity = 0.98;
            shapes.push(Shape::Ball(ball.translate_to(position)));
        }
    }

    let mut constraints: Vec<Constraint> = Vec::new();

    let mut shapes: Vec<Shape> = Vec::new();
    generate_balls(N, &mut shapes);

    let mut collisions: Vec<(usize, Collision)> = Vec::new();

    let top_wall = Line::new(vector![50., 50.], vector![1000., 50.]);
    let left_wall = Line::new(vector![50., 1000.], vector![50., 50.]);
    let right_wall = Line::new(vector![1000., 50.], vector![1000., 1000.]);
    let bottom_wall = Line::new(vector![1000., 1000.], vector![50., 1000.]);

    shapes.push(Shape::Line(bottom_wall));
    shapes.push(Shape::Line(top_wall));
    shapes.push(Shape::Line(left_wall));
    shapes.push(Shape::Line(right_wall));

    let initial_state: Vec<Shape> = shapes.clone();

    let mut ball_focused = false;

    let constraint1 = SpringConstraint {
        index_0: 0,
        index_1: 1,
        distance: 50.,
        k: 50.,
        dampen: 0.1,
    };
    let constraint2 = SpringConstraint {
        index_0: 1,
        index_1: 2,
        distance: 50.,
        k: 50.,
        dampen: 0.1,
    };
    let constraint3 = SpringConstraint {
        index_0: 2,
        index_1: 0,
        distance: 50.,
        k: 50.,
        dampen: 0.1,
    };
    constraints.push(Constraint::Spring(constraint1));
    constraints.push(Constraint::Spring(constraint2));
    constraints.push(Constraint::Spring(constraint3));

    let dist1 = DistanceConstraint::new(4, 5, 40.);
    let dist2 = DistanceConstraint::new(5, 6, 40.);
    let dist3 = DistanceConstraint::new(6, 4, 40.);

    constraints.push(Constraint::Distance(dist1));
    constraints.push(Constraint::Distance(dist2));
    constraints.push(Constraint::Distance(dist3));

    use std::time::Instant;

    let mut t = 0.;
    let mut integrator = RungeKuttaIntegrator::new(dt);
    loop {
        let mut forces: Vec<Box<dyn ForceGenerator>> = Vec::new();
        let now: Instant = Instant::now();
        clear_background(RED);

        let mpos = input::mouse_position();
        let mpoint = vector![mpos.0, mpos.1];

        if input::is_key_down(KeyCode::R) {
            // reset
            shapes = initial_state.clone();
        }
        if input::is_key_pressed(KeyCode::M) {
            integrator.increase_dt();
        }
        if input::is_key_pressed(KeyCode::N) {
            integrator.decrease_dt();
        }
        if input::is_key_pressed(KeyCode::F) {
            FPS = !FPS;
        }
        if input::is_key_pressed(KeyCode::Space) {
            GRAVITY = !GRAVITY;
        }

        for (i, shape) in shapes.iter_mut().enumerate() {
            match shape {
                Shape::Ball(ball) => {
                    // let mut mouse_point_force_generator =
                    //     PointForceGenerator::new(10., vector![0., 0.], i);
                    // mouse_point_force_generator.position = mpoint;
                    // forces.push(Box::new(mouse_point_force_generator));

                    let gravity = ObjectForceGenerator::new(9.8, vector![0., 1.], i);
                    forces.push(Box::new(gravity));
                    if is_mouse_button_down(MouseButton::Left) {
                        if ball.clicked {
                            let force = interpolate_mouse_force(
                                ball.position,
                                mpoint,
                                ball.velocity,
                                30. / integrator.dt(),
                                0.9,
                            );
                            println!("{}", force);
                            ball.velocity = force;
                            // let drag: ObjectForceGenerator =
                            // ObjectForceGenerator::new(0., force.normalize(), i);
                            // forces.push(Box::new(drag));
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

                    render_ball(ball);
                }
                Shape::Line(line) => render_line(&line),
            }
        }

        for (i, obj1) in shapes.iter().enumerate() {
            for (j, obj2) in shapes[i + 1..].iter().enumerate() {
                let shapes_j_index = i + j + 1;
                // collision logic
                match (obj1, obj2) {
                    (Shape::Ball(ball1), Shape::Ball(ball2)) => {
                        // ball 1 is i; ball2 is shapes_j_index
                        if ball_ball_collision(ball1, ball2) {
                            // NOTE: it might be better to calculate everything here and have Collision store only the final values
                            // Then in application loop, all that happens is that ball is translated by pos stored in Collision
                            // And ball velocity is set to whatever is stored in Collision
                            // It would simplify Collision
                            let m_a = ball1.mass;
                            let m_b = ball2.mass;

                            let vi_a = ball1.velocity;
                            let vi_b = ball2.velocity;

                            let c_r = ball1.elasticity.min(ball2.elasticity); // just use the lesser elasticity value kind of a hack

                            // used for normal and for position transform
                            let d = ball2.position - ball1.position;
                            let distance = d.magnitude();
                            if distance < 1e-6 {
                                return;
                            } // Avoid division by zero or very small distances
                            let total_radius = ball1.radius + ball2.radius;
                            let collision_depth = total_radius - distance;
                            let normal = d / distance;

                            let mass_sum = m_a + m_b;
                            let translate_by = collision_position_delta(normal, collision_depth);

                            // Tranlsations are prop. to the masses of the balls - i.e when equal, is equivalnet to just dividing translation equally.
                            let translate_by_a = (translate_by * (m_b / mass_sum));
                            let translate_by_b = (translate_by * (m_a / mass_sum));

                            let (vf_a, vf_b) = elastic_collision_velocity(ball1, ball2);

                            let collision_1 = Collision::new(-translate_by_a, c_r * vf_a);
                            let collision_2 = Collision::new(translate_by_b, c_r * vf_b);

                            collisions.push((i, collision_1));
                            collisions.push((shapes_j_index, collision_2));
                        }
                    }
                    (Shape::Ball(ball), Shape::Line(line)) => {
                        if ball_line_collision(ball, line) {
                            // println!("Collision detected between ball and line!");
                            let mut normal = line.normal();
                            let ball_to_line = line.start_point - ball.position;
                            let c_r = ball.elasticity.min(line.elasticity);
                            let friction = ball.friction.min(line.friction);

                            if ball_to_line.dot(&normal) < 0.0 {
                                // If the normal is facing the wrong way, flip it
                                normal = -normal;
                            }

                            // Calculate the collision depth
                            let collision_depth =
                                ball.radius - point_line_distance(&line, &ball.position);

                            // If collision depth is positive, calculate the translation vector to separate them
                            if collision_depth > 0. {
                                let translate_by_ball =
                                    -collision_position_delta(normal, collision_depth);

                                // Compute the final velocity after collision
                                let vf_ball =
                                    wall_collision_velocity(normal, c_r, friction, dt, ball);

                                // Create a collision object to store translation and velocity updates for the ball
                                let collision = Collision::new(translate_by_ball, vf_ball);

                                // Add collision object for the ball
                                collisions.push((i, collision));
                            }
                        }
                    }
                    (Shape::Line(line), Shape::Ball(ball)) => {
                        if ball_line_collision(ball, line) {
                            // Calculate collision normal (line's normal direction)
                            let mut normal = line.normal();
                            let ball_to_line = line.start_point - ball.position;
                            let c_r = ball.elasticity.min(line.elasticity);
                            let friction = ball.friction.min(line.friction);

                            if ball_to_line.dot(&normal) > 0.0 {
                                // If the normal is facing the wrong way, flip it
                                normal = -normal;
                            }
                            // Calculate the collision depth
                            let collision_depth =
                                ball.radius - point_line_distance(&line, &ball.position);

                            // If collision depth is positive, calculate the translation vector to separate them
                            if collision_depth > 0. {
                                let translate_by_ball =
                                    collision_position_delta(normal, collision_depth);

                                // Compute the final velocity after collision
                                let vf_ball =
                                    wall_collision_velocity(normal, c_r, friction, dt, ball);

                                // Create a collision object to store translation and velocity updates for the ball
                                let collision = Collision::new(translate_by_ball, vf_ball);

                                // Add collision object for the ball
                                collisions.push((shapes_j_index, collision)); // Note the different index here
                            }
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
                    let pos_delta = collision.translate_by;

                    // TODO: remove elasticity from collision, should just be property of lines and balls
                    ball.translate_by(pos_delta);

                    let ball_vel = collision.vf;

                    ball.velocity = ball_vel;
                }

                (Shape::Line(line)) => {
                    // println!("match line");
                }
                _ => {}
            }
        }
        collisions.clear();

        let mut updates: Vec<ConstraintUpdate> = Vec::new();
        for constraint in &constraints {
            match constraint {
                (Constraint::Spring(constraint)) => {
                    if let (Shape::Ball(ball1), Shape::Ball(ball2)) =
                        (&shapes[constraint.index_0], &shapes[constraint.index_1])
                    {
                        // Calculate the delta between ball positions
                        let p0 = ball1.position;
                        let p1 = ball2.position;
                        let v0 = ball1.velocity;
                        let v1 = ball2.velocity;

                        let delta = p1 - p0;
                        let distance = delta.magnitude();
                        let direction = delta / distance;

                        let required_delta = direction * constraint.distance;
                        let force = constraint.k * (required_delta - delta);

                        let mut update_1 = ConstraintUpdate::default();
                        let mut update_2 = ConstraintUpdate::default();

                        // let force = ObjectForceGenerator {
                        //     strength: force.magnitude(),
                        //     direction: force.normalize(),
                        //     entity_idx: constraint.index_0,
                        // };

                        // let force2 = ObjectForceGenerator {
                        //     strength: force.magnitude(),
                        //     direction: force.normalize(),
                        //     entity_idx: constraint.index_0,
                        // };

                        // Update velocities due to spring force
                        update_1.velocity_update = -force * dt / ball1.mass;
                        update_2.velocity_update = force * dt / ball2.mass;

                        // Calculate relative velocity along the direction of the spring
                        let vrel = (v1 - v0).dot(&direction);

                        // Apply damping factor
                        let damping_factor = (-constraint.k * dt).exp();
                        let new_vrel = vrel * damping_factor;
                        let vrel_delta = new_vrel - vrel;
                        let vrel_delta_vec = vrel_delta * direction;

                        // Update velocities due to damping
                        update_1.velocity_update += -vrel_delta_vec / 2.0;
                        update_2.velocity_update += vrel_delta_vec / 2.0;

                        update_1.index = constraint.index_0;
                        update_2.index = constraint.index_1;

                        // let force = SpringForceGenerator {
                        //     b: constraint.dampen,
                        //     k: constraint.k,
                        //     length: constraint.distance,
                        //     displacement: delta / 2.,
                        //     entity_idx: constraint.index_0,
                        // };

                        // let force2 = SpringForceGenerator {
                        //     b: constraint.dampen,
                        //     k: constraint.k,
                        //     length: constraint.distance,
                        //     displacement: -delta / 2.,
                        //     entity_idx: constraint.index_1,
                        // };

                        // forces.push(Box::new(force));
                        // forces.push(Box::new(force2));
                        updates.push(update_1);
                        updates.push(update_2);
                        // Optionally: Draw the spring
                        render_line(&Line::new(ball1.position, ball2.position));
                    }
                }
                (Constraint::Distance(constraint)) => {
                    if let (Shape::Ball(ball1), Shape::Ball(ball2)) =
                        (&shapes[constraint.index_0], &shapes[constraint.index_1])
                    {
                        let delta = ball2.position - ball1.position;
                        let total_correction = delta.magnitude() - constraint.distance;
                        let norm = delta.normalize();
                        let offset = norm * total_correction;

                        let mut update_1 = ConstraintUpdate::default();
                        let mut update_2 = ConstraintUpdate::default();

                        update_1.position_update = offset / 2.;
                        update_2.position_update = -offset / 2.;

                        update_1.index = constraint.index_0;
                        update_2.index = constraint.index_1;

                        updates.push(update_1);
                        updates.push(update_2);
                    }
                }
                (Constraint::FixedPoint(constraint)) => {
                    if let (Shape::Ball(ball)) = &shapes[constraint.index] {
                        let delta = ball.position - constraint.position;
                        let mut update = ConstraintUpdate::default();
                        update.position_update = -delta;
                        update.index = constraint.index;
                        updates.push(update);
                    }
                }
            }
        }

        for update in &updates {
            let shape = &mut shapes[update.index];
            match shape {
                Shape::Ball(ball) => {
                    ball.position += update.position_update;
                    ball.color = BLUE;
                    ball.velocity += update.velocity_update;
                    // ball.force += update.force_update;
                }
                _ => {}
            }
        }
        updates.clear();

        draw_text(
            format!("{}", integrator.dt()).as_str(),
            100.,
            20.0,
            20.0,
            WHITE,
        );
        let elapsed = now.elapsed();
        let mut fps_count = 0;
        fps_count = 1000 / elapsed.as_millis().max(1);
        if FPS {
            draw_text(
                format!("FPS: {}", fps_count).as_str(),
                100.,
                40.0,
                20.0,
                WHITE,
            );
        }

        for force in &forces {
            let idx = force.get_entity_idx();
            // TODO: have trait of shapes be that they have EntityState, then for line make start pos and end pos a fn and have line be defined
            // normally with 'x' in entity state (pos), and then a norm vector and a length, so x is start point, norm is direction of line
            // for v just always 0 uness implement v for lines

            // TODO: set velocity to 0 if very small
            if let Shape::Ball(ball) = &mut shapes[idx] {
                let state = EntityState {
                    velocity: ball.velocity,
                    position: ball.position,
                    mass: ball.mass,
                };

                let (x_update, v_update) = integrator.integrate(&state, &forces, t);
                // println!("x update {:?}", x_update);
                // println!("v update {:?}", v_update);

                ball.velocity += v_update;
                ball.position += x_update;
            };
        }
        t += dt;
        next_frame().await;
    }
}
