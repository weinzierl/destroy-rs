# 🟦 Lektion 1 – Ein Sprite über den Bildschirm bewegen

> *"Bewegung ist Leben. Ein Pixel, der sich bewegt, ist bereits ein Spiel."*

In dieser Lektion schreiben wir unser erstes echtes Bevy-Programm: Ein farbiges
Rechteck erscheint auf dem Bildschirm und bewegt sich. Das klingt simpel – und das
ist es auch, bewusst. Denn auf dem Weg dahin lernen wir die grundlegende Struktur
jedes Bevy-Spiels kennen, und die ist das Wichtigste in diesem ganzen Workshop.

Am Ende dieser Lektion hast du ein Programm von etwa 50 Zeilen, das du vollständig
verstehst.

---

## Was wir nicht machen

In dieser Lektion laden wir keine externen Dateien. Das Sprite ist ein einfarbiges
Rechteck, das wir direkt im Code erzeugen. Echte Sprites mit Bilddateien kommen
in Lektion 3. Das hat einen guten Grund: Wenn wir Sprites und Asset-Loading gleichzeitig
einführen, ist es schwerer zu erkennen, was zu was gehört. Eins nach dem anderen.

---

## Die Grundstruktur eines Bevy-Spiels

Bevor wir Code schreiben, brauchen wir ein mentales Modell. Bevy folgt einem Muster
namens **ECS – Entity Component System**. Es ist das Fundament, auf dem alles andere
aufbaut.

### Entities

Eine **Entity** ist ein Ding in der Spielwelt – aber nur ein Bezeichner, eine Nummer.
Der Spieler ist eine Entity. Der Ball ist eine Entity. Der Hintergrund ist eine Entity.
Entities selbst haben keine Daten und kein Verhalten.

### Components

**Components** sind die Daten, die einer Entity angehängt werden. Ein Entity "Spieler"
könnte folgende Components haben:

- `Transform` – Position, Rotation, Skalierung
- `Sprite` – wie er gezeichnet werden soll
- `Velocity` – wie schnell er sich bewegt (eine eigene Component, die wir selbst
  definieren)
- `Health` – Lebenspunkte

Ein Component ist fast immer ein einfacher Rust-Struct.

### Systems

**Systems** sind Funktionen, die jedes Frame ausgeführt werden. Sie lesen und
verändern Components. Ein System "BewegungSystem" könnte alle Entities mit einer
`Velocity`-Component suchen und deren `Transform` entsprechend verschieben.

Das ist das ganze Modell:

```
Entities  ←  sind Behälter für
Components ←  werden verarbeitet von
Systems    ←  laufen jeden Frame
```

---

## Der vollständige Code

```rust
use bevy::prelude::*;

// Unser eigenes Component: die Bewegungsgeschwindigkeit
#[derive(Component)]
struct Velocity(Vec2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Lektion 1 – Sprite bewegen".into(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, move_sprite)
        .run();
}

// Dieses System läuft einmal beim Start
fn setup(mut commands: Commands) {
    // Kamera spawnen – ohne Kamera sehen wir nichts
    commands.spawn(Camera2d);

    // Unser Sprite spawnen: ein blaues Rechteck
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.4, 0.9),
            custom_size: Some(Vec2::new(80.0, 30.0)),
            ..default()
        },
        Transform::from_xyz(-300.0, 0.0, 0.0),
        Velocity(Vec2::new(200.0, 0.0)),
    ));
}

// Dieses System läuft jeden Frame
fn move_sprite(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>,
) {
    for (velocity, mut transform) in &mut query {
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();
    }
}
```

Starte das Programm mit `cargo run`. Ein blaues Rechteck erscheint links und gleitet
nach rechts aus dem Bild.

---

## Den Code verstehen

### `#[derive(Component)]`

```rust
#[derive(Component)]
struct Velocity(Vec2);
```

Das ist unser erstes eigenes Component. Das `derive`-Makro teilt Bevy mit, dass
dieser Struct als Component an Entities gehängt werden darf. `Vec2` ist Bevys
zweidimensionaler Vektor – zwei `f32`-Werte für X und Y.

### `add_systems(Startup, setup)`

```rust
.add_systems(Startup, setup)
.add_systems(Update, move_sprite)
```

Hier registrieren wir unsere Systeme. `Startup` bedeutet: dieses System läuft genau
einmal, wenn das Spiel startet. `Update` bedeutet: dieses System läuft jeden Frame.

