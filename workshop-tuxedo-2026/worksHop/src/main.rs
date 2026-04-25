//! Minimal Bevy 0.18 audio demo with a polyphony limit.
//!
//! Music keys (independent toggles):
//!   A = intro.ogg, S = level-x.ogg, D = outro.ogg
//! SFX keys (one-shot):
//!   Q = laser-single-a, W = brick-c6, E = emp, R = siren,
//!   T = slide-open, Y = trampoline-hi, U = win
//! Number keys set the maximum simultaneous *sfx* voices (music is
//! unaffected):
//!   0 = unlimited, 1..9 = that many. When the cap is reached the
//!   oldest sfx voice is cut off (round-robin steal).

use bevy::prelude::*;

// --- Music --------------------------------------------------------------

/// Marker for music tracks; carries the key so we can detect "same key
/// pressed again -> stop".
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

// --- Voice management ---------------------------------------------------

/// Marker for sfx voices that count toward the channel budget. Carries a
/// monotonically increasing `ord`; the smallest ord is the oldest voice.
/// Music tracks deliberately do NOT carry this component.
#[derive(Component)]
struct Voice {
    ord: u64,
}

/// Hands out monotonically increasing voice ordinals.
#[derive(Resource, Default)]
struct VoiceCounter(u64);

impl VoiceCounter {
    fn next(&mut self) -> u64 {
        let n = self.0;
        self.0 = self.0.wrapping_add(1);
        n
    }
}

/// `0` means unlimited; any other value caps the simultaneous voices.
#[derive(Resource)]
struct MaxChannels(usize);

impl Default for MaxChannels {
    fn default() -> Self {
        MaxChannels(0)
    }
}

/// Status line component (so we can update it each frame).
#[derive(Component)]
struct StatusText;

// --- App ----------------------------------------------------------------

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<VoiceCounter>()
        .init_resource::<MaxChannels>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_max_channels,
                enforce_voice_limit,
                handle_music,
                handle_sfx,
                update_status,
            )
                .chain(),
        )
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

    let mut lines = String::from("MUSIC (toggle on/off):\n");
    for (key, path) in MUSIC_TRACKS {
        lines.push_str(&format!("  {}  {}\n", key_label(*key), path));
    }
    lines.push_str("\nSFX (one-shot):\n");
    for (key, path) in SFX_SAMPLES {
        lines.push_str(&format!("  {}  {}\n", key_label(*key), path));
    }
    lines.push_str("\nSFX channels:\n  0 = unlimited\n  1..9 = max simultaneous sfx\n");

    // Static legend
    commands.spawn((
        Text::new(lines),
        text_font.clone(),
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
    ));

    // Live status line at the bottom
    commands.spawn((
        Text::new(""),
        text_font,
        TextColor(Color::srgb(1.0, 0.9, 0.4)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        StatusText,
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

const DIGIT_KEYS: &[(KeyCode, usize)] = &[
    (KeyCode::Digit0, 0),
    (KeyCode::Digit1, 1),
    (KeyCode::Digit2, 2),
    (KeyCode::Digit3, 3),
    (KeyCode::Digit4, 4),
    (KeyCode::Digit5, 5),
    (KeyCode::Digit6, 6),
    (KeyCode::Digit7, 7),
    (KeyCode::Digit8, 8),
    (KeyCode::Digit9, 9),
];

/// Read the number-row keys to update the polyphony cap.
fn handle_max_channels(
    keys: Res<ButtonInput<KeyCode>>,
    mut max_channels: ResMut<MaxChannels>,
) {
    for (key, n) in DIGIT_KEYS {
        if keys.just_pressed(*key) {
            max_channels.0 = *n;
        }
    }
}

/// If the current voice count exceeds the cap, despawn the oldest voices
/// (smallest `ord`) until we're back under it. Runs every frame so that
/// lowering the cap takes effect immediately, even without a new keypress.
fn enforce_voice_limit(
    mut commands: Commands,
    max_channels: Res<MaxChannels>,
    voices: Query<(Entity, &Voice)>,
) {
    let max = max_channels.0;
    if max == 0 {
        return;
    }
    let mut all: Vec<(Entity, u64)> = voices.iter().map(|(e, v)| (e, v.ord)).collect();
    if all.len() <= max {
        return;
    }
    // Sort oldest-first and drop the excess from the front.
    all.sort_by_key(|(_, ord)| *ord);
    let to_drop = all.len() - max;
    for (entity, _) in all.into_iter().take(to_drop) {
        commands.entity(entity).despawn();
    }
}

/// Toggle music tracks independently. Music does not consume a voice slot
/// — the channel cap only applies to sfx.
fn handle_music(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    playing: Query<(Entity, &Music)>,
) {
    for (key, path) in MUSIC_TRACKS {
        if !keys.just_pressed(*key) {
            continue;
        }
        // Look for an entity already playing this exact track.
        let mut found = None;
        for (entity, music) in &playing {
            if music.0 == *key {
                found = Some(entity);
                break;
            }
        }
        match found {
            Some(entity) => {
                info!("music toggle OFF: {:?}", path);
                commands.entity(entity).despawn();
            }
            None => {
                info!("music toggle ON: {:?}", path);
                commands.spawn((
                    AudioPlayer::new(asset_server.load(*path)),
                    PlaybackSettings::LOOP,
                    Music(*key),
                ));
            }
        }
    }
}

/// Fire-and-forget sfx playback. Each press spawns a new voice; the entity
/// despawns itself when the sample finishes.
fn handle_sfx(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut counter: ResMut<VoiceCounter>,
) {
    for (key, path) in SFX_SAMPLES {
        if keys.just_pressed(*key) {
            commands.spawn((
                AudioPlayer::new(asset_server.load(*path)),
                PlaybackSettings::DESPAWN,
                Voice { ord: counter.next() },
            ));
        }
    }
}

fn update_status(
    max_channels: Res<MaxChannels>,
    voices: Query<&Voice>,
    mut status: Single<&mut Text, With<StatusText>>,
) {
    let active_sfx = voices.iter().count();
    let cap = if max_channels.0 == 0 {
        "unlimited".to_string()
    } else {
        max_channels.0.to_string()
    };
    status.0 = format!("sfx voices: {active_sfx}   max: {cap}");
}
