//! Procedural macros for turtle-lib
//!
//! This crate provides the `turtle_main` procedural macro that simplifies
//! creating turtle graphics programs by automatically setting up the
//! macroquad window, turtle initialization, and the main rendering loop.

use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

/// A convenience macro that wraps your turtle drawing code with the necessary
/// boilerplate for running a turtle graphics program.
///
/// This macro:
/// - Wraps your code with `#[macroquad::main]`
/// - Creates a turtle instance (`turtle`)
/// - Sets up the `TurtleApp` with your drawing commands
/// - Provides a main loop with rendering and quit handling (ESC or Q)
/// - Adds command-line parameter support for SVG export (when `svg` feature is enabled)
///
/// # Command-Line Parameters
///
/// When the `svg` feature is enabled, the following command-line parameter is available:
///
/// * `--export-svg <filename>` - Exports the drawing to an SVG file and exits immediately
///   without opening the window. Example: `cargo run --features svg -- --export-svg output.svg`
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
/// # SVG Export Example
///
/// ```bash
/// # Run with SVG export (requires svg feature)
/// cargo run --package turtle-lib --example macro_demo --features svg -- --export-svg output.svg
/// ```
///
/// This expands to approximately:
///
/// ```ignore
/// use turtle_lib::*;
///
/// fn main() {
///     // Handle optional SVG export headlessly without opening a window
///     if let Some(filename) = turtle_lib::export::parse_svg_export_arg() {
///         let mut build_commands = |turtle: &mut turtle_lib::TurtlePlan| {
///             my_drawing(turtle);
///         };
///         if let Err(e) = turtle_lib::export::run_headless_svg_export(&mut build_commands, &filename) {
///             eprintln!("Error exporting SVG: {:?}", e);
///             std::process::exit(1);
///         }
///         return;
///     }
///
///     // Normal interactive GUI mode with window
///     turtle_lib::macroquad::Window::new("My Turtle Drawing", async {
///         let mut turtle = create_turtle_plan();
///         my_drawing(&mut turtle);
///
///         let mut app = TurtleApp::new().with_commands(turtle.build());
///
///         loop {
///             turtle_lib::macroquad::prelude::clear_background(turtle_lib::macroquad::prelude::WHITE);
///             app.update();
///             app.render();
///             turtle_lib::macroquad::prelude::draw_text("Press ESC or Q to quit", 10.0, 40.0, 16.0, turtle_lib::macroquad::prelude::DARKGRAY);
///             
///             if turtle_lib::macroquad::prelude::is_key_pressed(turtle_lib::macroquad::prelude::KeyCode::Escape)
///                 || turtle_lib::macroquad::prelude::is_key_pressed(turtle_lib::macroquad::prelude::KeyCode::Q)
///             {
///                 break;
///             }
///             
///             turtle_lib::macroquad::prelude::next_frame().await;
///         }
///     });
/// }
/// ```
fn validate_parameter_type(ty: &syn::Type) -> Result<(), syn::Error> {
    match ty {
        syn::Type::Reference(type_ref) => {
            if type_ref.mutability.is_none() {
                return Err(syn::Error::new_spanned(
                    type_ref,
                    "#[turtle_main] parameter must be a mutable reference: `&mut TurtlePlan`",
                ));
            }

            if let syn::Type::Path(type_path) = &*type_ref.elem {
                let is_turtle_plan = type_path
                    .path
                    .segments
                    .last()
                    .is_some_and(|seg| seg.ident == "TurtlePlan");

                if is_turtle_plan {
                    return Ok(());
                }
            }

            Err(syn::Error::new_spanned(
                &type_ref.elem,
                "#[turtle_main] expected reference to `TurtlePlan`, e.g. `&mut TurtlePlan`",
            ))
        }
        _ => Err(syn::Error::new_spanned(
            ty,
            "#[turtle_main] parameter must be of type `&mut TurtlePlan`",
        )),
    }
}

