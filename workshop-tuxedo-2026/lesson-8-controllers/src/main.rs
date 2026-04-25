use bevy::prelude::*;

const SPEED: f32 = 400.0;
const BOX_Y: f32 = -250.0;

#[derive(Resource, Default)]
struct InputToggles {
    cursor_keys: bool,
    wasd: bool,
    mouse_touch: bool,
    gamepad: bool,
}

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<InputToggles>()
        .add_systems(Startup, setup)
        .add_systems(Update, (toggle_inputs, move_player, update_title))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_color(Color::srgb(0.3, 0.7, 1.0), Vec2::new(60.0, 60.0)),
        Transform::from_xyz(0.0, BOX_Y, 0.0),
        Player,
    ));
}

fn toggle_inputs(keys: Res<ButtonInput<KeyCode>>, mut toggles: ResMut<InputToggles>) {
    if keys.just_pressed(KeyCode::Digit1) {
        toggles.cursor_keys = !toggles.cursor_keys;
    }
    if keys.just_pressed(KeyCode::Digit2) {
        toggles.wasd = !toggles.wasd;
    }
    if keys.just_pressed(KeyCode::Digit3) {
        toggles.mouse_touch = !toggles.mouse_touch;
    }
    if keys.just_pressed(KeyCode::Digit4) {
        toggles.gamepad = !toggles.gamepad;
    }
}

fn move_player(
    time: Res<Time>,
    toggles: Res<InputToggles>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    gamepads: Query<&Gamepad>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    let mut dx = 0.0;

    if toggles.cursor_keys {
        if keys.pressed(KeyCode::ArrowLeft) {
            dx -= 1.0;
        }
        if keys.pressed(KeyCode::ArrowRight) {
            dx += 1.0;
        }
    }

    if toggles.wasd {
        if keys.pressed(KeyCode::KeyA) {
            dx -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            dx += 1.0;
        }
    }

    if toggles.gamepad {
        for gamepad in &gamepads {
            if gamepad.pressed(GamepadButton::DPadLeft) {
                dx -= 1.0;
            }
            if gamepad.pressed(GamepadButton::DPadRight) {
                dx += 1.0;
            }
            if let Some(x) = gamepad.get(GamepadAxis::LeftStickX)
                && x.abs() > 0.1
            {
                dx += x;
            }
        }
    }

    player.translation.x += dx.clamp(-1.0, 1.0) * SPEED * time.delta_secs();

    // Mouse / touch: snap horizontally to pointer position
    if toggles.mouse_touch {
        let pointer_screen = if let Some(touch) = touches.iter().next() {
            Some(touch.position())
        } else if mouse_buttons.pressed(MouseButton::Left) {
            window.cursor_position()
        } else {
            None
        };

        if let Some(screen_pos) = pointer_screen {
            let (cam, cam_transform) = *camera;
            if let Ok(world_pos) = cam.viewport_to_world_2d(cam_transform, screen_pos) {
                player.translation.x = world_pos.x;
            }
        }
    }
}

fn update_title(toggles: Res<InputToggles>, mut window: Single<&mut Window>) {
    if !toggles.is_changed() {
        return;
    }
    let mark = |b: bool| if b { "ON" } else { "OFF" };
    window.title = format!(
        "1:Cursor[{}]  2:WASD[{}]  3:Mouse/Touch[{}]  4:Gamepad[{}]",
        mark(toggles.cursor_keys),
        mark(toggles.wasd),
        mark(toggles.mouse_touch),
        mark(toggles.gamepad),
    );
}
