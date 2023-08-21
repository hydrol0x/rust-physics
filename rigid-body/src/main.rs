use kiss3d::light::Light;
use kiss3d::nalgebra::{Point, Point2, Point3, Translation2, Translation3, Vector, Vector3};
use kiss3d::scene::{PlanarSceneNode, SceneNode};
use kiss3d::window::Window;
use std::thread::sleep;
use std::time::{Duration, Instant};

const dt: f32 = 0.001;
// TODO import vector type; currently 1d spring
#[derive(Default)]
struct Spring {
    // mass_1_block: SceneNode,
    // mass_2_block: SceneNode,
    stiffness: f32, // Spring constant
    length: f32,
    pos_1: Vector3<f32>, // first end of spring position
    pos_2: Vector3<f32>, // second end of spring position
    vel_1: Vector3<f32>,
    vel_2: Vector3<f32>,
    mass: f32,   // mass of both ends for now
    dampen: f32, // damp factor, c in the equation F_damp = c*v
}

impl Spring {
    // TODO: force will be a vector
    fn force(&self) -> Vector3<f32> {
        // F = -*stiffness*x
        let r_hat = (self.pos_2 - self.pos_1) * (self.pos_2 - self.pos_1).norm();
        let dx = (self.pos_2 - self.pos_1).norm() - self.length;
        // Fdamp = F=c*v
        let damping_force = self.dampen * (self.vel_1.norm() + self.vel_2.norm()) * r_hat;
        -(self.stiffness * dx * r_hat) - damping_force
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

    fn draw_spring(&self, window: &mut Window) {
        let a = Point3::new(self.pos_1.x, self.pos_1.y, self.pos_1.z);
        let b = Point3::new(self.pos_2.x, self.pos_2.y, self.pos_2.z);
        let color = Point3::new(1.0, 1.0, 1.0);
        window.set_line_width(2.0);

        // window.draw_planar_line(&a, &b, &c)
        window.draw_line(&a, &b, &color);
    }
}

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    window.set_light(Light::StickToCamera);

    let mut spring = Spring {
        stiffness: 100.0,
        length: 2.0,
        pos_1: Vector3::new(1.0, 0.0, 0.0),
        pos_2: Vector3::new(-3.9, 0.0, 0.0),
        mass: 1.0,
        dampen: 10.0,
        ..Default::default()
    };
    println!(
        "Zeta: {}",
        zeta(spring.dampen, spring.mass, spring.stiffness)
    );

    let mut cube1 = window.add_cube(0.1, 0.1, 0.1);
    let mut cube2 = window.add_cube(0.1, 0.1, 0.1);
    cube1.set_color(0.0, 1.0, 0.0);

    const ITERS_PER_SEC: f32 = 60.0;
    let time_interval = Duration::from_secs_f32(1.0 / ITERS_PER_SEC);

    let mut previous_time = Instant::now();
    let mut accumulator: Duration = Duration::from_millis(0);

    while window.render() {
        let current_time = Instant::now();
        let elapsed_time = current_time - previous_time;
        previous_time = current_time;

        accumulator += elapsed_time;

        let mut i = 0.0;

        while accumulator >= time_interval {
            i += 1.0;
            // DO physics calculations
            spring.update_vel();
            spring.update_pos();
            translate_to(&mut cube1, spring.pos_1.x, spring.pos_1.y, spring.pos_1.z);
            translate_to(&mut cube2, spring.pos_2.x, spring.pos_2.y, spring.pos_2.z);

            spring.draw_spring(&mut window);

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

fn planar_translate_to(object: &mut PlanarSceneNode, x: f32, y: f32) {
    let t = Translation2::new(x, y);
    object.set_local_translation(t);
}

fn planar_translate(object: &mut PlanarSceneNode, x: f32, y: f32) {
    let t = Translation2::new(x, y);
    object.prepend_to_local_translation(&t);
}

fn zeta(c: f32, m: f32, k: f32) -> f32 {
    c / (2.0 * f32::sqrt(m * k))
}
