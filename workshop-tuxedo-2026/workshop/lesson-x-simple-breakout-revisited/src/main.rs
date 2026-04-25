use bevy::prelude::*;

// ── layout constants ───────────────────────────────────────────────
const TILE: f32 = 32.0;
const BRICK_W: f32 = 64.0;
const BRICK_H: f32 = 32.0;
const BALL_SIZE: f32 = 16.0;
const BALL_R: f32 = BALL_SIZE / 2.0;
const BALL_SPEED: f32 = 320.0;
const PADDLE_SPEED: f32 = 550.0;

const COLS: usize = 13;
const FIELD_W: f32 = COLS as f32 * BRICK_W; // 832

// inner playing-field edges (collision boundaries)
const LEFT: f32 = -(FIELD_W / 2.0); // -416
const RIGHT: f32 = FIELD_W / 2.0; //  416
const TOP: f32 = 300.0;
const BOTTOM: f32 = -350.0; // ball-death line

// paddle
const PADDLE_Y: f32 = -280.0;
const PADDLE_SEGS: usize = 5; // lpipecap + 3×hpipe + rpipecap
const PADDLE_W: f32 = PADDLE_SEGS as f32 * TILE; // 160
const PADDLE_H: f32 = TILE; // 32

// wall placement
const WALL_LX: f32 = LEFT - TILE / 2.0; // -432
const WALL_RX: f32 = RIGHT + TILE / 2.0; //  432
const WALL_TY: f32 = TOP + TILE / 2.0; //  316

// shadow offset applied to every shadow sprite
const SH_DX: f32 = 4.0;
const SH_DY: f32 = -4.0;
const SH_DZ: f32 = -0.5;

// brick rows
const ROWS: usize = 4;
const BRICK_START_Y: f32 = 200.0;

// z-layers (higher = closer to camera)
const Z_SHADOW: f32 = 0.0;
const Z_WALL: f32 = 1.0;
const Z_BRICK: f32 = 2.0;
const Z_PADDLE: f32 = 3.0;
const Z_BALL: f32 = 4.0;

// ── components & resources ─────────────────────────────────────────

#[derive(Component)]
struct Ball {
    velocity: Vec2,
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Brick;

#[derive(Resource)]
struct BrickHitSound(Handle<AudioSource>);

// ── entry point ────────────────────────────────────────────────────

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "asset".into(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Arkanoid".into(),
                        resolution: (960u32, 720u32).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (move_paddle, move_ball_and_collide).chain())
        .run();
}

// ── setup ──────────────────────────────────────────────────────────

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // camera
    commands.spawn(Camera2d);

    // background music (looping)
    commands.spawn((
        AudioPlayer::new(asset_server.load("music/levelx.ogg")),
        PlaybackSettings::LOOP,
    ));

    // brick-hit sound effect (kept as resource)
    commands.insert_resource(BrickHitSound(asset_server.load("sfx/brick-a6.ogg")));

    spawn_walls(&mut commands, &asset_server);
    spawn_bricks(&mut commands, &asset_server);
    spawn_paddle(&mut commands, &asset_server);
    spawn_ball(&mut commands, &asset_server);
}

// ── wall helpers ───────────────────────────────────────────────────

/// Spawn one static wall tile (object + shadow). No parent entity needed
/// because walls never move.
fn wall_tile(cmd: &mut Commands, a: &AssetServer, obj: &'static str, shd: &'static str, x: f32, y: f32, flip: bool) {
    cmd.spawn((
        Sprite {
            image: a.load(obj),
            custom_size: Some(Vec2::splat(TILE)),
            flip_x: flip,
            ..default()
        },
        Transform::from_xyz(x, y, Z_WALL),
    ));
    cmd.spawn((
        Sprite {
            image: a.load(shd),
            custom_size: Some(Vec2::splat(TILE)),
            flip_x: flip,
            ..default()
        },
        Transform::from_xyz(x + SH_DX, y + SH_DY, Z_SHADOW),
    ));
}

// ── wall construction ──────────────────────────────────────────────

