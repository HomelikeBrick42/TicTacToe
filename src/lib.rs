mod board;
mod rendering;

use std::sync::Arc;

pub use board::*;
pub use rendering::*;

use eframe::egui;

pub struct App {}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let wgpu_render_state = cc.wgpu_render_state.as_ref().unwrap();

        let render_state = RenderState::new(&wgpu_render_state);
        wgpu_render_state
            .renderer
            .write()
            .paint_callback_resources
            .insert(render_state);

        Self {}
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("Settings").show(ctx, |ui| {
            ui.label("Hello");
            ui.allocate_space(ui.available_size());
        });

        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                // let size = egui::vec2(640.0, 480.0);
                let size = ui.available_size();
                let (rect, _response) = ui.allocate_exact_size(size, egui::Sense::click());
                ui.painter().add(egui::PaintCallback {
                    rect,
                    callback: Arc::new(
                        eframe::egui_wgpu::CallbackFn::new()
                            .prepare(move |device, queue, encoder, resources| {
                                let state: &RenderState = resources.get().unwrap();
                                state.prepare(device, queue, encoder);
                                vec![]
                            })
                            .paint(move |info, render_pass, resources| {
                                let state: &RenderState = resources.get().unwrap();
                                state.render(info, render_pass);
                            }),
                    ),
                });
            });
    }
}
