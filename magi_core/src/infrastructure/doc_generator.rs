//! Documentation generator utilities

use std::path::Path;
use anyhow::{Result, Context};
use serde::Deserialize;
use serde_json::Value;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

/// Represents a requirement entry loaded from `Requirements.json`.
#[derive(Debug, Deserialize)]
pub struct Requirement {
    pub id: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "type")]
    pub kind: String,
    // Preserve any additional fields for forward compatibility
    #[serde(flatten)]
    pub extra: Value,
}

/// Load the JSON file containing an array of requirements.
fn load_requirements<P: AsRef<Path>>(path: P) -> Result<Vec<Requirement>> {
    let data = std::fs::read_to_string(path.as_ref())
        .with_context(|| format!("Failed to read requirements file: {}", path.as_ref().display()))?;
    let reqs: Vec<Requirement> = serde_json::from_str(&data)
        .with_context(|| "Failed to deserialize Requirements.json")?;
    Ok(reqs)
}

/// Export the requirements as a simple PDF file.
///
/// The layout consists of a title page and then each requirement rendered as
/// "[id] title (type)" followed by its description.
pub fn export_requirements_to_pdf<P: AsRef<Path>, Q: AsRef<Path>>(
    json_path: P,
    pdf_path: Q,
) -> Result<()> {
    let reqs = load_requirements(json_path)?;

    // Create a new PDF document (A4 size)
    let (doc, mut page_id, mut layer_id) =
        PdfDocument::new("Requirements", Mm(210.0), Mm(297.0), "Layer 1");
    let mut current_layer = doc.get_page(page_id).get_layer(layer_id);

    // Use built‑in Helvetica font – no external files required
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;

    let mut cursor_y = Mm(280.0);
    let line_height = Mm(7.0);

    // Title
    current_layer.use_text("Project Requirements", 24.0, Mm(20.0), cursor_y, &font);
    cursor_y -= Mm(15.0);

    for req in reqs {
        // Simple overflow handling: if we near the bottom, start a new page.
        if cursor_y.0 < 30.0 {
            let (new_page, new_layer) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
            page_id = new_page;
            layer_id = new_layer;
            current_layer = doc.get_page(page_id).get_layer(layer_id);
            cursor_y = Mm(280.0);
        }

        let header = format!("[{}] {} ({})", req.id, req.title, req.kind);
        current_layer.use_text(&header, 12.0, Mm(20.0), cursor_y, &font);
        cursor_y -= line_height;

        // Render description line‑by‑line
        for line in req.description.lines() {
            current_layer.use_text(line, 10.0, Mm(25.0), cursor_y, &font);
            cursor_y -= Mm(5.0);
        }
        cursor_y -= Mm(5.0); // extra spacing between entries
    }

    // Write the PDF to the specified path
    let file = File::create(pdf_path.as_ref())
        .with_context(|| format!("Failed to create PDF file: {}", pdf_path.as_ref().display()))?;
    doc.save(&mut BufWriter::new(file))?;
    Ok(())
}
