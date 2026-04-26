# ✨ Lektion 5 – PBR-Sprites

> *"Licht macht aus einer Grafik eine Welt."*

Unsere Sprites sehen bisher gut aus – aber sie sind flach. Egal woher das Licht
kommt, egal welche Atmosphäre wir erzeugen wollen: Ein normales Sprite ist immer
gleichmäßig beleuchtet, immer gleich hell. In dieser Lektion fügen wir eine Dimension
hinzu: **physikalisch basiertes Rendering** für 2D-Sprites.

Das Ergebnis ist bemerkenswert: Sprites, die auf Licht reagieren, Schatten werfen
und eine Tiefe vermitteln, die weit über ihre zwei Dimensionen hinausgeht.

---

## Was ist PBR – die Kurzfassung

**Physically Based Rendering** (PBR) ist ein Rendering-Ansatz, der echte physikalische
Lichtgesetze simuliert. Statt Licht fest in die Textur "einzubacken" (was bei normalen
Sprites passiert), beschreiben wir die *Materialeigenschaften* der Oberfläche – und
die Engine berechnet, wie Licht mit ihr interagiert.

Die Details von PBR und wie man PBR-Sprites in Blender erstellt, erklärt
[Intermission 1 – Sprites](../../intermissions/intermission-1-sprites/README.md).
Hier konzentrieren wir uns auf die *Verwendung* in Bevy.

Ein PBR-Sprite besteht aus mehreren Texturschichten:

| Textur | Aufgabe |
|--------|---------|
| **Albedo (Color Map)** | Die Grundfarbe, ohne Beleuchtungseffekte |
| **Normal Map** | Täuscht dreidimensionale Oberflächenstruktur vor |
| **Roughness/Metallic Map** | Wie rau oder metallisch ist die Oberfläche? |
| **Emissive Map** | Welche Teile leuchten selbst? |

---

## PBR in 2D: Der Ansatz in Bevy

Bevy ist primär eine 3D-Engine, die auch 2D sehr gut beherrscht. PBR-Sprites
können auf zwei Wegen realisiert werden:

**Weg A: 3D-Mesh mit 2D-Kameraprojektion.**
Wir verwenden echte 3D-Objekte (flache Quads) mit `StandardMaterial`, platzieren
sie in einer orthografischen 3D-Szene und rendern sie wie 2D. Das gibt uns das
volle PBR-System von Bevy – inklusive dynamischer Lichter.

**Weg B: Custom Shader für 2D-Sprites.**
Mit einem eigenen WGSL-Shader können wir Normal Maps und Beleuchtung direkt
auf die `Sprite`-Komponente anwenden, ohne den 3D-Renderer zu bemühen. Das ist
eleganter, erfordert aber etwas mehr Shader-Wissen.

Wir verwenden **Weg A** – er ist einfacher einzurichten und nutzt Bevys
eingebautes PBR-System vollständig.

---

## Eine PBR-Szene aufsetzen

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Orthografische 3D-Kamera (statt Camera2d)
    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::FixedVertical {
                viewport_height: 600.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Lichtquelle
    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(100.0, 200.0, 300.0),
    ));

    // Umgebungslicht (damit es nicht zu dunkel wird)
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
    });

    // Ein PBR-Sprite: flaches Quad mit Normal Map
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("sprites/block_albedo.png")),
        normal_map_texture: Some(asset_server.load("sprites/block_normal.png")),
        metallic: 0.0,
        perceptual_roughness: 0.7,
        ..default()
    });

    let mesh = meshes.add(Rectangle::new(60.0, 20.0));

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_xyz(0.0, 100.0, 0.0),
    ));
}
```

---

## Die Normal Map

Die Normal Map ist das Herzstück des PBR-Sprite-Effekts. Sie ist ein Bild, in dem
jeder Pixel keine Farbe, sondern eine *Richtung* speichert – die Richtung, in die
die Oberfläche an diesem Punkt "zeigt". Die Kanäle R, G, B kodieren dabei X, Y und
Z der Oberflächennormalen.

Für Bevy wichtig: Normal Maps müssen im **Tangenten-Raum** vorliegen (was das
Standard-Format für Spielentwicklung ist). Außerdem muss Bevy wissen, dass die
Textur eine Normal Map ist und nicht als SRGB-Farbe interpretiert werden soll:

```rust
// Normal Maps NICHT als SRGB laden!
let normal_handle: Handle<Image> = asset_server.load(
    GltfAssetLabel::Primitive { mesh: 0, primitive: 0 }.from_asset("sprites/block_normal.png")
);
```

In der Praxis ist es einfacher, die Normal Map über `StandardMaterial` zu laden –
Bevy behandelt sie dann automatisch korrekt:

```rust
StandardMaterial {
    normal_map_texture: Some(asset_server.load("sprites/block_normal.png")),
    // Bevy setzt automatisch das richtige Texturformat
    ..default()
}
```

---

## Dynamische Lichter animieren

Das Schöne an PBR ist, dass wir das Licht zur Laufzeit bewegen können – und die
Sprites reagieren darauf sofort und korrekt.

```rust
#[derive(Component)]
struct MovingLight;

fn animate_light(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MovingLight>>,
) {
    for mut transform in &mut query {
        let t = time.elapsed_secs();
        transform.translation.x = (t * 0.8).sin() * 300.0;
        transform.translation.y = (t * 0.5).cos() * 200.0;
    }
}
```

Das ist ein einfacher Effekt, der aber sofort zeigt, wie lebendig PBR-Sprites
wirken können.

---

## Spiellogik bleibt unberührt

Ein wichtiger Punkt: Die gesamte Spiellogik aus Lektion 2 – Kollisionserkennung,
Bewegung, Input – bleibt identisch. Wir ersetzen nur die Render-Komponenten:

- Statt `Sprite` + `Camera2d` verwenden wir `Mesh3d` + `MeshMaterial3d` + `Camera3d`
- Die `Transform`-Komponente ist in beiden Welten dieselbe
- Alle anderen Components (Paddle, Ball, Block, Velocity) bleiben unverändert

Das ist ECS in seiner schönsten Form: Systeme, die Logik implementieren, und
Systeme, die Rendering implementieren, kennen einander nicht. Wir können zwischen
beiden wechseln, ohne die Spielmechanik anzufassen.

---

## Performance-Überlegungen

PBR ist teurer als einfaches Sprite-Rendering. Für ein Breakout-Spiel mit wenigen
Dutzend Objekten ist das absolut kein Problem. Aber es ist gut zu wissen:

- Jede zusätzliche Lichtquelle erhöht den Render-Aufwand spürbar.
- `shadows_enabled: true` bei Lichtern ist teuer – sparsam einsetzen.
- Normal Maps brauchen keine besondere Optimierung: Sie kosten fast nichts extra.

Für ein Spielejam-Projekt ist "so viel PBR wie möglich" die richtige Einstellung.
Für ein kommerzielles Spiel käme hier Profiling zum Einsatz.

---

## Zusammenfassung

In dieser Lektion haben wir:

- Verstanden, was PBR-Sprites sind und warum sie besser aussehen
- Eine orthografische 3D-Kamera als 2D-Ersatz eingerichtet
- `StandardMaterial` mit Albedo- und Normal-Map-Texturen verwendet
- Dynamische Lichter für lebendige Beleuchtung hinzugefügt
- Gesehen, dass die Spiellogik von der Rendering-Entscheidung komplett getrennt ist

---

*Weiter mit [Lektion 6 – Fortgeschrittenes Audio](../lesson-6-advanced-sound/README.md)*
