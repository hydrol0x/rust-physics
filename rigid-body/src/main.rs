use kiss3d::window::Window;
use kiss3d::{light::Light, nalgebra::Vector3};
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
        // Spring::new_default_spring(&mut window),
        // Spring::new_default_spring(&mut window),
    ];
    // springs[0].length = 9.8;
    // springs[0].dampen = 1.0;
    // springs[0].stiffness = 1000000.0;
    // println!("zeta {}", springs[0].zeta());

    let mut spring_1 = Spring::new_default_spring(&mut window);
    let mut spring_2 = Spring::new_default_spring(&mut window);
    spring_1.node_1.pos = Vector3::new(1.0, 1.0, 1.0);
    spring_1.node_2.pos = Vector3::new(11.5, 1.0, 1.0);
    spring_2.node_1.pos = Vector3::new(1.0, 1.0, 1.0);
    spring_2.node_2.pos = Vector3::new(1.0, 11.0, 1.0);

    println!("{}", spring_1.zeta());
    springs.push(spring_1);
    springs.push(spring_2);

    // springs[1].length = 9.8;

    // Initializes the physics runner
    const ITERS_PER_SEC: f32 = 120.0;
    let previous_time = Instant::now();
    let time_interval = Duration::from_secs_f32(1.0 / ITERS_PER_SEC);
    let accumulator = Duration::new(0, 0);
    let mut runner = DiscreteTimeStepSim::new(previous_time, time_interval, accumulator);

    // Function that does physics calculations
    fn physics_calc(springs: &mut Vec<Spring>, window: &mut Window) {
        for spring in springs {
            spring.step_spring(DT, window);
            // spring.print();
        }
    }

    // Runs physics calculation
    while window.render() {
        runner.run_sim(&mut springs, &mut window, physics_calc)
    }
}
