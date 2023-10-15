use bevy::{math::vec3, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_particles)
        .add_systems(Update, (list_particles, update_positions))
        .run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Particle;

#[derive(Component)]
struct Position(Vec3);

#[derive(Component)]
struct Name(String);

// fn print_position_system(query: Query<&Position>) {
//     for position in &query {
//         println!("position: {} {}", position.x, position.y);
//     }
// }

fn add_particles(mut commands: Commands) {
    commands.spawn((
        Particle,
        Position(Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }),
    ));
    commands.spawn((
        Particle,
        Position(Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }),
    ));
    commands.spawn((
        Particle,
        Position(Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }),
    ));
}

fn update_positions(mut query: Query<&mut Position, With<Particle>>) {
    for mut position in query.iter_mut() {
        position.0 += vec3(1., 1., 1.);
    }
}

fn list_particles(query: Query<&Position, With<Particle>>) {
    for position in &query {
        println!("x {}, y {}, z {}", position.0.x, position.0.y, position.0.z);
    }
}
