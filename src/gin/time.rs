
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TimeUniform {
    time: [f32; 4],
}

impl TimeUniform {
    pub fn new() -> Self {
        Self {
            time: cgmath::Vector4::new(0.0, 0.0, 0.0, 0.0).into(),
        }
    }

    pub fn update_time(&mut self, dt: [f32; 4]) {
        self.time = dt;
    }
}