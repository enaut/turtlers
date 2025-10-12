# turtle-lib-macroquad

The main turtle graphics library built on Macroquad with Lyon tessellation.

**See the [main README](../README.md) for complete documentation.**

## Features

✅ **Complete Lyon Integration** - All drawing operations use GPU-optimized tessellation
- Unified rendering pipeline for lines, arcs, circles, and fills
- ~410 lines of code eliminated through architectural simplification
- Consistent high-quality rendering across all primitives

✅ **Multi-Contour Fill System** - Advanced fill capabilities with automatic hole detection
- EvenOdd fill rule for complex shapes with holes (like cheese or yin-yang symbols)
- `pen_up()` closes current contour, `pen_down()` opens next contour
- Progressive fill preview during animations
- Support for self-intersecting paths

✅ **Smooth Animation** - Tweening system with live rendering
- Configurable speed control
- Frame-rate independent animation
- Live fill preview during circle/arc drawing

## Quick Start

### Using the `turtle_main` Macro (Recommended for Beginners)

The easiest way to create turtle programs is with the `turtle_main` macro:

```rust
use macroquad::prelude::*;
use turtle_lib_macroquad::*;

#[turtle_main("My First Drawing")]
fn my_drawing(turtle: &mut TurtlePlan) {
    turtle.set_pen_color(RED);
    turtle.forward(100.0);
    turtle.right(90.0);
    turtle.forward(100.0);
}
```

The macro automatically handles:
- Window creation and setup
- Turtle initialization
- Rendering loop
- Quit handling (ESC or Q keys)

### Manual Setup (For Advanced Use)

For more control over the application loop:

```rust
use macroquad::prelude::*;
use turtle_lib_macroquad::*;

#[macroquad::main("Turtle")]
async fn main() {
    let mut turtle = create_turtle();
    turtle.forward(100.0).right(90.0);
    
    let mut app = TurtleApp::new().with_commands(turtle.build());
    
    loop {
        clear_background(WHITE);
        app.update();
        app.render();
        next_frame().await;
    }
}
```

## Quick Examples

All examples now use the `turtle_main` macro for simplicity:

```bash
# Run from this directory
cargo run --example hello_turtle      # Minimal 10-line example
cargo run --example macro_demo        # Simple square with macro
cargo run --example square            # Basic square drawing
cargo run --example shapes            # Different turtle shapes
cargo run --example yinyang           # Multi-contour fills with holes
cargo run --example koch              # Recursive fractals
cargo run --example fill_demo         # Fill with holes (donut)
cargo run --example cheese_macro      # Cheese example using macro
cargo run --example fill_advanced     # Complex shapes (manual setup)
```

Most examples use `turtle_main` for simplicity. A few keep manual setup for custom UI or logging.

## Architecture Highlights

### Rendering Pipeline
All drawing operations → Lyon tessellation → GPU mesh rendering

### DrawCommand Enum
Simplified from 5 variants to 1:
- `Mesh(MeshData)` - unified variant for all drawing operations

### Fill System
- `FillState` tracks multiple contours (completed + current)
- Pen state management automatically handles contour creation
- EvenOdd tessellation provides automatic hole detection

See [LYON_COMPLETE.md](LYON_COMPLETE.md) and [MULTI_CONTOUR_FILLS.md](MULTI_CONTOUR_FILLS.md) for implementation details.

## Status

✅ **Stable** - Lyon integration complete, multi-contour fills working, all examples passing.

See [../README.md](../README.md) for full API documentation and project status.
