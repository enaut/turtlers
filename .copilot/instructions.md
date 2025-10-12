## Project Context
- **turtle-lib**: Heavy Bevy-based turtle graphics (0.17.1) with ECS architecture
- **turtle-lib-macroquad**: Lightweight macroquad implementation with Lyon tessellation (current focus)
- **turtle-lyon-poc**: Proof of concept for Lyon (COMPLETED - integrated into main crate)
- **turtle-skia-poc**: Alternative tiny-skia rendering approach
- **Status**: Lyon migration complete, fill quality issues resolved

## Architecture
```
turtle-lib-macroquad/src/
├── lib.rs              - Public API & TurtleApp
├── state.rs            - TurtleState & TurtleWorld
├── commands.rs         - TurtleCommand & CommandQueue
├── builders.rs         - Builder traits (DirectionalMovement, Turnable, CurvedMovement)
├── execution.rs        - Command execution
├── tweening.rs         - Animation controller
├── drawing.rs          - Macroquad rendering
├── tessellation.rs     - Lyon tessellation (355 lines - polygons, circles, arcs, strokes)
├── circle_geometry.rs  - Circle/arc geometry calculations
├── shapes.rs           - Turtle shapes
└── general/            - Type definitions (Angle, Length, Color, etc.)
```

## Current Status
1. ✅ **Lyon integration complete**: Using Lyon 1.0 for all tessellation
2. ✅ **Fill quality fixed**: EvenOdd fill rule handles complex fills and holes automatically
3. ✅ **Simplified codebase**: Replaced manual triangulation with Lyon's GPU-optimized tessellation
4. ✅ **Full feature set**: Polygons, circles, arcs, strokes all using Lyon

## Key Features
- **Builder API**: Fluent interface for turtle commands
- **Animation system**: Tweening controller with configurable speeds (Instant/Animated)
- **Lyon tessellation**: Automatic hole detection, proper winding order, GPU-optimized
- **Fill support**: Multi-contour fills with automatic hole handling
- **Shapes**: Arrow, circle, square, triangle, classic turtle shapes

## Response Style Rules
- NO emoji/smileys
- NO extensive summaries
- Use bullet points for lists
- Be concise and direct
- Focus on code solutions

# Tools to use
- when in doubt you can always use #fetch to get additional docs and online information.
- when the userinput is incomplete generate a brief text and let the user confirm your understanding.

## Code Patterns

### Lyon Tessellation (Current)
```rust
// tessellation.rs - Lyon integration
pub fn tessellate_polygon(vertices: &[Vec2], color: Color) -> Result<MeshData, Box<dyn std::error::Error>>
pub fn tessellate_multi_contour(contours: &[Vec<Vec2>], color: Color) -> Result<MeshData, Box<dyn std::error::Error>>
pub fn tessellate_stroke(vertices: &[Vec2], color: Color, width: f32, closed: bool) -> Result<MeshData, Box<dyn std::error::Error>>
pub fn tessellate_circle(center: Vec2, radius: f32, color: Color, filled: bool, stroke_width: f32) -> Result<MeshData, Box<dyn std::error::Error>>
pub fn tessellate_arc(center: Vec2, radius: f32, start_angle: f32, arc_angle: f32, color: Color, stroke_width: f32, segments: usize) -> Result<MeshData, Box<dyn std::error::Error>>
```

### Fill with Holes
```rust
// Multi-contour fills automatically detect holes using EvenOdd fill rule
let contours = vec![outer_boundary, hole1, hole2];
let mesh = tessellate_multi_contour(&contours, color)?;
```

## Builder API
```rust
let mut t = create_turtle();
t.forward(100).right(90)
 .circle_left(50.0, 180.0, 36)
 .begin_fill()
 .set_fill_color(BLACK)
 .circle_left(90.0, 180.0, 36)
 .end_fill();
let app = TurtleApp::new().with_commands(t.build());
```

## File Links
- Main crate: [turtle-lib-macroquad/src/lib.rs](turtle-lib-macroquad/src/lib.rs)
- Tessellation: [turtle-lib-macroquad/src/tessellation.rs](turtle-lib-macroquad/src/tessellation.rs)
- Rendering: [turtle-lib-macroquad/src/drawing.rs](turtle-lib-macroquad/src/drawing.rs)
- Animation: [turtle-lib-macroquad/src/tweening.rs](turtle-lib-macroquad/src/tweening.rs)
- Examples: [turtle-lib-macroquad/examples/](turtle-lib-macroquad/examples/)

## Testing
Run examples to verify Lyon integration:
```bash
cargo run --example yinyang
cargo run --example stern
cargo run --example nikolaus
```

## Code Quality
Run clippy with strict checks on turtle-lib-macroquad:
```bash
cargo clippy --package turtle-lib-macroquad -- -Wclippy::pedantic -Wclippy::cast_precision_loss -Wclippy::cast_sign_loss -Wclippy::cast_possible_truncation
```
Note: Cast warnings are intentionally allowed for graphics code where precision loss is acceptable.

## Dependencies
- macroquad 0.4 - Game framework and rendering
- lyon 1.0 - Tessellation (fills, strokes, circles, arcs)
- tween 2.1.0 - Animation easing
- tracing 0.1 - Logging (with log features)