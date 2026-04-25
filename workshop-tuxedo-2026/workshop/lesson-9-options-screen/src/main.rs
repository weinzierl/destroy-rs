use bevy::prelude::*;

// ── Main game phase state ──────────────────────────────────────────

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum Phase {
    #[default]
    Title,
    Story,
    Playing,
    GameOver,
    EndStory,
    HighScore,
}

// ── Level tracking (only meaningful during Playing) ────────────────

#[derive(Resource)]
struct Level(u32);

const MAX_LEVEL: u32 = 3;

// ── Options overlay (independent of Phase) ─────────────────────────

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum Overlay {
    #[default]
    None,
    Options,
}

// ── Markers ────────────────────────────────────────────────────────

#[derive(Component)]
struct ScreenLabel;

#[derive(Component)]
struct OptionsPanel;

// ── Setup ──────────────────────────────────────────────────────────

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Game Phases Demo".into(),
                resolution: (800, 600).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<Phase>()
        .init_state::<Overlay>()
        .insert_resource(Level(1))
        .add_systems(Startup, setup_camera)
        // Enter / exit for each phase
        .add_systems(OnEnter(Phase::Title),     show("=== TITLE SCREEN ===\n\nPress SPACE to start"))
        .add_systems(OnEnter(Phase::Story),     show("=== STORY ===\n\nThe kingdom is in peril!\n\nPress SPACE to continue"))
        .add_systems(OnEnter(Phase::Playing),   show_level)
        .add_systems(OnEnter(Phase::GameOver),  show("=== GAME OVER ===\n\nYou failed!\n\nPress SPACE for High Scores"))
        .add_systems(OnEnter(Phase::EndStory),  show("=== THE END ===\n\nYou saved the kingdom!\n\nPress SPACE for High Scores"))
        .add_systems(OnEnter(Phase::HighScore), show("=== HIGH SCORES ===\n\n1. AAA  9999\n2. BBB  7777\n3. CCC  5555\n\nPress SPACE to return to Title"))
        .add_systems(OnExit(Phase::Title),     clear_label)
        .add_systems(OnExit(Phase::Story),     clear_label)
        .add_systems(OnExit(Phase::Playing),   clear_label)
        .add_systems(OnExit(Phase::GameOver),  clear_label)
        .add_systems(OnExit(Phase::EndStory),  clear_label)
        .add_systems(OnExit(Phase::HighScore), clear_label)
        // Input per phase
        .add_systems(Update, nav_title.run_if(in_state(Phase::Title)))
        .add_systems(Update, nav_story.run_if(in_state(Phase::Story)))
        .add_systems(Update, nav_playing.run_if(in_state(Phase::Playing)))
        .add_systems(Update, nav_gameover.run_if(in_state(Phase::GameOver)))
        .add_systems(Update, nav_endstory.run_if(in_state(Phase::EndStory)))
        .add_systems(Update, nav_highscore.run_if(in_state(Phase::HighScore)))
        // Options overlay – always active
        .add_systems(Update, toggle_options)
        .add_systems(OnEnter(Overlay::Options), show_options)
        .add_systems(OnExit(Overlay::Options),  hide_options)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// ── Screen helpers ─────────────────────────────────────────────────

fn show(text: &'static str) -> impl Fn(Commands) {
    move |mut commands: Commands| {
        spawn_label(&mut commands, text);
    }
}

fn spawn_label(commands: &mut Commands, text: &str) {
    commands.spawn((
        ScreenLabel,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Text::new(text.to_string()),
            TextFont { font_size: 30.0, ..default() },
            TextColor(Color::WHITE),
            TextLayout { justify: Justify::Center, ..default() },
        )],
    ));
}

fn show_level(mut commands: Commands, level: Res<Level>) {
    let text = format!(
        "=== LEVEL {} of {} ===\n\n\
         SPACE  = complete level\n\
         F      = fail (Game Over)\n\
         Escape = Options",
        level.0, MAX_LEVEL
    );
    spawn_label(&mut commands, &text);
}

fn clear_label(mut commands: Commands, q: Query<Entity, With<ScreenLabel>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}

// ── Navigation systems ─────────────────────────────────────────────

fn nav_title(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<Phase>>) {
    if keys.just_pressed(KeyCode::Space) {
        next.set(Phase::Story);
    }
}

fn nav_story(
    keys: Res<ButtonInput<KeyCode>>,
    mut next: ResMut<NextState<Phase>>,
    mut level: ResMut<Level>,
) {
    if keys.just_pressed(KeyCode::Space) {
        level.0 = 1;
        next.set(Phase::Playing);
    }
}

fn nav_playing(
    keys: Res<ButtonInput<KeyCode>>,
    mut next: ResMut<NextState<Phase>>,
    mut level: ResMut<Level>,
) {
    if keys.just_pressed(KeyCode::KeyF) {
        next.set(Phase::GameOver);
    } else if keys.just_pressed(KeyCode::Space) {
        if level.0 >= MAX_LEVEL {
            next.set(Phase::EndStory);
        } else {
            level.0 += 1;
            // Re-enter Playing with incremented level.
            // Bevy won't re-trigger OnEnter for the same state,
            // so we briefly go through a "trampoline" by using
            // the despawn + respawn approach below instead.
            next.set(Phase::Playing);
        }
    }
}

fn nav_gameover(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<Phase>>) {
    if keys.just_pressed(KeyCode::Space) {
        next.set(Phase::HighScore);
    }
}

fn nav_endstory(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<Phase>>) {
    if keys.just_pressed(KeyCode::Space) {
        next.set(Phase::HighScore);
    }
}

fn nav_highscore(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<Phase>>) {
    if keys.just_pressed(KeyCode::Space) {
        next.set(Phase::Title);
    }
}

// ── Options overlay ────────────────────────────────────────────────

fn toggle_options(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<Overlay>>,
    mut next: ResMut<NextState<Overlay>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match state.get() {
            Overlay::None    => next.set(Overlay::Options),
            Overlay::Options => next.set(Overlay::None),
        }
    }
}

fn show_options(mut commands: Commands) {
    commands.spawn((
        OptionsPanel,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            right: Val::Px(20.0),
            padding: UiRect::all(Val::Px(24.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.1, 0.1, 0.3, 0.92)),
        children![(
            Text::new("OPTIONS\n\nSound: ON\nMusic: ON\nDifficulty: HARD\n\nPress Escape to close"),
            TextFont { font_size: 22.0, ..default() },
            TextColor(Color::srgb(0.9, 0.9, 0.2)),
        )],
    ));
}

fn hide_options(mut commands: Commands, q: Query<Entity, With<OptionsPanel>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}
