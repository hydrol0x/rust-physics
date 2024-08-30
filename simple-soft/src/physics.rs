use na::{vector, Vector, Vector2};

use crate::shapes::{Ball, Line};

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

pub fn ball_line_collision(ball: &Ball, line: &Line) -> bool {
    // https://stackoverflow.com/a/1084899
    let f = line.start_point - ball.position; // Vector from sphere center to line start point
    let d = line.end_point - line.start_point; // Direction vector of the line

    let a = d.dot(&d); // Dot product of direction vector with itself
    let b = 2.0 * f.dot(&d); // 2 * dot product of f and direction vector
    let c = f.dot(&f) - ball.radius * ball.radius; // Dot product of f with itself minus square of radius

    // Discriminant of the quadratic equation
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        // No intersection
        false
    } else {
        // There is a potential intersection
        let discriminant_sqrt = discriminant.sqrt();

        // Calculate both t values for possible intersections
        let t1 = (-b - discriminant_sqrt) / (2.0 * a);
        let t2 = (-b + discriminant_sqrt) / (2.0 * a);

        // Check for the different intersection scenarios
        if (t1 >= 0.0 && t1 <= 1.0) || (t2 >= 0.0 && t2 <= 1.0) {
            // Impale, Poke, or ExitWound
            true
        } else {
            // No valid intersection found
            false
        }
    }
}
