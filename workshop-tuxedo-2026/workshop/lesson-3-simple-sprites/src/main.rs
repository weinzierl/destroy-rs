use bevy::prelude::*;

const SPEED: f32 = 250.0;

#[derive(Component)]
struct Ship;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Sprite Demo".into(),
                        resolution: (800, 600).into(),
                        ..default()
                    }),
                    ..default()
                })
                // Nearest-neighbor sampling so pixel art stays crisp
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (move_ship, wrap_around))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Ship,
        Sprite::from_image(asset_server.load("ship.png")),
        Transform::from_scale(Vec3::splat(3.0)),
    ));
}

fn move_ship(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut ship: Single<&mut Transform, With<Ship>>,
) {
    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction == Vec2::ZERO {
        return;
    }

    let direction = direction.normalize();

    ship.translation.x += direction.x * SPEED * time.delta_secs();
    ship.translation.y += direction.y * SPEED * time.delta_secs();

    // Rotate ship to face movement direction
    let angle = direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2;
    ship.rotation = Quat::from_rotation_z(angle);
}

fn wrap_around(
    window: Single<&Window>,
    mut ship: Single<&mut Transform, With<Ship>>,
) {
    let half_w = window.width() / 2.0;
    let half_h = window.height() / 2.0;

    let t = &mut ship.translation;

    if t.x > half_w {
        t.x = -half_w;
    }
    if t.x < -half_w {
        t.x = half_w;
    }
    if t.y > half_h {
        t.y = -half_h;
    }
    if t.y < -half_h {
        t.y = half_h;
    }
}
