# Bevy 0.18 PBR Sprite Demo

A minimal demonstration of physically-based rendering applied to flat sprite quads in Bevy 0.18, comparing how diffuse, normal, and combined textures interact with PBR lighting.

## Running

```bash
cargo run
```

Required assets in `assets/`:

- `{prefix}diffuse.png` — flat albedo, no shading baked in
- `{prefix}normal.png` — tangent-space normal map (OpenGL convention)
- `{prefix}combined.png` — fully pre-lit reference sprite
- `environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2`
- `environment_maps/pisa_specular_rgb9e5_zstd.ktx2`

The texture prefix is configured via the `SPRITE_PREFIX` constant at the top of `src/main.rs` (default: `"ball"`). Environment maps are from the official Bevy assets repo.

## Scene

A row of five quads sits on the equator with a chrome test sphere floating above them. A single point light orbits the row in the XY plane, and the camera has both an `EnvironmentMapLight` (for PBR reflections) and a `Skybox` (so you can see what's being reflected).

### Test sphere (top)

Pure white metal, near-mirror finish (`metallic: 1.0`, `perceptual_roughness: 0.05`). No textures. Its only job is to verify that the environment map is loading — if you see the Pisa courtyard reflected here, IBL is working.

### Sprite 1 — Diffuse + normal map (PBR)

The "correct" way to do a PBR sprite. The diffuse texture provides flat albedo; the normal map provides per-pixel surface direction. Lighting and reflections are computed by the shader and react to the orbiting light. Tangents are generated on the quad mesh so the normal map can be sampled correctly.

### Sprite 2 — Combined, unlit

The pre-lit reference. `unlit: true` skips all PBR computation, so this sprite always looks identical regardless of light position. This is what a classic 2D sprite in a non-PBR engine looks like — useful as a baseline for comparison.

### Sprite 3 — Diffuse only (PBR, no normal map)

Same diffuse texture as sprite 1, but without the normal map. The flat quad uniformly brightens and dims as the light orbits, with no surface relief. Demonstrates how much the normal map contributes — strip it out and the sprite looks like a flat decal.

### Sprite 4 — Combined + normal map (PBR)

A "what not to do" example. The combined (already-lit) texture is fed to PBR as if it were albedo, with the normal map on top. The result has two layers of lighting fighting each other: baked highlights stay put while shader-computed highlights move with the light, and the shadowed sides go too dark. Shows why albedo and lighting must be separated for PBR.

### Sprite 5 — Diffuse + normal map, max metallic (PBR)

Same as sprite 1 but with `metallic: 1.0`. On a pure metal, the diffuse term vanishes and only specular reflections remain, tinted by the base color. The environment map shows up here, distorted by the normal map — bricks become a textured mirror.

## Configuration

Two constants at the top of `src/main.rs`:

- `SPRITE_PREFIX` — texture filename prefix (e.g. `"ball"`, `"brick3"`)
- `PERCEPTUAL_ROUGHNESS` — surface roughness applied to all PBR sprites (0.0 = mirror, 1.0 = fully matte). Default `0.2` to keep reflections visible.

## Key technical notes

- **Tangents are required for normal maps.** `Plane3d` doesn't generate them; `mesh.generate_tangents()` is called explicitly.
- **`AlphaMode::Mask(0.5)`** is used so transparent PNG pixels are discarded properly during the shadow pass, avoiding rectangular shadow artifacts.
- **`NotShadowCaster`** is added to the sprites — flat quads casting shadows on themselves looks odd. Remove it if you want sprite-shaped shadows on the ground.
- **An environment map is required for metallic surfaces to look correct.** Without one, pure metals appear black except where the point light hits them.
