# 🛠️ Lektion 0 – Setup & Hello World

> *"Ein Programm, das kompiliert, ist kein kleiner Sieg. Es ist der erste."*

Diese Lektion hat ein einziges Ziel: Sicherstellen, dass alle Werkzeuge auf deinem
Rechner funktionieren, bevor wir anfangen, echten Spielcode zu schreiben. Klingt
unspektakulär – ist es aber nicht, denn ein nicht funktionierendes Setup ist der
häufigste Grund, warum Workshops ins Stocken geraten.

Am Ende dieser Lektion hast du:

- Bestätigt, dass Rust korrekt installiert ist und ein einfaches Programm kompiliert
- Ein minimales Bevy-Projekt aufgesetzt
- Das einfachste denkbare Bevy-Programm zum Laufen gebracht – ein leeres Fenster

---

## Teil 1: Rust-Installation prüfen

### Rust installieren (falls noch nicht geschehen)

Der offizielle Weg ist `rustup` – das Rust-Versions- und Toolchain-Management-Tool:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Auf Windows: Den Installer von [rustup.rs](https://rustup.rs) herunterladen und
ausführen.

Nach der Installation die Shell neu starten oder:

```bash
source "$HOME/.cargo/env"
```

### Installation verifizieren

```bash
rustc --version
cargo --version
```

Erwartete Ausgabe (Versionsnummern können neuer sein):

```
rustc 1.78.0 (9b00956e5 2024-04-29)
cargo 1.78.0 (54d8815d0 2024-04-09)
```

Wenn das funktioniert: ✅

### Das allererste Rust-Programm

```bash
cargo new hello-rust
cd hello-rust
cargo run
```

Erwartete Ausgabe:

```
   Compiling hello-rust v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/hello-rust`
Hello, world!
```

Wenn das erscheint: ✅ Rust funktioniert auf deinem Rechner.

---

## Teil 2: Bevy-Voraussetzungen

Bevy hat ein paar Systemabhängigkeiten, die außerhalb von Cargo installiert werden
müssen. Diese variieren je nach Betriebssystem.

### Linux (Ubuntu/Debian)

```bash
sudo apt install g++ pkg-config libx11-dev libasound2-dev libudev-dev \
  libxkbcommon-x11-0 libwayland-dev libxkbcommon-dev
```

Für Vulkan-Unterstützung (empfohlen für bessere Render-Performance):

```bash
sudo apt install libvulkan1 mesa-vulkan-drivers
```

### Linux (Arch / Manjaro)

```bash
sudo pacman -S pkgconf alsa-lib systemd-libs libxkbcommon-x11 wayland
```

### macOS

Auf macOS sind die nötigen Systembibliotheken in der Regel über Xcode Command
Line Tools vorhanden:

```bash
xcode-select --install
```

### Windows

Auf Windows sind keine zusätzlichen Installationsschritte nötig, wenn Visual
Studio Build Tools oder Visual Studio mit C++-Unterstützung installiert sind.

---

## Teil 3: Das erste Bevy-Projekt

### Neues Projekt anlegen

```bash
cargo new hello-bevy
cd hello-bevy
```

### Bevy als Abhängigkeit eintragen

Die `Cargo.toml` öffnen und editieren:

```toml
[package]
name = "hello-bevy"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy = "0.15"

# Für schnellere Entwicklungs-Builds (siehe Kommentar unten)
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```

> 💡 **Warum die `profile`-Einträge?** Bevy ist eine große Bibliothek. Ohne diese
> Einstellung kompiliert Bevy in der Entwicklungs-Konfiguration sehr langsam und
> läuft träge. Mit `opt-level = 3` für Abhängigkeiten wird Bevy selbst optimiert
> kompiliert (und damit schnell), während unser eigener Code mit minimaler
> Optimierung gebaut wird (schneller Kompilierzyklus). Das ist das Standard-Setup
> für Bevy-Entwicklung.

### Der erste Bevy-Code

`src/main.rs` öffnen und mit folgendem ersetzen:

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
```

Das ist das kleinstmögliche Bevy-Programm. Es tut nichts außer ein Fenster zu öffnen.

### Kompilieren und starten

```bash
cargo run
```

Der erste Build dauert **einige Minuten** – das ist normal. Bevy muss beim ersten
Mal komplett gebaut werden. Alle weiteren Builds sind wesentlich schneller.

Wenn ein leeres Fenster erscheint: ✅ Bevy funktioniert!

> 💡 Das Fenster hat einen schwarzen Hintergrund und zeigt den Titel "Bevy App".
> Zum Schließen: `Escape` drücken oder das Fenster normal schließen.

---

## Teil 4: Was haben wir gerade geschrieben?

Auch wenn der Code minimal ist, lohnt es sich, ihn zu verstehen. Hier tauchen die
zentralen Konzepte von Bevy zum ersten Mal auf.

### `App::new()`

`App` ist der Einstiegspunkt jeder Bevy-Anwendung. Sie ist der Container, in dem
alles registriert wird: Systeme, Ressourcen, Events, Plugins.

### `.add_plugins(DefaultPlugins)`

`DefaultPlugins` ist ein vorkonfiguriertes Bündel von Plugins, das alles enthält,
was man für ein normales Spiel braucht:

- Fensterverwaltung
- Rendering (2D und 3D)
- Asset-Loading
- Audio
- Input-Verarbeitung (Tastatur, Maus, Gamepad)
- Zeitverwaltung
- Logging

Wir könnten auch jedes Plugin einzeln hinzufügen, aber `DefaultPlugins` ist der
bequeme Weg für den Anfang.

### `.run()`

Startet die Bevy-Hauptschleife. Ab diesem Punkt läuft das Spiel, bis das Fenster
geschlossen wird. Diese Methode kehrt erst zurück, wenn das Spiel endet.

---

## Teil 5: Fenstertitel und Grundkonfiguration

Als kleine Übung passen wir das Fenster an:

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mein erstes Bevy-Spiel".into(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .run();
}
```

Wenn das Fenster mit dem neuen Titel und der neuen Größe erscheint: ✅ Alles bereit
für die nächste Lektion.

---

## Häufige Probleme

**`cargo run` schlägt fehl mit "linker not found" o.ä.**
Auf Linux fehlen Systempakete. Die Ausgabe von `cargo run` nennt meistens genau,
welche Bibliothek fehlt. Die oben genannten `apt install`-Befehle für den
entsprechenden Fehler erneut ausführen.

**Der Build dauert sehr lange.**
Das ist beim ersten Mal normal. Kaffee holen. Beim zweiten `cargo run` geht es
deutlich schneller. Die `profile`-Einstellungen in der `Cargo.toml` sorgen für
gute Entwicklungs-Performance ab dem zweiten Build.

**Das Fenster öffnet sich kurz und schließt sich sofort.**
Das kann an einem Fehler im Code liegen. `cargo run` in der Konsole ausführen und
die Fehlermeldung lesen – Bevy schreibt sehr ausführliche Hinweise.

**Auf Linux erscheint ein Fehler zu Wayland oder X11.**
Je nach Desktop-Umgebung die fehlenden Pakete aus der obigen Liste installieren.
Alternativ lässt sich der Renderer mit einer Umgebungsvariable erzwingen:
`WINIT_UNIX_BACKEND=x11 cargo run`

---

## Zusammenfassung

In dieser Lektion haben wir:

- Die Rust-Installation verifiziert
- Bevy als Abhängigkeit eingerichtet
- Die ersten wichtigen Build-Optimierungen in der `Cargo.toml` eingetragen
- Das kleinstmögliche Bevy-Programm geschrieben und verstanden
- `App`, `DefaultPlugins` und `.run()` kennengelernt

---

*Weiter mit [Lektion 1 – Ein Sprite bewegen](../lesson-1-move-a-sprite/README.md)*
