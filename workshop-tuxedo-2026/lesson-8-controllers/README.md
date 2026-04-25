# Bevy Input Demo

A minimal Bevy 0.18 app demonstrating four independently toggleable input methods controlling the horizontal movement of a box.

## Controls

| Key | Action |
|-----|--------|
| `1` | Toggle cursor keys (← / →) |
| `2` | Toggle WASD (A / D) |
| `3` | Toggle mouse / touch (drag horizontally with left button or finger) |
| `4` | Toggle gamepad (D-pad left/right or left analog stick X) |

All toggles are independent and additive — multiple methods can be active simultaneously. The window title bar shows which methods are currently `ON`. Vertical inputs (W, S, ↑, ↓, stick Y) are intentionally ignored.

Run with `cargo run --release`.

## Code overview

**`InputToggles`** is a single resource holding four bools — one per input method. `toggle_inputs` flips them on number-key press.

**`move_player`** is the heart of the app. It reads every enabled input source and contributes to a shared `dx` accumulator:

- Keyboard sources (cursor keys, WASD) and gamepad sources each add `±1.0` per pressed direction.
- The gamepad analog stick adds its raw X value (with a 0.1 deadzone).
- The accumulator is `clamp(-1.0, 1.0)`-ed before being scaled by `SPEED * delta_secs()`, so two methods both pushing left don't stack into double speed.

Mouse and touch take a different path: they **snap** the box's X to the pointer's world-space X (via `viewport_to_world_2d`) while the left mouse button or a touch is active. This is added on top of the keyboard/gamepad delta, so the last input "wins" on any given frame.

**`update_title`** rewrites the window title only when toggles change (gated by `is_changed()`), so it doesn't run every frame.

## Bevy 0.18 idioms used

- `Single<&Window>`, `Single<&mut Transform, With<Player>>`, etc. — the idiomatic way to assert "exactly one matching entity" without `Query::single()` boilerplate. The system silently no-ops on frames where any `Single` doesn't match (e.g. during window close).
- `Sprite::from_color(...)` instead of the older `SpriteBundle` — Bevy's required-component model means you spawn a `Sprite` and a `Transform` directly.
- `Query<&Gamepad>` — gamepads are entities, not a global resource. Each connected pad is one entity with a `Gamepad` component.
- `gamepad.pressed(GamepadButton::DPadLeft)` and `gamepad.get(GamepadAxis::LeftStickX)` — the post-0.15 unified gamepad API.
- `let ... && ...` chains in `if` — requires Rust edition 2024 (set in `Cargo.toml`).

## Things deliberately left out

- No deadzone configuration beyond the 0.1 stick threshold. For real games, configure `GamepadSettings` per pad.
- No screen bounds clamping — the box can fly off-screen.
- No pointer-only mode that ignores keyboard while held; all active methods always contribute.
