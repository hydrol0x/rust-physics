use std::cmp;

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

fn perpendicular_component(a: &Vector2<f32>, b: &Vector2<f32>) -> Vector2<f32> {
    // Calculate the projection of `a` onto `b`
    let proj_a_on_b = (a.dot(b) / b.dot(b)) * b;

    // Calculate the perpendicular component
    a - proj_a_on_b
}

pub fn line_norm_component(vector: &Vector2<f32>, line: &Line) -> Vector2<f32> {
    // Return the component of `vector` that is perpendicular to the line (in direction of the lines norm)
    let d = line.end_point - line.start_point;
    perpendicular_component(&vector, &d)
}
pub fn line_line_norm_component(line_1: &Line, line_2: &Line) -> Vector2<f32> {
    // Return the component of `line_1` that is perpendicular to the line_2 (in direction of the lines norm)
    let d = &line_1.end_point - &line_1.start_point;
    line_norm_component(&d, line_2)
}

pub fn ball_ball_collision(ball_1: &Ball, ball_2: &Ball) -> bool {
    let d = ball_2.position - ball_1.position;
    if d.magnitude() < (2.0 * ball_1.radius.max(ball_2.radius)) {
        return true;
    }
    false
}
