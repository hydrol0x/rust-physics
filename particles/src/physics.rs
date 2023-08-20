/* physics functions */
// use constants::{k, G};
mod constants;
use constants::{k, G};
use kiss3d::nalgebra::{Translation3, Vector3};
use kiss3d::scene::SceneNode;

// Particle type
pub struct Particle {
    pub sphere: SceneNode,
    pub pos: Vector3<f32>,
    pub momentum: Vector3<f32>,
    pub mass: f32,
    pub radius: f32,
    pub charge: f32,
}

// Gravitational Force
pub fn grav_force(particle1: &Particle, particle2: &Particle) -> Vector3<f32> {
    // Gmm/r^2
    let r = particle2.pos - particle1.pos;
    let distance = r.norm();

    let rhat = r / r.norm(); // Orient the force vector
    let force = (G * particle1.mass * particle2.mass) / f32::powf(r.norm(), 2.0);
    const DAMPEN_FACTOR: f32 = 0.5;
    let dampen_force = (1.0 / (1.0 + DAMPEN_FACTOR * distance)) * force;
    rhat * (force - dampen_force)
}

// Electric Force
pub fn electric_force(particle1: &Particle, particle2: &Particle) -> Vector3<f32> {
    // k * qq/r^2
    let r = particle2.pos - particle1.pos;
    let distance = r.norm();

    let rhat = r / r.norm(); // Orient the force vector
    let force =
        (k * f32::abs(particle1.charge) * f32::abs(particle2.charge)) / f32::powf(r.norm(), 2.0);
    const DAMPEN_FACTOR: f32 = 0.5;
    let dampen_force = (1.0 / (1.0 + DAMPEN_FACTOR * distance)) * force;
    if particle1.charge * particle2.charge > 0.0 {
        // same charge
        return rhat * (-force - dampen_force);
    }
    rhat * (force - dampen_force)
}

// Collision force
pub fn are_colliding(particle1: &Particle, particle2: &Particle) -> bool {
    let sum_r = particle1.radius + particle2.radius;
    let distance = (particle2.pos - particle1.pos).norm();
    distance < sum_r
}

pub fn calc_collision_imp(particle1: &Particle, particle2: &Particle) -> Vector3<f32> {
    let m1 = particle1.mass;
    let m2 = particle2.mass;

    let v1 = particle1.momentum / m1;
    let v2 = particle2.momentum / m2;

    // Calculate norm vector for collision
    let r = particle2.pos - particle1.pos;
    let col_vec = r / r.norm();

    // Coef of restitution
    const EPS: f32 = 0.98;

    let m_reduced = 1.0 / ((1.0 / m1) + (1.0 / m2));
    let rel_vel = v2 - v1;
    let impact_v = col_vec.dot(&rel_vel); // comp of velocity in collision direction

    let imp = (1.0 + EPS) * m_reduced * impact_v; // impulse of collision
    col_vec * imp // Orient in direction of collision
}

// Translation function
pub fn translate(particle: &mut Particle, x: f32, y: f32, z: f32) {
    // TODO: change name of function to add_translation or something and or
    //  Make this a function that just translates to a position x,y,z
    // and a separet functin add_translation that wraps around
    // particle.pos = Vector3::new(x, y, z);
    particle.pos.x += x;
    particle.pos.y += y;
    particle.pos.z += z;
    let t = Translation3::new(x, y, z);
    particle.sphere.append_translation(&t);
}
