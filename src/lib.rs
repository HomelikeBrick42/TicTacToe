mod board;

use std::sync::Arc;

pub use board::*;

use eframe::egui;

pub struct App {}

impl App {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {}
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello");

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let (rect, _response) =
                    ui.allocate_exact_size(egui::vec2(200.0, 200.0), egui::Sense::click());
                ui.painter().add(egui::PaintCallback {
                    rect,
                    callback: Arc::new(
                        eframe::egui_wgpu::CallbackFn::new()
                            .prepare(move |_device, _queue, _command_encoder, _type_map| {
                                // do nothing for now
                                vec![]
                            })
                            .paint(move |_info, _render_pass, _type_map| {
                                // do nothing for now
                            }),
                    ),
                });
            });
        });
    }
}
