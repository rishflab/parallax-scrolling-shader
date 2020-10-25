use cgmath::{Deg, Rad};
use glam::{Mat4, Vec3, Vec4};
use num_traits::real::Real;
use std::f32;

#[derive(Clone, Copy)]
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
        let z_near = 10.0;
        let z_far = 100.0;
        let aspect_ratio = 1.5;
        let fovy = 1.0;
        let fovx = aspect_ratio * fovy;

        let mx_perspective = glam::Mat4::perspective_lh(fovy, self.aspect_ratio, z_near, z_far);

        let (sin_fovy, cos_fovy) = (0.5 * fovy).sin_cos();
        let (sin_fovx, cos_fovx) = (0.5 * fovx).sin_cos();
        let h = z_near * sin_fovy / cos_fovy;
        let w = h * aspect_ratio;

        let mx_ortho = glam::Mat4::orthographic_lh(-w, w, -h, h, z_far, z_near);

        let mx_view = look_to_lh(self.eye, Vec3::new(0.0, 0.0, -1.0), -Vec3::unit_y());

        // mx_perspective * mx_view
        mx_ortho * mx_view
    }

    pub fn update(&mut self, eye: glam::Vec3, look_at: glam::Vec3) {
        self.eye = eye;
        self.look_at = look_at;
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            glam::Vec3::new(0.0, -5.0, 0.0),
            glam::Vec3::new(0.0, 0.0, 0.0),
            1.0,
            1.5,
        )
    }
}

fn look_to_lh(eye: Vec3, dir: Vec3, up: Vec3) -> Mat4 {
    let f = dir.normalize();
    let s = up.cross(f).normalize();
    let u = f.cross(s);
    let (fx, fy, fz) = f.into();
    let (sx, sy, sz) = s.into();
    let (ux, uy, uz) = u.into();
    Mat4::from_cols(
        Vec4::new(sx, ux, fx, 0.0),
        Vec4::new(sy, uy, fy, 0.0),
        Vec4::new(sz, uz, fz, 0.0),
        Vec4::new(-s.dot(eye), -u.dot(eye), -f.dot(eye), 1.0),
    )
}
