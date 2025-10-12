# Turtle Graphics Library - AI Agent Instructions

## Project Overview

Rust workspace with turtle graphics implementations. **Primary focus: `turtle-lib`** - lightweight library using Macroquad + Lyon for GPU-accelerated rendering.

### Workspace Structure
```
turtlers/
├── turtle-lib/        # MAIN LIBRARY - Macroquad + Lyon (focus here)
└── turtle-lib-macros/ # Proc macro for turtle_main
```

## Architecture (`turtle-lib`)

### Core Design Pattern: Command Queue + Tweening
- **Builder API** (`TurtlePlan`) accumulates commands
- **Command Queue** stores execution plan
- **Tween Controller** interpolates between states for animation
- **Lyon Tessellation** converts all primitives to GPU meshes

### Key Files
```
src/
├── lib.rs           - Public API, TurtleApp (main loop), re-exports
├── builders.rs      - Fluent API traits (forward/right/etc chain)
├── commands.rs      - TurtleCommand enum (Move/Turn/Circle/etc)
├── execution.rs     - Execute commands, update state
├── tweening.rs      - Animation interpolation, speed control
├── drawing.rs       - Render Lyon meshes with Macroquad
├── tessellation.rs  - Lyon integration (polygons/strokes/fills/arcs)
├── state.rs         - TurtleState, TurtleWorld, FillState
└── circle_geometry.rs - Arc/circle math
```

### Critical Concepts

**1. Consolidated Commands** (reduces duplication):
- `Move(distance)` - negative = backward
- `Turn(angle)` - positive = right, negative = left (degrees)
- `Circle{radius, angle, steps, direction}` - unified left/right

**2. Fill System** (multi-contour with holes):
- `FillState` tracks `Vec<Vec<Vec2>>` (multiple contours)
- `pen_up()` closes current contour, `pen_down()` opens new
- Lyon's EvenOdd fill rule auto-detects holes
- Example: Donut = outer circle + inner circle (2 contours)

**3. Speed Modes**:
- `< 999`: Animated with tweening
- `>= 999`: Instant execution
- Controlled via `SetSpeed` commands (dynamic switching)

**4. Lyon Tessellation Pipeline**:
All drawing → Lyon → GPU mesh → Macroquad rendering
- ~410 lines eliminated vs manual triangulation
- Functions: `tessellate_polygon/stroke/circle/arc/multi_contour`

## Developer Workflows

### Building & Testing
```bash
# Main library
cargo build --package turtle-lib
cargo test --package turtle-lib
cargo clippy --package turtle-lib -- -Wclippy::pedantic \
  -Aclippy::cast_precision_loss -Aclippy::cast_sign_loss -Aclippy::cast_possible_truncation

# Run examples (15+ examples available)
cargo run --package turtle-lib --example hello_turtle
cargo run --package turtle-lib --example yinyang
cargo run --package turtle-lib --example cheese_macro
```

### Macro Crate
```bash
cargo build --package turtle-lib-macros
```

### Code Quality Standards
- Clippy pedantic mode enabled
- Cast warnings allowed for graphics math
- All examples must build warning-free
- Use `#[must_use]` on builder methods

## Project-Specific Patterns

### 1. The `turtle_main` Macro (PREFERRED for examples)
Simplest way to create turtle programs:
```rust
use turtle_lib::*;

#[turtle_main("Window Title")]
fn draw(turtle: &mut TurtlePlan) {
    turtle.forward(100.0).right(90.0);
}
```

Generates: window setup + render loop + quit handling (ESC/Q)

### 2. Import Convention
Only need: `use turtle_lib::*;`
- Re-exports: `vec2`, `RED/BLUE/GREEN/etc`, all turtle types
- No `use macroquad::prelude::*` needed (causes unused warnings)

### 3. Builder Chain Pattern
```rust
let mut t = create_turtle();
t.forward(100).right(90)
 .set_pen_color(BLUE)
 .circle_left(50.0, 360.0, 36)
 .begin_fill()
 .end_fill();
let app = TurtleApp::new().with_commands(t.build());
```

### 4. Multi-Contour Fill Example
```rust
turtle.begin_fill();
turtle.circle_left(100.0, 360.0, 72);  // Outer circle
turtle.pen_up();  // Closes contour
turtle.goto(vec2(0.0, -30.0));
turtle.pen_down(); // Opens new contour
turtle.circle_left(30.0, 360.0, 36);   // Inner (becomes hole)
turtle.end_fill(); // EvenOdd rule creates donut
```

### 5. Manual Setup (advanced control)
```rust
#[macroquad::main("Custom")]
async fn main() {
    let mut turtle = create_turtle();
    // ... drawing code ...
    let mut app = TurtleApp::new().with_commands(turtle.build());
    
    loop {
        clear_background(WHITE);
        app.update();
        app.render();
        next_frame().await;
    }
}
```

## Common Tasks

### Adding New Turtle Command
1. Add variant to `TurtleCommand` enum in `commands.rs`
2. Implement builder method in `builders.rs` (chain with `self`)
3. Add execution logic in `execution.rs`
4. Update tessellation/rendering if needed

### Adding Example
- Prefer `turtle_main` macro for simplicity
- Use only `use turtle_lib::*;`
- Keep examples focused (one concept each)
- See `examples/hello_turtle.rs` for minimal template

### Debugging Lyon Issues
- Enable tracing: `RUST_LOG=turtle_lib=debug cargo run`
- Check `tessellation.rs` for Lyon API usage
- EvenOdd fill rule: holes must have opposite winding

## Dependencies & Integration

### Main Dependencies
- `macroquad = "0.4"` - Window/rendering framework
- `lyon = "1.0"` - Tessellation (fills, strokes, circles)
- `tween = "2.1.0"` - Animation easing
- `tracing = "0.1"` - Optional logging (zero overhead when unused)

### Proc Macro Crate
- Separate crate required by Rust (proc-macro = true)
- Uses `syn`, `quote`, `proc-macro2`
- Generates full macroquad app boilerplate

## What NOT to Do

- Don't add `use macroquad::prelude::*` in examples when not required
- Don't manually triangulate - use Lyon functions
- Don't add commands for Forward/Backward separately (use Move)
- Don't create summary/comparison docs unless requested

## Key Documentation Files

- `README.md` - Main API docs
- `turtle-lib/README.md` - Library-specific docs
- `turtle-lib-macros/README.md` - Macro docs

## Response Style

- Be concise, no extensive summaries
- No emojis in technical responses
- Focus on code solutions over explanations
- Use bullet points for lists
- Reference specific files when helpful
