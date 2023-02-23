pub struct RenderState {}

impl RenderState {
    pub fn new(_wgpu_render_state: &eframe::egui_wgpu::RenderState) -> Self {
        Self {}
    }

    pub fn prepare(
        &self,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _encoder: &mut wgpu::CommandEncoder,
    ) {
    }

    pub fn render(
        &self,
        _info: eframe::epaint::PaintCallbackInfo,
        _render_pass: &mut wgpu::RenderPass,
    ) {
    }
}