fn spawn_walls(cmd: &mut Commands, a: &AssetServer) {
    // ── corners ────────────────────────────────────────────────────
    wall_tile(cmd, a, "sprite/tlbendobject.png", "sprite/tlbendshadow.png", WALL_LX, WALL_TY, false);
    wall_tile(cmd, a, "sprite/trbendobject.png", "sprite/trbendshadow.png", WALL_RX, WALL_TY, false);

    // ── top horizontal pipe run ────────────────────────────────────
    let h_start = WALL_LX + TILE;
    let h_count = ((WALL_RX - WALL_LX) / TILE) as i32 - 1;
    for i in 0..h_count {
        let x = h_start + i as f32 * TILE;
        let (obj, shd) = if i % 7 == 3 {
            ("sprite/hellbowobject.png", "sprite/hellbowshadow.png")
        } else {
            ("sprite/hpipeobject.png", "sprite/hpipeshadow.png")
        };
        wall_tile(cmd, a, obj, shd, x, WALL_TY, false);
    }

    // ── vertical pipe runs (left & right) ──────────────────────────
    let v_count: i32 = 22;
    for i in 0..v_count {
        let y = WALL_TY - (i + 1) as f32 * TILE;
        let (obj, shd) = if i % 7 == 3 {
            ("sprite/vellboobject.png", "sprite/vellboshadow.png")
        } else {
            ("sprite/vpipeobject.png", "sprite/vpipeshadow.png")
        };
        wall_tile(cmd, a, obj, shd, WALL_LX, y, false);
        wall_tile(cmd, a, obj, shd, WALL_RX, y, false);
    }

    // ── bottom caps ────────────────────────────────────────────────
    let cap_y = WALL_TY - (v_count + 1) as f32 * TILE;
    wall_tile(cmd, a, "sprite/bppipecapobject.png", "sprite/bppipecapshadow.png", WALL_LX, cap_y, false);
    wall_tile(cmd, a, "sprite/bppipecapobject.png", "sprite/bppipecapshadow.png", WALL_RX, cap_y, true);
}

// ── brick construction ─────────────────────────────────────────────

const BRICK_OBJ: [&str; ROWS] = [
    "sprite/brick1object.png",
    "sprite/brick2object.png",
    "sprite/brick3object.png",
    "sprite/brick1object.png", // row 4 reuses brick1
];
const BRICK_SHD: [&str; ROWS] = [
    "sprite/brick1shadow.png",
    "sprite/brick2shadow.png",
    "sprite/brick3shadow.png",
    "sprite/brick1shadow.png",
];

fn spawn_bricks(cmd: &mut Commands, a: &AssetServer) {
    let brick_size = Vec2::new(BRICK_W, BRICK_H);
    for row in 0..ROWS {
        let y = BRICK_START_Y - row as f32 * BRICK_H;
        for col in 0..COLS {
            let x = LEFT + BRICK_W / 2.0 + col as f32 * BRICK_W;
            cmd.spawn((
                Brick,
                Transform::from_xyz(x, y, 0.0),
                Visibility::default(),
            ))
            .with_children(|p| {
                p.spawn((
                    Sprite {
                        image: a.load(BRICK_OBJ[row]),
                        custom_size: Some(brick_size),
                        ..default()
                    },
                    Transform::from_xyz(0.0, 0.0, Z_BRICK),
                ));
                p.spawn((
                    Sprite {
                        image: a.load(BRICK_SHD[row]),
                        custom_size: Some(brick_size),
                        ..default()
                    },
                    Transform::from_xyz(SH_DX, SH_DY, Z_SHADOW),
                ));
            });
        }
    }
}

// ── paddle construction ────────────────────────────────────────────

