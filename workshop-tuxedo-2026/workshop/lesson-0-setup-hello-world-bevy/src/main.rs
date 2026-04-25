use bevy::prelude::*;

const SPEED: f32 = 300.0;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Arrow Key Mover".into(),
                resolution: (800u32, 600u32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

fn setup(mut commands: Commands) {
    // 2D camera
    commands.spawn(Camera2d);

    // Player: a coloured square sprite
    commands.spawn((
        Player,
        Sprite {
            color: Color::srgb(0.25, 0.75, 1.0),
            custom_size: Some(Vec2::new(48.0, 48.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    let mut dir = Vec2::ZERO;

    if input.pressed(KeyCode::ArrowLeft)  { dir.x -= 1.0; }
    if input.pressed(KeyCode::ArrowRight) { dir.x += 1.0; }
    if input.pressed(KeyCode::ArrowUp)    { dir.y += 1.0; }
    if input.pressed(KeyCode::ArrowDown)  { dir.y -= 1.0; }

    if dir != Vec2::ZERO {
        let delta = dir.normalize() * SPEED * time.delta_secs();
        player.translation += delta.extend(0.0);
    }
}
