# 🧱 Lektion 2 – Simples Breakout

> *"Das erste spielbare Spiel ist ein Meilenstein. Auch wenn es noch aussieht wie 1972."*

Das ist die Lektion, auf die alles hingearbeitet hat. Wir bauen Breakout – ein
echtes, spielbares Spiel. Kein Tutorial-Spielzeug, kein "fast ein Spiel". Etwas,
das man spielen *kann* und das sich wie ein Spiel *anfühlt*.

Und wir machen es in etwa 150 Zeilen Code.

---

## Motivation: Warum Breakout?

Breakout (Atari, 1976) ist eine der klügsten Entscheidungen für einen
Spieleentwicklungs-Workshop, und das aus mehreren Gründen:

**Es ist einfach genug.** Die Spielmechanik ist verständlich: Ball trifft Paddle,
Ball trifft Block, Block verschwindet, Punkte. Keine komplizierte KI, kein
Inventarsystem, keine Karte.

**Es ist reichhaltig genug.** Um Breakout zu bauen, brauchen wir tatsächlich fast
alles, was auch in größeren Spielen gebraucht wird: Bewegung, Kollisionserkennung,
Spielzustand, Win- und Lose-Bedingungen, Input-Verarbeitung.

**Es skaliert schön.** Von unseren heutigen 150 Zeilen (nur Rechtecke, kein Sound,
keine Grafik) bis zu einem polierten Spiel mit Sprites, Sounds und Animationen ist
es ein gerader, nachvollziehbarer Weg. Jede Lektion danach fügt eine Schicht hinzu.

**Es ist historisch bedeutsam.** Steve Jobs und Steve Wozniak haben Breakout für
Atari entwickelt. Es war das zweite Spiel, das Wozniak auf dem Apple I laufen ließ.
Es ist Teil der Geschichte unseres Fachs.

---

## Was wir bauen

- Ein **Paddle**, das sich mit den Pfeiltasten bewegt
- Ein **Ball**, der sich bewegt und von Wänden und Paddle abprallt
- Eine Reihe von **Blöcken**, die beim Treffer verschwinden
- Eine einfache **Lose-Bedingung** (Ball fällt unten heraus)
- Einen einfachen **Win-Zustand** (alle Blöcke zerstört)

Alles aus einfachen farbigen Rechtecken – keine externen Assets.

---

## Architektur-Überblick

Bevor wir den Code sehen, schauen wir kurz auf die Struktur. In Breakout gibt es
drei Arten von Dingen:

```
Spielwelt
├── Paddle     (bewegt sich mit Input, begrenzt auf X-Achse)
├── Ball       (bewegt sich autonom, prallt ab)
├── Blöcke     (stehen still, verschwinden bei Treffer)
└── Wände      (unsichtbare Kollisionsgrenzen, oder sichtbare Ränder)
```

Jedes dieser Dinge wird eine Entity mit Components. Die Logik steckt in Systemen.

---

## Der vollständige Code

