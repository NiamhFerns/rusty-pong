use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod components;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(components::Ball::spawn)
        .add_startup_system(components::Bat::spawn_players)
        .add_system(components::Ball::movement)
        .add_system(components::Bat::player_movement)
        .run()
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
