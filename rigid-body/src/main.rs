use kiss3d::light::Light;
use kiss3d::window::Window;
use std::time::{Duration, Instant};

// Spring type
mod spring;
use spring::Spring;

// Physics type
mod physics;
use physics::DiscreteTimeStepSim;

const DT: f32 = 0.00001;

fn main() {
    let mut window = Window::new("Spring Simulation");
    window.set_light(Light::StickToCamera);

    let mut springs: Vec<Spring> = vec![
        Spring::new_default_spring(&mut window),
        // Spring::new_default_spring(&mut window),
    ];
    springs[0].length = 9.5;
    springs[0].dampen = 10.0;
    springs[0].stiffness = 1000000.0;

    // springs[1].length = 9.8;

    // Initializes the physics runner
    const ITERS_PER_SEC: f32 = 60.0;
    let previous_time = Instant::now();
    let time_interval = Duration::from_secs_f32(1.0 / ITERS_PER_SEC);
    let accumulator = Duration::new(0, 0);
    let mut runner = DiscreteTimeStepSim::new(previous_time, time_interval, accumulator);

    // Function that does physics calculations
    fn physics_calc(springs: &mut Vec<Spring>, window: &mut Window) {
        for spring in springs {
            spring.step_spring(DT, window);
        }
    }

    // Runs physics calculation
    while window.render() {
        runner.run_sim(&mut springs, &mut window, physics_calc)
    }
}
