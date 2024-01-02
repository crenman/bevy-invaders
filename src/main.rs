use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}

#[derive(Component)]
struct Ship;

#[derive(Component)]
struct Invader;

#[derive(Component)]
struct Position;

#[derive(Component)]
struct Velocity(Vec2);

fn hello_world() {
    println!("hello world!");
}
