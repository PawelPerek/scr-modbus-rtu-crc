#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::num::ParseIntError;
use eframe::egui;

mod crc;
mod lut;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([700.0, 380.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Modbus CRC Paweł Perek",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_zoom_factor(2.0);

            Box::<MyApp>::default()
        }),
    )
}

const MAX_ITERATIONS: usize = 1_000_000_000;

struct MyApp {
    input: String,
    iterations: usize,
    result: Option<String>,
    error: Option<String>,
    execution_time: Option<std::time::Duration>,
    iteration_time: Option<std::time::Duration>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input: "01 10 00 11 00 03 06 1A C4 BA D0".to_string(),
            iterations: 1,
            result: None,
            error: None,
            execution_time: None,
            iteration_time: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Licznik CRC (Modbus)");
            ui.horizontal(|ui| {
                let name_label = ui.label("Bajty: ");
                ui.text_edit_singleline(&mut self.input)
                    .labelled_by(name_label.id);
            });

            ui.horizontal(|ui| {
                let slider_label = ui.label("Liczba powtórzeń: ");
                ui.add(
                    egui::Slider::new(&mut self.iterations, 1..=MAX_ITERATIONS).logarithmic(true),
                )
                .labelled_by(slider_label.id);
            });

            if ui.button("Oblicz").clicked() {
                let clean_input = self.input.replace(" ", "");

                if clean_input.len() > 256 * 2 {
                    self.error = Some("Maksymalna długość ciągu to 256 bajtów".to_string());
                    return;
                }

                let iterations = self.iterations;
                let parse_result: Result<Vec<usize>, ParseIntError> = clean_input
                    .as_str()
                    .as_bytes()
                    .chunks(2)
                    .map(|chunk| {
                        let chunk_str = std::str::from_utf8(chunk).unwrap();
                        usize::from_str_radix(chunk_str, 16)
                    })
                    .collect();

                if parse_result.is_err() {
                    self.error = Some(format!("Błąd parsowania: {:?}", parse_result.err().unwrap()));
                    return;
                }

                let bytes = parse_result.unwrap();

                let mut output: usize = 0;

                let execution_timer = std::time::Instant::now();

                for _ in 0..iterations {
                    output = crc::CRC::new().calculate(&bytes);
                }

                self.execution_time = Some(execution_timer.elapsed());
                self.iteration_time = Some(self.execution_time.unwrap() / iterations as u32);
                self.result = Some(format!("{:04X}", output));
                self.error = None;
            }

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Wynik: ");

                if let Some(result) = &self.result {
                    ui.label(result);
                }
            });

            ui.horizontal(|ui| {
                ui.label("Czas wykonania: ");

                if let Some(execution_time) = &self.execution_time {
                    ui.label(format!("{:?}", execution_time));
                }
            });

            ui.horizontal(|ui| {
                ui.label("Czas iteracji: ");

                if let Some(iteration_time) = &self.iteration_time {
                    ui.label(format!("{:?}", iteration_time));
                }
            });

            if let Some(error) = &self.error {
                ui.label(eframe::egui::RichText::new(error).color(egui::Color32::RED));
            }
        });
    }
}
