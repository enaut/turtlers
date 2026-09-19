//! Export backend trait and core export types.
#[cfg(feature = "svg")]
use crate::state::TurtleWorld;
use crate::TurtlePlan;

#[derive(Debug)]
pub enum ExportError {
    Io(std::io::Error),
    Format(String),
    // Additional formats can be added here.
}

#[derive(Clone, Copy, Debug)]
pub enum DrawingFormat {
    #[cfg(feature = "svg")]
    Svg,
    // Additional formats: Png, Pdf, …
}

#[cfg(feature = "svg")]
pub(crate) trait DrawingExporter {
    /// Export the drawing to the specified format and filename
    ///
    /// # Errors
    ///
    /// Returns an error if the export fails (e.g., file I/O error)
    fn export(&self, world: &TurtleWorld, filename: &str) -> Result<(), ExportError>;
}

/// Check command-line arguments for the `--export-svg <filename>` flag.
#[must_use]
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

/// Headless SVG export that executes drawing commands and writes an SVG file
/// without opening a graphics window and without calling `std::process::exit`.
///
/// # Errors
///
/// Returns `ExportError` if file I/O fails or if the `svg` feature is not enabled.
pub fn run_headless_svg_export<F>(mut build_commands: F, filename: &str) -> Result<(), ExportError>
where
    F: FnMut(&mut TurtlePlan),
{
    #[cfg(feature = "svg")]
    {
        let mut turtle = crate::create_turtle_plan();
        build_commands(&mut turtle);

        let mut app = crate::TurtleApp::new();
        app.execute_immediate(0, turtle);

        app.export_drawing(filename, crate::export::DrawingFormat::Svg)
    }

    #[cfg(not(feature = "svg"))]
    {
        let _ = &mut build_commands;
        let _ = filename;
        Err(ExportError::Format(
            "SVG export feature is not enabled. Please rebuild with --features svg".to_string(),
        ))
    }
}

/// Handle the optional `--export-svg` CLI flag.
///
/// Delegates to [`run_headless_svg_export`].
pub fn handle_svg_export<F>(build_commands: F)
where
    F: FnMut(&mut TurtlePlan),
{
    if let Some(filename) = parse_svg_export_arg() {
        match run_headless_svg_export(build_commands, &filename) {
            Ok(()) => {
                println!("SVG exported successfully to: {filename}");
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Error exporting SVG: {e:?}");
                std::process::exit(1);
            }
        }
    }
}
