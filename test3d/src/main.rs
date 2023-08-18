extern crate kiss3d;

use itertools::{enumerate, Itertools, Permutations};
use kiss3d::light::Light;
use kiss3d::nalgebra::vector;
use kiss3d::nalgebra::{Translation3, Vector3};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use rand::prelude::*;
use std::cmp;
use std::thread::sleep;
use std::time::{Duration, Instant};

struct Particle {
    sphere: SceneNode,
    pos: Vector3<f32>,
    momentum: Vector3<f32>,
    mass: f32,
    radius: f32,
}

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    window.set_light(Light::StickToCamera);
    const NUM_PARTICLES: usize = 10;
    let (mut window, mut particles) = create_particles(NUM_PARTICLES, window);

    let mut rng = rand::thread_rng();
    for particle in &mut particles {
        let x: f32 = rng.gen_range(0.0..=100.0); // generates a float between 0 and 10
        let y: f32 = rng.gen_range(0.0..=100.0); // generates a float between 0 and 10
        let z: f32 = rng.gen_range(0.0..=100.0); // generates a float between
        let px: f32 = rng.gen_range(5.0..=10.0); // generates a float between 0 and 10
        let py: f32 = rng.gen_range(5.0..=10.0); // generates a float between 0 and 10
        let pz: f32 = rng.gen_range(5.0..=10.0); // generates a float between
        translate(particle, x, y, z); // adds translation
        particle.momentum = Vector3::new(0.0 * px, 0.0 * py, 0.0 * pz);
    }

    const dt: f32 = 0.01;
    // const dt: f32 = 1.0;
    const iters_per_sec: f32 = 60.0;
    let time_interval = Duration::from_secs_f32(1.0 / iters_per_sec);

    let mut previous_time = Instant::now();
    let mut accumulator: Duration = Duration::from_millis(0);

    while window.render() {
        let current_time = Instant::now();
        let elapsed_time = current_time - previous_time;
        previous_time = current_time;

        accumulator += elapsed_time;

        while accumulator >= time_interval {
            let zero_vec: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
            let mut dps = [zero_vec; NUM_PARTICLES];
            // Calculate impulses
            for i in 0..particles.len() {
                for j in 0..particles.len() {
                    if i != j {
                        // Not the same particle
                        let particle1 = &particles[i];
                        let particle2 = &particles[j];
                        let force = grav_force(particle1, particle2);
                        let dp = force * dt;
                        dps[i] += dp;
                        dps[j] -= dp;
                    }
                }
            }
            // Apply impulses
            for i in 0..dps.len() {
                let particle = &mut particles[i];
                let imp = dps[i];
                particle.momentum += imp;
                let dr = (particle.momentum / particle.mass) * dt;
                // println!("{}", dr);
                translate(particle, dr.x, dr.y, dr.z);
            }

            accumulator -= time_interval;
        }
        sleep(cmp::max(
            Duration::from_secs(0),
            time_interval - (Instant::now() - current_time),
        ));
    }
}

fn translate(particle: &mut Particle, x: f32, y: f32, z: f32) {
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

fn particle(sphere: SceneNode, pos: (f32, f32, f32), mom: (f32, f32, f32), mass: f32) -> Particle {
    let position: Vector3<f32> = Vector3::new(pos.0, pos.1, pos.2);
    let momentum: Vector3<f32> = Vector3::new(mom.0, mom.1, mom.2);
    Particle {
        sphere: sphere,
        pos: position,
        momentum: momentum,
        mass: mass,
        radius: 1.0,
    }
}

fn create_particles(num_p: usize, mut window: Window) -> (Window, Vec<Particle>) {
    let mut particles = Vec::new();
    for i in 0..num_p {
        let sphere = window.add_sphere(1.0);
        let pos = (0.0, 0.0, 0.0);
        let mom = (0.0, 0.0, 0.0);
        let mass = 1.0;
        let particle = particle(sphere, pos, mom, mass);
        particles.push(particle);
        println!("Created particle {}", i);
    }
    (window, particles)
}
fn grav_force(particle1: &Particle, particle2: &Particle) -> Vector3<f32> {
    // Gmm/r^2
    const G: f32 = 1000.0;
    let r = particle2.pos - particle1.pos;
    let distance = r.norm();

    let rhat = r / r.norm(); // Orient the force vector
    let force = (G * particle1.mass * particle2.mass) / f32::powf(r.norm(), 2.0);
    const K: f32 = 0.5;
    let dampen_force = (1.0 / (1.0 + K * distance)) * force;
    rhat * (force - dampen_force)
}
