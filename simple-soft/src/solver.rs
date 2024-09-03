use crate::physics::ForceGenerator;
use nalgebra::{vector, Vector2};
#[derive(Debug, Default)]
pub struct EntityState {
    pub velocity: Vector2<f32>,
    pub position: Vector2<f32>,
    pub mass: f32,
}

#[derive(Debug, Default)]
pub struct Derivative {
    dv: Vector2<f32>,
    dx: Vector2<f32>,
}
// https://gafferongames.com/post/integration_basics/

// Runge-Kutta integrator
// x' = f(x,t) = v(t)

// x_k+1 = x_k + dt f_2
// f_1 = f(x_k, t_k) 'euler step'
// f_2 = f(x_k + dt/2, t_k + dt/2)

// pos_k+1 = pos + dt * f_2
// f_1 = dt * v(t)
// f_2 =
// d = pos - f_1(t)
// d.unit() * d.magnitude() / 2. step in half euler
//
// TODO in real simulation, the acceleration function would be calculated based on the sum of the forces on an object

pub enum TimeIntegrator {}

// fn acceleration(state: &State, time: f32) -> Vector2<f32> {
//     let k = 15.0;
//     let b = 0.1;
//     // for mass = 1
//     -k * state.x - b * state.v
// }

pub struct RungeKuttaIntegrator {
    dt: f32,
}

type accelerationFunction = fn(&EntityState, &Vec<Box<dyn ForceGenerator>>, f32) -> Vector2<f32>; // return acceleration given entityState, force states, and current time
type forcesVec = Vec<Box<dyn ForceGenerator>>;

fn calculate_net_acceleration(
    state: &EntityState,
    forces: &Vec<Box<dyn ForceGenerator>>,
    t: f32,
) -> Vector2<f32> {
    let mut net_force = vector![0., 0.];
    for force in forces {
        net_force = force.accumulate(state, &net_force);
    }
    net_force / state.mass
}

impl RungeKuttaIntegrator {
    pub fn new(dt: f32) -> Self {
        Self { dt }
    }
    pub fn evaluate(
        initial: &EntityState,
        derivative: &Derivative,
        forces: &forcesVec,
        t: f32,
        dt: f32,
    ) -> Derivative {
        let mut new_state = EntityState::default();

        new_state.position = initial.position + derivative.dx * dt;
        new_state.velocity = initial.velocity + derivative.dv * dt;

        let mut new_derivative = Derivative::default();
        new_derivative.dx = new_state.velocity;
        new_derivative.dv = calculate_net_acceleration(initial, forces, t + dt);
        new_derivative
    }
    pub fn integrate(
        self: &Self,
        state: &EntityState,
        forces: &forcesVec,
        t: f32,
    ) -> (Vector2<f32>, Vector2<f32>) {
        // f_1 = f(x_k, t_k) = x'
        // f_2 = f(x_k + 0.5dt f_1, t_k + 0.5dt)
        // f_3 = f(x_k + 0.5dt f_2, t_k + 0.5dt)
        // f_4 = f(x_k + dt f_2, t_k + dt)

        // x_k+1 = x_k + 1/6 ( f_1 + 2f_2 + 2f_3 + f_4) * dt

        let f_1 = Self::evaluate(state, &Derivative::default(), forces, t, 0.0);
        let f_2 = Self::evaluate(state, &f_1, forces, t, self.dt / 2.);
        let f_3 = Self::evaluate(state, &f_2, forces, t, self.dt / 2.);
        let f_4 = Self::evaluate(state, &f_3, forces, t, self.dt);

        let x_update = 1. / 6. * (f_1.dx + 2. * f_2.dx + 2. * f_3.dx + f_4.dx) * self.dt;

        let v_update = 1. / 6. * (f_1.dv + 2. * f_2.dv + 2. * f_3.dv + f_4.dv) * self.dt;

        (x_update, v_update)
    }

    pub fn set_dt(self: &mut Self, new_dt: f32) {
        self.dt = new_dt;
    }

    pub fn dt(self: &Self) -> f32 {
        self.dt
    }

    pub fn increase_dt(self: &mut Self) {
        self.dt *= 2.;
    }

    pub fn decrease_dt(self: &mut Self) {
        self.dt /= 2.;
    }
}
