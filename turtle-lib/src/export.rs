//! Export backend trait and core export types.
#[cfg(feature = "svg")]
use crate::state::TurtleWorld;
use crate::TurtlePlan;

#[derive(Debug)]
pub enum ExportError {
    Io(std::io::Error),
    Format(String),
    Execution(String),
    MissingFeature(String),
    // Additional formats can be added here.
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O error: {err}"),
            Self::Format(msg) => write!(f, "format error: {msg}"),
            Self::Execution(msg) => write!(f, "execution error: {msg}"),
            Self::MissingFeature(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for ExportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Format(_) | Self::Execution(_) | Self::MissingFeature(_) => None,
        }
    }
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

/// Parse the `--export-svg` parameter from a `pico_args::Arguments` instance.
///
/// # Errors
///
/// Returns `pico_args::Error` if `--export-svg` is provided without a valid filename.
pub fn parse_svg_export_from_args(
    args: &mut pico_args::Arguments,
) -> Result<Option<String>, pico_args::Error> {
    match args.opt_value_from_str::<_, String>("--export-svg") {
        Ok(Some(s)) if s.trim().is_empty() || s.starts_with('-') => {
            Err(pico_args::Error::OptionWithoutAValue("--export-svg"))
        }
        other => other,
    }
}

/// Check command-line arguments for the `--export-svg <filename>` flag.
#[must_use]
pub fn parse_svg_export_arg() -> Option<String> {
    let mut args = pico_args::Arguments::from_env();
    parse_svg_export_from_args(&mut args).ok().flatten()
}

