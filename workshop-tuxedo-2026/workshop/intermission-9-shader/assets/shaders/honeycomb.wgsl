#import bevy_sprite::mesh2d_vertex_output::VertexOutput

// Honeycomb background shader.
//
// The hex grid is built from two interlaced rectangular grids (A and B).
// For any pixel we compute the offset to the nearest center in each grid,
// keep the shorter one (= `gv`), then evaluate a flat-top hexagon SDF on it.
//
// Grid geometry (in "grid units"):
//   period   r = (1, sqrt(3))
//   offset   h = r / 2
//   NN distance between centers = 1
//   ⇒ hex inradius  = 0.5   (apothem, center → edge midpoint)
//   ⇒ hex circumradius = 1/sqrt(3) ≈ 0.577  (center → vertex)
//
// Flat-top hexagon SDF (inradius normalized to 0.5):
//   d = max( |y|,  |x|·√3/2 + |y|·½ )
//   d = 0   at center
//   d = 0.5 at every point on the boundary

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {

    // World-space pixel coordinate (1 unit = 1 pixel with the default 2-D camera).
    // Rotate 30° so the grid becomes flat-top oriented.
    let c30 = 0.8660254; // cos(30°)
    let s30 = 0.5;       // sin(30°)
    let raw = in.world_position.xy;
    let pos = vec2<f32>(raw.x * c30 - raw.y * s30,
                        raw.x * s30 + raw.y * c30);

    // ── hex size ─────────────────────────────────────────────────────────────
    // Change hex_ir to scale the hexagons.
    // hex_ir is the inradius in pixels (center → flat edge midpoint).
    let hex_ir: f32 = 36.0;

    // ── convert to grid coordinates ──────────────────────────────────────────
    // In grid coords the period vector is r = (1, √3), so:
    //   1 grid unit in x  =  2·hex_ir  pixels
    //   1 grid unit in y  =  2·hex_ir  pixels   (same scale → regular hexagons)
    let p = pos / (2.0 * hex_ir);

    let r = vec2<f32>(1.0, 1.7320508); // (1, sqrt(3))
    let h = r * 0.5;

    // fract(x) = x − floor(x)  ∈ [0, 1)  — handles negative coords correctly.
    // Grid A: centers at  (i, j·√3)
    let a = fract(p / r) * r - h;
    // Grid B: centers at  (i + ½, (j + ½)·√3)
    let b = fract((p - h) / r) * r - h;

    // `gv` = offset from the nearest hex center (whichever grid it belongs to)
    let gv = select(b, a, dot(a, a) < dot(b, b));

    // ── flat-top hexagon SDF ─────────────────────────────────────────────────
    // Pointy-top hexagon SDF — matches the actual Voronoi cell shape.
    // (The r=(1,√3) interlaced grid produces pointy-top cells: vertices at
    //  top/bottom, flat edges left/right.  Swapping x↔y vs. flat-top SDF.)
    let d = max(abs(gv.x), abs(gv.y) * 0.8660254 + abs(gv.x) * 0.5);

    // ── colour palette ───────────────────────────────────────────────────────
    let center_color = vec3<f32>(0.14, 0.32, 0.72);  // bright blue  (hex centre)
    let body_color   = vec3<f32>(0.07, 0.18, 0.52);  // mid blue     (hex body)
    let border_color = vec3<f32>(0.01, 0.03, 0.10);  // near-black   (cell edge)

    // t ∈ [0, 1]:  0 = centre,  1 = Voronoi boundary (= touching neighbour)
    // Cells are space-filling: every pixel belongs to exactly one cell.
    let t = d / 0.5;

    // Radial gradient across the fill
    let base = mix(center_color, body_color, t * t);

    // Dark border *inside* the cell — makes cells visually distinct without gaps
    let border_t = smoothstep(0.80, 1.0, t);
    let color = mix(base, border_color, border_t);

    return vec4<f32>(color, 1.0);
}
