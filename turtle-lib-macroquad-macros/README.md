# turtle-lib-macroquad-macros

Procedural macros for `turtle-lib-macroquad`.

## `turtle_main` Macro

The `turtle_main` macro simplifies creating turtle graphics programs by automatically setting up:
- The Macroquad window
- Turtle initialization
- The main rendering loop
- Quit handling (ESC or Q keys)

### Usage

#### With a function parameter:

```rust
use macroquad::prelude::*;
use turtle_lib_macroquad::*;

#[turtle_main("My Drawing")]
fn my_drawing(turtle: &mut TurtlePlan) {
    turtle.set_pen_color(RED);
    turtle.forward(100.0);
    turtle.right(90.0);
    turtle.forward(100.0);
}
```

#### With inline code:

```rust
use macroquad::prelude::*;
use turtle_lib_macroquad::*;

#[turtle_main("My Drawing")]
fn my_drawing() {
    turtle.set_pen_color(RED);
    turtle.forward(100.0);
    turtle.right(90.0);
    turtle.forward(100.0);
}
```

### What it does

The macro expands your code into a full Macroquad application with:
- `#[macroquad::main]` attribute for window creation
- Turtle instance creation
- TurtleApp initialization with your commands
- A main loop that:
  - Clears the background to WHITE
  - Updates the turtle app
  - Renders the drawing
  - Shows "Press ESC or Q to quit" message
  - Handles quit keys

### Benefits

- **Less boilerplate**: No need to write the same loop structure in every example
- **Consistent UI**: All examples have the same quit behavior
- **Beginner-friendly**: Makes turtle graphics examples more approachable
- **Focus on drawing**: Your code focuses on the turtle commands, not the framework

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