/// Headless SVG export that executes drawing commands and writes an SVG file
/// without opening a graphics window and without calling `std::process::exit`.
///
/// # Errors
///
/// Returns `ExportError` if file I/O fails, drawing execution panics, or if the `svg` feature is not enabled.
pub fn run_headless_svg_export<F>(mut build_commands: F, filename: &str) -> Result<(), ExportError>
where
    F: FnMut(&mut TurtlePlan),
{
    #[cfg(feature = "svg")]
    {
        use std::panic::AssertUnwindSafe;

        let mut turtle = crate::create_turtle_plan();

        let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
            build_commands(&mut turtle);
        }));

        match result {
            Ok(()) => {
                let mut app = crate::TurtleApp::new();
                app.execute_immediate(0, turtle);

                app.export_drawing(filename, crate::export::DrawingFormat::Svg)
            }
            Err(payload) => {
                let raw_msg = if let Some(s) = payload.downcast_ref::<&str>() {
                    (*s).to_string()
                } else if let Some(s) = payload.downcast_ref::<String>() {
                    s.clone()
                } else {
                    String::new()
                };

                let is_context_panic = raw_msg.contains("THREAD_ID.is_some()")
                    || raw_msg.contains("macroquad");

                let err_msg = if is_context_panic {
                    "Drawing function called Macroquad window/GUI functions (e.g. `screen_width()`, `screen_height()`) which are unavailable in headless SVG export mode.".to_string()
                } else if !raw_msg.is_empty() {
                    raw_msg
                } else {
                    "Drawing function panicked during execution".to_string()
                };
                Err(ExportError::Execution(err_msg))
            }
        }
    }

    #[cfg(not(feature = "svg"))]
    {
        let _ = &mut build_commands;
        let _ = filename;
        Err(ExportError::MissingFeature(
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
    let mut args = pico_args::Arguments::from_env();
    let export_arg = parse_svg_export_from_args(&mut args);

    #[cfg(not(feature = "svg"))]
    {
        let _ = build_commands;
        if export_arg.is_err() || matches!(export_arg, Ok(Some(_))) {
            eprintln!("Error: SVG export feature is not enabled. Please rebuild with --features svg");
            std::process::exit(1);
        }
    }

    #[cfg(feature = "svg")]
    {
        match export_arg {
            Ok(None) => {}
            Err(_) => {
                eprintln!("Error: --export-svg: option requires an argument");
                std::process::exit(1);
            }
            Ok(Some(filename)) => {
                match run_headless_svg_export(build_commands, &filename) {
                    Ok(()) => {
                        println!("SVG exported successfully to: {filename}");
                        std::process::exit(0);
                    }
                    Err(e) => {
                        if let ExportError::Execution(ref msg) = e {
                            if msg.contains("unavailable in headless SVG export mode") {
                                eprintln!("\n================================================================================");
                                eprintln!("Headless SVG Export Note:");
                                eprintln!("A Macroquad window/rendering function (e.g. `screen_width()`, `screen_height()`,");
                                eprintln!("or input check) was called while running in headless export mode.");
                                eprintln!("Headless export does not initialize a graphics window. To resolve this:");
                                eprintln!("  - Use relative turtle commands or fixed coordinates instead of window queries, or");
                                eprintln!("  - Run the program in windowed GUI mode without the `--export-svg` flag.");
                                eprintln!("================================================================================\n");
                            }
                        }
                        eprintln!("Error: {e}");
                        std::process::exit(1);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;

    fn make_args(args: &[&str]) -> pico_args::Arguments {
        let os_args: Vec<OsString> = args.iter().map(OsString::from).collect();
        pico_args::Arguments::from_vec(os_args)
    }

    #[test]
    fn test_parse_svg_export_with_filename() {
        let mut args = make_args(&["--export-svg", "output.svg"]);
        let res = parse_svg_export_from_args(&mut args);
        assert_eq!(res.unwrap(), Some("output.svg".to_string()));
    }

    #[test]
    fn test_parse_svg_export_with_equals() {
        let mut args = make_args(&["--export-svg=output.svg"]);
        let res = parse_svg_export_from_args(&mut args);
        assert_eq!(res.unwrap(), Some("output.svg".to_string()));
    }

    #[test]
    fn test_parse_svg_export_missing_argument_at_end() {
        let mut args = make_args(&["--export-svg"]);
        let res = parse_svg_export_from_args(&mut args);
        assert!(matches!(res, Err(pico_args::Error::OptionWithoutAValue(_))));
    }

    #[test]
    fn test_parse_svg_export_missing_argument_followed_by_flag() {
        let mut args = make_args(&["--export-svg", "--verbose"]);
        let res = parse_svg_export_from_args(&mut args);
        assert!(matches!(res, Err(pico_args::Error::OptionWithoutAValue(_))));
    }

    #[test]
    fn test_parse_svg_export_empty_string() {
        let mut args = make_args(&["--export-svg", ""]);
        let res = parse_svg_export_from_args(&mut args);
        assert!(matches!(res, Err(pico_args::Error::OptionWithoutAValue(_))));
    }

    #[test]
    fn test_parse_svg_export_empty_equals() {
        let mut args = make_args(&["--export-svg="]);
        let res = parse_svg_export_from_args(&mut args);
        assert!(matches!(res, Err(pico_args::Error::OptionWithoutAValue(_))));
    }

    #[test]
    fn test_parse_svg_export_not_present() {
        let mut args = make_args(&["--verbose", "input.txt"]);
        let res = parse_svg_export_from_args(&mut args);
        assert_eq!(res.unwrap(), None);
    }

    #[test]
    fn test_parse_svg_export_preserves_unknown_args() {
        let mut args = make_args(&["--other-flag", "--export-svg", "out.svg", "positional"]);
        let res = parse_svg_export_from_args(&mut args);
        assert_eq!(res.unwrap(), Some("out.svg".to_string()));
        assert!(args.contains("--other-flag"));
    }

    #[test]
    fn test_export_error_display_missing_feature() {
        let err = ExportError::MissingFeature(
            "SVG export feature is not enabled. Please rebuild with --features svg".to_string(),
        );
        assert_eq!(
            err.to_string(),
            "SVG export feature is not enabled. Please rebuild with --features svg"
        );
    }

    #[cfg(feature = "svg")]
    #[test]
    fn test_headless_svg_export_success() {
        use crate::Movement;

        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join(format!("turtle_test_export_{}.svg", std::process::id()));
        let path_str = path.to_str().expect("valid utf-8 path");

        let res = run_headless_svg_export(|turtle| {
            turtle.forward(10.0);
        }, path_str);

        assert!(res.is_ok());
        assert!(path.exists());
        let content = std::fs::read_to_string(&path).expect("read temp svg");
        assert!(content.contains("<svg"));
        let _ = std::fs::remove_file(&path);
    }

    #[cfg(feature = "svg")]
    #[test]
    fn test_headless_svg_export_macroquad_panic() {
        let res = run_headless_svg_export(|_turtle| {
            panic!("assertion failed: THREAD_ID.is_some()");
        }, "unused.svg");

        match res {
            Err(ExportError::Execution(msg)) => {
                assert!(msg.contains("unavailable in headless SVG export mode"));
            }
            other => panic!("expected ExportError::Execution, got {other:?}"),
        }
    }

    #[cfg(feature = "svg")]
    #[test]
    fn test_headless_svg_export_custom_panic() {
        let res = run_headless_svg_export(|_turtle| {
            panic!("custom user panic occurred");
        }, "unused.svg");

        match res {
            Err(ExportError::Execution(msg)) => {
                assert_eq!(msg, "custom user panic occurred");
            }
            other => panic!("expected ExportError::Execution, got {other:?}"),
        }
    }
}