```rust
use bevy::prelude::*;

// --- Konstanten ---
const PADDLE_WIDTH: f32 = 100.0;
const PADDLE_HEIGHT: f32 = 15.0;
const PADDLE_SPEED: f32 = 500.0;
const PADDLE_Y: f32 = -260.0;

const BALL_SIZE: f32 = 12.0;
const BALL_SPEED: f32 = 350.0;

const BLOCK_COLS: usize = 10;
const BLOCK_ROWS: usize = 5;
const BLOCK_WIDTH: f32 = 60.0;
const BLOCK_HEIGHT: f32 = 20.0;
const BLOCK_GAP: f32 = 5.0;

const WALL_THICKNESS: f32 = 10.0;
const ARENA_WIDTH: f32 = 800.0;
const ARENA_HEIGHT: f32 = 600.0;

// --- Components ---

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball {
    velocity: Vec2,
}

#[derive(Component)]
struct Block;

#[derive(Component)]
struct Wall;

// --- Main ---

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Breakout".into(),
                resolution: (ARENA_WIDTH, ARENA_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            move_paddle,
            move_ball,
            ball_paddle_collision,
            ball_block_collision,
        ))
        .run();
}

// --- Setup ---

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Paddle
    commands.spawn((
        Paddle,
        Sprite {
            color: Color::srgb(0.8, 0.8, 0.8),
            custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(0.0, PADDLE_Y, 0.0),
    ));

    // Ball
    commands.spawn((
        Ball { velocity: Vec2::new(1.0, 1.5).normalize() * BALL_SPEED },
        Sprite {
            color: Color::srgb(1.0, 0.9, 0.2),
            custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, -100.0, 0.0),
    ));

    // Blöcke
    let colors = [
        Color::srgb(0.9, 0.2, 0.2),
        Color::srgb(0.9, 0.6, 0.1),
        Color::srgb(0.8, 0.9, 0.1),
        Color::srgb(0.2, 0.8, 0.2),
        Color::srgb(0.2, 0.4, 0.9),
    ];
    let grid_width = BLOCK_COLS as f32 * (BLOCK_WIDTH + BLOCK_GAP) - BLOCK_GAP;
    let start_x = -grid_width / 2.0 + BLOCK_WIDTH / 2.0;
    let start_y = 150.0;

    for row in 0..BLOCK_ROWS {
        for col in 0..BLOCK_COLS {
            let x = start_x + col as f32 * (BLOCK_WIDTH + BLOCK_GAP);
            let y = start_y - row as f32 * (BLOCK_HEIGHT + BLOCK_GAP);
            commands.spawn((
                Block,
                Sprite {
                    color: colors[row],
                    custom_size: Some(Vec2::new(BLOCK_WIDTH, BLOCK_HEIGHT)),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
            ));
        }
    }
}

// --- Systeme ---

fn move_paddle(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    let Ok(mut transform) = query.get_single_mut() else { return };
    let mut direction = 0.0;
    if keyboard.pressed(KeyCode::ArrowLeft)  { direction -= 1.0; }
    if keyboard.pressed(KeyCode::ArrowRight) { direction += 1.0; }

    transform.translation.x += direction * PADDLE_SPEED * time.delta_secs();

    // Paddle innerhalb des Spielfelds halten
    let limit = ARENA_WIDTH / 2.0 - PADDLE_WIDTH / 2.0 - WALL_THICKNESS;
    transform.translation.x = transform.translation.x.clamp(-limit, limit);
}

fn move_ball(
    time: Res<Time>,
    mut query: Query<(&mut Ball, &mut Transform)>,
) {
    let Ok((mut ball, mut transform)) = query.get_single_mut() else { return };

    transform.translation.x += ball.velocity.x * time.delta_secs();
    transform.translation.y += ball.velocity.y * time.delta_secs();

    let half_w = ARENA_WIDTH  / 2.0 - WALL_THICKNESS - BALL_SIZE / 2.0;
    let half_h = ARENA_HEIGHT / 2.0 - WALL_THICKNESS - BALL_SIZE / 2.0;

    // Seitliche Wände
    if transform.translation.x > half_w || transform.translation.x < -half_w {
        ball.velocity.x *= -1.0;
        transform.translation.x = transform.translation.x.clamp(-half_w, half_w);
    }
    // Decke
    if transform.translation.y > half_h {
        ball.velocity.y *= -1.0;
        transform.translation.y = half_h;
    }
    // Boden – Verloren!
    if transform.translation.y < -half_h - 50.0 {
        println!("Verloren!");
        transform.translation = Vec3::new(0.0, -100.0, 0.0);
        ball.velocity = Vec2::new(1.0, 1.5).normalize() * BALL_SPEED;
    }
}

fn ball_paddle_collision(
    mut ball_query: Query<(&mut Ball, &Transform)>,
    paddle_query: Query<&Transform, With<Paddle>>,
) {
    let Ok((mut ball, ball_t)) = ball_query.get_single_mut() else { return };
    let Ok(paddle_t) = paddle_query.get_single()             else { return };

    let bx = ball_t.translation.x;
    let by = ball_t.translation.y;
    let px = paddle_t.translation.x;
    let py = paddle_t.translation.y;

    let overlap_x = (bx - px).abs() < (PADDLE_WIDTH  + BALL_SIZE) / 2.0;
    let overlap_y = (by - py).abs() < (PADDLE_HEIGHT + BALL_SIZE) / 2.0;

    if overlap_x && overlap_y && ball.velocity.y < 0.0 {
        // Abprallwinkel abhängig von Trefferposition auf dem Paddle
        let hit_pos = (bx - px) / (PADDLE_WIDTH / 2.0);
        let angle = hit_pos * 60_f32.to_radians(); // max. ±60°
        let speed = ball.velocity.length();
        ball.velocity = Vec2::new(angle.sin(), angle.cos().abs()) * speed;
    }
}

fn ball_block_collision(
    mut commands: Commands,
    mut ball_query: Query<(&mut Ball, &Transform)>,
    blocks: Query<(Entity, &Transform), With<Block>>,
) {
    let Ok((mut ball, ball_t)) = ball_query.get_single_mut() else { return };

    for (entity, block_t) in &blocks {
        let bx = ball_t.translation.x;
        let by = ball_t.translation.y;
        let ex = block_t.translation.x;
        let ey = block_t.translation.y;

        let overlap_x = (bx - ex).abs() < (BLOCK_WIDTH  + BALL_SIZE) / 2.0;
        let overlap_y = (by - ey).abs() < (BLOCK_HEIGHT + BALL_SIZE) / 2.0;

        if overlap_x && overlap_y {
            commands.entity(entity).despawn();
            ball.velocity.y *= -1.0;
            break; // Nur ein Block pro Frame treffen
        }
    }
}
```

