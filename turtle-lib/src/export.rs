//! Export backend trait and core export types.
#[cfg(feature = "svg")]
use crate::state::TurtleWorld;
use crate::TurtlePlan;

#[derive(Debug)]
pub enum ExportError {
    Io(std::io::Error),
    Format(String),
    Execution(String),
    // Additional formats can be added here.
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O error: {err}"),
            Self::Format(msg) => write!(f, "format error: {msg}"),
            Self::Execution(msg) => write!(f, "execution error: {msg}"),
        }
    }
}

impl std::error::Error for ExportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Format(_) | Self::Execution(_) => None,
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

#[cfg(feature = "svg")]
type PanicHookFn = Box<dyn Fn(&std::panic::PanicHookInfo<'_>) + Sync + Send + 'static>;

#[cfg(feature = "svg")]
struct PanicHookGuard {
    prev_hook: Option<std::sync::Arc<PanicHookFn>>,
}

#[cfg(feature = "svg")]
impl Drop for PanicHookGuard {
    fn drop(&mut self) {
        if let Some(prev) = self.prev_hook.take() {
            std::panic::set_hook(Box::new(move |info| prev(info)));
        }
    }
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
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        let mut turtle = crate::create_turtle_plan();

        let is_mq_panic = Arc::new(AtomicBool::new(false));
        let is_mq_clone = Arc::clone(&is_mq_panic);

        let prev_hook = Arc::new(std::panic::take_hook());
        let prev_hook_for_closure = Arc::clone(&prev_hook);

        let guard = PanicHookGuard {
            prev_hook: Some(prev_hook),
        };

        std::panic::set_hook(Box::new(move |info| {
            prev_hook_for_closure(info);

            let msg = if let Some(s) = info.payload().downcast_ref::<&str>() {
                *s
            } else if let Some(s) = info.payload().downcast_ref::<String>() {
                s.as_str()
            } else {
                ""
            };

            let loc_file = info.location().map_or("", std::panic::Location::file);
            let is_context_panic = msg.contains("THREAD_ID.is_some()")
                || (loc_file.contains("macroquad") && msg.contains("assertion failed"));

            if is_context_panic {
                is_mq_clone.store(true, Ordering::SeqCst);
                eprintln!("\n================================================================================");
                eprintln!("Headless SVG Export Note:");
                eprintln!("A Macroquad window/rendering function (e.g. `screen_width()`, `screen_height()`,");
                eprintln!("or input check) was called while running in headless export mode.");
                eprintln!("Headless export does not initialize a graphics window. To resolve this:");
                eprintln!("  - Use relative turtle commands or fixed coordinates instead of window queries, or");
                eprintln!("  - Run the program in windowed GUI mode without the `--export-svg` flag.");
                eprintln!("================================================================================\n");
            }
        }));

        let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
            build_commands(&mut turtle);
        }));

        drop(guard);

        match result {
            Ok(()) => {
                let mut app = crate::TurtleApp::new();
                app.execute_immediate(0, turtle);

                app.export_drawing(filename, crate::export::DrawingFormat::Svg)
            }
            Err(payload) => {
                let err_msg = if is_mq_panic.load(Ordering::SeqCst) {
                    "Drawing function called Macroquad window/GUI functions (e.g. `screen_width()`, `screen_height()`) which are unavailable in headless SVG export mode.".to_string()
                } else if let Some(s) = payload.downcast_ref::<&str>() {
                    (*s).to_string()
                } else if let Some(s) = payload.downcast_ref::<String>() {
                    s.clone()
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
                eprintln!("Error exporting SVG: {e}");
                std::process::exit(1);
            }
        }
    }
}