### `commands.spawn((...))`

```rust
commands.spawn((
    Sprite { ... },
    Transform::from_xyz(-300.0, 0.0, 0.0),
    Velocity(Vec2::new(200.0, 0.0)),
));
```

`commands.spawn()` erzeugt eine neue Entity. Wir übergeben ein Tupel von Components,
die sofort an die Entity gehängt werden. Die Entity selbst – nur eine Zahl – bekommen
wir hier nicht einmal zu sehen. Das ist der Normalfall: Entities sind anonym, wir
arbeiten nur mit ihren Components.

`Transform::from_xyz(-300.0, 0.0, 0.0)` setzt die Startposition: 300 Pixel links
vom Bildschirmmittelpunkt. In Bevy liegt der Koordinatenursprung (0, 0) in der
Bildschirmmitte, X wächst nach rechts, Y nach oben.

### `Query<(&Velocity, &mut Transform)>`

```rust
fn move_sprite(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>,
) {
```

Das ist die Bevy-Art, Systemen Daten zu geben. `Query<(&Velocity, &mut Transform)>`
bedeutet: "Gib mir alle Entities, die *sowohl* eine `Velocity`- als auch eine
`Transform`-Component haben. `Velocity` nur zum Lesen (`&`), `Transform` zum
Verändern (`&mut`)."

Bevy löst diese Anfrage automatisch auf und liefert uns nur die passenden Entities.
Wenn wir später Entities ohne `Velocity` haben (z. B. den Hintergrund), werden diese
von diesem System ignoriert.

### `time.delta_secs()`

```rust
transform.translation.x += velocity.0.x * time.delta_secs();
```

`time.delta_secs()` gibt die Zeit in Sekunden zurück, die seit dem letzten Frame
vergangen ist. Wir multiplizieren unsere Geschwindigkeit damit, damit die Bewegung
**frame-rate-unabhängig** ist: Das Sprite bewegt sich mit 200 Pixeln pro Sekunde,
egal ob das Spiel mit 30 oder 120 FPS läuft.

Ohne diese Multiplikation würde das Spiel auf schnellen Rechnern viel schneller
laufen als auf langsamen – ein klassischer Fehler.

---

## Erweiterung: Das Sprite zurückprallen lassen

Ein Rechteck, das aus dem Bild fliegt, ist wenig spannend. Wir erweitern das System,
damit das Sprite von den Rändern abprallt:

```rust
fn move_sprite(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut Transform)>,
) {
    for (mut velocity, mut transform) in &mut query {
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();

        // Von den seitlichen Rändern abprallen (800 Pixel breit → ±400)
        if transform.translation.x > 360.0 || transform.translation.x < -360.0 {
            velocity.0.x *= -1.0;
        }
    }
}
```

Damit springt das Rechteck endlos hin und her. Hier sieht man auch, warum `&mut`
bei `Velocity` wichtig ist: Wir verändern jetzt die Geschwindigkeit, nicht nur die
Position.

---

## Was ist `Res<Time>`?

`Res<Time>` ist eine **Ressource** – das zweite zentrale Konzept in Bevy neben
Components. Während Components an Entities gebunden sind (es kann viele Spieler geben,
jeder mit eigener Position), sind Ressourcen global und einmalig. `Time` existiert
genau einmal und ist immer verfügbar. Wir greifen darauf zu, indem wir sie als
Parameter in unsere System-Funktion schreiben – Bevy kümmert sich um den Rest.

Ressourcen sind ideal für globale Spielzustände wie den aktuellen Score, die Spielzeit,
oder – wie hier – die Uhr.

---

## Zusammenfassung

In dieser Lektion haben wir:

- Das ECS-Prinzip (Entity, Component, System) verstanden
- Ein eigenes Component (`Velocity`) definiert
- Die Unterschiede zwischen `Startup`- und `Update`-Systemen gelernt
- `commands.spawn()` zum Erzeugen von Entities verwendet
- `Query` benutzt, um gezielt Entities mit bestimmten Components abzurufen
- `Res<Time>` für frame-rate-unabhängige Bewegung eingesetzt

Das sind die Grundbausteine. Alles, was ab jetzt kommt, baut darauf auf.

---

*Weiter mit [Lektion 2 – Simples Breakout](../lesson-2-simple-breakout/README.md)*
