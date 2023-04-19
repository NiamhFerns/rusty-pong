use bevy::math::vec3;
use bevy::prelude::*;
use bevy::sprite::SpriteBundle;
use bevy::window::PrimaryWindow;

const BAT_DIMENSIONS: (u32, u32) = (24, 96);
const BALL_SPEED: f32 = 400.0;

#[derive(Component)]
pub struct Ball {
    direction: Vec3,
}

impl Ball {
    pub fn spawn(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        let window = window_query
            .get_single()
            .expect("Tried to get single for primary window when multiple exist.");

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/ball.png"),
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                ..default()
            },
            Ball {
                direction: vec3(1.0, 0.0, 0.0),
            },
        ));
    }

    pub fn movement(
        mut ball_query: Query<(&mut Transform, &mut Ball)>,
        window_query: Query<&Window, With<PrimaryWindow>>,
        time: Res<Time>,
    ) {
        let window = window_query
            .get_single()
            .expect("Tried to get single for primary window when multiple exist.");

        if let Ok((mut transform, mut ball)) = ball_query.get_single_mut() {
            transform.translation += ball.direction * BALL_SPEED * time.delta_seconds();
            ball.check_bounds(&mut transform.translation, window);
        }
    }

    pub fn check_bounds(&mut self, transform: &mut Vec3, window: &Window) {
        if transform.x > window.width() - 32.0 || transform.x < 32.0 {
            self.direction.x *= -1.0;
            transform.x = f32::max(32.0, f32::min(window.width() - 32.0, transform.x));
        }
    }
}

#[derive(Component)]
pub struct Bat {
    up_key: char,
    down_key: char,
}

impl Bat {
    pub fn spawn_players(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        let window = window_query
            .get_single()
            .expect("Tried to get single for primary window when multiple exist.");

        // Player Left
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/bat.png"),
                transform: Transform::from_xyz(
                    window.width() - BAT_DIMENSIONS.0 as f32,
                    window.height() / 2.0,
                    0.0,
                ),
                ..default()
            },
            Bat {
                up_key: 'e',
                down_key: 'n',
            },
        ));

        // Player Right
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/bat.png"),
                transform: Transform::from_xyz(BAT_DIMENSIONS.0 as f32, window.height() / 2.0, 0.0),
                ..default()
            },
            Bat {
                up_key: 'f',
                down_key: 's',
            },
        ));
    }
}
