mod board;
mod rendering;
mod vertex;

use std::sync::Arc;

pub use board::*;
use encase::ShaderType;
pub use rendering::*;
pub use vertex::*;

use eframe::egui;

#[derive(Clone, Copy, ShaderType)]
pub struct Camera {
    pub position: cgmath::Vector2<f32>,
    pub screen_size: cgmath::Vector2<f32>,
    pub scale: f32,
}

pub struct App {
    camera: Camera,
}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let camera = Camera {
            position: (0.0, 0.0).into(),
            screen_size: (1.0, 1.0).into(),
            scale: 1.0,
        };

        let wgpu_render_state = cc.wgpu_render_state.as_ref().unwrap();

        let render_state = RenderState::new(&wgpu_render_state);
        wgpu_render_state
            .renderer
            .write()
            .paint_callback_resources
            .insert(render_state);

        Self { camera }
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

                self.camera.screen_size = (size.x, size.y).into();

                ui.painter().add(egui::PaintCallback {
                    rect,
                    callback: Arc::new(
                        eframe::egui_wgpu::CallbackFn::new()
                            .prepare({
                                let camera = self.camera; // copy the camera so self doesnt get captured
                                move |device, queue, encoder, resources| {
                                    let state: &mut RenderState = resources.get_mut().unwrap();
                                    state.prepare(camera, device, queue, encoder);
                                    vec![]
                                }
                            })
                            .paint(move |_info, render_pass, resources| {
                                let state: &RenderState = resources.get().unwrap();
                                state.render(render_pass);
                            }),
                    ),
                });
            });
    }
}
