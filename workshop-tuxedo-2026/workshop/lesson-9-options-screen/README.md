# 🎛️ Lektion 9 – Optionsbildschirm & nicht-linearer Spielfluss

> *"Ein Spiel ist kein lineares Programm. Es ist eine Zustandsmaschine."*

Bisher ist unser Spiel linear: Es startet, man spielt, man verliert oder gewinnt,
fertig. Das entspricht aber nicht dem, wie echte Spiele funktionieren. Echte Spiele
haben ein Hauptmenü, Pausenbildschirme, Optionen, Credits – Zustände, zwischen denen
man hin und her wechseln kann, manchmal mitten im Spiel.

In dieser letzten Lektion lernen wir Bevys **State-System** kennen, das genau dafür
gemacht ist. Als konkretes Beispiel bauen wir einen Optionsbildschirm, der jederzeit
aufgerufen werden kann – auch während das Spiel läuft.

---

## Was ist ein Zustand (State)?

Ein **Zustand** (State) ist ein benannter Modus, in dem sich das Spiel befindet.
Beispiele:

- `MainMenu` – Das Spiel zeigt das Hauptmenü
- `Playing` – Das Spiel läuft
- `Paused` – Das Spiel ist pausiert
- `Options` – Der Optionsbildschirm ist geöffnet
- `GameOver` – Das Spiel ist beendet

Zu jedem Zeitpunkt ist das Spiel in genau einem Zustand. Beim Wechsel zwischen
Zuständen können Systeme automatisch aktiviert und deaktiviert werden, und
bestimmte Setups und Teardowns können ausgeführt werden.

---

## Zustände in Bevy definieren

```rust
use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    Options,
    GameOver,
}
```

Das `derive`-Makro `States` teilt Bevy mit, dass dieser Enum als State-Typ
verwendet werden darf. `Default` bestimmt den Startzustand.

Registrieren in der App:

```rust
App::new()
    .add_plugins(DefaultPlugins)
    .init_state::<GameState>()
    // ...
```

---

## Systeme an Zustände binden

Mit `run_if(in_state(...))` läuft ein System nur, wenn das Spiel im angegebenen
Zustand ist:

```rust
.add_systems(Update, (
    move_paddle.run_if(in_state(GameState::Playing)),
    move_ball.run_if(in_state(GameState::Playing)),
    ball_block_collision.run_if(in_state(GameState::Playing)),
))
.add_systems(Update,
    update_options_menu.run_if(in_state(GameState::Options))
)
```

Im `Paused`- oder `Options`-Zustand laufen `move_paddle` und `move_ball` nicht –
das Spiel friert quasi ein.

---

## Auf Zustandsübergänge reagieren: `OnEnter` und `OnExit`

Bevy hat spezielle Schedules für Zustandsübergänge:

```rust
.add_systems(OnEnter(GameState::Playing),   setup_game)
.add_systems(OnExit(GameState::Playing),    cleanup_game)
.add_systems(OnEnter(GameState::Options),   setup_options_menu)
.add_systems(OnExit(GameState::Options),    cleanup_options_menu)
.add_systems(OnEnter(GameState::GameOver),  show_game_over_screen)
```

`OnEnter` läuft genau einmal, wenn der Zustand betreten wird.
`OnExit` läuft genau einmal, wenn der Zustand verlassen wird.

Das ist ideal für Setup und Teardown: Wir spawnen Menü-Entities in `OnEnter`
und despawnen sie in `OnExit` – ohne dass wir uns manuell darum kümmern müssen.

---

## Den Zustand wechseln

```rust
fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Escape: Optionen öffnen / schließen
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Playing => next_state.set(GameState::Options),
            GameState::Options => next_state.set(GameState::Playing),
            GameState::MainMenu => { /* ignorieren */ }
            _ => {}
        }
    }

    // Enter im Hauptmenü: Spiel starten
    if keyboard.just_pressed(KeyCode::Enter) {
        if *current_state.get() == GameState::MainMenu {
            next_state.set(GameState::Playing);
        }
    }
}
```

`NextState::set()` plant den Zustandswechsel für das Ende des aktuellen Frames.
Der Wechsel passiert nicht sofort, sondern sauber zwischen den Frames.

---

## Den Optionsbildschirm bauen

Wir bauen einen einfachen Optionsbildschirm mit Bevy UI.

### Marker-Component für Optionen-Entities

```rust
#[derive(Component)]
struct OptionsMenuRoot;
```

Alle Entities des Optionsbildschirms bekommen dieses Marker-Component. Beim
Verlassen des Options-Zustands despawnen wir einfach alle Entities mit diesem Marker.

### Setup des Optionsbildschirms

