# Turtle Graphics Library for Macroquad

A turtle graphics library built on [Macroquad](https://macroquad.rs/), providing an intuitive API for creating drawings and animations.

## Features

- **Simple Builder API**: Chain commands like `forward(100).right(90)`
- **Smooth Animations**: Tweening support with easing functions
- **Instant Mode**: Execute commands immediately without animation (speed > 999.0)
- **Lightweight**: Fast compilation (~30-60 seconds from clean build)
- **Macroquad Integration**: Built on the simple and fast Macroquad framework

## Quick Start

Add to your `Cargo.toml`:
```toml
[dependencies]
turtle-lib-macroquad = { path = "../turtle-lib-macroquad" }
macroquad = "0.4"
```

### Basic Example

```rust
use macroquad::prelude::*;
use turtle_lib_macroquad::*;

#[macroquad::main("Turtle")]
async fn main() {
    // Create a turtle plan
    let mut plan = create_turtle();
    
    // Draw a square
    for _ in 0..4 {
        plan.forward(100).right(90);
    }
    
    // Create app with animation (100 pixels/sec)
    let mut app = TurtleApp::new().with_commands(plan.build(), 100.0);
    
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

// Appearance
plan.set_color(RED);
plan.set_pen_width(5.0);
plan.hide();
plan.show();

// Turtle shape
plan.shape(ShapeType::Triangle);
plan.shape(ShapeType::Turtle);    // Default classic turtle shape
plan.shape(ShapeType::Circle);
plan.shape(ShapeType::Square);
plan.shape(ShapeType::Arrow);

// Custom shape
let custom = TurtleShape::new(
    vec![vec2(10.0, 0.0), vec2(-5.0, 5.0), vec2(-5.0, -5.0)],
    true  // filled
);
plan.set_shape(custom);

// Chaining
plan.forward(100).right(90).forward(50);
```

### Execution Modes

```rust
// Animated mode (speed in pixels/sec, 0.5-999.0)
let app = TurtleApp::new().with_commands(queue, 100.0);

// Instant mode (speed >= 999.0)
let app = TurtleApp::new().with_commands(queue, 1000.0);
```

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

## Examples

Run examples with:
```bash
cargo run --example square
cargo run --example koch
cargo run --example shapes
cargo run --example yinyang
cargo run --example stern
cargo run --example nikolaus
```

### Available Examples

- **square.rs**: Basic square drawing
- **koch.rs**: Koch snowflake fractal
- **shapes.rs**: Demonstrates different turtle shapes
- **yinyang.rs**: Yin-yang symbol drawing
- **stern.rs**: Star pattern drawing
- **nikolaus.rs**: Nikolaus (Santa) drawing

## Turtle Shapes

The library supports multiple turtle shapes that can be changed during drawing:

### Built-in Shapes

- **Triangle** (default): Simple arrow shape
- **Turtle**: Classic turtle shape with detailed outline
- **Circle**: Circular shape
- **Square**: Square shape
- **Arrow**: Arrow-like shape

### Using Shapes

```rust
// Using built-in shapes
plan.shape(ShapeType::Turtle);

// Creating custom shapes
let my_shape = TurtleShape::new(
    vec![
        vec2(15.0, 0.0),   // Point at front
        vec2(-10.0, -8.0),  // Bottom back
        vec2(-10.0, 8.0),   // Top back
    ],
    true  // filled
);
plan.set_shape(my_shape);
```

Shapes are automatically rotated to match the turtle's heading direction.

## Architecture

The library is designed for easy extension and potential multi-threading support:

- **State Management**: Clean separation between turtle state and world state
- **Command Queue**: Commands are queued and can be executed immediately or with tweening
- **Tweening System**: Smooth interpolation between states with easing functions
- **Rendering**: Direct Macroquad drawing calls with earcutr polygon triangulation
- **Shape System**: Extensible turtle shapes with support for both convex and concave polygons

### Module Structure

```
src/
├── lib.rs          - Public API and TurtleApp
├── state.rs        - TurtleState and TurtleWorld
├── commands.rs     - TurtleCommand enum and CommandQueue
├── builders.rs     - Builder traits (DirectionalMovement, Turnable, etc.)
├── execution.rs    - Command execution logic
├── tweening.rs     - Animation/tweening controller
├── drawing.rs      - Macroquad rendering
├── shapes.rs       - Turtle shape definitions
└── general/        - Type definitions (Angle, Length, etc.)
```

## Design Decisions

### Multi-threading Ready

While multi-threading is not implemented yet, the architecture supports future additions:
- State and world are separated
- Commands can be generated on separate threads
- Rendering happens on main thread (Macroquad requirement)

### Tweening vs Interpolation

We use "tweening" terminology throughout the codebase for clarity and game development conventions.

### No API Compatibility Constraints

This library is designed from scratch without backwards compatibility requirements, allowing for optimal design choices.

## Future Enhancements

Potential additions (not yet implemented):
- Multi-threading support for interactive games
- Filled shapes and polygons
- Text rendering
- Image stamps

## License

MIT OR Apache-2.0
