# 🖼️ Lektion 3 – Externe Sprites laden

> *"Jetzt sieht es endlich wie ein Spiel aus."*

Bisher waren unsere Spielelemente farbige Rechtecke. Das war bewusst so: Wir wollten
uns auf die Mechanik konzentrieren, ohne uns in Asset-Details zu verlieren. Aber jetzt
ist es Zeit für echte Grafik.

In dieser Lektion lernen wir, wie Bevy externe Bilddateien lädt und sie als Sprites
auf dem Bildschirm anzeigt. Das ist das grundlegende Asset-System, das für alle
Ressourcentypen in Bevy gilt – Bilder, Sounds, Fonts, Szenen.

---

## Das Asset-System in Bevy

Bevy verwaltet alle externen Ressourcen über den **`AssetServer`**. Das ist eine
globale Ressource, die:

- Dateien asynchron im Hintergrund lädt (das Spiel blockiert nicht während des Ladens)
- Assets im Speicher vorhält, damit dieselbe Datei nicht zweimal geladen wird
- Abhängigkeiten zwischen Assets verwaltet
- Assets bei Bedarf wieder freigibt (wenn niemand mehr einen Handle darauf hält)

Der `AssetServer` gibt beim Laden keinen fertigen Asset zurück, sondern einen
**`Handle`** – eine Art Ticket. Der Asset ist irgendwann *bereit*, aber vielleicht
nicht sofort.

Das klingt kompliziert, ist aber in der Praxis sehr einfach zu benutzen.

---

## Verzeichnisstruktur

Bevy sucht Assets standardmäßig in einem Ordner namens `assets/` im Projekt-Root
(neben `src/` und `Cargo.toml`).

```
mein-projekt/
├── Cargo.toml
├── src/
│   └── main.rs
└── assets/
    └── sprites/
        ├── paddle.png
        ├── ball.png
        └── block_blue.png
```

Diese Struktur ist Konvention. Der Pfad kann in der App-Konfiguration geändert
werden, aber wir bleiben bei der Voreinstellung.

---

## Ein Sprite aus einer Datei laden

Der einfachste Fall: Ein einzelnes Bild als Sprite anzeigen.

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // Das Bild laden und als Sprite anzeigen
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/ball.png")),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
```

`asset_server.load("sprites/ball.png")` gibt einen `Handle<Image>` zurück.
`Sprite::from_image()` nimmt diesen Handle und erzeugt einen Sprite daraus.
Wenn das Bild noch nicht geladen ist, bleibt das Sprite unsichtbar, bis es fertig ist.

---

## Sprite-Größe kontrollieren

Standardmäßig wird das Sprite in seiner natürlichen Pixelgröße angezeigt
(1 Pixel = 1 Spieleinheit). Das lässt sich anpassen:

```rust
// Option A: Größe explizit setzen (ignoriert die Bildgröße)
Sprite {
    image: asset_server.load("sprites/paddle.png"),
    custom_size: Some(Vec2::new(100.0, 15.0)),
    ..default()
}

// Option B: Über Transform skalieren
Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(2.0))
```

Für Spielsituationen ist `custom_size` oft besser: Die logische Größe im Spiel
bleibt stabil, unabhängig von der tatsächlichen Bildauflösung.

---

## Sprite Atlas – animierte Sprites

Wenn ein Sprite mehrere Frames hat (z. B. eine Laufanimation oder verschiedene
Block-Zustände), verwenden wir einen **Sprite Atlas**: eine einzige Bilddatei,
die alle Frames enthält.

```rust
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    let texture = asset_server.load("sprites/block_sheet.png");

    // Layout des Atlas: 4 Spalten, 1 Zeile, jeder Frame 64×32 Pixel
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(64, 32),
        4,    // Spalten
        1,    // Zeilen
        None, // Padding
        None, // Offset
    );
    let layout_handle = layouts.add(layout);

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: layout_handle,
                index: 0, // Welcher Frame?
            },
        ),
        Transform::from_xyz(0.0, 100.0, 0.0),
    ));
}
```

Der `index` bestimmt, welches Teilbild aus dem Atlas angezeigt wird. Durch
Verändern des Index in einem System entsteht eine Animation.

---

## Breakout mit Sprites

Wir erweitern unser Breakout-Spiel aus Lektion 2. Die Änderungen sind minimal –
das ist das Schöne an der ECS-Architektur: Wir tauschen nur aus, *wie* etwas
gezeichnet wird, ohne die Spiellogik anzufassen.

```rust
// In setup(): Paddle mit Sprite statt farbigem Rechteck

