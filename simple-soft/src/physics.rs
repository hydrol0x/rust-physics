use std::{cmp, f32::NEG_INFINITY};

use na::{vector, UnitVector2, Vector, Vector2};

use crate::shapes::{Ball, Line};

#[derive(Debug)]
pub struct Collision {
    pub translate_by: Vector2<f32>,
    pub vf: Vector2<f32>,
}
impl Collision {
    pub fn new(translate_by: Vector2<f32>, vf: Vector2<f32>) -> Self {
        Self { translate_by, vf }
    }
}
enum ForceGenerator {
    PointForce(PointForceGenerator),
    GlobalForce(GlobalForceGenerator),
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

pub struct GlobalForceGenerator {
    pub strength: f32,
    pub direction: Vector2<f32>,
}

impl GlobalForceGenerator {
    pub fn new(strength: f32, direction: Vector2<f32>) -> Self {
        Self {
            strength,
            direction: direction.normalize(),
        }
    }

    pub fn force(&self) -> Vector2<f32> {
        self.strength * self.direction
    }
}

pub fn gforce(mass: f32) -> Vector2<f32> {
    let gravity_generator = GlobalForceGenerator::new(9.8, vector![0., 1.]);
    mass * gravity_generator.force()
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

fn elastic_collision_velocity_mag(m_a: f32, m_b: f32, u_a: f32, u_b: f32, c_r: f32) -> (f32, f32) {
    // https://en.wikipedia.org/wiki/Inelastic_collision

    let ma_ua = m_a * u_a;
    let mb_ub = m_b * u_b;
    let mass_recipricol = 1. / (m_a + m_b);

    let v_a = c_r * (m_b * (u_b - u_a) + ma_ua + mb_ub) * mass_recipricol;
    let v_b = c_r * (m_a * (u_a - u_b) + ma_ua + mb_ub) * mass_recipricol;

    (v_a, v_b)
}

pub fn elastic_collision_velocity(ball_a: &Ball, ball_b: &Ball) -> (Vector2<f32>, Vector2<f32>) {
    // I don't know why I didn't do on vectors directly, will change later, idk if the equations will work if you just plug the vectors in.

    let u_a = ball_a.velocity;
    let u_b = ball_b.velocity;

    let m_a = ball_a.mass;
    let m_b = ball_b.mass;

    let c_r = ball_a.elasticity.min(ball_b.elasticity);

    let (va_x, vb_x) = elastic_collision_velocity_mag(m_a, m_b, u_a.x, u_b.y, c_r);
    let (va_y, vb_y) = elastic_collision_velocity_mag(m_a, m_b, u_a.y, u_b.y, c_r);

    let va = vector![va_x, va_y];
    let vb = vector![vb_x, vb_y];

    (va, vb)
}

pub fn wall_collision_velocity(ball: &Ball, line_normal: Vector2<f32>) -> Vector2<f32> {
    // Coefficient of restitution (elasticity)
    let c_r = ball.elasticity;

    // Get the velocity of the ball
    let u_a = ball.velocity;

    // Calculate the relative velocity along the normal
    let relative_velocity = u_a.dot(&line_normal);

    // If the ball is moving towards the line (negative relative velocity)
    if relative_velocity < 0.0 {
        // Split the velocity into normal and tangential components
        let u_a_normal = relative_velocity * line_normal;
        let u_a_tangent = u_a - u_a_normal;

        // Compute the final velocity in the normal direction after collision
        let v_a_normal = -c_r * u_a_normal; // reverse and scale by elasticity

        // The final velocity is the sum of the unchanged tangential component and the reflected normal component
        let final_velocity = u_a_tangent + v_a_normal;

        final_velocity
    } else {
        // If moving away from the line, no collision response is needed
        u_a
    }
}

// pub fn wall_collision_velocity(ball: &Ball, line: &Line, normal: Vector2<f32>) -> Vector2<f32> {
//     // Mass of the ball and line
//     let m_a = ball.mass;
//     let m_b = line.mass; // Assuming the line has a mass property

//     // Coefficient of restitution (elasticity)
//     let c_r = ball.elasticity.min(line.elasticity); // Use the lesser elasticity for the collision

//     // Get the velocity of the ball
//     let u_a = ball.velocity;

//     // Assume the line is stationary for simplicity (velocity is zero)
//     let u_b = vector![0.0, 0.0];

//     // Calculate the relative velocity along the normal
//     let relative_velocity = u_a.dot(&normal);

//     // If the relative velocity is moving towards the line (negative), calculate the collision response
//     if relative_velocity < 0.0 {
//         // Project the velocity onto the normal direction
//         let u_a_normal = relative_velocity * normal;

//         // Use the elastic collision formula for magnitudes
//         let (v_a_mag, _) = elastic_collision_velocity_mag(m_a, m_b, relative_velocity, 0.0, c_r);

//         // Calculate the final velocity of the ball in the normal direction
//         let v_a_normal = v_a_mag * normal;

//         // Calculate the final velocity of the ball by subtracting the change in velocity along the normal
//         let final_velocity = u_a - (u_a_normal - v_a_normal);

//         final_velocity
//     } else {
//         // No collision response needed if moving away from the line
//         u_a
//     }
// }

pub fn collision_position_delta(normal: Vector2<f32>, depth: f32) -> Vector2<f32> {
    let unit_normal = normal.normalize();
    // if depth >= 0. {
    // }
    return unit_normal * depth;
    // vector![0., 0.]
}

pub fn collision_force(normal: Vector2<f32>, ball: &Ball) -> Vector2<f32> {
    let unit_normal = normal.normalize();
    let delta = 2. * unit_normal * ball.velocity.dot(&unit_normal);
    let force = delta / 0.001;

    force
}
