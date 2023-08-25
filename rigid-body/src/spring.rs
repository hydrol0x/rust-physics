use kiss3d::nalgebra::{Point3, Translation3, Vector, Vector3};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

// Defines spring type
// how to make a nuclear bomb: obtain ~1500kgs of uranium from ebay, enrich it, then make a dmeon core :D

// Todo: make the coupled list be a list of SpringNodes so that I don't have to keep track of which end is coupeld with which other
pub struct SpringNode<'a> {
    pub pos: Vector3<f32>,
    pub vel: Vector3<f32>,
    pub force: Vector3<f32>,
    pub coupled: Vec<&'a mut SpringNode<'a>>,
    pub mass: f32,
}

impl<'a> SpringNode<'a> {
    pub fn couple(&mut self, node: &'a mut SpringNode<'a>) {
        self.coupled.push(node);
    }
}
// TODO: put spring block into springNode so that their position can be updated in step_spring

pub struct Spring<'a> {
    pub node_1: SpringNode<'a>,
    pub node_2: SpringNode<'a>,
    pub node_1_block: SceneNode,
    pub node_2_block: SceneNode,
    pub stiffness: f32,
    pub dampen: f32,
    pub length: f32,
}

impl<'a> Spring<'a> {
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
        let mut node_1_block = window.add_cube(0.5, 0.5, 0.5);
        let mut node_2_block = window.add_cube(0.5, 0.5, 0.5);
        translate_to(&mut node_1_block, pos_1.x, pos_1.y, pos_1.z);
        translate_to(&mut node_2_block, pos_2.x, pos_2.y, pos_2.z);
        let node_1 = SpringNode {
            pos: pos_1,
            vel: vel_1,
            force: Vector3::new(0.0, 0.0, 0.0),
            mass: mass,
            coupled: Vec::new(),
        };

        let node_2 = SpringNode {
            pos: pos_2,
            vel: vel_2,
            force: Vector3::new(0.0, 0.0, 0.0),
            mass: mass,
            coupled: Vec::new(),
        };

        Self {
            node_1: node_1,
            node_2: node_2,
            node_1_block: node_1_block,
            node_2_block: node_2_block,
            stiffness: stiffness,
            dampen: dampen,
            length: length,
        }
    }

    pub fn new_default_spring(window: &mut Window) -> Self {
        let pos_1 = Vector3::new(-5.0, 0.0, 0.0);
        let pos_2 = Vector3::new(5.0, 0.0, 0.0);
        let vel_1 = Vector3::new(0.0, 0.0, 0.0);
        let vel_2 = Vector3::new(0.0, 0.0, 0.0);

        // Default values for stiffness, length, mass, and dampen
        let default_stiffness = 10000.0;
        let default_length = 10.0;
        let default_mass = 1.0;
        let default_dampen = 10.0;

        println!(
            "Initialized Spring with stiff:{}, len:{}, pos1:{}, pos2:{}, vel1:{}, vel2:{}, mass:{}, damp:{}",
            default_stiffness,
            default_length,
            pos_1,
            pos_2,
            vel_1,
            vel_2,
            default_mass,
            default_dampen
        );
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

    fn spring_force(&mut self) {
        // F = -*stiffness*x
        let node_1 = &mut self.node_1;
        let node_2 = &mut self.node_2;

        let r_hat = (node_2.pos - node_1.pos) / (node_2.pos - node_1.pos).norm();
        let dx = (node_2.pos - node_1.pos).norm() - self.length;
        // Fdamp = F=c*v
        let damping_force = self.dampen * (node_1.vel.norm() + node_2.vel.norm()) * r_hat;
        let force = -(self.stiffness * dx * r_hat) - damping_force;
        // println!("dx {}", dx);
        // println!("force mag {}", force);
        // println!("damp force mag {}", damping_force);
        // println!("r_hat {}", r_hat);
        node_1.force = -force;
        node_2.force = force;
    }

    // List of SpringNodes == automatically keeps track of which connected to which
    fn coupling_force(&mut self) /*-> Vector3<f32>*/
    {
        let mut force_1: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
        let mut force_2: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
        for spring_node in &self.node_1.coupled {
            force_1 += spring_node.force;
        }
        for spring_node in &self.node_2.coupled {
            force_2 += spring_node.force;
        }
        self.node_1.force += force_1;
        self.node_2.force += force_2;
    }

    fn update_vel(&mut self, dt: f32) {
        self.spring_force();
        self.coupling_force();
        self.coupling_force();
        let node_1 = &mut self.node_1;
        let node_2 = &mut self.node_2;
        let dp_1 = node_1.force * dt;
        let dp_2 = node_2.force * dt;
        let dv_1 = dp_1 / node_1.mass;
        let dv_2 = dp_2 / node_2.mass;
        node_1.vel += dv_1;
        node_2.vel += dv_2;
    }

    fn update_pos(&mut self, dt: f32) {
        let node_1 = &mut self.node_1;
        let node_2 = &mut self.node_2;
        node_1.pos += node_1.vel * dt;
        node_2.pos += node_2.vel * dt;
    }

    pub fn step_spring(&mut self, dt: f32, window: &mut Window) {
        self.update_pos(dt);
        self.update_vel(dt);
        self.draw_spring(window);
        let node_1 = &mut self.node_1;
        let node_2 = &mut self.node_2;
        translate_to(
            &mut self.node_1_block,
            node_1.pos.x,
            node_1.pos.y,
            node_1.pos.z,
        );
        translate_to(
            &mut self.node_2_block,
            node_2.pos.x,
            node_2.pos.y,
            node_2.pos.z,
        );
        // for spring_node in self.node_1.coupled {
        //     spring_node.node_block
        // }
    }

    fn draw_spring(&self, window: &mut Window) {
        let node_1 = &self.node_1;
        let node_2 = &self.node_2;
        let a = Point3::new(node_1.pos.x, node_1.pos.y, node_1.pos.z);
        let b = Point3::new(node_2.pos.x, node_2.pos.y, node_2.pos.z);
        let color = Point3::new(1.0, 1.0, 1.0);
        window.set_line_width(2.0);

        window.draw_line(&a, &b, &color);
    }

    pub fn zeta(&self) -> f32 {
        self.dampen / (2.0 * f32::sqrt(self.node_1.mass * self.stiffness))
    }

    pub fn print(&self) {
        println!(
            "\n
            Node1: {{\n
                pos:   {},\n
                vel:   {},\n
                force: {},\n
                mass:  {},\n 
            \n}}\n\n
            Node2: {{
                pos:   {},\n
                vel:   {},\n
                force: {},\n
                mass:  {},\n
            }}\n\n
            Spring: {{
                stiffness: {},\n
                dampen: {},\n
                length: {},\n
            }}
        \n",
            self.node_1.pos,
            self.node_1.vel,
            self.node_1.force,
            self.node_1.mass,
            self.node_2.pos,
            self.node_2.vel,
            self.node_2.force,
            self.node_2.mass,
            self.stiffness,
            self.dampen,
            self.length
        );
    }
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
