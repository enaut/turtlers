//! Export-Backend-Trait und zentrale Export-Typen

use crate::state::TurtleWorld;
use crate::TurtlePlan;

#[derive(Debug)]
pub enum ExportError {
    Io(std::io::Error),
    Format(String),
    // Weitere Formate können ergänzt werden
}

#[derive(Clone, Copy, Debug)]
pub enum DrawingFormat {
    #[cfg(feature = "svg")]
    Svg,
    // Weitere Formate wie Png, Pdf, ...
}

pub trait DrawingExporter {
    /// Export the drawing to the specified format and filename
    ///
    /// # Errors
    ///
    /// Returns an error if the export fails (e.g., file I/O error)
    fn export(&self, world: &TurtleWorld, filename: &str) -> Result<(), ExportError>;
}

pub fn parse_svg_export_arg() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--export-svg" && i + 1 < args.len() {
            return Some(args[i + 1].clone());
        }
        i += 1;
    }
    None
}

/// Handle the optional `--export-svg` CLI flag.
///
/// The feature gating lives inside `turtle-lib`, so the `turtle_main` macro
/// no longer needs to reference cfg flags from the consuming crate.
pub fn handle_svg_export<F>(build_commands: F)
where
    F: FnMut(&mut TurtlePlan),
{
    // Avoid unused warnings when the feature is disabled
    let _ = &build_commands;

    if let Some(filename) = parse_svg_export_arg() {
        #[cfg(feature = "svg")]
        {
            let mut build_commands = build_commands;
            let mut turtle = crate::create_turtle_plan();
            build_commands(&mut turtle);

            let mut app = crate::TurtleApp::new().with_commands(turtle.build());
            app.set_all_turtles_speed(crate::AnimationSpeed::Instant(1000));

            while !app.all_animations_complete() {
                app.update();
            }

            match app.export_drawing(&filename, crate::export::DrawingFormat::Svg) {
                Ok(_) => {
                    println!("SVG exported successfully to: {}", filename);
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("Error exporting SVG: {:?}", e);
                    std::process::exit(1);
                }
            }
        }

        #[cfg(not(feature = "svg"))]
        {
            let _ = &filename;
            eprintln!("Error: SVG export feature is not enabled.");
            eprintln!("Please rebuild with --features svg");
            std::process::exit(1);
        }
    }
}
