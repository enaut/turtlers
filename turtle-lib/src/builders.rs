//! Builder pattern traits for creating turtle command sequences

use crate::commands::{CommandQueue, TurtleCommand};
use crate::general::{AnimationSpeed, Color, Coordinate, Precision};
use crate::shapes::{ShapeType, TurtleShape};

/// Trait for adding commands to a queue
pub trait WithCommands {
    fn get_commands_mut(&mut self) -> &mut CommandQueue;
    fn get_commands(self) -> CommandQueue;
}

/// Trait for forward/backward movement
pub trait DirectionalMovement: WithCommands {
    /// Moves the turtle forward by the specified distance.
    ///
    /// The turtle moves in the direction of its current heading.
    /// If the pen is down, a line is drawn.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use turtle_lib::*;
    /// #
    /// #[turtle_main("Forward Example")]
    /// fn draw(turtle: &mut TurtlePlan) {
    ///     // Move forward 100 pixels
    ///     turtle.forward(100.0);
    ///
    ///     // Chain movements
    ///     turtle.forward(50.0).right(90.0).forward(50.0);
    /// }
    /// ```
    fn forward<T>(&mut self, distance: T) -> &mut Self
    where
        T: Into<Precision>,
    {
        let dist: Precision = distance.into();
        self.get_commands_mut().push(TurtleCommand::Move(dist));
        self
    }

    /// Moves the turtle backward by the specified distance.
    ///
    /// The turtle moves opposite to its current heading without changing
    /// the heading direction. If the pen is down, a line is drawn.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use turtle_lib::*;
    /// #
    /// #[turtle_main("Backward Example")]
    /// fn draw(turtle: &mut TurtlePlan) {
    ///     // Move backward 100 pixels
    ///     turtle.backward(100.0);
    ///
    ///     // Draw a line forward, then retrace backward
    ///     turtle.forward(100.0).backward(50.0);
    /// }
    /// ```
    fn backward<T>(&mut self, distance: T) -> &mut Self
    where
        T: Into<Precision>,
    {
        let dist: Precision = distance.into();
        self.get_commands_mut().push(TurtleCommand::Move(-dist));
        self
    }
}

/// Trait for turning operations
pub trait Turnable: WithCommands {
    /// Turns the turtle left (counter-clockwise) by the specified angle in degrees.
    ///
    /// Changes the turtle's heading without moving its position.
    /// Does not draw anything.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use turtle_lib::*;
    /// #
    /// #[turtle_main("Left Turn Example")]
    /// fn draw(turtle: &mut TurtlePlan) {
    ///     // Draw a square using left turns
    ///     for _ in 0..4 {
    ///         turtle.forward(100.0).left(90.0);
    ///     }
    /// }
    /// ```
    fn left<T>(&mut self, angle: T) -> &mut Self
    where
        T: Into<Precision>,
    {
        let degrees: Precision = angle.into();
        self.get_commands_mut().push(TurtleCommand::Turn(-degrees));
        self
    }

    /// Turns the turtle right (clockwise) by the specified angle in degrees.
    ///
    /// Changes the turtle's heading without moving its position.
    /// Does not draw anything.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use turtle_lib::*;
    /// #
    /// #[turtle_main("Right Turn Example")]
    /// fn draw(turtle: &mut TurtlePlan) {
    ///     // Draw a triangle using right turns
    ///     for _ in 0..3 {
    ///         turtle.forward(100.0).right(120.0);
    ///     }
    /// }
    /// ```
    fn right<T>(&mut self, angle: T) -> &mut Self
    where
        T: Into<Precision>,
    {
        let degrees: Precision = angle.into();
        self.get_commands_mut().push(TurtleCommand::Turn(degrees));
        self
    }
}

