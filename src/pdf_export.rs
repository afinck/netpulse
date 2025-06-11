// filepath: /netpulse/netpulse/src/pdf_export.rs
use printpdf::PdfDocument;
use std::fs::File;
use std::io::BufWriter;

pub fn export_to_pdf(data: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (doc, page1, layer1) = PdfDocument::new("Measurement Data", printpdf::Mm(300.0), printpdf::Mm(300.0), "Layer 1");
    let layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_external_font(File::open("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf")?)?;
    layer.set_font(&font, 48.0);
    layer.write_text(data, &font);

    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);
    doc.save(&mut writer)?;

    Ok(())
}

// Additional functions for HTML to PDF conversion can be added here.