# 📚 Lektionen – Der rote Faden des Workshops

Willkommen beim Hauptlehrplan des Workshops!

Diese Lektionen sind das Herzstück. Sie bauen konsequent aufeinander auf und führen
dich von einem leeren Rust-Projekt bis zu einem vollständigen, spielbaren
Breakout-Klon mit Sprites, Sounds, Konfiguration und dynamischem Level-Loading.
Am Ende bist du bereit für deinen ersten Game Jam.

---

## Wie die Lektionen aufgebaut sind

Jede Lektion hat ein klares Ziel. Wir beginnen mit dem absoluten Minimum – einem
Programm, das einfach nur *läuft* – und fügen in jedem Schritt genau eine Idee
hinzu. Keine Lektion versucht, zu viel auf einmal zu erklären.

Das klingt langsam, ist es aber nicht. Du wirst überrascht sein, wie schnell aus
ein paar Zeilen Rust ein echtes, spielbares Spiel wird.

**Die Lektionen sind Pflichtprogramm** – im Gegensatz zu den
[Intermissions](../intermissions/README.md), die optional und in freier Reihenfolge
wählbar sind. Jede Lektion setzt das Wissen der vorherigen voraus.

---

## Übersicht der Lektionen

| # | Verzeichnis | Thema | Ziel |
|---|-------------|-------|------|
| 0 | [`lesson-0-setup-hello-world`](./lesson-0-setup-hello-world/README.md) | Setup & Hello World | Rust und Bevy zum Laufen bringen |
| 1 | [`lesson-1-move-a-sprite`](./lesson-1-move-a-sprite/README.md) | Ein Sprite bewegen | Erste Bevy-App, ECS verstehen |
| 2 | [`lesson-2-simple-breakout`](./lesson-2-simple-breakout/README.md) | Simples Breakout | Spielbares Spiel in ~150 Zeilen |
| 3 | [`lesson-3-simple-sprites`](./lesson-3-simple-sprites/README.md) | Externe Sprites laden | Assets, `AssetServer`, `Sprite` |
| 4 | [`lesson-4-simple-sounds`](./lesson-4-simple-sounds/README.md) | Sounds laden und abspielen | Audio-System, Musik, Effekte |
| 5 | [`lesson-5-pbr-sprites`](./lesson-5-pbr-sprites/README.md) | PBR-Sprites | Beleuchtete 2D-Grafik |
| 6 | [`lesson-6-advanced-sound`](./lesson-6-advanced-sound/README.md) | Fortgeschrittenes Audio | Clipping, Kompressor, Polyphonie |
| 7 | [`lesson-7-config`](./lesson-7-config/README.md) | Konfiguration laden | Externe Config, Serialisierung |
| 8 | [`lesson-8-level-loading`](./lesson-8-level-loading/README.md) | Level dynamisch laden | Assets zur Laufzeit, Szenen |
| 9 | [`lesson-9-options-screen`](./lesson-9-options-screen/README.md) | Optionsbildschirm & States | Nicht-linearer Spielfluss |

---

## Das große Bild

```
Lektion 0          Lektion 1          Lektion 2
Rust läuft    →   Sprite bewegt  →   Spielbar!
                  sich                (nur Rechtecke)

Lektion 3          Lektion 4          Lektion 5
Echte Sprites →   Sounds &      →   PBR-Beleuchtung
                  Musik

Lektion 6          Lektion 7          Lektion 8          Lektion 9
Sauberes    →     Konfiguration →   Level dynamisch →  Options-
Audio             von außen         laden              screen & States
```

---

## Vorkenntnisse

Wir setzen voraus, dass du:

- Grundlegende Rust-Kenntnisse mitbringst (Variablen, Funktionen, Structs, Enums,
  Traits, die Ownership-Grundidee)
- Die Kommandozeile nicht scheust
- Bereit bist, manchmal etwas nicht sofort zu verstehen – das ist normal und gut

Du musst kein Rust-Experte sein. Wir erklären jeden neuen Bevy-Mechanismus, wenn
er zum ersten Mal auftaucht. Rust-spezifische Feinheiten erläutern wir, wenn sie
relevant werden.

---

## Die Intermissions

Nach bestimmten Lektionen passen thematisch gut passende Intermissions. Eine
Empfehlung:

| Nach Lektion | Passende Intermission |
|---|---|
| 0 | – |
| 1 | [Intermission 4 – Pixel Art](../intermissions/intermission-4-pixelart/README.md) |
| 2 | [Intermission 5 – Tilemaps](../intermissions/intermission-5-tiles/README.md) |
| 3 | [Intermission 1 – Sprites in Blender](../intermissions/intermission-1-sprites/README.md) |
| 4 | [Intermission 2 – Sounds](../intermissions/intermission-2-sounds/README.md) · [Intermission 3 – Musik](../intermissions/intermission-3-music/README.md) |
| 5 | – |
| 6 | – |
| 7 | – |
| 8 | [Intermission 6 – Kompilieren](../intermissions/intermission-6-compile-bevy/README.md) |
| 9 | [Intermission 7 – WASM](../intermissions/intermission-7-wasm/README.md) · [Intermission 8 – Controller](../intermissions/intermission-8-controllers/README.md) |

---

*Bereit? Dann geht es los mit [Lektion 0 – Setup & Hello World](./lesson-0-setup-hello-world/README.md).*
