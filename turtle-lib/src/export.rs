//! Export-Backend-Trait und zentrale Export-Typen

use crate::state::TurtleWorld;

#[derive(Debug)]
pub enum ExportError {
    Io(std::io::Error),
    Format(String),
    // Weitere Formate können ergänzt werden
}

pub enum DrawingFormat {
    #[cfg(feature = "svg")]
    Svg,
    // Weitere Formate wie Png, Pdf, ...
}

pub trait DrawingExporter {
    fn export(&self, world: &TurtleWorld, filename: &str) -> Result<(), ExportError>;
}
