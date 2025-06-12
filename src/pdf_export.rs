// filepath: /netpulse/netpulse/src/pdf_export.rs
use std::io::Write;
use printpdf::*;
use std::io::{BufWriter, Cursor};
use serde_json;

pub fn export_to_pdf(data: &serde_json::Value, _file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let (doc, page1, layer1) = PdfDocument::new("Export", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let pretty_text = serde_json::to_string_pretty(&data)?;
    current_layer.use_text(&pretty_text, 12.0, Mm(10.0), Mm(287.0), &font);

    let mut buffer = Cursor::new(Vec::new());
    let mut writer = BufWriter::new(&mut buffer);
    doc.save(&mut writer)?;
    writer.flush()?;
    drop(writer); // Explicitly drop writer before using buffer

    Ok(buffer.into_inner())
}

// Additional functions for HTML to PDF conversion can be added here.

pub fn convert_json_to_pdf(json_string: &str, file_path: &str) -> Vec<u8> {
    let json_value: serde_json::Value = serde_json::from_str(&json_string).unwrap();
    export_to_pdf(&json_value, file_path).unwrap_or_default()
}