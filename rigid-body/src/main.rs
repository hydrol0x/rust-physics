use kiss3d::light::Light;
use kiss3d::window::Window;
use std::thread::sleep;
use std::time::{Duration, Instant};

mod spring;
use spring::Spring;

mod physics;
use physics::discrete_time_step_sim;

const dt: f32 = 0.00001;

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    window.set_light(Light::StickToCamera);

    let mut springs: Vec<Spring> = vec![
        // Spring::new_default_spring(&mut window),
        Spring::new_default_spring(&mut window),
    ];
    springs[0].length = 9.5;
    springs[0].dampen = 10.0;
    springs[0].stiffness = 1000000.0;

    // springs[1].length = 9.8;

    const ITERS_PER_SEC: f32 = 60.0;

    let mut previous_time = Instant::now();
    let time_interval = Duration::from_secs_f32(1.0 / 60.0); // Assuming 60 FPS for the time interval, adjust as needed
    let mut accumulator = Duration::new(0, 0);

    fn physics_calc(springs: &mut Vec<Spring>, window: &mut Window) {
        for spring in springs {
            spring.step_spring(dt, window);
        }
    }

    let mut runner = discrete_time_step_sim::new(previous_time, time_interval, accumulator);
    while window.render() {
        runner.run_sim(&mut springs, &mut window, physics_calc)
        // let current_time = Instant::now();
        // let elapsed_time = current_time - previous_time;
        // previous_time = current_time;

        // accumulator += elapsed_time;

        // while accumulator >= time_interval {
        //     // DO physics calculations
        //     physics_calc(&mut springs, &mut window);

        //     accumulator -= time_interval;
        // }

        // let sleep_duration = if time_interval > (Instant::now() - current_time) {
        //     time_interval - (Instant::now() - current_time)
        // } else {
        //     Duration::from_secs(0)
        // };

        // sleep(sleep_duration);
    }
}

// Translation function