fn spawn_paddle(cmd: &mut Commands, a: &AssetServer) {
    let half = (PADDLE_SEGS as f32 - 1.0) / 2.0; // 2.0
    let tile_size = Vec2::splat(TILE);

    cmd.spawn((
        Paddle,
        Transform::from_xyz(0.0, PADDLE_Y, 0.0),
        Visibility::default(),
    ))
    .with_children(|p| {
        for i in 0..PADDLE_SEGS {
            let lx = (i as f32 - half) * TILE;
            let (obj, shd) = match i {
                0 => ("sprite/lpipecapobject.png", "sprite/lpipecapshadow.png"),
                i if i == PADDLE_SEGS - 1 => {
                    ("sprite/rpipecapobject.png", "sprite/rpipecapshadow.png")
                }
                _ => ("sprite/hpipeobject.png", "sprite/hpipeshadow.png"),
            };
            p.spawn((
                Sprite {
                    image: a.load(obj),
                    custom_size: Some(tile_size),
                    ..default()
                },
                Transform::from_xyz(lx, 0.0, Z_PADDLE),
            ));
            p.spawn((
                Sprite {
                    image: a.load(shd),
                    custom_size: Some(tile_size),
                    ..default()
                },
                Transform::from_xyz(lx + SH_DX, SH_DY, Z_PADDLE + SH_DZ),
            ));
        }
    });
}

// ── ball construction ──────────────────────────────────────────────

fn spawn_ball(cmd: &mut Commands, a: &AssetServer) {
    let velocity = Vec2::new(0.5, 1.0).normalize() * BALL_SPEED;
    let ball_size = Vec2::splat(BALL_SIZE);

    cmd.spawn((
        Ball { velocity },
        Transform::from_xyz(0.0, PADDLE_Y + 40.0, 0.0),
        Visibility::default(),
    ))
    .with_children(|p| {
        p.spawn((
            Sprite {
                image: a.load("sprite/ballobject.png"),
                custom_size: Some(ball_size),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, Z_BALL),
        ));
        p.spawn((
            Sprite {
                image: a.load("sprite/ballshadow.png"),
                custom_size: Some(ball_size),
                ..default()
            },
            Transform::from_xyz(SH_DX, SH_DY, Z_BALL + SH_DZ),
        ));
    });
}

// ── paddle input ───────────────────────────────────────────────────

fn move_paddle(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut q: Query<&mut Transform, With<Paddle>>,
) {
    let Ok(mut tf) = q.single_mut() else {
        return;
    };

    let mut dx = 0.0;
    if input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA) {
        dx -= 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD) {
        dx += 1.0;
    }
    if dx == 0.0 {
        return;
    }

    tf.translation.x += dx * PADDLE_SPEED * time.delta_secs();

    let half = PADDLE_W / 2.0;
    tf.translation.x = tf.translation.x.clamp(LEFT + half, RIGHT - half);
}

// ── ball movement & all collisions ─────────────────────────────────

