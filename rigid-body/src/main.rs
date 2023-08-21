use kiss3d::light::Light;
use kiss3d::nalgebra::{Translation3, Vector3};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use std::thread::sleep;
use std::time::{Duration, Instant};

const dt: f32 = 0.01;
// TODO import vector type; currently 1d spring
#[derive(Default)]
struct Spring {
    stiffness: f32, // Spring constant
    length: f32,
    pos_1: f32, // first end of spring position
    pos_2: f32, // second end of spring position
    vel_1: f32,
    vel_2: f32,
    mass: f32, // mass of both ends for now
}

impl Spring {
    // TODO: force will be a vector
    fn force(&self) -> f32 {
        // F = -*stiffness*x
        let dx = (self.pos_2 - self.pos_1) - self.length;
        -self.stiffness * dx
    }

    fn update_vel(&mut self) {
        let force = self.force();
        let dp = force * dt;
        let dv = dp / self.mass;
        self.vel_1 -= dv;
        self.vel_2 += dv;
    }

    fn update_pos(&mut self) {
        self.pos_1 += self.vel_1 * dt;
        self.pos_2 += self.vel_2 * dt;
    }
}

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    window.set_light(Light::StickToCamera);
    let mut spring = Spring {
        stiffness: 1.0,
        length: 10.0,
        pos_1: -10.0,
        pos_2: 5.0,
        mass: 1.0,
        ..Default::default()
    };
    let mut cube1 = window.add_cube(1.0, 1.0, 1.0);
    let mut cube2 = window.add_cube(1.0, 1.0, 1.0);

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
            spring.update_vel();
            spring.update_pos();
            translate_to(&mut cube1, spring.pos_1, 0.0, 0.0);
            translate_to(&mut cube2, spring.pos_2, 0.0, 0.0);
            // println!("");
            // println!("================================================");
            // println!("Pos1: {}, Pos2: {}", spring.pos_1, spring.pos_2);
            // println!("Vel1: {}, Vel2: {}", spring.vel_1, spring.vel_2);
            // println!("================================================");
            // println!("");

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
fn translate(object: &mut SceneNode, x: f32, y: f32, z: f32) {
    let t = Translation3::new(x, y, z);
    object.prepend_to_local_translation(&t);
}
fn translate_to(object: &mut SceneNode, x: f32, y: f32, z: f32) {
    let t = Translation3::new(x, y, z);
    object.set_local_translation(t);
}