fn validate_input(input_fn: &ItemFn) -> Result<(), syn::Error> {
    if input_fn.sig.asyncness.is_some() {
        return Err(syn::Error::new_spanned(
            input_fn.sig.fn_token,
            "#[turtle_main] functions cannot be async",
        ));
    }

    if input_fn.sig.inputs.len() > 1 {
        return Err(syn::Error::new_spanned(
            &input_fn.sig.inputs,
            "#[turtle_main] functions must take either 0 arguments or a single `&mut TurtlePlan`",
        ));
    }

    if let Some(arg) = input_fn.sig.inputs.first() {
        match arg {
            syn::FnArg::Receiver(receiver) => {
                return Err(syn::Error::new_spanned(
                    receiver,
                    "#[turtle_main] functions cannot take a `self` parameter",
                ));
            }
            syn::FnArg::Typed(pat_type) => {
                match &*pat_type.pat {
                    syn::Pat::Ident(pat_ident)
                        if pat_ident.by_ref.is_none() && pat_ident.subpat.is_none() => {}
                    syn::Pat::Wild(_) => {}
                    _ => {
                        return Err(syn::Error::new_spanned(
                            &pat_type.pat,
                            "#[turtle_main] unsupported parameter pattern; expected an identifier like `turtle` or `t`",
                        ));
                    }
                }

                validate_parameter_type(&pat_type.ty)?;
            }
        }
    }

    if matches!(input_fn.sig.output, syn::ReturnType::Type(..)) {
        return Err(syn::Error::new_spanned(
            &input_fn.sig.output,
            "#[turtle_main] functions cannot have a return type",
        ));
    }

    Ok(())
}

