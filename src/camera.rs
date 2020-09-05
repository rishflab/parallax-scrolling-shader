pub const UP_VEC: cgmath::Vector3<f32> = cgmath::Vector3::new(0.0, 0.0, 1.0);

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub look_at: cgmath::Point3<f32>,
}

impl Camera {
    pub fn new(eye: cgmath::Point3<f32>, look_at: cgmath::Point3<f32>) -> Self {
        Camera { eye, look_at }
    }
    pub fn generate_matrix(&self, aspect_ratio: f32) -> cgmath::Matrix4<f32> {
        let mx_projection = cgmath::perspective(cgmath::Deg(45f32), aspect_ratio, 1.0, 100.0);
        let mx_view = cgmath::Matrix4::look_at(self.eye, self.look_at, UP_VEC);
        mx_projection * mx_view
    }
    pub fn update(&mut self, eye: cgmath::Point3<f32>, look_at: cgmath::Point3<f32>) {
        self.eye = eye;
        self.look_at = look_at;
    }
}
