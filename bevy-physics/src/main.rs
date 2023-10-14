use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
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

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

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

fn hello_world() {
    println!("Hello world");
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}
