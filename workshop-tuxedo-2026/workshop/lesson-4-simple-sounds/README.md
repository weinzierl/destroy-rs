# destroy_audio_demo

Minimal Bevy 0.18 audio demo — plays music and sfx from the destroy.rs assets folder on key press.

## Layout

Place your existing `assets/` folder (the one whose tree starts with `font/ gfx/ music/ sfx/ ...`) next to `Cargo.toml`:

```
destroy_audio_demo/
├── Cargo.toml
├── src/main.rs
└── assets/
    ├── font/unscii-8.ttf
    ├── music/{intro,level-x,outro}.ogg
    └── sfx/*.ogg
```

## Run

```sh
cargo run
```

## Controls

| Key | Action |
|-----|--------|
| `A` | Toggle `music/intro.ogg` (loop) |
| `S` | Toggle `music/level-x.ogg` (loop) |
| `D` | Toggle `music/outro.ogg` (loop) |
| `Q` | `sfx/laser-single-a.ogg` |
| `W` | `sfx/brick-c6.ogg` |
| `E` | `sfx/emp.ogg` |
| `R` | `sfx/siren.ogg` |
| `T` | `sfx/slide-open.ogg` |
| `Y` | `sfx/trampoline-hi.ogg` |
| `U` | `sfx/win.ogg` |

Pressing a music key while a different track is playing stops the current track and starts the new one. Pressing the same music key again stops it.
