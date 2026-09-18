# Turtle Graphics Library - AI Agent Instructions

## Project Overview

Rust workspace with turtle graphics implementations. **Primary focus: `turtle-lib`** - lightweight library using Macroquad + Lyon for GPU-accelerated rendering with multi-turtle support and optional threading.

### Workspace Structure
```
turtlers/
├── turtle-lib/        # MAIN LIBRARY - Macroquad + Lyon (focus here)
│   └── examples/      # 30 examples including threading patterns
└── turtle-lib-macros/ # Proc macro for turtle_main
```

## Architecture (`turtle-lib`)

### Core Design Pattern: Turtle Entities + Persistent Animation Controllers
- **Builder API** (`TurtlePlan`) accumulates commands into immutable `CommandQueue`
- **TurtleWorld** maintains persistent `Vec<Turtle>` (`world.turtles`), each encapsulating state and a `TweenController`
- **TweenController** manages command execution, queue consumption, and animation interpolation per turtle
- **Lyon Tessellation** converts all primitives to GPU meshes
- **Multi-Turtle** support: Create multiple turtles with `add_turtle()` or threading channels

### Key Architectural Decision: Turtle Entity Encapsulation
Each turtle in `TurtleWorld` is represented by a `Turtle` struct (`state.rs`) storing its own `turtle_id`, `TurtleParams`, fill state, tessellated drawing commands, SVG log, and an embedded `TweenController`.
- `turtle_id` is stored directly on `Turtle` (and passed to `TweenController::update` for logging and side effects); `TweenController` itself manages animation state without needing turtle identity.
- Rendering (`drawing.rs`) iterates through `world.turtles` sequentially, directly accessing each turtle's `tween_controller.current_tween()`, `commands`, and `filling`.

### Key Files
```
src/
├── lib.rs              - TurtleApp, multi-turtle API, channel integration
├── builders.rs         - Fluent API traits (forward/right/circle/reset/etc)
├── commands.rs         - TurtleCommand enum (Move/Turn/Circle/Reset/etc)
├── execution.rs        - Command execution (immediate) + state updates
├── tweening.rs         - Animation + tween interpolation (TweenController per Turtle)
├── drawing.rs          - Lyon mesh rendering with Macroquad
├── state.rs            - Turtle, TurtleParams, TurtleWorld (persistent state)
├── tessellation.rs     - Lyon integration (polygons/strokes/fills/arcs)
├── circle_geometry.rs  - Arc/circle math helpers
└── commands_channel.rs - Async channels for threading patterns
```

### Critical Concepts

**1. Consolidated Move Commands**:
- `Move(distance)` - negative = backward (no separate Backward)
- `Turn(angle)` - positive = right, negative = left (degrees)
- `Circle{radius, angle, steps, direction}` - unified left/right via CircleDirection
- `Reset` - clears drawings, animations, fill state; resets params to defaults

**2. Fill System (Multi-Contour with Holes)**:
- `FillState` tracks `Vec<Vec<Coordinate>>` (multiple closed contours)
- `pen_up()` closes current contour, `pen_down()` opens new one
- Lyon's EvenOdd fill rule auto-detects holes (inner contours with opposite winding)
- Example: Donut = outer circle (pen_down) → pen_up → inner circle → end_fill

**3. Animation Modes**:
- Speed `>= 1000`: Instant mode (no tweening, executes with `max(1, speed - 1000)` draw calls per frame)
- Speed `< 1000`: Animated mode (tweens with CubicInOut easing, duration based on distance/speed)
- Dynamic switching via `SetSpeed` command mid-animation

**4. Multi-Turtle Architecture**:
- Each turtle in `world.turtles` owns its persistent `TweenController`
- Rendering directly inspects each turtle's active tween via `turtle.tween_controller.current_tween()` during sequential traversal of `world.turtles`
- Supports concurrent animation of multiple turtles
- Threading channels: `create_turtle_channel(buffer_size)` returns `TurtleCommandSender`, with the receiver managed internally by `TurtleApp`

**5. Threading Pattern** (for interactive apps like Hangman):
- `create_turtle_channel(buffer_size)` returns `TurtleCommandSender` (clonable, Send)
- Spawn game logic on thread, send `CommandQueue` batches via channel
- Main render loop calls `app.process_commands()` before `update()` to drain channels
- Example: `hangman_threaded.rs` spawns stdin reader, sends drawing commands when player guesses

## Developer Workflows

### Building & Testing
```bash
# Main library
cargo build --package turtle-lib
cargo test --package turtle-lib
cargo clippy --package turtle-lib -- -Wclippy::pedantic \
  -Aclippy::cast_precision_loss -Aclippy::cast_sign_loss -Aclippy::cast_possible_truncation

# Run examples
cargo run --package turtle-lib --example hello_turtle
cargo run --package turtle-lib --example yinyang
cargo run --package turtle-lib --example hangman_threaded
```