fn move_ball_and_collide(
    mut commands: Commands,
    time: Res<Time>,
    sfx: Res<BrickHitSound>,
    mut ball_q: Query<(&mut Ball, &mut Transform), (Without<Paddle>, Without<Brick>)>,
    paddle_q: Query<&Transform, (With<Paddle>, Without<Ball>, Without<Brick>)>,
    brick_q: Query<(Entity, &Transform), (With<Brick>, Without<Ball>, Without<Paddle>)>,
) {
    let Ok((mut ball, mut bt)) = ball_q.single_mut() else {
        return;
    };
    let Ok(pt) = paddle_q.single() else {
        return;
    };

    let dt = time.delta_secs();

    // ── integrate position ─────────────────────────────────────────
    bt.translation.x += ball.velocity.x * dt;
    bt.translation.y += ball.velocity.y * dt;

    // ── wall collisions ────────────────────────────────────────────
    if bt.translation.x - BALL_R < LEFT {
        bt.translation.x = LEFT + BALL_R;
        ball.velocity.x = ball.velocity.x.abs();
    }
    if bt.translation.x + BALL_R > RIGHT {
        bt.translation.x = RIGHT - BALL_R;
        ball.velocity.x = -(ball.velocity.x.abs());
    }
    if bt.translation.y + BALL_R > TOP {
        bt.translation.y = TOP - BALL_R;
        ball.velocity.y = -(ball.velocity.y.abs());
    }

    // ── paddle collision (only when ball moves downward) ───────────
    if ball.velocity.y < 0.0 {
        let paddle_pos = Vec2::new(pt.translation.x, pt.translation.y);
        let paddle_half = Vec2::new(PADDLE_W / 2.0, PADDLE_H / 2.0);
        let ball_pos = Vec2::new(bt.translation.x, bt.translation.y);

        if let Some(normal) = circle_aabb(ball_pos, BALL_R, paddle_pos, paddle_half) {
            // push ball out of paddle
            let closest = closest_on_aabb(ball_pos, paddle_pos, paddle_half);
            let pen = BALL_R - (ball_pos - closest).length();
            if pen > 0.0 {
                bt.translation.x += normal.x * pen;
                bt.translation.y += normal.y * pen;
            }

            // angled reflection: hit position controls outgoing angle
            let offset = ((bt.translation.x - pt.translation.x) / (PADDLE_W / 2.0)).clamp(-1.0, 1.0);
            let angle = offset * std::f32::consts::FRAC_PI_3; // ±60°
            let speed = ball.velocity.length();
            ball.velocity = Vec2::new(speed * angle.sin(), speed * angle.cos());
        }
    }

    // ── brick collisions (one per frame for simplicity) ────────────
    let brick_half = Vec2::new(BRICK_W / 2.0, BRICK_H / 2.0);
    let ball_pos = Vec2::new(bt.translation.x, bt.translation.y);

    for (entity, brick_tf) in &brick_q {
        let brick_pos = Vec2::new(brick_tf.translation.x, brick_tf.translation.y);

        if let Some(normal) = circle_aabb(ball_pos, BALL_R, brick_pos, brick_half) {
            // reflect velocity
            let d = ball.velocity.dot(normal);
            if d < 0.0 {
                ball.velocity -= 2.0 * d * normal;
            }

            // push ball out of brick
            let closest = closest_on_aabb(ball_pos, brick_pos, brick_half);
            let pen = BALL_R - (ball_pos - closest).length();
            if pen > 0.0 {
                bt.translation.x += normal.x * pen;
                bt.translation.y += normal.y * pen;
            }

            // destroy brick (children auto-despawn thanks to ChildOf)
            commands.entity(entity).despawn();

            // play hit sound
            commands.spawn((
                AudioPlayer::new(sfx.0.clone()),
                PlaybackSettings::DESPAWN,
            ));

            break; // one brick per frame
        }
    }

    // ── ball death ─────────────────────────────────────────────────
    if bt.translation.y + BALL_R < BOTTOM {
        bt.translation.x = 0.0;
        bt.translation.y = PADDLE_Y + 40.0;
        ball.velocity = Vec2::new(
            if rand_sign() { 0.5 } else { -0.5 },
            1.0,
        )
        .normalize()
            * BALL_SPEED;
    }
}

/// Poor-man's coin flip using the fractional part of elapsed time.
fn rand_sign() -> bool {
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    t % 2 == 0
}

// ── collision helpers ──────────────────────────────────────────────

/// Closest point on an AABB to `p`.
fn closest_on_aabb(p: Vec2, center: Vec2, half: Vec2) -> Vec2 {
    let d = p - center;
    center + d.clamp(-half, half)
}

/// Circle-vs-AABB intersection test.
/// Returns the collision normal (pointing from AABB toward circle centre),
/// or `None` if there is no overlap.
fn circle_aabb(circle: Vec2, r: f32, aabb_center: Vec2, aabb_half: Vec2) -> Option<Vec2> {
    let closest = closest_on_aabb(circle, aabb_center, aabb_half);
    let delta = circle - closest;
    let dist_sq = delta.length_squared();

    if dist_sq < r * r {
        if dist_sq > f32::EPSILON {
            Some(delta / dist_sq.sqrt())
        } else {
            // circle centre is inside the AABB – push upward
            Some(Vec2::Y)
        }
    } else {
        None
    }
}
