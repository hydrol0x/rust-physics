use std::{cmp, f32::NEG_INFINITY};

use na::{vector, Vector, Vector2};

use crate::shapes::{Ball, Line};

#[derive(Debug)]
pub struct Collision {
    pub normal: Vector2<f32>,
    pub depth: f32,
}

pub struct PointForceGenerator {
    // acts as an attractive or repulsive force of charge `strength` and assuming all objects have a 'charge' of 1
    pub strength: f32,
    pub position: Vector2<f32>,
}

impl PointForceGenerator {
    pub fn new(strength: f32, position: Vector2<f32>) -> Self {
        Self { strength, position }
    }
}

pub fn point_force(point: &Vector2<f32>, force_generator: &PointForceGenerator) -> Vector2<f32> {
    // 1/r dropoff
    let d = force_generator.position - point;
    let unit = d.normalize();
    let force = force_generator.strength;
    force * unit
}

impl Collision {
    pub fn new(normal: Vector2<f32>, depth: f32) -> Self {
        Self {
            normal: normal,
            depth: depth, // depth of NEG INF represents no collision
        }
    }
    pub fn new_default() -> Self {
        Self {
            normal: vector![0., 0.],
            depth: NEG_INFINITY, // depth of NEG INF represents no collision
        }
    }
}

pub fn gforce(mass: f32) -> Vector2<f32> {
    (9.8 * mass) * vector![0.0, 1.0] // in this, down is positive
}

pub fn calc_acceleration(force: &Vector2<f32>, mass: f32) -> Vector2<f32> {
    force / mass
}

pub fn calc_vel(
    current_velocity: &Vector2<f32>,
    acceleration: &Vector2<f32>,
    dt: f32,
) -> Vector2<f32> {
    current_velocity + acceleration * dt
}

pub fn calc_pos(current_pos: &Vector2<f32>, vel: &Vector2<f32>, dt: f32) -> Vector2<f32> {
    current_pos + vel * dt
}

pub fn interpolate_mouse_force(
    ball_position: Vector2<f32>,
    mouse_position: Vector2<f32>,
    current_force: Vector2<f32>,
    max_speed: f32,
    damping: f32,
) -> Vector2<f32> {
    let direction = mouse_position - ball_position;
    let distance = direction.magnitude();

    // Normalize the direction
    let normalized_direction = if distance != 0.0 {
        direction / distance
    } else {
        Vector2::zeros()
    };

    // Interpolating force towards the direction of the mouse
    let desired_speed = max_speed * (1.0 - (-distance / 100.0).exp()); // An exponential function to control acceleration
    let desired_force = normalized_direction * desired_speed;

    // Apply damping to current force for smooth deceleration
    current_force + (desired_force - current_force) * damping
}

pub fn wall_collision_velocity(collision: &Collision, ball: &Ball) -> Vector2<f32> {
    let normal = collision.normal;
    let unit_normal = normal.normalize();
    let depth = collision.depth;
    let delta = unit_normal * ball.velocity.dot(&unit_normal);
    ball.velocity - 2. * delta
}

pub fn wall_collision_position_delta(collision: &Collision) -> Vector2<f32> {
    let normal = collision.normal;
    let unit_normal = normal.normalize();
    let mut depth = collision.depth;

    -unit_normal * (depth)
}