### Code Quality Standards
- Clippy pedantic enabled (graphics math casts allowed)
- Examples must build warning-free
- Use `#[must_use]` on builder methods
- Builder methods return `&mut Self` (never owned Self) for chaining

## Project-Specific Patterns

### 1. Single Turtle (Default)
```rust
#[turtle_main("Simple")]
fn draw(turtle: &mut TurtlePlan) {
    turtle.forward(100.0).right(90.0);
}
```

### 2. Multi-Turtle Direct Setup
```rust
let mut app = TurtleApp::new();
let t0_id = app.add_turtle();  // Default setup
let t1_id = app.add_turtle();

app.append_commands(t0_id, turtle1_plan.build());
app.append_commands(t1_id, turtle2_plan.build());
```

### 3. Threading Pattern (Hangman Example)
```rust
let mut app = TurtleApp::new();
let turtle_tx = app.create_turtle_channel(100);

// Spawn game thread
let tx = turtle_tx.clone();
std::thread::spawn(move || {
    loop {
        let letter = get_input();  // Blocks
        let mut plan = create_turtle();
        plan.forward(50.0);
        tx.send(plan.build()).ok();
    }
});

// Main loop
loop {
    clear_background(WHITE);
    app.process_commands();  // ← Drains channel
    app.update();
    app.render();
    next_frame().await;
}
```

### 4. Multi-Contour Fills (Donut)
```rust
turtle.set_fill_color(BLUE)
      .begin_fill()
      .circle_left(100.0, 360.0, 72);   // Outer
      
turtle.pen_up()
      .go_to(vec2(0.0, -30.0))
      .pen_down()
      .circle_left(30.0, 360.0, 36);    // Inner (hole)

turtle.end_fill();  // EvenOdd creates hole
```

### 5. Reset Turtle
```rust
turtle.forward(100.0)
      .reset()              // Clears drawings, resets to defaults
      .forward(50.0);       // Fresh start
```

## Common Tasks

### Adding New Turtle Command
1. Add variant to `TurtleCommand` enum in `commands.rs`
2. Implement builder method in `builders.rs` (always return `&mut Self`)
3. Add execution in `execution.rs` (for immediate state changes) or `tweening.rs` (for animated state)
4. If animated, implement in `calculate_target_state()` in `tweening.rs`
5. Update drawing/tessellation if needed

### Adding an Example
- Use `turtle_main` macro for simplicity
- Import only `use turtle_lib::*;` (all exports included)
- For threading: use `create_turtle_channel()` + `process_commands()`
- Place in `turtle-lib/examples/` and update README examples

### Debugging Animation Issues
```bash
RUST_LOG=turtle_lib=debug cargo run --example yinyang
```
- Check `tweening.rs` for state transitions
- Verify `command_creates_drawing()` includes your command type
- Circle direction: Left = counter-clockwise, Right = clockwise

## Critical Implementation Details

### TurtleCommand::Reset Behavior
- Clears `Turtle::commands` (all drawings)
- Clears `Turtle::filling` (ongoing fill operations)
- Resets `TurtleParams` to defaults (position 0,0, heading 0, pen down, etc.)
- Preserves `turtle_id` after reset
- Called via `execute_command()` in both instant and animated modes

### Turtle Entity and State Encapsulation
- Each `Turtle` struct owns its `turtle_id`, visual `params`, `filling` state, tessellated `commands`, `svg_log`, and `tween_controller`
- `TweenController` is responsible purely for command queue management and interpolation, decoupled from turtle identity
- Rendering iterates sequentially over `world.turtles`, accessing each turtle's `commands`, active tween (`turtle.tween_controller.current_tween()`), and live fill preview in place

### Lyon Tessellation
- All drawing → `tessellate_arc/stroke/circle/multi_contour` → `MeshData` → Macroquad `Mesh`
- Circle direction affects angle stepping: Left subtracts, Right adds
- Start angle for circles: `geom.start_angle_from_center.to_degrees()` (NOT rotation)
- EvenOdd fill rule: holes detected by contour winding (no explicit flag needed)

## Dependencies & Integration

### Main Dependencies
- `macroquad = "0.4"` - Window/rendering framework
- `lyon = "1.0"` - Tessellation (fills, strokes, circles)
- `tween = "2.2.0"` - Animation easing (CubicInOut)
- `tracing = "0.1"` - Optional logging (zero cost when unused)
- `crossbeam = "0.8"` - Threading pattern support (channels)

## What NOT to Do

- Don't assume `TweenController` stores `turtle_id` or look up active tweens globally by ID (rendering iterates `world.turtles` and inspects `turtle.tween_controller` directly)
- Don't add `use macroquad::prelude::*` without explicit need (causes unused imports)
- Don't manually triangulate—always use Lyon `tessellate_*` functions
- Don't separate Forward/Backward—use negative `Move` values
- Don't call `reset()` expecting to preserve drawing state—it clears everything

## Response Style

- Be concise, actionable, focused on code
- Reference specific files/lines when helpful
- Use examples from `examples/` directory
- No generic advice; focus on THIS project's patterns
