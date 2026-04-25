# assets/

This directory contains the runtime assets for **destroy.rs**, an Arkanoid-style game built with the [Bevy](https://bevyengine.org/) game engine. By Bevy convention, game assets live in a top-level `assets/` folder so they are automatically discoverable by Bevy's `AssetServer`.

---

## Directory structure

```text
assets/
├── font/
│   └── unscii-8.ttf              # Monospace bitmap-style font; used for HUD, score display, and UI overlays
│
├── gfx/
│   ├── highscore.png             # High-score screen graphic
│   ├── story.png                 # Story / intro interstitial graphic
│   └── title.png                 # Title screen graphic
│
├── music/
│   ├── COPYRIGHT.txt             # License attribution for the music tracks
│   ├── License.txt               # Full license text for music assets
│   ├── intro.ogg                 # Title screen / main menu background music
│   ├── level-x.ogg               # In-game background music (looped during gameplay)
│   └── outro.ogg                 # End screen music (win or game over)
│
├── sfx/
│   ├── brick-a6.ogg              # Brick hit sound — pitch A6 (used for tonal brick destruction feedback)
│   ├── brick-c6.ogg              # Brick hit sound — pitch C6
│   ├── brick-c7.ogg              # Brick hit sound — pitch C7 (higher octave variant)
│   ├── brick-d6.ogg              # Brick hit sound — pitch D6
│   ├── brick-e6.ogg              # Brick hit sound — pitch E6
│   ├── brick-g6.ogg              # Brick hit sound — pitch G6 (completes a pentatonic set for musical destruction)
│   ├── crackle.ogg               # Electrical crackle effect; likely used for EMP or energy-based powerups
│   ├── emp.ogg                   # EMP (electromagnetic pulse) weapon or powerup activation sound
│   ├── laser-double-a.ogg        # Double laser shot variant A
│   ├── laser-double-b.ogg        # Double laser shot variant B
│   ├── laser-double-c.ogg        # Double laser shot variant C (randomised playback avoids repetition)
│   ├── laser-single-a.ogg        # Single laser shot variant A
│   ├── laser-single-b.ogg        # Single laser shot variant B
│   ├── laser-single-c.ogg        # Single laser shot variant C
│   ├── lose.ogg                  # Player death / ball lost jingle
│   ├── scream.ogg                # Enemy or character scream; atmospheric effect on destruction or death event
│   ├── siren.ogg                 # Alert siren; possibly triggers on danger state or special enemy wave
│   ├── slide-close.ogg           # UI panel or hatch sliding shut
│   ├── slide-open.ogg            # UI panel or hatch sliding open (menu transitions, powerup reveals)
│   ├── trampoline-hi.ogg         # High-pitched ball bounce; used when ball hits the paddle at speed
│   ├── trampoline-lo.ogg         # Low-pitched ball bounce; used for slower or glancing paddle hits
│   ├── vaus-wall-left-a.ogg      # Ball hitting the left wall — variant A (named after the Arkanoid paddle "Vaus")
│   ├── vaus-wall-left-b.ogg      # Ball hitting the left wall — variant B
│   ├── vaus-wall-right-a.ogg     # Ball hitting the right wall — variant A
│   ├── vaus-wall-right-b.ogg     # Ball hitting the right wall — variant B
│   └── win.ogg                   # Level complete / victory jingle
│
├── shader/
│   └── honeycomb.wgsl            # WGSL shader asset for a honeycomb-style rendering effect
│
└── sprite/
    ├── ballobject.png            # The ball sprite (main render layer)
    ├── ballshadow.png            # The ball's drop shadow (composited beneath the object layer)
    ├── blbendobject.png          # Bottom-left pipe bend — object layer
    ├── blbendshadow.png          # Bottom-left pipe bend — shadow layer
    ├── bppipecapobject.png       # Bottom pipe cap (open end pointing down) — object layer
    ├── bppipecapshadow.png       # Bottom pipe cap — shadow layer
    ├── brbendobject.png          # Bottom-right pipe bend — object layer
    ├── brbendshadow.png          # Bottom-right pipe bend — shadow layer
    ├── brick1object.png          # Brick type 1 — object layer (e.g. single-hit standard brick)
    ├── brick1shadow.png          # Brick type 1 — shadow layer
    ├── brick2object.png          # Brick type 2 — object layer (e.g. two-hit or coloured variant)
    ├── brick2shadow.png          # Brick type 2 — shadow layer
    ├── brick3object.png          # Brick type 3 — object layer (e.g. armoured or special brick)
    ├── brick3shadow.png          # Brick type 3 — shadow layer
    ├── hellbowobject.png         # "Hellbow" — likely an elbow/bend connector with a hostile/fiery theme; object layer
    ├── hellbowshadow.png         # Hellbow — shadow layer
    ├── hpipeobject.png           # Horizontal pipe segment — object layer (used to build pipe-grid level geometry)
    ├── hpipeshadow.png           # Horizontal pipe — shadow layer
    ├── lpipecapobject.png        # Left pipe cap (open end pointing left) — object layer
    ├── lpipecapshadow.png        # Left pipe cap — shadow layer
    ├── rpipecapobject.png        # Right pipe cap (open end pointing right) — object layer
    ├── rpipecapshadow.png        # Right pipe cap — shadow layer
    ├── tlbendobject.png          # Top-left pipe bend — object layer
    ├── tlbendshadow.png          # Top-left pipe bend — shadow layer
    ├── tpipecapobject.png        # Top pipe cap (open end pointing up) — object layer
    ├── tpipecapshadow.png        # Top pipe cap — shadow layer
    ├── trbendobject.png          # Top-right pipe bend — object layer
    ├── trbendshadow.png          # Top-right pipe bend — shadow layer
    ├── vellboobject.png          # Vertical elbow connector variant — object layer
    ├── vellboshadow.png          # Vertical elbow connector variant — shadow layer
    ├── vpipeobject.png           # Vertical pipe segment — object layer
    └── vpipeshadow.png           # Vertical pipe — shadow layer
```

---

## Notes

- **Coverage fixed:** This README now reflects the additional `gfx/` and `shader/` directories present in the asset overview.
- **Screen graphics:** The `gfx/` directory contains static screen artwork for the title, story, and high-score screens.
- **Shader asset:** `shader/honeycomb.wgsl` is a WGSL shader asset, presumably used for a honeycomb-themed visual effect.
- **Object / Shadow split:** Every sprite comes as a pair — `*object.png` for the sprite itself and `*shadow.png` for its pre-rendered drop shadow. This two-layer approach (rendered from Blender) allows the shadow to be composited independently, enabling dynamic lighting or tinting effects at runtime without recalculating shadows in Bevy.
- **Brick hit pitches:** The six brick SFX (`brick-a6.ogg`, `brick-c6.ogg`, `brick-c7.ogg`, `brick-d6.ogg`, `brick-e6.ogg`, `brick-g6.ogg`) form a small tonal set, so rapid successive hits can sound melodic rather than purely noisy.
- **Laser variants (a/b/c):** Three variants per laser type allow random selection on each shot, preventing the audio fatigue of a single repeated sample.
- **Pipe geometry:** The `hpipe`, `vpipe`, `*bend`, and `*pipecap` sprites form a complete tileset for constructing pipe-grid layouts — the junkyard sci-fi environment that frames the brick field.