/// Trait for curved movement (circles)
pub trait CurvedMovement: WithCommands {
    /// Draws a circular arc turning to the left (counter-clockwise).
    ///
    /// The turtle draws a circular arc with the specified radius, sweeping through
    /// the given angle. The circle center is positioned to the left of the turtle.
    ///
    /// # Parameters
    ///
    /// - `radius`: Distance from turtle to circle center (in pixels)
    /// - `angle`: Arc sweep angle in degrees (360° = full circle)
    /// - `steps`: Number of line segments to approximate the arc (more = smoother)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use turtle_lib::*;
    /// #
    /// #[turtle_main("Circle Left Example")]
    /// fn draw(turtle: &mut TurtlePlan) {
    ///     // Draw a full circle
    ///     turtle.circle_left(50.0, 360.0, 36);
    ///
    ///     // Filled circle
    ///     turtle.pen_up().go_to(vec2(100.0, 0.0)).pen_down();
    ///     turtle.set_fill_color(RED)
    ///           .begin_fill()
    ///           .circle_left(50.0, 360.0, 72)
    ///           .end_fill();
    /// }
    /// ```
    fn circle_left<R, A>(&mut self, radius: R, angle: A, steps: usize) -> &mut Self
    where
        R: Into<Precision>,
        A: Into<Precision>,
    {
        let r: Precision = radius.into();
        let a: Precision = angle.into();
        self.get_commands_mut().push(TurtleCommand::Circle {
            radius: r,
            angle: a,
            steps,
            direction: crate::circle_geometry::CircleDirection::Left,
        });
        self
    }

    /// Draws a circular arc turning to the right (clockwise).
    ///
    /// The turtle draws a circular arc with the specified radius, sweeping through
    /// the given angle. The circle center is positioned to the right of the turtle.
    ///
    /// # Parameters
    ///
    /// - `radius`: Distance from turtle to circle center (in pixels)
    /// - `angle`: Arc sweep angle in degrees (360° = full circle)
    /// - `steps`: Number of line segments to approximate the arc (more = smoother)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use turtle_lib::*;
    /// #
    /// #[turtle_main("Circle Right Example")]
    /// fn draw(turtle: &mut TurtlePlan) {
    ///     // Draw an S-curve using both directions
    ///     turtle.circle_left(50.0, 180.0, 36)
    ///           .circle_right(50.0, 180.0, 36);
    ///
    ///     // Yin-yang pattern uses circle_left and circle_right
    ///     turtle.set_fill_color(BLACK)
    ///           .begin_fill()
    ///           .circle_right(100.0, 180.0, 36)
    ///           .circle_right(50.0, 180.0, 36)
    ///           .circle_left(50.0, 180.0, 36)
    ///           .end_fill();
    /// }
    /// ```
    fn circle_right<R, A>(&mut self, radius: R, angle: A, steps: usize) -> &mut Self
    where
        R: Into<Precision>,
        A: Into<Precision>,
    {
        let r: Precision = radius.into();
        let a: Precision = angle.into();
        self.get_commands_mut().push(TurtleCommand::Circle {
            radius: r,
            angle: a,
            steps,
            direction: crate::circle_geometry::CircleDirection::Right,
        });
        self
    }
}

/// Builder for creating turtle command sequences
#[derive(Default, Debug)]
pub struct TurtlePlan {
    queue: CommandQueue,
}

