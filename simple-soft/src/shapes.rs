extern crate nalgebra as na;
use macroquad::{
    color::{BLACK, WHITE},
    miniquad::gl::WGL_ACCELERATION_ARB,
    prelude::Color,
};
use na::{distance, vector, Vector, Vector2};

#[derive(Debug)]
pub struct Ball {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub force: Vector2<f32>,
    pub mass: f32,
    pub radius: f32,
    pub color: Color,
    pub clicked: bool,
}

impl Ball {
    pub fn new(
        position: Vector2<f32>,
        velocity: Vector2<f32>,
        acceleration: Vector2<f32>,
        force: Vector2<f32>,
        mass: f32,
        radius: f32,
    ) -> Self {
        Self {
            position,
            velocity,
            acceleration,
            force,
            mass,
            radius,
            color: WHITE,
            clicked: false,
        }
    }

    pub fn new_default() -> Self {
        // Self {
        //     position: Vector2::new(0.0, 0.0),
        //     velocity: Vector2::new(0.0, 0.0),
        //     mass: 1.0,
        //     radius: 10.0,
        //     color: WHITE,
        // }
        Self::new(
            Vector2::new(0.0, 0.0),
            Vector2::new(0.0, 0.0),
            vector![0., 0.],
            vector![0., 0.],
            1.0,
            10.0,
        )
    }

    pub fn translate_to(mut self, position: Vector2<f32>) -> Self {
        // todo remove return
        self.position = position;
        self
    }

    pub fn translate_by(&mut self, delta: Vector2<f32>) {
        self.position += delta;
    }
}

#[derive(Debug)]
pub struct Line {
    pub start_point: Vector2<f32>,
    pub end_point: Vector2<f32>,
    pub color: Color,
    pub d: Vector2<f32>,
}

impl Line {
    pub fn new(start: Vector2<f32>, end: Vector2<f32>) -> Self {
        Self {
            start_point: start,
            end_point: end,
            color: BLACK,
            d: end - start,
        }
    }

    pub fn from_vector(vector: Vector2<f32>) -> Self {
        // creates a new line from origin to position specified by vector
        Self {
            start_point: vector![0., 0.],
            end_point: vector,
            color: BLACK,
            d: vector - vector![0., 0.],
        }
    }

    pub fn to_vector(line: &Line) -> Vector2<f32> {
        let dx = line.end_point[0] - line.start_point[0];
        let dy = line.end_point[1] - line.start_point[1];
        vector![dx, dy]
    }

    pub fn translate_by(&mut self, delta: Vector2<f32>) {
        self.start_point += delta;
        self.end_point += delta;
    }

    pub fn translate_to(&mut self, position: Vector2<f32>) {
        let d = self.end_point - self.start_point;
        self.start_point = position;
        self.end_point = position + d;
    }

    pub fn normal(&self) -> Vector2<f32> {
        let dx = self.end_point[0] - self.start_point[0];
        let dy = self.end_point[1] - self.start_point[1];
        Line::to_vector(&Line::new(vector![-dy, dx], vector![dy, -dx]))
    }

    pub fn normal_line(&self) -> Line {
        let dx = self.end_point[0] - self.start_point[0];
        let dy = self.end_point[1] - self.start_point[1];
        Line::new(vector![-dy, dx], vector![dy, -dx])
    }
}

#[derive(Debug)]
pub enum Shape {
    Ball(Ball),
    Line(Line),
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

pub fn ball_point_collision(ball: &Ball, point: &Vector2<f32>, threshold: f32) -> bool {
    let d = point - ball.position;
    if d.magnitude() < ball.radius + threshold {
        return true;
    }
    false
}

pub fn line_line_collision(line_1: &Line, line_2: &Line) -> bool {
    false
}

pub fn point_line_distance(line: &Line, point: &Vector2<f32>) -> f32 {
    // https://stackoverflow.com/a/1501725
    let l2 = line.d.magnitude().powf(2.0);
    if (l2 == 0.0) {
        return (point - line.start_point).magnitude(); // line has 0 length so it's a point, return distance between the two points
    };
    let d_start = point - line.start_point;
    // Consider the line extending the segment, parameterized as v + t (w - v).
    // We find projection of point p onto the line.
    // It falls where t = [(p-v) . (w-v)] / |w-v|^2
    // We clamp t from [0,1] to handle points outside the segment vw.
    let t = (0f32).max((1f32).min(d_start.dot(&line.d) / l2));
    let proj = line.start_point + t * line.d;
    return (point - proj).magnitude(); // distance from point to its projection on the line
}
