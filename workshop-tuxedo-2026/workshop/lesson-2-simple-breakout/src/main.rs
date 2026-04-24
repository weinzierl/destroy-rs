use bevy::prelude::*;

// ── Window / world constants ─────────────────────────────────────────────────
const WIN_W: f32 = 800.0;
const WIN_H: f32 = 600.0;
const HALF_W: f32 = WIN_W / 2.0;
const HALF_H: f32 = WIN_H / 2.0;

// ── Paddle ───────────────────────────────────────────────────────────────────
const PADDLE_W: f32 = 120.0;
const PADDLE_H: f32 = 16.0;
const PADDLE_Y: f32 = -HALF_H + 40.0;
const PADDLE_SPEED: f32 = 500.0;

// ── Ball ─────────────────────────────────────────────────────────────────────
const BALL_SIZE: f32 = 14.0;
const BALL_SPEED: f32 = 420.0;

// ── Components ───────────────────────────────────────────────────────────────
#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball {
    vel: Vec2,
}

// ── Main ─────────────────────────────────────────────────────────────────────
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Breakout".into(),
                resolution: bevy::window::WindowResolution::new(WIN_W, WIN_H),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_paddle, move_ball).chain())
        .run();
}

// ── Setup ─────────────────────────────────────────────────────────────────────
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Paddle
    commands.spawn((
        Paddle,
        Sprite {
            color: Color::srgb(0.9, 0.9, 0.9),
            custom_size: Some(Vec2::new(PADDLE_W, PADDLE_H)),
            ..default()
        },
        Transform::from_xyz(0.0, PADDLE_Y, 0.0),
    ));

    // Ball — launch diagonally up-right
    commands.spawn((
        Ball {
            vel: Vec2::new(1.0, 1.0).normalize() * BALL_SPEED,
        },
        Sprite {
            color: Color::srgb(1.0, 0.4, 0.2),
            custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

// ── Paddle movement ───────────────────────────────────────────────────────────
fn move_paddle(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut paddle: Single<&mut Transform, With<Paddle>>,
) {
    let mut dx = 0.0f32;
    if input.pressed(KeyCode::ArrowLeft)  { dx -= 1.0; }
    if input.pressed(KeyCode::ArrowRight) { dx += 1.0; }

    paddle.translation.x += dx * PADDLE_SPEED * time.delta_secs();

    // Clamp so paddle stays inside the walls
    let limit = HALF_W - PADDLE_W / 2.0;
    paddle.translation.x = paddle.translation.x.clamp(-limit, limit);
}

// ── Ball movement + collisions ────────────────────────────────────────────────
fn move_ball(
    time: Res<Time>,
    mut ball_query: Query<(&mut Transform, &mut Ball), Without<Paddle>>,
    paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
) {
    let Ok((ref mut ball_tf, ref mut ball)) = ball_query.get_single_mut() else { return };
    let Ok(paddle_transform) = paddle_query.get_single() else { return };

    // Move
    let dt = time.delta_secs();
    ball_tf.translation.x += ball.vel.x * dt;
    ball_tf.translation.y += ball.vel.y * dt;

    let bx = ball_tf.translation.x;
    let by = ball_tf.translation.y;
    let half_ball = BALL_SIZE / 2.0;

    // ── Wall collisions ──────────────────────────────────────────────────────
    // Left / right walls
    if bx - half_ball < -HALF_W {
        ball_tf.translation.x = -HALF_W + half_ball;
        ball.vel.x = ball.vel.x.abs();
    } else if bx + half_ball > HALF_W {
        ball_tf.translation.x = HALF_W - half_ball;
        ball.vel.x = -ball.vel.x.abs();
    }

    // Top wall
    if by + half_ball > HALF_H {
        ball_tf.translation.y = HALF_H - half_ball;
        ball.vel.y = -ball.vel.y.abs();
    }

    // ── Paddle collision ─────────────────────────────────────────────────────
    let px = paddle_transform.translation.x;
    let py = paddle_transform.translation.y;

    let overlap_x = (bx - px).abs() < (PADDLE_W / 2.0 + half_ball);
    let overlap_y = (by - py).abs() < (PADDLE_H / 2.0 + half_ball);

    if overlap_x && overlap_y && ball.vel.y < 0.0 {
        ball_tf.translation.y = py + PADDLE_H / 2.0 + half_ball;

        // Angle the bounce based on where the ball hits the paddle
        let offset = (bx - px) / (PADDLE_W / 2.0); // -1 .. +1
        ball.vel.x = offset * BALL_SPEED;
        ball.vel.y = (BALL_SPEED * BALL_SPEED - ball.vel.x * ball.vel.x)
            .sqrt()
            .copysign(1.0); // always bounces up
    }

    // ── Ball fell below paddle → restart ────────────────────────────────────
    if by + half_ball < -HALF_H {
        ball_tf.translation = Vec3::ZERO;
        ball.vel = Vec2::new(1.0, 1.0).normalize() * BALL_SPEED;
    }
}