impl TurtlePlan {
    /// Creates a new empty turtle command plan.
    ///
    /// This has to be used when not using the `turtle_main` macro.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use turtle_lib::*;
    /// use macroquad::prelude::*;
    ///
    /// #[macroquad::main("Manual Setup")]
    /// async fn main() {
    ///     let mut turtle = TurtlePlan::new();
    ///     turtle.forward(100.0).right(90.0).forward(100.0);
    ///     
    ///     let mut app = TurtleApp::new().with_commands(turtle.build());
    ///     
    ///     loop {
    ///         clear_background(WHITE);
    ///         app.update();
    ///         app.render();
    ///         
    ///         if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
    ///             break;
    ///         }
    ///         next_frame().await;
    ///     }
    /// }
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            queue: CommandQueue::new(),
        }
    }

    /// Sets the animation speed for turtle movements.
    ///
    /// Speed controls how fast the turtle moves during animations:
    /// - Values `>= 1000`: Instant mode - commands execute immediately without animation.
    ///                    The bigger the number, the more segments are drawn per frame.
    /// - Values `< 1000`: Animated mode - turtle moves at specified pixels per second
    ///
    /// You can dynamically switch between instant and animated modes during execution.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use turtle_lib::*;
    /// #
    /// #[turtle_main("Speed Example")]
    /// fn draw(turtle: &mut TurtlePlan) {
    ///     // Slow animation at 50 pixels/second
    ///     turtle.set_speed(50.0)
    ///           .forward(100.0);
    ///
    ///     // Switch to instant mode
    ///     turtle.set_speed(1000.0)
    ///           .forward(100.0);  // Executes immediately
    /// }
    /// ```
    pub fn set_speed(&mut self, speed: impl Into<AnimationSpeed>) -> &mut Self {
        self.queue.push(TurtleCommand::SetSpeed(speed.into()));
        self
    }

    /// Sets the pen color for drawing lines.
    ///
    /// The pen color affects all subsequent drawing operations (forward, backward, circles)
    /// until changed again. Does not affect fill color.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use turtle_lib::*;
    /// #
    /// #[turtle_main("Pen Color Example")]
    /// fn draw(turtle: &mut TurtlePlan) {
    ///     // Draw with predefined colors
    ///     turtle.set_pen_color(RED)
    ///           .forward(100.0)
    ///           .set_pen_color(BLUE)
    ///           .right(90.0)
    ///           .forward(100.0);
    /// }
    /// ```
    pub fn set_pen_color(&mut self, color: Color) -> &mut Self {
        self.queue.push(TurtleCommand::SetColor(color));
        self
    }

    /// Sets the pen width (thickness) for drawing lines.
    ///
    /// The width is measured in pixels. Default is typically 2.0.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use turtle_lib::*;
    /// #
    /// #[turtle_main("Pen Width Example")]
    /// fn draw(turtle: &mut TurtlePlan) {
    ///     // Thin line
    ///     turtle.set_pen_width(1.0)
    ///           .forward(100.0);
    ///
    ///     // Thick line
    ///     turtle.set_pen_width(10.0)
    ///           .forward(100.0);
    /// }
    /// ```
    pub fn set_pen_width(&mut self, width: Precision) -> &mut Self {
        self.queue.push(TurtleCommand::SetPenWidth(width));
        self
    }

    /// Sets the turtle's absolute heading direction in degrees.
    ///
    /// - `0°` points to the right (east)
    /// - `90°` points up (north)
    /// - `180°` points left (west)
    /// - `270°` points down (south)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use turtle_lib::*;
    /// #
    /// #[turtle_main("Heading Example")]
    /// fn draw(turtle: &mut TurtlePlan) {
    ///     // Point upward
    ///     turtle.set_heading(90.0)
    ///           .forward(100.0);
    ///
    ///     // Point left
    ///     turtle.set_heading(180.0)
    ///           .forward(100.0);
    /// }
    /// ```
    pub fn set_heading(&mut self, heading: Precision) -> &mut Self {
        self.queue.push(TurtleCommand::SetHeading(heading));
        self
    }

    /// Lifts the pen up so the turtle can move without drawing.
    ///
    /// When filling shapes, `pen_up()` also closes the current contour,
    /// allowing you to create multi-contour fills (e.g., shapes with holes).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use turtle_lib::*;
    /// #
    /// #[turtle_main("Pen Up/Down Example")]
    /// fn draw(turtle: &mut TurtlePlan) {
    ///     // Move without drawing
    ///     turtle.pen_up()
    ///           .forward(100.0)  // No line drawn
    ///           .pen_down()
    ///           .forward(100.0); // Line drawn
    ///
    ///     // Create a donut shape (outer circle with inner hole)
    ///     turtle.set_fill_color(BLUE)
    ///           .begin_fill()
    ///           .circle_left(100.0, 360.0, 72)  // Outer circle
    ///           .pen_up()  // Close first contour
    ///           .go_to(vec2(0.0, -30.0))
    ///           .pen_down()  // Start second contour
    ///           .circle_left(30.0, 360.0, 36)   // Inner circle (becomes hole)
    ///           .end_fill();
    /// }
    /// ```
    pub fn pen_up(&mut self) -> &mut Self {
        self.queue.push(TurtleCommand::PenUp);
        self
    }

    /// Lowers the pen so the turtle draws when moving.
    ///
    /// This is the default state. When filling shapes, `pen_down()` starts
    /// a new contour after `pen_up()` was called.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use turtle_lib::*;
    /// #
    /// #[turtle_main("Pen Down Example")]
    /// fn draw(turtle: &mut TurtlePlan) {
    ///     turtle.pen_up()
    ///           .forward(50.0)    // Move without drawing
    ///           .pen_down()       // Start drawing
    ///           .forward(100.0);  // Line appears
    /// }
    /// ```
    pub fn pen_down(&mut self) -> &mut Self {
        self.queue.push(TurtleCommand::PenDown);
        self
    }

    /// Hides the turtle cursor from view.
    ///
    /// The turtle will still execute commands and draw, but the cursor
    /// (typically an arrow or triangle) won't be visible.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use turtle_lib::*;
    /// #
    /// #[turtle_main("Hide Turtle Example")]
    /// fn draw(turtle: &mut TurtlePlan) {
    ///     turtle.hide()  // Turtle cursor invisible
    ///           .forward(100.0)
    ///           .right(90.0)
    ///           .forward(100.0);
    /// }
    /// ```
    pub fn hide(&mut self) -> &mut Self {
        self.queue.push(TurtleCommand::HideTurtle);
        self
    }

    /// Shows the turtle cursor.
    ///
    /// Makes the turtle cursor visible if it was previously hidden.
    /// This is the default state.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use turtle_lib::*;
    /// #
    /// #[turtle_main("Show Turtle Example")]
    /// fn draw(turtle: &mut TurtlePlan) {
    ///     turtle.hide()
    ///           .forward(100.0)
    ///           .show()  // Turtle becomes visible again
    ///           .forward(100.0);
    /// }
    /// ```
    pub fn show(&mut self) -> &mut Self {
        self.queue.push(TurtleCommand::ShowTurtle);
        self
    }

    /// Sets the turtle's shape using a `TurtleShape` object.
    ///
    /// For most use cases, prefer using `shape()` which accepts a `ShapeType` enum.
    ///
    /// # Examples
    ///
    /// ```
    /// # use turtle_lib::*;
    /// #
    /// #[turtle_main("Shape Example")]
    /// fn draw(turtle: &mut TurtlePlan) {
    /// let custom_shape = ShapeType::Arrow.to_shape();
    /// turtle.set_shape(custom_shape);
    /// }
    /// ```
    pub fn set_shape(&mut self, shape: TurtleShape) -> &mut Self {
        self.queue.push(TurtleCommand::SetShape(shape));
        self
    }

    /// Sets the turtle's visual appearance.
    ///
    /// Available shapes: `Arrow`, `Triangle`, `Square`, `Circle`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use turtle_lib::*;
    /// #
    /// #[turtle_main("Shape Example")]
    /// fn draw(turtle: &mut TurtlePlan) {
    ///     // Use different shapes
    ///     turtle.shape(ShapeType::Arrow)
    ///           .forward(50.0)
    ///           .shape(ShapeType::Circle)
    ///           .forward(50.0);
    /// }
    /// ```
    pub fn shape(&mut self, shape_type: ShapeType) -> &mut Self {
        self.set_shape(shape_type.to_shape())
    }

    /// Starts recording a shape to be filled.
    ///
    /// All turtle movements between `begin_fill()` and `end_fill()` define
    /// the shape's outline. The shape is filled using the fill color when
    /// `end_fill()` is called.
    ///
    /// Multiple contours can be created using `pen_up()` and `pen_down()`.
    /// The `EvenOdd` fill rule automatically creates holes for inner contours.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use turtle_lib::*;
    /// #
    /// #[turtle_main("Fill Example")]
    /// fn draw(turtle: &mut TurtlePlan) {
    ///     // Fill a square
    ///     turtle.set_fill_color(BLUE)
    ///           .begin_fill();
    ///     for _ in 0..4 {
    ///         turtle.forward(100.0).right(90.0);
    ///     }
    ///     turtle.end_fill();
    ///
    ///     // Fill a circle
    ///     turtle.pen_up().go_to(vec2(150.0, 0.0)).pen_down();
    ///     turtle.set_fill_color(RED)
    ///           .begin_fill()
    ///           .circle_left(50.0, 360.0, 36)
    ///           .end_fill();
    /// }
    /// ```
    pub fn begin_fill(&mut self) -> &mut Self {
        self.queue.push(TurtleCommand::BeginFill);
        self
    }

    /// Completes the fill operation started with `begin_fill()`.
    ///
    /// Closes the current shape and fills it with the fill color.
    /// All contours recorded since `begin_fill()` are filled together.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use turtle_lib::*;
    /// #
    /// #[turtle_main("End Fill Example")]
    /// fn draw(turtle: &mut TurtlePlan) {
    ///     // Triangle with fill
    ///     turtle.set_fill_color(GREEN)
    ///           .begin_fill();
    ///     for _ in 0..3 {
    ///         turtle.forward(100.0).right(120.0);
    ///     }
    ///     turtle.end_fill();
    /// }
    /// ```
    pub fn end_fill(&mut self) -> &mut Self {
        self.queue.push(TurtleCommand::EndFill);
        self
    }

    /// Sets the color used to fill shapes.
    ///
    /// This affects all shapes filled with `begin_fill()`/`end_fill()`.
    /// Independent from the pen color used for outlines.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use turtle_lib::*;
    /// #
    /// #[turtle_main("Fill Color Example")]
    /// fn draw(turtle: &mut TurtlePlan) {
    ///     // Yellow fill with blue outline
    ///     turtle.set_fill_color(YELLOW)
    ///           .set_pen_color(BLUE)
    ///           .begin_fill()
    ///           .circle_left(50.0, 360.0, 36)
    ///           .end_fill();
    /// }
    /// ```
    pub fn set_fill_color(&mut self, color: impl Into<Color>) -> &mut Self {
        self.queue
            .push(TurtleCommand::SetFillColor(Some(color.into())));
        self
    }

    /// Moves the turtle to an absolute position.
    ///
    /// The turtle moves in a straight line to the specified coordinates.
    /// If the pen is down, a line is drawn. The turtle's heading is not changed.
    ///
    /// Coordinates are in screen space:
    /// - `(0, 0)` is at the center
    /// - Positive x goes right
    /// - Positive y goes down
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use turtle_lib::*;
    /// #
    /// #[turtle_main("Goto Example")]
    /// fn draw(turtle: &mut TurtlePlan) {
    ///     // Draw a triangle by connecting points
    ///     turtle.go_to(vec2(0.0, 0.0));
    ///     turtle.go_to(vec2(100.0, 0.0));
    ///     turtle.go_to(vec2(50.0, 86.6));
    ///     turtle.go_to(vec2(0.0, 0.0));
    /// }
    /// ```
    pub fn go_to(&mut self, coord: impl Into<Coordinate>) -> &mut Self {
        self.queue.push(TurtleCommand::Goto(coord.into()));
        self
    }

    /// Consumes the `TurtlePlan` and returns the command queue.
    ///
    /// Use this to finalize the turtle commands and pass them to `TurtleApp`.
    /// This method consumes `self`, so the plan cannot be used afterward.
    ///
    /// # Examples
    ///
    /// ```
    /// # use turtle_lib::*;
    /// #
    /// let mut turtle = TurtlePlan::new();
    /// turtle.forward(100.0).right(90.0).forward(100.0);
    ///
    /// // Build and get the command queue
    /// let commands = turtle.build();
    /// # assert!(!commands.is_empty());
    /// ```
    #[must_use]
    pub fn build(self) -> CommandQueue {
        self.queue
    }
}

impl WithCommands for TurtlePlan {
    fn get_commands_mut(&mut self) -> &mut CommandQueue {
        &mut self.queue
    }

    fn get_commands(self) -> CommandQueue {
        self.queue
    }
}

impl DirectionalMovement for TurtlePlan {}
impl Turnable for TurtlePlan {}
impl CurvedMovement for TurtlePlan {}