```rust
fn setup_options_menu(mut commands: Commands) {
    commands
        .spawn((
            OptionsMenuRoot,
            Node {
                width:           Val::Percent(100.0),
                height:          Val::Percent(100.0),
                flex_direction:  FlexDirection::Column,
                align_items:     AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap:         Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        ))
        .with_children(|parent| {
            // Titel
            parent.spawn((
                Text::new("Optionen"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Lautstärke-Anzeige
            parent.spawn((
                Text::new("Musik-Lautstärke: [◄ ─────── ►]"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));

            // Hinweis
            parent.spawn((
                Text::new("ESC – Zurück zum Spiel"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
            ));
        });
}
```

### Teardown

```rust
fn cleanup_options_menu(
    mut commands: Commands,
    query: Query<Entity, With<OptionsMenuRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
```

`despawn_recursive()` löscht die Entity und alle ihre Kinder-Entities – also den
kompletten UI-Baum mit einem einzigen Aufruf.

---

## Lautstärke im Optionsmenü regeln

```rust
#[derive(Resource)]
struct AudioSettings {
    music_volume:   f32,
    effects_volume: f32,
}

fn update_options_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<AudioSettings>,
    music_query: Query<&AudioSink, With<BackgroundMusic>>,
) {
    // Pfeiltasten regeln die Musiklautstärke
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        settings.music_volume = (settings.music_volume - 0.1).max(0.0);
    }
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        settings.music_volume = (settings.music_volume + 0.1).min(1.0);
    }

    // Lautstärke des laufenden Musik-Sinks aktualisieren
    for sink in &music_query {
        sink.set_volume(settings.music_volume);
    }
}
```

`AudioSink` ist Bevys Schnittstelle zu einem laufenden Audio-Stream. Damit können
wir Lautstärke und Wiedergabe steuern, während der Sound läuft.

---

## Substates: Pausen-Hierarchie

Manchmal möchte man Zustände schachteln. Zum Beispiel: "Optionen" soll als
Untermenü sowohl von `Playing` als auch von `MainMenu` erreichbar sein, und beim
Zurückgehen soll der richtige übergeordnete Zustand wieder aktiv sein.

Bevy unterstützt **Substates** mit `SubStates`:

```rust
#[derive(SubStates, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[source(GameState = GameState::Playing)]
enum PlayingState {
    #[default]
    Running,
    Paused,
    Options,
}
```

Der Substate `PlayingState` ist nur aktiv, wenn der übergeordnete State
`GameState::Playing` aktiv ist. Das macht verschachtelte Zustandsmaschinen
sehr sauber und ausdrucksstark.

---

## Der komplette Zustandsgraph unseres Spiels

```
           ┌─────────────┐
           │  MainMenu   │◄────────────────────────┐
           └──────┬──────┘                          │
                  │ Enter                           │
                  ▼                                 │
           ┌─────────────┐   Escape   ┌──────────┐  │
           │   Playing   │◄──────────►│ Options  │  │
           └──────┬──────┘            └──────────┘  │
                  │                                  │
          ┌───────┴───────┐                         │
          │               │                         │
    ┌─────▼──────┐  ┌─────▼──────┐                 │
    │  Gewonnen! │  │ Verloren!  │                  │
    │ (GameOver) │  │ (GameOver) │                  │
    └─────┬──────┘  └─────┬──────┘                 │
          │               │                         │
          └───────┬───────┘                         │
                  │ R – Neustart                     │
                  └─────────────────────────────────┘
```

---

## Das war's – und der Anfang von etwas

Mit dieser Lektion hast du den kompletten Lehrplan absolviert. Dein Spiel hat jetzt:

- Echte Sprites und PBR-Beleuchtung
- Sounds und Musik
- Externe Konfiguration
- Dynamisches Level-Loading
- Einen nicht-linearen Spielfluss mit States

Das ist eine vollständige, solide Basis für einen Game Jam. Die Intermissions haben
dir außerdem gezeigt, wie du Sprites selbst erstellst, das Spiel im Web veröffentlichst,
und Controller einbindest.

Was kommt jetzt? Du machst dein Spiel. 🚀

---

## Zusammenfassung

In dieser Lektion haben wir:

- Bevys State-System mit `States` und `init_state()` eingeführt
- Systeme an Zustände gebunden (`run_if(in_state(...))`)
- `OnEnter` und `OnExit` für State-Setup und -Teardown verwendet
- Einen Optionsbildschirm mit Bevy UI gebaut
- `NextState::set()` für Zustandsübergänge genutzt
- `AudioSink` zur Laufzeit-Steuerung von Audio kennengelernt
- Das Konzept der Substates vorgestellt

---

*Zurück zur [Lektionen-Übersicht](../README.md)*
