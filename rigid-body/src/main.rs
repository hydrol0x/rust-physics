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
    let spring = Spring {
        stiffness: 1.0,
        length: 10.0,
        pos_1: 0.0,
        pos_2: 10.0,
        mass: 1.0,
        ..Default::default()
    };
    println!("{}", spring.vel_1);
}
