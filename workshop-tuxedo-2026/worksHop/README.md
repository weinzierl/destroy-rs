# destroy_audio_demo

Minimal Bevy 0.18 audio demo — plays music and sfx from the destroy.rs assets folder, with a configurable polyphony cap to demonstrate clipping vs. voice stealing.

## Layout

Place your existing `assets/` folder next to `Cargo.toml`:

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
| `0` | Max sfx voices: unlimited |
| `1`–`9` | Max sfx voices: 1..9 |

Music keys toggle independently — press all three to layer them. **Music is not subject to the channel cap**; only sfx voices are counted and stolen.

## Clipping demo idea

Set the cap to `0` (unlimited), start all three music loops, then mash sfx keys — sources sum, the master bus exceeds 0 dBFS, you get clipping. Now press `3` or `4`: voices get stolen, the sum stays manageable.
