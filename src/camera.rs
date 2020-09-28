use glam;

pub struct Camera {
    pub eye: glam::Vec3,
    pub look_at: glam::Vec3,
    pub height: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(eye: glam::Vec3, look_at: glam::Vec3, height: f32, aspect_ratio: f32) -> Self {
        Camera {
            eye,
            look_at,
            height,
            aspect_ratio,
            near: 0.0,
            far: 1000.0,
        }
    }
    pub fn generate_matrix(&self) -> glam::Mat4 {
        let mx_projection = glam::Mat4::orthographic_lh(
            -self.height * self.aspect_ratio / 2.0,
            self.height * self.aspect_ratio / 2.0,
            -self.height / 2.0,
            self.height / 2.0,
            self.near,
            self.far,
        );

        let _mx_view =
            glam::Mat4::look_at_lh(self.eye, self.look_at, glam::Vec3::new(0.0, 1.0, 0.0));
        mx_projection
    }

    pub fn update(&mut self, eye: glam::Vec3, look_at: glam::Vec3) {
        self.eye = eye;
        self.look_at = look_at;
    }
}
