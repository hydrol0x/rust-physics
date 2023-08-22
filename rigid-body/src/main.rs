use kiss3d::light::Light;
use kiss3d::window::Window;
use std::thread::sleep;
use std::time::{Duration, Instant};

mod spring;
use spring::default_spring;
use spring::new_spring;
use spring::Spring;

const dt: f32 = 0.00001;

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    window.set_light(Light::StickToCamera);

    let mut springs: Vec<Spring> = vec![
        Spring::new_default_spring(&mut window),
        Spring::new_default_spring(&mut window),
    ];
    springs[0].length = 2.0;
    springs[1].length = 5.0;

    const ITERS_PER_SEC: f32 = 60.0;
    let time_interval = Duration::from_secs_f32(1.0 / ITERS_PER_SEC);

    let mut previous_time = Instant::now();
    let mut accumulator: Duration = Duration::from_millis(0);

    while window.render() {
        let current_time = Instant::now();
        let elapsed_time = current_time - previous_time;
        previous_time = current_time;

        accumulator += elapsed_time;

        while accumulator >= time_interval {
            // DO physics calculations
            for spring in &mut springs {
                spring.step_spring(dt, &mut window);
            }

            accumulator -= time_interval;
        }

        let sleep_duration = if time_interval > (Instant::now() - current_time) {
            time_interval - (Instant::now() - current_time)
        } else {
            Duration::from_secs(0)
        };

        sleep(sleep_duration);
    }
}

// Translation function
