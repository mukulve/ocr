use std::{path::PathBuf, sync::Arc};
use eframe::egui::{self, Color32, RichText};
use ocrmypdf_rs::{Ocr, OcrMyPdf};
use rfd::FileDialog;

struct OcrApp {
    input_paths: Option<Vec<PathBuf>>,
    status: String,
    processing: bool,
}

impl Default for OcrApp {
    fn default() -> Self {
        Self {
            input_paths: None,
            status: "Select a PDF to OCR".to_string(),
            processing: false,
        }
    }
}

impl eframe::App for OcrApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("PDF OCR App");

            if self.input_paths.is_some() {
                ui.label("Selected PDFs:");
            }

            ui.vertical(|ui| {
                let path_clones: Vec<PathBuf> = self.input_paths.as_ref().unwrap_or(&Vec::new()).clone();
                
                for chunk in path_clones.chunks(3) {
                    ui.horizontal(|ui| {
                        for path in chunk {
                            ui.vertical(|ui| {
                                ui.label("📄");
                                ui.label(path.file_name().unwrap_or_default().to_string_lossy());
                                
                                let button_text = format!("Delete '{}'", 
                                    path.file_name().unwrap_or_default().to_string_lossy());
                                
                                if ui.button(RichText::new(button_text.as_str()).color(Color32::RED)).clicked() {
                                    if let Some(paths) = &mut self.input_paths {
                                        paths.retain(|p| p.as_path() != path.as_path());
                                    }
                                }
                            });
                        }
                    });
                }
            });

            ui.horizontal(|ui| {
                if ui.button("Select Input PDF").clicked() {
                    if let Some(path) = FileDialog::new()
                        .add_filter("PDF", &["pdf"])
                        .add_filter("Images", &["png", "jpg", "jpeg", "tiff", "gif"])
                        .pick_file() {
    
                        if self.input_paths.is_none() {
                            self.input_paths = Some(Vec::new());
                        }   
    
                        if self.input_paths.is_some() {
                            self.input_paths.as_mut().unwrap().push(path.clone());
                        } 
                        self.status = format!("Selected: {:?}", self.input_paths);
                    }
                }

                if ui.button("Clear").clicked() {
                    if self.input_paths.is_some() {
                        self.input_paths.as_mut().unwrap().clear();
                    }

                    self.input_paths = None;
                }
            });

            if self.processing {
                ui.spinner();
                ui.label("Processing PDF...");
            }

            if ui.button("Start OCR").clicked() {
                if self.input_paths.is_some() {
                    self.processing = true;
                    run_ocr_on_pdfs(self.input_paths.as_ref().unwrap());
                    self.processing = false;
                }
            }
        });
    }
}

fn run_ocr_on_pdfs(paths: &[PathBuf]) {
    let mut ocr = OcrMyPdf::new(None, None, None);
    for path in paths {
        let output_path = path.with_file_name(format!(
            "{}_ocr.pdf",
            path.file_stem().unwrap_or_default().to_string_lossy()
        ));

        ocr.set_input_path(path.clone().to_string_lossy().to_string())
            .set_output_path(output_path.clone().to_string_lossy().to_string())
            .set_args(vec!["--force-ocr".into(), "--image-dpi".into(), "300".into()])
            .execute();
    }
}

fn main() -> eframe::Result<()> {
    let icon = eframe::icon_data::from_png_bytes(include_bytes!("./icon.png"))
        .expect("The icon data must be valid");


    let mut options = eframe::NativeOptions::default();
    options.viewport.icon = Some(Arc::new(icon));
    eframe::run_native(
        "PDF OCR App",
        options,
        Box::new(|_cc| Ok(Box::new(OcrApp::default()))),
    )
}
