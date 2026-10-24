use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};

use super::Mesh;
use crate::camera::Camera;
use crate::math::project_point;
use crate::render::{Color, Framebuffer};

impl Mesh {
    pub fn draw(
        &self,
        framebuffer: &mut Framebuffer,
        model: Mat4,
        camera: &Camera,
        base_color: Color,
        atom_id: usize,
    ) {
        let width = framebuffer.width;
        let height = framebuffer.height;
        let aspect_ratio = width as f32 / height as f32;
        let view_proj = camera.projection_matrix(aspect_ratio) * camera.view_matrix();

        let light_dir = Vec3::new(1.0, 3.0, 2.5).normalize(); // light position

        let world: Vec<Vec3> = self
            .vertices
            .iter()
            .map(|v| (model * Vec4::from((v.position, 1.0))).xyz())
            .collect();

        let world_normals: Vec<Vec3> = self
            .normals
            .iter()
            .map(|n| (model * Vec4::from((*n, 0.0))).xyz().normalize_or_zero())
            .collect();

        let screen: Vec<Option<(f32, f32, f32)>> = world
            .iter()
            .map(|&p| project_point(p, view_proj, width, height))
            .collect();

        let fill_dir = Vec3::new(-1.0, 0.3, -1.0).normalize();

        let vertex_intensity: Vec<f32> = world_normals
            .iter()
            .map(|n| {
                let key = n.dot(light_dir).max(0.0);
                let fill = n.dot(fill_dir).max(0.0) * 0.3;
                let brightness = (key + fill).min(1.0);
                0.35 + 0.65 * brightness
            })
            .collect();

        for chunk in self.indices.chunks(3) {
            let i0 = chunk[0] as usize;
            let i1 = chunk[1] as usize;
            let i2 = chunk[2] as usize;

            let v0 = world[i0];
            let v1 = world[i1];
            let v2 = world[i2];

            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let face_n = edge1.cross(edge2);

            if face_n.length_squared() < 1e-12 {
                continue;
            }

            let face_normal = face_n.normalize();
            let view_dir = (camera.position - v0).normalize();
            if face_normal.dot(view_dir) <= 0.0 {
                continue;
            }

            // 255.0 value can be change to change the color of the object
            // let r = (base_color.r as f32 * intensity) as u8;
            // let g = (base_color.g as f32 * intensity) as u8;
            // let b = (base_color.b as f32 * intensity) as u8;
            // let shaded_color = Color::new(r, g, b);

            if let (Some((x0, y0, w0)), Some((x1, y1, w1)), Some((x2, y2, w2))) =
                (screen[i0], screen[i1], screen[i2])
            {
                framebuffer.draw_triangle(
                    (x0, y0),
                    (x1, y1),
                    (x2, y2),
                    w0,
                    w1,
                    w2,
                    vertex_intensity[i0],
                    vertex_intensity[i1],
                    vertex_intensity[i2],
                    base_color,
                    atom_id,
                );
                // framebuffer.draw_line_depth(x0 as i32, y0 as i32, x1 as i32, y1 as i32, w0, w1, Color::BLACK);
                // framebuffer.draw_line_depth(x1 as i32, y1 as i32, x2 as i32, y2 as i32, w1, w2, Color::BLACK);
                // framebuffer.draw_line_depth(x2 as i32, y2 as i32, x0 as i32, y0 as i32, w2, w0, Color::BLACK);
            }
        }
    }
}
