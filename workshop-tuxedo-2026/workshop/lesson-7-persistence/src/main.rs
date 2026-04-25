//! Simplest Bevy 0.18 game-state serialization demo.
//!
//! A ball bounces around the window. Press:
//!   S - save state to `savegame.ron`
//!   L - load state from `savegame.ron`
//!   R - reset to a fresh state
//!
//! Demonstrates serializing both ECS components (Position, Velocity)
//! and a resource (Score), via plain serde + RON.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

const SAVE_PATH: &str = "savegame.ron";
const BALL_SIZE: f32 = 30.0;

// ---------- The bits we serialize ----------

#[derive(Component, Serialize, Deserialize, Debug, Clone, Copy)]
struct Position(Vec2);

#[derive(Component, Serialize, Deserialize, Debug, Clone, Copy)]
struct Velocity(Vec2);

#[derive(Resource, Serialize, Deserialize, Debug, Default, Clone)]
struct Score {
    bounces: u32,
}

/// Marker so we can find "the ball" with a Single<...> query.
#[derive(Component)]
struct Ball;

/// What actually gets written to disk: a snapshot of the
/// ball's state plus the score resource. Keeping this as
/// its own struct makes the on-disk schema explicit.
#[derive(Serialize, Deserialize, Debug)]
struct SaveData {
    position: Position,
    velocity: Velocity,
    score: Score,
}

// ---------- App ----------

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 0.18 — serde demo (S=save, L=load, R=reset)".into(),
                resolution: (640u32, 480u32).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Score::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_save_load_reset,
                move_ball,
                bounce_off_walls,
                sync_transform,
                update_hud,
            )
                .chain(),
        )
        .run();
}

#[derive(Component)]
struct Hud;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    spawn_ball(
        &mut commands,
        Position(Vec2::ZERO),
        Velocity(Vec2::new(180.0, 130.0)),
    );

    // Simple text HUD.
    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(8.0),
            left: Val::Px(8.0),
            ..default()
        },
        Hud,
    ));
}

fn spawn_ball(commands: &mut Commands, pos: Position, vel: Velocity) {
    commands.spawn((
        Ball,
        pos,
        vel,
        Sprite {
            color: Color::srgb(0.9, 0.4, 0.3),
            custom_size: Some(Vec2::splat(BALL_SIZE)),
            ..default()
        },
        Transform::from_xyz(pos.0.x, pos.0.y, 0.0),
    ));
}

// ---------- Gameplay systems ----------

fn move_ball(time: Res<Time>, mut q: Query<(&mut Position, &Velocity), With<Ball>>) {
    let dt = time.delta_secs();
    for (mut pos, vel) in &mut q {
        pos.0 += vel.0 * dt;
    }
}

fn bounce_off_walls(
    windows: Query<&Window>,
    mut score: ResMut<Score>,
    mut q: Query<(&mut Position, &mut Velocity), With<Ball>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let half_w = window.width() * 0.5 - BALL_SIZE * 0.5;
    let half_h = window.height() * 0.5 - BALL_SIZE * 0.5;

    for (mut pos, mut vel) in &mut q {
        let mut bounced = false;
        if pos.0.x.abs() > half_w {
            pos.0.x = pos.0.x.clamp(-half_w, half_w);
            vel.0.x = -vel.0.x;
            bounced = true;
        }
        if pos.0.y.abs() > half_h {
            pos.0.y = pos.0.y.clamp(-half_h, half_h);
            vel.0.y = -vel.0.y;
            bounced = true;
        }
        if bounced {
            score.bounces += 1;
        }
    }
}

fn sync_transform(mut q: Query<(&Position, &mut Transform), With<Ball>>) {
    for (pos, mut tf) in &mut q {
        tf.translation.x = pos.0.x;
        tf.translation.y = pos.0.y;
    }
}

fn update_hud(score: Res<Score>, mut q: Query<&mut Text, With<Hud>>) {
    for mut text in &mut q {
        **text = format!(
            "Bounces: {}\n[S] save  [L] load  [R] reset",
            score.bounces
        );
    }
}

// ---------- Save / Load / Reset ----------

fn handle_save_load_reset(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    balls: Query<(Entity, &Position, &Velocity), With<Ball>>,
) {
    if keys.just_pressed(KeyCode::KeyS) {
        if let Ok((_, pos, vel)) = balls.single() {
            let data = SaveData {
                position: *pos,
                velocity: *vel,
                score: score.clone(),
            };
            match ron::ser::to_string_pretty(&data, ron::ser::PrettyConfig::default()) {
                Ok(s) => match fs::write(SAVE_PATH, s) {
                    Ok(_) => info!("Saved -> {SAVE_PATH}"),
                    Err(e) => warn!("Save write failed: {e}"),
                },
                Err(e) => warn!("Serialize failed: {e}"),
            }
        }
    }

    if keys.just_pressed(KeyCode::KeyL) {
        match fs::read_to_string(SAVE_PATH) {
            Ok(s) => match ron::from_str::<SaveData>(&s) {
                Ok(data) => {
                    // Despawn existing ball(s), respawn from save.
                    for (e, _, _) in &balls {
                        commands.entity(e).despawn();
                    }
                    spawn_ball(&mut commands, data.position, data.velocity);
                    *score = data.score;
                    info!("Loaded <- {SAVE_PATH}");
                }
                Err(e) => warn!("Deserialize failed: {e}"),
            },
            Err(e) => warn!("No save to load ({e})"),
        }
    }

    if keys.just_pressed(KeyCode::KeyR) {
        for (e, _, _) in &balls {
            commands.entity(e).despawn();
        }
        spawn_ball(
            &mut commands,
            Position(Vec2::ZERO),
            Velocity(Vec2::new(180.0, 130.0)),
        );
        *score = Score::default();
        info!("Reset");
    }
}
