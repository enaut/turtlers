//! Export-Backend-Trait und zentrale Export-Typen

use crate::state::TurtleWorld;

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