---

## Neue Konzepte erklärt

### Marker-Components

```rust
#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Block;
```

`Paddle` und `Block` speichern keine Daten – sie sind leere Structs. Ihr einziger
Zweck ist das "Beschriften" einer Entity: "Diese Entity ist ein Paddle." Mit
`Query<&Transform, With<Paddle>>` können wir dann gezielt nur die Paddle-Entity
abrufen, auch wenn hundert andere Entities ebenfalls eine `Transform`-Component
haben.

### `query.get_single_mut()`

```rust
let Ok(mut transform) = query.get_single_mut() else { return };
```

`get_single_mut()` gibt `Ok(...)` zurück, wenn es *genau eine* passende Entity
gibt, und `Err(...)` bei null oder mehr als einer. Das `let Ok(...) else { return }`
ist ein idiomatischer Rust-Weg, mit diesem Fall umzugehen: Wenn es kein Paddle
gibt (z. B. weil das Spiel gerade startet), machen wir einfach nichts.

### `commands.entity(entity).despawn()`

```rust
commands.entity(entity).despawn();
```

Damit löschen wir eine Entity – und alle ihre Components – aus der Spielwelt.
Der Block verschwindet. `despawn()` löscht die Entity sofort am Ende des Frames.

### Kollisionserkennung mit AABB

Wir verwenden die einfachste mögliche Kollisionsmethode: **AABB** (Axis-Aligned
Bounding Box). Zwei Rechtecke überlappen, wenn sie sich auf *beiden* Achsen
überschneiden. Das prüfen wir getrennt für X und Y.

Diese Methode ist nicht perfekt (der Ball könnte bei sehr hoher Geschwindigkeit
durch einen Block "tunneln"), aber für unsere Zwecke ist sie ausreichend schnell
und einfach.

---

## Was noch fehlt (und in späteren Lektionen kommt)

- Echte Sprites statt Rechtecke (Lektion 3)
- Sounds beim Treffer (Lektion 4)
- Ein richtiger Gewinn-/Verlier-Bildschirm (Lektion 9)
- Konfigurierbare Spielparameter (Lektion 7)
- Wechsel zwischen Levels (Lektion 8)

Das Gerüst steht. Jetzt bauen wir es aus.

---

## Zusammenfassung

In dieser Lektion haben wir:

- Marker-Components (`Paddle`, `Block`) kennengelernt
- Mehrere Systeme registriert und in einem Tupel übergeben
- `get_single()` und `get_single_mut()` für eindeutige Entities verwendet
- `commands.entity().despawn()` zum Löschen von Entities genutzt
- Einfache AABB-Kollisionserkennung implementiert
- Das Konzept "Abprallwinkel aus Trefferposition" verstanden

---

*Weiter mit [Lektion 3 – Externe Sprites laden](../lesson-3-simple-sprites/README.md)*
