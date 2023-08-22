use kiss3d::nalgebra::{Point3, Translation3, Vector3};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

// Defines spring type
// how to make a nuclear bomb: obtain ~1500kgs of uranium from ebay, enrich it, then make a dmeon core :D
pub struct Spring {
    pub mass_1_block: SceneNode,
    pub mass_2_block: SceneNode,
    pub stiffness: f32, // Spring constant
    pub length: f32,
    pub pos_1: Vector3<f32>, // first end of spring position
    pub pos_2: Vector3<f32>, // second end of spring position
    pub vel_1: Vector3<f32>,
    pub vel_2: Vector3<f32>,
    pub mass: f32,   // mass of both ends for now
    pub dampen: f32, // damp factor, c in the equation F_damp = c*v
}

impl Spring {
    pub fn new(
        window: &mut Window,
        stiffness: f32,
        length: f32,
        pos_1: Vector3<f32>,
        pos_2: Vector3<f32>,
        vel_1: Vector3<f32>,
        vel_2: Vector3<f32>,
        mass: f32,
        dampen: f32,
    ) -> Self {
        let mut mass_1_block = window.add_cube(0.5, 0.5, 0.5);
        let mut mass_2_block = window.add_cube(0.5, 0.5, 0.5);
        translate_to(&mut mass_1_block, pos_1.x, pos_1.y, pos_1.z);
        translate_to(&mut mass_2_block, pos_2.x, pos_2.y, pos_2.z);
        Self {
            mass_1_block: mass_1_block,
            mass_2_block: mass_2_block,
            stiffness: stiffness,
            length: length,
            pos_1: pos_1,
            pos_2: pos_2,
            vel_1: vel_1,
            vel_2: vel_2,
            mass: mass,
            dampen: dampen,
        }
    }

    pub fn new_default_spring(window: &mut Window) -> Self {
        let pos_1 = Vector3::new(-5.0, 0.0, 0.0);
        let pos_2 = Vector3::new(5.0, 0.0, 0.0);
        let vel_1 = Vector3::new(0.0, 0.0, 0.0);
        let vel_2 = Vector3::new(0.0, 0.0, 0.0);

        // Default values for stiffness, length, mass, and dampen
        let default_stiffness = 100.0;
        let default_length = 10.0;
        let default_mass = 1.0;
        let default_dampen = 10.0;

        Self::new(
            window,
            default_stiffness,
            default_length,
            pos_1,
            pos_2,
            vel_1,
            vel_2,
            default_mass,
            default_dampen,
        )
    }

    fn force(&self) -> Vector3<f32> {
        // F = -*stiffness*x
        let r_hat = (self.pos_2 - self.pos_1) * (self.pos_2 - self.pos_1).norm();
        let dx = (self.pos_2 - self.pos_1).norm() - self.length;
        // Fdamp = F=c*v
        let damping_force = self.dampen * (self.vel_1.norm() + self.vel_2.norm()) * r_hat;
        -(self.stiffness * dx * r_hat) - damping_force
    }

    fn update_vel(&mut self, dt: f32) {
        let force = self.force();
        let dp = force * dt;
        let dv = dp / self.mass;
        self.vel_1 -= dv;
        self.vel_2 += dv;
    }

    fn update_pos(&mut self, dt: f32) {
        self.pos_1 += self.vel_1 * dt;
        self.pos_2 += self.vel_2 * dt;
    }

    pub fn step_spring(&mut self, dt: f32, window: &mut Window) {
        self.update_pos(dt);
        self.update_vel(dt);
        self.draw_spring(window);
        translate_to(
            &mut self.mass_1_block,
            self.pos_1.x,
            self.pos_1.y,
            self.pos_1.z,
        );
        translate_to(
            &mut self.mass_2_block,
            self.pos_2.x,
            self.pos_2.y,
            self.pos_2.z,
        );
    }

    fn draw_spring(&self, window: &mut Window) {
        let a = Point3::new(self.pos_1.x, self.pos_1.y, self.pos_1.z);
        let b = Point3::new(self.pos_2.x, self.pos_2.y, self.pos_2.z);
        let color = Point3::new(1.0, 1.0, 1.0);
        window.set_line_width(2.0);

        window.draw_line(&a, &b, &color);
    }

    pub fn zeta(&self) -> f32 {
        self.dampen / (2.0 * f32::sqrt(self.mass * self.stiffness))
    }
}

// TODO: maybe make below methods
pub fn new_spring(
    mass_1: SceneNode,
    mass_2: SceneNode,
    stiffness: f32,
    length: f32,
    pos_1: Vector3<f32>,
    pos_2: Vector3<f32>,
    vel_1: Vector3<f32>,
    vel_2: Vector3<f32>,
    mass: f32,
    dampen: f32,
) -> Spring {
    Spring {
        mass_1_block: mass_1,
        mass_2_block: mass_2,
        stiffness: stiffness,
        length: length,
        pos_1: pos_1,
        pos_2: pos_2,
        vel_1: vel_1,
        vel_2: vel_2,
        mass: mass,
        dampen: dampen,
    }
}

pub fn default_spring(window: &mut Window) -> Spring {
    let mass_1_block = window.add_cube(0.5, 0.5, 0.5);
    let mass_2_block = window.add_cube(0.5, 0.5, 0.5);
    let pos_1 = Vector3::new(-5.0, 0.0, 0.0);
    let pos_2 = Vector3::new(5.0, 0.0, 0.0);
    let vel_1 = Vector3::new(0.0, 0.0, 0.0);
    let vel_2 = Vector3::new(0.0, 0.0, 0.0);
    new_spring(
        mass_1_block,
        mass_2_block,
        10.0,
        10.0,
        pos_1,
        pos_2,
        vel_1,
        vel_2,
        10.0,
        100.0,
    )
}

// helper functions for translation
fn translate(object: &mut SceneNode, x: f32, y: f32, z: f32) {
    let t = Translation3::new(x, y, z);
    object.prepend_to_local_translation(&t);
}

fn translate_to(object: &mut SceneNode, x: f32, y: f32, z: f32) {
    let t = Translation3::new(x, y, z);
    object.set_local_translation(t);
}
