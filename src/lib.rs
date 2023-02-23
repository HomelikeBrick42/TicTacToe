mod board;
mod per_object_data;
mod rendering;
mod vertex;

use encase::ShaderType;
use std::sync::Arc;

pub use board::*;
pub use per_object_data::*;
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
    last_frame_time: std::time::Instant,
    rotation: f32,
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

        Self {
            camera,
            last_frame_time: std::time::Instant::now(),
            rotation: 0.0,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let time = std::time::Instant::now();
        let ts = time.duration_since(self.last_frame_time).as_secs_f32();
        self.last_frame_time = time;

        self.rotation += 90.0 * ts;
        ctx.request_repaint();

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
                                let camera = self.camera;
                                let rotation = self.rotation;
                                move |device, queue, encoder, resources| {
                                    let state: &mut RenderState = resources.get_mut().unwrap();
                                    state.prepare(
                                        camera,
                                        &[PerObjectData {
                                            object_position: (0.0, 0.0).into(),
                                            rotation: cgmath::Rad::from(cgmath::Deg(rotation)).0,
                                            scale: (1.0, 1.0).into(),
                                            is_circle: 0,
                                            circle_width: 0.0,
                                        }],
                                        device,
                                        queue,
                                        encoder,
                                    );
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
