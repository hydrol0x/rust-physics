use dict::{Dict, DictIface};
use kiss3d::window::Window;
use kiss3d::{light::Light, nalgebra::Vector3};
use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

// Spring type
mod spring;
use spring::Spring;

// Physics type
mod physics;
use physics::DiscreteTimeStepSim;

const DT: f32 = 0.001;

fn main() {
    let mut window = Window::new("Spring Simulation");
    window.set_light(Light::StickToCamera);

    let mut springs: HashMap<u32, Spring> = HashMap::new();
    let spring_0 = Spring::new_default_spring(&mut window);
    let spring_1 = Spring::new_default_spring(&mut window);
    springs.insert(0, spring_0);
    springs.insert(1, spring_1);

    if let Some(spring) = springs.get_mut(&0) {
        spring.length = 9.0;
    }

    // Initializes the physics runner
    const ITERS_PER_SEC: f32 = 120.0;
    let previous_time = Instant::now();
    let time_interval = Duration::from_secs_f32(1.0 / ITERS_PER_SEC);
    let accumulator = Duration::new(0, 0);
    let mut runner = DiscreteTimeStepSim::new(previous_time, time_interval, accumulator);

    // Function that does physics calculations
    fn physics_calc(springs: &mut HashMap<u32, Spring>, window: &mut Window) {
        for (key, spring) in springs {
            spring.step_spring(DT, window);
        }
    }

    // Runs physics calculation
    while window.render() {
        runner.run_sim(&mut springs, &mut window, physics_calc)
    }
}
