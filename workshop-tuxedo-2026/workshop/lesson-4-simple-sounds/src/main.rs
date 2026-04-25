//! Minimal Bevy 0.18 audio demo.
//!
//! Music keys (toggle):
//!   A = intro.ogg, S = level-x.ogg, D = outro.ogg
//! SFX keys (one-shot):
//!   Q = laser-single-a, W = brick-c6, E = emp, R = siren,
//!   T = slide-open, Y = trampoline-hi, U = win
//!
//! Drop this `src/main.rs` next to your `assets/` folder (which must contain
//! `font/`, `music/` and `sfx/` as in the supplied overview.txt).

use bevy::prelude::*;

// --- Music --------------------------------------------------------------

/// Marks an entity that is currently playing a music track.
/// We keep the key code so we can detect "same key pressed again -> stop".
#[derive(Component)]
struct Music(KeyCode);

const MUSIC_TRACKS: &[(KeyCode, &str)] = &[
    (KeyCode::KeyA, "music/intro.ogg"),
    (KeyCode::KeyS, "music/level-x.ogg"),
    (KeyCode::KeyD, "music/outro.ogg"),
];

// --- SFX ----------------------------------------------------------------

const SFX_SAMPLES: &[(KeyCode, &str)] = &[
    (KeyCode::KeyQ, "sfx/laser-single-a.ogg"),
    (KeyCode::KeyW, "sfx/brick-c6.ogg"),
    (KeyCode::KeyE, "sfx/emp.ogg"),
    (KeyCode::KeyR, "sfx/siren.ogg"),
    (KeyCode::KeyT, "sfx/slide-open.ogg"),
    (KeyCode::KeyY, "sfx/trampoline-hi.ogg"),
    (KeyCode::KeyU, "sfx/win.ogg"),
];

// --- App ----------------------------------------------------------------

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_music, handle_sfx))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let font = asset_server.load("font/unscii-8.ttf");
    let text_font = TextFont {
        font,
        font_size: 20.0,
        ..default()
    };

    let mut lines = String::from("MUSIC (toggle):\n");
    for (key, path) in MUSIC_TRACKS {
        lines.push_str(&format!("  {}  {}\n", key_label(*key), path));
    }
    lines.push_str("\nSFX (one-shot):\n");
    for (key, path) in SFX_SAMPLES {
        lines.push_str(&format!("  {}  {}\n", key_label(*key), path));
    }

    commands.spawn((
        Text::new(lines),
        text_font,
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
    ));
}

fn key_label(key: KeyCode) -> &'static str {
    match key {
        KeyCode::KeyA => "A",
        KeyCode::KeyS => "S",
        KeyCode::KeyD => "D",
        KeyCode::KeyQ => "Q",
        KeyCode::KeyW => "W",
        KeyCode::KeyE => "E",
        KeyCode::KeyR => "R",
        KeyCode::KeyT => "T",
        KeyCode::KeyY => "Y",
        KeyCode::KeyU => "U",
        _ => "?",
    }
}

/// Toggle music tracks. Pressing the same key again stops it; pressing a
/// different music key stops whatever is playing and starts the new track.
fn handle_music(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    playing: Query<(Entity, &Music)>,
) {
    for (key, path) in MUSIC_TRACKS {
        if keys.just_pressed(*key) {
            // Is this exact track already playing? If so, stop it (toggle off).
            let mut toggled_off = false;
            for (entity, music) in &playing {
                if music.0 == *key {
                    commands.entity(entity).despawn();
                    toggled_off = true;
                } else {
                    // A different track is playing; stop it first.
                    commands.entity(entity).despawn();
                }
            }
            if !toggled_off {
                commands.spawn((
                    AudioPlayer::new(asset_server.load(*path)),
                    PlaybackSettings::LOOP,
                    Music(*key),
                ));
            }
            break; // only handle one music key per frame
        }
    }
}

/// Fire-and-forget sfx playback. The entity despawns itself when finished.
fn handle_sfx(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
) {
    for (key, path) in SFX_SAMPLES {
        if keys.just_pressed(*key) {
            commands.spawn((
                AudioPlayer::new(asset_server.load(*path)),
                PlaybackSettings::DESPAWN,
            ));
        }
    }
}
