//! Procedural macros for turtle-lib
//!
//! This crate provides the `turtle_main` procedural macro that simplifies
//! creating turtle graphics programs by automatically setting up the
//! macroquad window, turtle initialization, and the main rendering loop.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// A convenience macro that wraps your turtle drawing code with the necessary
/// boilerplate for running a turtle graphics program.
///
/// This macro:
/// - Wraps your code with `#[macroquad::main]`
/// - Creates a turtle instance (`turtle`)
/// - Sets up the `TurtleApp` with your drawing commands
/// - Provides a main loop with rendering and quit handling (ESC or Q)
///
/// # Example
///
/// ```ignore
/// use turtle_lib::*;
///
/// #[turtle_main("My Turtle Drawing")]
/// fn my_drawing(turtle: &mut TurtlePlan) {
///     // Use colors from turtle_lib (re-exported from macroquad)
///     turtle.set_pen_color(RED);
///     turtle.forward(100.0);
///     turtle.right(90.0);
///     turtle.forward(100.0);
/// }
/// ```
///
/// If you need macroquad types not re-exported by `turtle_lib`:
///
/// ```ignore
/// use macroquad::prelude::SKYBLUE;  // Import specific items
/// use turtle_lib::*;
///
/// #[turtle_main("My Drawing")]
/// fn my_drawing(turtle: &mut TurtlePlan) {
///     turtle.set_pen_color(SKYBLUE);
///     turtle.forward(100.0);
/// }
/// ```
///
/// This expands to approximately:
///
/// ```ignore
/// use macroquad::prelude::*;
/// use turtle_lib::*;
///
/// #[macroquad::main("My Turtle Drawing")]
/// async fn main() {
///     let mut turtle = create_turtle_plan();
///     
///     // Your drawing code here
///     turtle.set_pen_color(RED);
///     turtle.forward(100.0);
///     turtle.right(90.0);
///     turtle.forward(100.0);
///
///     let mut app = TurtleApp::new().with_commands(turtle.build());
///
///     loop {
///         clear_background(WHITE);
///         app.update();
///         app.render();
///         draw_text("Press ESC or Q to quit", 10.0, 40.0, 16.0, DARKGRAY);
///         
///         if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
///             break;
///         }
///         
///         next_frame().await;
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn turtle_main(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    // Parse the window title from args (default to "Turtle Graphics")
    let window_title = if args.is_empty() {
        quote! { "Turtle Graphics" }
    } else {
        let args_str = args.to_string();
        // Remove quotes if present
        let title = args_str.trim().trim_matches('"');
        quote! { #title }
    };

    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;

    // Check if the function has the expected signature
    let has_turtle_param = input_fn.sig.inputs.len() == 1;

    let expanded = if has_turtle_param {
        // Function takes a turtle parameter
        quote! {
            #[macroquad::main(#window_title)]
            async fn main() {
                let mut turtle = turtle_lib::create_turtle_plan();

                // Call the user's function with the turtle
                #fn_name(&mut turtle);

                let mut app = turtle_lib::TurtleApp::new()
                    .with_commands(turtle.build());

                loop {
                    macroquad::prelude::clear_background(macroquad::prelude::WHITE);
                    app.update();
                    app.render();
                    macroquad::prelude::draw_text(
                        "Press ESC or Q to quit",
                        10.0,
                        40.0,
                        16.0,
                        macroquad::prelude::DARKGRAY
                    );

                    if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Escape)
                        || macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Q)
                    {
                        break;
                    }

                    macroquad::prelude::next_frame().await;
                }
            }

            fn #fn_name(turtle: &mut turtle_lib::TurtlePlan) #fn_block
        }
    } else {
        // Function takes no parameters - inline the code
        quote! {
            #[macroquad::main(#window_title)]
            async fn main() {
                let mut turtle = turtle_lib::create_turtle_plan();

                // Inline the user's code
                #fn_block

                let mut app = turtle_lib::TurtleApp::new()
                    .with_commands(turtle.build());

                loop {
                    macroquad::prelude::clear_background(macroquad::prelude::WHITE);
                    app.update();
                    app.render();
                    macroquad::prelude::draw_text(
                        "Press ESC or Q to quit",
                        10.0,
                        40.0,
                        16.0,
                        macroquad::prelude::DARKGRAY
                    );

                    if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Escape)
                        || macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Q)
                    {
                        break;
                    }

                    macroquad::prelude::next_frame().await;
                }
            }
        }
    };

    TokenStream::from(expanded)
}
