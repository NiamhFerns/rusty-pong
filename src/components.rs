use bevy::math::vec3;
use bevy::prelude::*;
use bevy::sprite::SpriteBundle;
use bevy::window::PrimaryWindow;

const BAT_DIMENSIONS: (u32, u32) = (24, 96);
const BALL_SPEED: f32 = 400.0;
const BAT_SPEED: f32 = 400.0;

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
                direction: vec3(1.0, 1.0, 0.0),
            },
        ));
    }

    pub fn movement(
        time: Res<Time>,
        mut ball_query: Query<(&mut Transform, &mut Ball)>,
        window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        let window = window_query
            .get_single()
            .expect("Tried to get single for primary window when multiple exist.");

        if let Ok((mut transform, mut ball)) = ball_query.get_single_mut() {
            transform.translation += ball.direction * BALL_SPEED * time.delta_seconds();
            ball.check_bounds(&mut transform.translation, window);
        }
    }

    // Lock ball onto the screen and change it's direction.
    pub fn check_bounds(&mut self, transform: &mut Vec3, window: &Window) {
        // This should ive a point to the correct side.
        if transform.x > window.width() - 32.0 || transform.x < 32.0 {
            self.direction.x *= -1.0;
            transform.x = f32::max(32.0, f32::min(window.width() - 32.0, transform.x));
        }

        // Just bounce off the top of the wall.
        if transform.y > window.height() - 32.0 || transform.y < 32.0 {
            self.direction.y *= -1.0;
            transform.y = f32::max(32.0, f32::min(window.height() - 32.0, transform.y));
        }
    }

    fn ray_to_rect() {
        /*private void rayToRect(Rectangle rt, Vector2D ray) {
            Vector2D activeEdge = new Vector2D(0.0f, 0.0f);
            activeEdge.x = max(rt.getX(), min(rt.getX() + rt.getWidth(), (int)nextX()));
            activeEdge.y = max(rt.getY(), min(rt.getY() + rt.getHeight(), (int)nextY()));

            ray.x = activeEdge.x - (float)nextX();
            ray.y = activeEdge.y - (float)nextY();

            // Debugging to draw the ray if needed.
            // line(activeEdge.x, activeEdge.y, nextX(), nextY());
        }*/
    }

    fn collides_with() {
        /*private void collidesWith(Rectangle rt) {
            // Will only be true it fhe ball is approaching from a corner.
            boolean cornerCollision =  (nextX() <= rt.getX() || nextX() >= rt.getX() + rt.getWidth())   // The ball is to the left or right of the bat.
                                    && (nextY() <= rt.getY() || nextY() >= rt.getY() + rt.getHeight()); // The ball is above or bellow the bat.

            // Yes I know this is gross. I ran out of time to refactor it to not be gross. ;-;
            if (cornerCollision) { print( "Velocity Before: (" + velocity.x + ", " + velocity.y + ")!\n"); velocity.invert(); print("Velocity After: (" + velocity.x + ", " + velocity.y + ")!\n"); return; }
            if (nextX() <= rt.getX() || nextX() >= rt.getX() + rt.getWidth()) velocity.invertX();
            if (nextY() <= rt.getY() || nextY() >= rt.getY() + rt.getHeight()) velocity.invertY();

            print("Collision!\n");
        }*/
    }

    pub fn check_collisions(
        mut ball_query: Query<(&mut Ball, &mut Transform)>,
        mut players_query: Query<&Transform, With<Bat>>,
    ) {
        if let Ok((ball, ball_transform)) = ball_query.get_single_mut() {
            let translation = ball_transform.translation;
            for &bat in players_query.iter() {
                /*private void checkCollisions() {
                    // Walls
                    if (nextX() + radius > width || nextX() - ball.radius < 0) velocity.invertX();
                    if (nextY() + radius > height || nextY() - ball.radius < 0) velocity.invertY();

                    Vector2D rToBat = new Vector2D(0.0, 0.0);
                    rayToRect(bat, rToBat);

                    // We only want one collision event. This is a very niave way of handling this issue and it doesn't fix the issue of hitting the ball in the direction it is already travelling in.
                    // To fix this more thoroughly I'd need to have someway to have a vector distance that the centre of the ball is *into* the bat and then move the ball along that vector as well as
                    // by the velocity already set. I'd also need to be able to some how detect if the ball is travelling downward when it hits the bottomside and only reflect it if it is travelling
                    // up when it hits the bottom, down when it hits the top, etc etc.
                    if (rToBat.norm() <= radius && free) { collidesWith(bat); free = false; }
                    else if (rToBat.norm() > radius) free = true;
                } */
            }
        }
    }
}

#[derive(Component)]
pub struct Bat {
    up_key: KeyCode,
    down_key: KeyCode,
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
                up_key: KeyCode::E,
                down_key: KeyCode::N,
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
                up_key: KeyCode::F,
                down_key: KeyCode::S,
            },
        ));
    }

    pub fn player_movement(
        input: Res<Input<KeyCode>>,
        time: Res<Time>,
        mut players_query: Query<(&Bat, &mut Transform)>,
        window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        let window = window_query
            .get_single()
            .expect("There should only be one PrimaryWindow entity.");

        // Move bats on key press.
        for (player, mut transform) in players_query.iter_mut() {
            if input.pressed(player.up_key) {
                transform.translation.y += BAT_SPEED * time.delta_seconds();
            }
            if input.pressed(player.down_key) {
                transform.translation.y += -1.0 * BAT_SPEED * time.delta_seconds();
            }

            // Lock bats onto the screen vertically.
            transform.translation.y = f32::max(
                BAT_DIMENSIONS.1 as f32 / 2.0,
                f32::min(
                    window.height() - (BAT_DIMENSIONS.1 as f32 / 2.0),
                    transform.translation.y,
                ),
            );
        }
    }
}
