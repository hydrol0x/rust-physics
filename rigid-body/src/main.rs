use kiss3d::light::Light;
use kiss3d::nalgebra::{Point, Point2, Point3, Translation2, Translation3, Vector, Vector3};
use kiss3d::scene::{self, PlanarSceneNode, SceneNode};
use kiss3d::window::{self, Window};
use std::thread::sleep;
use std::time::{Duration, Instant};

mod spring;
use spring::default_spring;
use spring::new_spring;
use spring::Spring;

const dt: f32 = 0.001;

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    window.set_light(Light::StickToCamera);

    let mut spring = Spring::new_default_spring(&mut window);
    spring.pos_2 = Vector3::new(-10.0, 0.0, 0.0);

    println!("Zeta: {}", spring.zeta());

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
            spring.step_spring(dt, &mut window);

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
