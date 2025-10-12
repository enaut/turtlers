# Turtle Graphics Library

A modern turtle graphics library for Rust built on [Macroquad](https://macroquad.rs/) with [Lyon](https://github.com/nical/lyon) for high-quality GPU-accelerated rendering.

## Project Status

✅ **Stable** - Complete Lyon integration with multi-contour fill system and live animation preview.

## Features

- 🎨 **Simple Builder API**: Chain commands like `forward(100).right(90)`
- ⚡ **Smooth Animations**: Tweening support with easing functions and live fill preview
- 🚀 **Instant Mode**: Execute commands immediately without animation (speed ≥ 999)
- 🎯 **High-Quality Rendering**: Complete Lyon tessellation pipeline with GPU acceleration
- **Multi-Contour Fills**: Automatic hole detection with EvenOdd fill rule - draw cheese with holes!
- 📐 **Self-Intersecting Paths**: Stars, complex shapes - all handled correctly
- 🐢 **Multiple Turtle Shapes**: Triangle, classic turtle, circle, square, arrow, and custom shapes
- 🔍 **Structured Logging**: Optional `tracing` integration for debugging (zero overhead when disabled)
- 💨 **Lightweight**: Fast compilation and runtime

## Quick Start

```rust
use macroquad::prelude::*;
use turtle_lib::*;

#[macroquad::main("Turtle")]
async fn main() {
    // Create a turtle plan
    let mut plan = create_turtle();
    
    // Set speed (part of the plan)
    plan.set_speed(100);
    
    // Draw a square
    for _ in 0..4 {
        plan.forward(100).right(90);
    }
    
    // Create app (speed is managed by commands)
    let mut app = TurtleApp::new().with_commands(plan.build());
    
    loop {
        clear_background(WHITE);
        app.update();
        app.render();
        next_frame().await
    }
}
```

## API Overview

### Creating Plans

```rust
let mut plan = create_turtle();

// Movement
plan.forward(100);
plan.backward(50);

// Rotation
plan.left(90);    // degrees
plan.right(45);

// Circular arcs
plan.circle_left(50.0, 180.0, 36);   // radius, angle (degrees), segments
plan.circle_right(50.0, 180.0, 36);  // draws arc to the right

// Pen control
plan.pen_up();
plan.pen_down();

// Filling (with automatic hole detection)
plan.set_fill_color(BLUE);
plan.begin_fill();
// ... draw shape ...
plan.end_fill();  // Auto-closes and applies fill

// Appearance
plan.set_color(RED);
plan.set_pen_width(5.0);
plan.hide();
plan.show();

// Speed control (dynamic)
plan.set_speed(100);  // Animated mode (< 999)
plan.set_speed(1000); // Instant mode (>= 999)

// Turtle shapes
plan.shape(ShapeType::Triangle);
plan.shape(ShapeType::Turtle);    // Classic turtle shape
plan.shape(ShapeType::Circle);
plan.shape(ShapeType::Square);
plan.shape(ShapeType::Arrow);

// Custom shapes
let custom = TurtleShape::new(
    vec![vec2(10.0, 0.0), vec2(-5.0, 5.0), vec2(-5.0, -5.0)],
    true  // filled
);
plan.set_shape(custom);

// Method chaining
plan.forward(100).right(90).forward(50);
```

### Execution Modes

Speed is now controlled via commands, allowing dynamic switching during execution:

```rust
let mut plan = create_turtle();

// Fast initial positioning (instant mode)
plan.set_speed(1000);
plan.pen_up();
plan.goto(vec2(-100.0, -100.0));

// Slow animated drawing
plan.set_speed(50);
plan.pen_down();
plan.forward(200);
plan.right(90);

// Create app (no speed parameter needed)
let app = TurtleApp::new().with_commands(plan.build());
```

**Speed Modes:**
- **Speed < 999**: Animated mode with smooth tweening
- **Speed >= 999**: Instant mode (no animation)
- Default speed is 100.0 if not specified

### Animation Loop

```rust
loop {
    clear_background(WHITE);
    
    app.update();  // Update animation state
    app.render();  // Draw to screen
    
    if app.is_complete() {
        // All commands executed
    }
    
    next_frame().await
}
```

## Debugging and Logging

The library uses [`tracing`](https://docs.rs/tracing) for structured diagnostic logging. This is completely optional - if you don't set up a subscriber, there's zero overhead.

### Enable Logging

```rust
// Add to your Cargo.toml:
// tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt"] }

tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .init();
```

Control verbosity with the `RUST_LOG` environment variable:

```bash
# Show debug output
RUST_LOG=turtle_lib=debug cargo run

# Very verbose trace output
RUST_LOG=turtle_lib=trace cargo run
```

**See the complete example**: [`examples/logging_example.rs`](turtle-lib/examples/logging_example.rs) demonstrates initialization, log levels, filtering, and example output.

## Examples

Run examples with:
```bash
cargo run --example square
cargo run --example koch
cargo run --example shapes
cargo run --example yinyang
cargo run --example stern
cargo run --example nikolaus

# Logging example - shows how to enable debug output
cargo run --example logging_example
RUST_LOG=turtle_lib=debug cargo run --example logging_example
```

### Available Examples

#### Basic Drawing
- **square.rs**: Basic square drawing
- **koch.rs**: Koch snowflake fractal
- **shapes.rs**: Demonstrates different turtle shapes
- **stern.rs**: Star pattern drawing
- **nikolaus.rs**: Nikolaus (Santa) drawing

#### Fill Examples
- **yinyang.rs**: Yin-yang symbol with automatic hole detection
- **fill_demo.rs**: Donut shape with hole
- **fill_requirements.rs**: Circle with red fill
- **fill_advanced.rs**: Complex shapes (star, swiss cheese, multiple holes)
- **fill_circle_test.rs**: Circle fills with different angles
- **fill_instant_test.rs**: Quick fill test in instant mode

#### Debugging
- **logging_example.rs**: Demonstrates how to enable and use tracing/logging output

## Why Lyon?

- Automatic hole detection via EvenOdd fill rule
- GPU-accelerated rendering
- Standards-compliant (matches SVG, HTML Canvas)
- Handles any self-intersecting path

### Basic Fill
```rust
let mut plan = create_turtle();
plan.set_fill_color(RED);
plan.begin_fill();

// Draw shape
for _ in 0..4 {
    plan.forward(100);
    plan.right(90);
}

plan.end_fill();  // Auto-closes and fills
```

### Fill with Holes (Multi-Contour)
```rust
plan.set_fill_color(BLUE);
plan.begin_fill();

// Outer circle (first contour)
plan.circle_left(90.0, 360.0, 72);

// pen_up() closes current contour
plan.pen_up();
plan.goto(vec2(0.0, -30.0));

// pen_down() starts new contour
plan.pen_down();

// Inner circle (becomes a hole automatically with EvenOdd rule!)
plan.circle_left(30.0, 360.0, 36);

plan.end_fill();  // Auto-detects holes and fills correctly
```

### Fill Features

- ✅ **Live Preview** - See fills progressively during animation
- ✅ **Auto-Close** - Automatically connects end point to start on `end_fill()`
- ✅ **Multi-Contour** - `pen_up()` closes contour, `pen_down()` opens next one
- ✅ **Automatic Hole Detection** - EvenOdd fill rule handles any complexity
- ✅ **Self-Intersecting Paths** - Stars and complex shapes work perfectly

## Architecture

### Module Structure

```
turtle-lib/src/
├── lib.rs          - Public API and TurtleApp
├── state.rs        - TurtleState and TurtleWorld
├── commands.rs     - TurtleCommand enum (consolidated commands)
├── builders.rs     - Builder traits (DirectionalMovement, Turnable, etc.)
├── execution.rs    - Command execution with fill support
├── tweening.rs     - Animation/tweening controller with dynamic speed
├── drawing.rs      - Rendering with Lyon tessellation
├── shapes.rs       - Turtle shape definitions
├── tessellation.rs - Lyon tessellation utilities
├── circle_geometry.rs - Circle arc calculations
└── general/        - Type definitions (Angle, Length, etc.)
```

### Design Principles

- **State Management**: Clean separation between turtle state and world state
- **Command Queue**: Commands queued and executed with optional tweening
- **Consolidated Commands**: Unified commands reduce duplication (Move, Turn, Circle)
- **Dynamic Speed Control**: Speed managed via SetSpeed commands for flexibility
- **Tweening System**: Smooth interpolation with easing functions
- **Unified Lyon Rendering**: All drawing operations use GPU-accelerated Lyon tessellation
  - Lines, arcs, circles, fills - single high-quality rendering pipeline
  - ~410 lines of code eliminated through architectural simplification
  - Consistent quality across all primitives

### Command Consolidation

The library uses consolidated commands to reduce code duplication:

- **Move(distance)**: Replaces separate Forward/Backward (negative = backward)
- **Turn(angle)**: Replaces separate Left/Right (negative = left, positive = right)
- **Circle{direction, ...}**: Unified circle command with CircleDirection enum

This design eliminates ~250 lines of duplicate code while maintaining the same user-facing API.

## Workspace Structure

```
turtlers/
├── turtle-lib/         - Main library (Macroquad + Lyon)
└── turtle-lib-macros/  - Procedural macros (turtle_main)
```

## Building and Running

```bash
# Check all packages
cargo check

# Run specific example
cargo run --example yinyang

# Build release version
cargo build --release
```

## Development Status

### ✅ Completed
- Complete Lyon integration for all drawing primitives
- Multi-contour fill system with automatic hole detection
- Turtle movement and rotation (consolidated Move/Turn commands)
- Circle arcs (left/right with unified Circle command)
- Pen control (up/down) with contour management
- Color and pen width
- Multiple turtle shapes with custom shape support
- Tweening system with easing functions
- Dynamic speed control via SetSpeed commands
- Instant mode (speed ≥ 999) and animated mode (speed < 999)
- **EvenOdd fill rule** for complex self-intersecting paths
- **Live fill preview** during animation with progressive rendering
- **Multi-contour support** - pen_up/pen_down manage contours
- **Command consolidation** (~250 lines eliminated)
- **Full Lyon migration** (~410 total lines eliminated)

### 🎯 Future Possibilities
- Advanced stroke styling (caps, joins, dashing)
- Bezier curves and custom path primitives
- Additional examples and tutorials

## What's New

### Complete Lyon Migration ✨
All drawing operations now use GPU-accelerated Lyon tessellation:
- **Unified pipeline**: Lines, arcs, circles, and fills - all use the same high-quality rendering
- **Simplified codebase**: ~410 lines of code eliminated
- **Better performance**: GPU tessellation is faster than CPU-based primitives
- **Consistent quality**: No more mixed rendering approaches

### Multi-Contour Fill System 🕳️
Advanced fill capabilities with automatic hole detection:
- **EvenOdd fill rule**: Draw shapes with holes - works like SVG and HTML Canvas
- **Pen state management**: `pen_up()` closes contour, `pen_down()` opens next
- **Live preview**: See fills progressively during animations
- **Self-intersecting paths**: Stars, complex shapes - all handled correctly

### Architectural Improvements 🏗️
- **Command consolidation**: Unified Move/Turn/Circle commands (~250 lines eliminated)
- **Dynamic speed control**: Change speed during execution via commands
- **Live animation preview**: Progressive fill rendering during circle/arc drawing

## License

MIT OR Apache-2.0

## Contributing

Contributions are welcome! The library now has a stable foundation with complete Lyon integration and multi-contour fill support.