// Vorher (Lektion 2):
commands.spawn((
    Paddle,
    Sprite {
        color: Color::srgb(0.8, 0.8, 0.8),
        custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
        ..default()
    },
    Transform::from_xyz(0.0, PADDLE_Y, 0.0),
));

// Nachher (Lektion 3):
commands.spawn((
    Paddle,
    Sprite {
        image: asset_server.load("sprites/paddle.png"),
        custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
        ..default()
    },
    Transform::from_xyz(0.0, PADDLE_Y, 0.0),
));
```

Die gesamte Kollisionslogik, Bewegungslogik und Spielmechanik bleibt unverändert.
Nur der visuelle Teil ändert sich.

---

## Asset-Zustand prüfen

Manchmal möchte man warten, bis alle Assets geladen sind, bevor das Spiel startet.
Bevy hat dafür einen eingebauten Zustand: `LoadingState`.

Eine einfachere Alternative: Den `AssetServer` befragen, ob ein Handle bereit ist.

```rust
fn check_loading(
    asset_server: Res<AssetServer>,
    my_handle: Res<MyImageHandle>, // Handle, den wir in setup() gespeichert haben
) {
    use bevy::asset::LoadState;
    match asset_server.load_state(my_handle.0.id()) {
        LoadState::Loaded    => println!("Fertig!"),
        LoadState::Loading   => println!("Lädt noch..."),
        LoadState::Failed(_) => println!("Fehler beim Laden!"),
        _                    => {}
    }
}
```

Für kleine Spiele ist das oft nicht nötig – Bevy zeigt einfach nichts an, bis das
Asset geladen ist, was in der Praxis kaum auffällt.

---

## Häufige Fehler

**Das Sprite ist nicht sichtbar, obwohl der Code korrekt aussieht.**
Häufigste Ursache: Datei nicht im richtigen Verzeichnis. Bevy sucht in `assets/`
*relativ zum Arbeitsverzeichnis beim Programmstart*, was bei `cargo run` der
Projekt-Root ist. Absoluter Pfad oder falscher Unterordner sind häufige Fallen.

**Das Sprite erscheint, aber ist ein pinkes Rechteck.**
Bevy zeigt eine magentafarbene Platzhalter-Textur, wenn eine Textur fehlt oder
nicht geladen werden konnte. Die Konsole gibt meistens Hinweise.

**Das Sprite ist riesig oder winzig.**
Wenn `custom_size` nicht gesetzt ist, wird die natürliche Bildgröße verwendet.
Ein 512×512-Pixel-Bild erscheint dann als 512×512 Spieleinheiten – was bei einer
800×600-Welt sehr groß ist.

**Das Bild sieht weich/unscharf aus (Pixel Art).**
Für Pixel Art brauchen wir `Nearest`-Interpolation. Das wird in
[Intermission 4 – Pixel Art](../../intermissions/intermission-4-pixelart/README.md)
erklärt.

---

## Zusammenfassung

In dieser Lektion haben wir:

- Den `AssetServer` und das Handle-System von Bevy kennengelernt
- Einzelne Bilder als Sprites geladen und angezeigt
- `custom_size` für logische Spielgrößen unabhängig von Bildauflösungen verwendet
- Sprite Atlases für mehrteilige Bilder eingerichtet
- Unsere Breakout-Sprites ohne Änderung der Spiellogik ausgetauscht

---

*Weiter mit [Lektion 4 – Sounds laden und abspielen](../lesson-4-simple-sounds/README.md)*
