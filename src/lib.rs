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
    pub rotation: f32,
    pub scale: f32,
}

pub struct App {
    camera: Camera,
    last_frame_time: std::time::Instant,
    board: Board,
    turn: State,
}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let camera = Camera {
            position: (0.0, 0.0).into(),
            screen_size: (1.0, 1.0).into(),
            rotation: cgmath::Rad::from(cgmath::Deg(0.0)).0,
            scale: 0.5,
        };

        let wgpu_render_state = cc.wgpu_render_state.as_ref().unwrap();

        let render_state = RenderState::new(&wgpu_render_state);
        wgpu_render_state
            .renderer
            .write()
            .paint_callback_resources
            .insert(render_state);

        let mut board = Board::default();
        board
            .elements
            .iter_mut()
            .flatten()
            .for_each(|e| *e = Element::Board(Box::new(Board::default())));

        Self {
            camera,
            last_frame_time: std::time::Instant::now(),
            board,
            turn: State::Cross,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let time = std::time::Instant::now();
        let ts = time.duration_since(self.last_frame_time).as_secs_f32();
        self.last_frame_time = time;

        // maybe not do this all the time? only do it when the camera is moving or smth
        ctx.request_repaint();

        egui::SidePanel::left("Settings").show(ctx, |ui| {
            ui.label(format!("Current Turn: {}", self.turn));
            ui.allocate_space(ui.available_size());
        });

        let egui::InnerResponse {
            inner: (rect, response),
            response: _,
        } = egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                let size = ui.available_size();
                let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

                self.camera.screen_size = (size.x, size.y).into();

                let mut per_object_data = vec![];
                render_board(
                    &self.board,
                    (0.0, 0.0).into(),
                    (1.0, 1.0).into(),
                    &mut per_object_data,
                );

                ui.painter().add(egui::PaintCallback {
                    rect,
                    callback: Arc::new(
                        eframe::egui_wgpu::CallbackFn::new()
                            .prepare({
                                let camera = self.camera;
                                move |device, queue, encoder, resources| {
                                    let state: &mut RenderState = resources.get_mut().unwrap();
                                    state.prepare(camera, &per_object_data, device, queue, encoder);
                                    vec![]
                                }
                            })
                            .paint(move |_info, render_pass, resources| {
                                let state: &RenderState = resources.get().unwrap();
                                state.render(render_pass);
                            }),
                    ),
                });

                (rect, response)
            });

        if response.clicked() {
            let click_pos = response.interact_pointer_pos().unwrap();
            if rect.contains(click_pos) {
                let ndc_coords = ((click_pos - rect.left_top()) / rect.size() * 2.0
                    - egui::Vec2::splat(1.0))
                    * egui::vec2(1.0, -1.0);
                println!("Click at {}, {}", ndc_coords.x, ndc_coords.y);
            }
        }

        if response.hovered() {
            ctx.input(|i| {
                if i.scroll_delta.y > 0.0 {
                    self.camera.scale *= 0.95;
                } else if i.scroll_delta.y < 0.0 {
                    self.camera.scale /= 0.95;
                }
            });
        }

        if !ctx.wants_keyboard_input() {
            ctx.input(|i| {
                const CAMERA_SPEED: f32 = 2.0;
                if i.key_down(egui::Key::W) || i.key_down(egui::Key::ArrowUp) {
                    self.camera.position.y += CAMERA_SPEED / self.camera.scale * ts;
                }
                if i.key_down(egui::Key::S) || i.key_down(egui::Key::ArrowDown) {
                    self.camera.position.y -= CAMERA_SPEED / self.camera.scale * ts;
                }
                if i.key_down(egui::Key::A) || i.key_down(egui::Key::ArrowLeft) {
                    self.camera.position.x -= CAMERA_SPEED / self.camera.scale * ts;
                }
                if i.key_down(egui::Key::D) || i.key_down(egui::Key::ArrowRight) {
                    self.camera.position.x += CAMERA_SPEED / self.camera.scale * ts;
                }
            });
        }
    }
}

fn render_board(
    board: &Board,
    position: cgmath::Vector2<f32>,
    scale: cgmath::Vector2<f32>,
    per_object_data: &mut Vec<PerObjectData>,
) {
    for (x, column) in board.elements.iter().enumerate() {
        for (y, element) in column.iter().enumerate() {
            for x in 0..=3 {
                per_object_data.push(PerObjectData {
                    object_position: position + cgmath::vec2((x as f32 - 1.5) * scale.x, 0.0),
                    rotation: cgmath::Rad::from(cgmath::Deg(0.0)).0,
                    scale: cgmath::vec2(0.05 * scale.x, 3.05 * scale.y),
                    color: (0.2, 0.2, 0.2).into(),
                    is_circle: 0,
                    circle_width: 0.0,
                });
            }
            for y in 0..=3 {
                per_object_data.push(PerObjectData {
                    object_position: position + cgmath::vec2(0.0, (y as f32 - 1.5) * scale.y),
                    rotation: cgmath::Rad::from(cgmath::Deg(0.0)).0,
                    scale: cgmath::vec2(3.05 * scale.x, 0.05 * scale.y),
                    color: (0.2, 0.2, 0.2).into(),
                    is_circle: 0,
                    circle_width: 0.0,
                });
            }

            let position =
                position + cgmath::vec2((x as f32 - 1.0) * scale.x, (y as f32 - 1.0) * scale.y);
            match element {
                Element::State(None) => {} // nothing to render
                Element::State(Some(State::Circle)) => {
                    per_object_data.push(PerObjectData {
                        object_position: position,
                        rotation: cgmath::Rad::from(cgmath::Deg(0.0)).0,
                        scale,
                        color: (0.0, 0.0, 1.0).into(),
                        is_circle: 1,
                        circle_width: 0.1,
                    });
                }
                Element::State(Some(State::Cross)) => {
                    per_object_data.push(PerObjectData {
                        object_position: position,
                        rotation: cgmath::Rad::from(cgmath::Deg(45.0)).0,
                        scale: cgmath::vec2(0.1 * scale.x, scale.y),
                        color: (1.0, 0.0, 0.0).into(),
                        is_circle: 0,
                        circle_width: 0.0,
                    });
                    per_object_data.push(PerObjectData {
                        object_position: position,
                        rotation: cgmath::Rad::from(cgmath::Deg(-45.0)).0,
                        scale: cgmath::vec2(0.1 * scale.x, scale.y),
                        color: (1.0, 0.0, 0.0).into(),
                        is_circle: 0,
                        circle_width: 0.0,
                    });
                }
                Element::Board(board) => {
                    render_board(board, position, scale / 3.0, per_object_data)
                }
            }
        }
    }
}