#[proc_macro_attribute]
pub fn turtle_main(args: TokenStream, input: TokenStream) -> TokenStream {
    turtle_main_impl(&args.into(), input.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn turtle_main_impl(
    args: &proc_macro2::TokenStream,
    input: proc_macro2::TokenStream,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let input_fn: ItemFn = syn::parse2(input)?;

    // Validate function signature
    validate_input(&input_fn)?;

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
    let fn_attrs = &input_fn.attrs;
    let fn_vis = if fn_name == "main" {
        None
    } else {
        Some(&input_fn.vis)
    };
    let has_turtle_param = input_fn.sig.inputs.len() == 1;

    let helper_name = if fn_name == "main" {
        quote::format_ident!("__turtle_main_draw")
    } else {
        fn_name.clone()
    };

    let helper_fn = if has_turtle_param {
        let param = &input_fn.sig.inputs[0];
        quote! {
            #(#fn_attrs)*
            #fn_vis fn #helper_name(#param) #fn_block
        }
    } else {
        quote! {
            #(#fn_attrs)*
            #fn_vis fn #helper_name(turtle: &mut turtle_lib::TurtlePlan) {
                let turtle = turtle;
                #fn_block
            }
        }
    };

    let expanded = quote! {
        fn main() {
            let mut build_commands = |turtle: &mut turtle_lib::TurtlePlan| {
                #helper_name(turtle);
            };

            // If --export-svg flag is present, export headlessly without opening a window
            if let Some(filename) = turtle_lib::export::parse_svg_export_arg() {
                match turtle_lib::export::run_headless_svg_export(&mut build_commands, &filename) {
                    Ok(()) => {
                        println!("SVG exported successfully to: {}", filename);
                        return;
                    }
                    Err(e) => {
                        eprintln!("Error exporting SVG: {}", e);
                        std::process::exit(1);
                    }
                }
            }

            // Normal rendering mode (interactive window)
            turtle_lib::macroquad::Window::new(#window_title, async {
                let mut turtle = turtle_lib::create_turtle_plan();
                #helper_name(&mut turtle);

                let mut app = turtle_lib::TurtleApp::new()
                    .with_commands(turtle.build());

                loop {
                    turtle_lib::macroquad::prelude::clear_background(turtle_lib::macroquad::prelude::WHITE);
                    app.update();
                    app.render();
                    turtle_lib::macroquad::prelude::draw_text(
                        "Press ESC or Q to quit",
                        10.0,
                        40.0,
                        16.0,
                        turtle_lib::macroquad::prelude::DARKGRAY
                    );

                    if turtle_lib::macroquad::prelude::is_key_pressed(turtle_lib::macroquad::prelude::KeyCode::Escape)
                        || turtle_lib::macroquad::prelude::is_key_pressed(turtle_lib::macroquad::prelude::KeyCode::Q)
                    {
                        break;
                    }

                    turtle_lib::macroquad::prelude::next_frame().await;
                }
            });
        }

        #helper_fn
    };

    Ok(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_valid_zero_args() {
        let input: ItemFn = parse_quote! {
            fn my_draw() {
                turtle.forward(100.0);
            }
        };
        assert!(validate_input(&input).is_ok());
    }

    #[test]
    fn test_valid_one_arg() {
        let input: ItemFn = parse_quote! {
            fn my_draw(t: &mut TurtlePlan) {
                t.forward(100.0);
            }
        };
        assert!(validate_input(&input).is_ok());
    }

    #[test]
    fn test_valid_mut_arg() {
        let input: ItemFn = parse_quote! {
            fn my_draw(mut t: &mut TurtlePlan) {
                t.forward(100.0);
            }
        };
        assert!(validate_input(&input).is_ok());
    }

    #[test]
    fn test_valid_wildcard_arg() {
        let input: ItemFn = parse_quote! {
            fn my_draw(_: &mut TurtlePlan) {}
        };
        assert!(validate_input(&input).is_ok());
    }

    #[test]
    fn test_rejects_async() {
        let input: ItemFn = parse_quote! {
            async fn my_draw() {}
        };
        let err = validate_input(&input).unwrap_err();
        assert!(err.to_string().contains("cannot be async"));
    }

    #[test]
    fn test_rejects_multiple_args() {
        let input: ItemFn = parse_quote! {
            fn my_draw(t: &mut TurtlePlan, extra: i32) {}
        };
        let err = validate_input(&input).unwrap_err();
        assert!(err.to_string().contains("must take either 0 arguments or a single"));
    }

    #[test]
    fn test_rejects_self() {
        let input: ItemFn = parse_quote! {
            fn my_draw(&mut self) {}
        };
        let err = validate_input(&input).unwrap_err();
        assert!(err.to_string().contains("cannot take a `self` parameter"));
    }

    #[test]
    fn test_rejects_unsupported_pattern() {
        let input: ItemFn = parse_quote! {
            fn my_draw((a, b): &mut TurtlePlan) {}
        };
        let err = validate_input(&input).unwrap_err();
        assert!(err.to_string().contains("unsupported parameter pattern"));
    }

    #[test]
    fn test_rejects_return_type() {
        let input: ItemFn = parse_quote! {
            fn my_draw() -> i32 {
                42
            }
        };
        let err = validate_input(&input).unwrap_err();
        assert!(err.to_string().contains("cannot have a return type"));
    }

    #[test]
    fn test_valid_qualified_type() {
        let input: ItemFn = parse_quote! {
            fn my_draw(t: &mut turtle_lib::TurtlePlan) {}
        };
        assert!(validate_input(&input).is_ok());
    }

    #[test]
    fn test_rejects_wrong_type() {
        let input: ItemFn = parse_quote! {
            fn my_draw(value: i32) {}
        };
        let err = validate_input(&input).unwrap_err();
        assert!(err.to_string().contains("parameter must be of type `&mut TurtlePlan`"));
    }

    #[test]
    fn test_rejects_immutable_reference() {
        let input: ItemFn = parse_quote! {
            fn my_draw(t: &TurtlePlan) {}
        };
        let err = validate_input(&input).unwrap_err();
        assert!(err.to_string().contains("parameter must be a mutable reference: `&mut TurtlePlan`"));
    }

    #[test]
    fn test_rejects_owned_type() {
        let input: ItemFn = parse_quote! {
            fn my_draw(t: TurtlePlan) {}
        };
        let err = validate_input(&input).unwrap_err();
        assert!(err.to_string().contains("parameter must be of type `&mut TurtlePlan`"));
    }

    #[test]
    fn test_rejects_wrong_reference_type() {
        let input: ItemFn = parse_quote! {
            fn my_draw(t: &mut i32) {}
        };
        let err = validate_input(&input).unwrap_err();
        assert!(err.to_string().contains("expected reference to `TurtlePlan`"));
    }

    #[test]
    fn test_expansion_preserves_custom_param_name() {
        let input = quote! {
            fn my_draw(t: &mut TurtlePlan) {
                t.forward(100.0);
            }
        };
        let output = turtle_main_impl(&quote!(), input).unwrap();
        let file: syn::File = syn::parse2(output).unwrap();

        let helper_fn = file
            .items
            .iter()
            .find_map(|item| {
                if let syn::Item::Fn(f) = item {
                    if f.sig.ident == "my_draw" {
                        return Some(f);
                    }
                }
                None
            })
            .expect("helper fn `my_draw` should exist");

        let first_arg = helper_fn.sig.inputs.first().expect("should have 1 arg");
        if let syn::FnArg::Typed(pat_type) = first_arg {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                assert_eq!(pat_ident.ident, "t");
            } else {
                panic!("expected ident pattern");
            }
        } else {
            panic!("expected typed arg");
        }
    }

    #[test]
    fn test_expansion_preserves_mut_param() {
        let input = quote! {
            fn my_draw(mut t: &mut TurtlePlan) {
                t.forward(100.0);
            }
        };
        let output = turtle_main_impl(&quote!(), input).unwrap();
        let file: syn::File = syn::parse2(output).unwrap();

        let helper_fn = file
            .items
            .iter()
            .find_map(|item| {
                if let syn::Item::Fn(f) = item {
                    if f.sig.ident == "my_draw" {
                        return Some(f);
                    }
                }
                None
            })
            .expect("helper fn `my_draw` should exist");

        let first_arg = helper_fn.sig.inputs.first().expect("should have 1 arg");
        if let syn::FnArg::Typed(pat_type) = first_arg {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                assert_eq!(pat_ident.ident, "t");
                assert!(pat_ident.mutability.is_some());
            } else {
                panic!("expected ident pattern");
            }
        } else {
            panic!("expected typed arg");
        }
    }

    #[test]
    fn test_expansion_main_fn_renamed() {
        let input = quote! {
            fn main(t: &mut TurtlePlan) {
                t.forward(100.0);
            }
        };
        let output = turtle_main_impl(&quote!(), input).unwrap();
        let file: syn::File = syn::parse2(output).unwrap();

        let helper_fn = file
            .items
            .iter()
            .find_map(|item| {
                if let syn::Item::Fn(f) = item {
                    if f.sig.ident == "__turtle_main_draw" {
                        return Some(f);
                    }
                }
                None
            })
            .expect("helper fn `__turtle_main_draw` should exist");

        let first_arg = helper_fn.sig.inputs.first().expect("should have 1 arg");
        if let syn::FnArg::Typed(pat_type) = first_arg {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                assert_eq!(pat_ident.ident, "t");
            } else {
                panic!("expected ident pattern");
            }
        } else {
            panic!("expected typed arg");
        }
    }

    #[test]
    fn test_expansion_zero_args() {
        let input = quote! {
            fn my_draw() {
                turtle.forward(100.0);
            }
        };
        let output = turtle_main_impl(&quote!(), input).unwrap();
        let file: syn::File = syn::parse2(output).unwrap();

        let helper_fn = file
            .items
            .iter()
            .find_map(|item| {
                if let syn::Item::Fn(f) = item {
                    if f.sig.ident == "my_draw" {
                        return Some(f);
                    }
                }
                None
            })
            .expect("helper fn `my_draw` should exist");

        let first_arg = helper_fn.sig.inputs.first().expect("should have 1 arg");
        if let syn::FnArg::Typed(pat_type) = first_arg {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                assert_eq!(pat_ident.ident, "turtle");
            } else {
                panic!("expected ident pattern");
            }
        } else {
            panic!("expected typed arg");
        }
    }
}
