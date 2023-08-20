extern crate kiss3d;

use kiss3d::light::Light;
use kiss3d::nalgebra::Vector3;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use rand::prelude::*;
use std::thread::sleep;
use std::time::{Duration, Instant};

// TODO:  import all physics functons from physics.rs instead of in this file.
// TODO: add electric force

// mod physics;

mod physics;
use physics::{are_colliding, calc_collision_imp, grav_force, translate, Particle};

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    window.set_light(Light::StickToCamera);
    const NUM_PARTICLES: usize = 2;
    let (mut window, mut particles) = create_particles(NUM_PARTICLES, window);

    let mut rng = rand::thread_rng();
    for particle in &mut particles {
        let x: f32 = rng.gen_range(-10.0..=100.0); // generates a float between 0 and 10
        let y: f32 = rng.gen_range(-10.0..=100.0); // generates a float between 0 and 10
        let z: f32 = rng.gen_range(-10.0..=100.0); // generates a float between
        let px: f32 = rng.gen_range(-10.0..=10.0); // generates a float between 0 and 10
        let py: f32 = rng.gen_range(-10.0..=10.0); // generates a float between 0 and 10
        let pz: f32 = rng.gen_range(-10.0..=10.0); // generates a float between
        translate(particle, x, y, z); // adds translation
        particle.momentum = Vector3::new(0.0 * px, 0.0 * py, 0.0 * pz);
    }

    // Two particle for testing
    // translate(&mut particles[0], 0.0, 4.0, 0.0);
    // translate(&mut particles[1], 0.0, -4.0, 0.0);
    // particles[0].momentum = Vector3::new(0.0, 0.0, 0.0);
    // particles[1].momentum = Vector3::new(0.0, 1.5, 0.0);

    const DT: f32 = 0.01;
    // const DT: f32 = 1.0;
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
                        let dp = force * DT;
                        dps[i] += dp;
                        dps[j] -= dp;

                        if are_colliding(particle1, particle2) {
                            let dp = calc_collision_imp(particle1, particle2);
                            dps[i] += dp;
                            dps[j] -= dp;
                        }
                    }
                }
            }
            // Apply impulses
            for i in 0..dps.len() {
                let particle = &mut particles[i];
                let imp = dps[i];
                particle.momentum += imp;
                let dr = (particle.momentum / particle.mass) * DT;
                // println!("{}", dr);
                translate(particle, dr.x, dr.y, dr.z);
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

fn particle(sphere: SceneNode, pos: (f32, f32, f32), mom: (f32, f32, f32), mass: f32) -> Particle {
    let position: Vector3<f32> = Vector3::new(pos.0, pos.1, pos.2);
    let momentum: Vector3<f32> = Vector3::new(mom.0, mom.1, mom.2);
    Particle {
        sphere: sphere,
        pos: position,
        momentum: momentum,
        mass: mass,
        radius: 1.0,
        charge: 0.0,
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
