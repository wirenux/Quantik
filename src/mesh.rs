use std::f32::consts::PI;

use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};
use crate::camera::Camera;
use crate::window::{Color, Framebuffer};

#[derive(Debug, Clone, Copy)]
pub struct Vertex { // like a "3D point"
    pub position: Vec3,
}

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>, // table with all the position of all the point of the object
    pub indices: Vec<u32>, // table with int group by 3, each group point to some position in vertices to create a triangle
}

impl Mesh {
    pub fn generate_cube(size: f32) -> Self {
        let half = size / 2.0;

        let vertices = vec![
            Vertex { position: Vec3::new(-half, -half, -half) }, // 0 back-bottom-left
            Vertex { position: Vec3::new( half, -half, -half) }, // 1 back-bottom-right
            Vertex { position: Vec3::new(-half,  half, -half) }, // 2 back-top-left
            Vertex { position: Vec3::new( half,  half, -half) }, // 3 back-top-right
            Vertex { position: Vec3::new(-half, -half,  half) }, // 4 front-bottom-left
            Vertex { position: Vec3::new( half, -half,  half) }, // 5 front-bottom-right
            Vertex { position: Vec3::new(-half,  half,  half) }, // 6 front-top-left
            Vertex { position: Vec3::new( half,  half,  half) }, // 7 front-top-right
        ];

        let indices = vec![
            // Front face (z = +half)
            4, 5, 7,   7, 6, 4,
            // Back face (z = -half)
            1, 0, 2,   2, 3, 1,
            // Top face (y = +half)
            6, 7, 3,   3, 2, 6,
            // Bottom face (y = -half)
            0, 1, 5,   5, 4, 0,
            // Right face (x = +half)
            5, 1, 3,   3, 7, 5,
            // Left face (x = -half)
            0, 4, 6,   6, 2, 0,
        ];

        Mesh { vertices, indices }
    }

    pub fn generate_uv_sphere(radius: f32, sectors: u32, stacks: u32) -> Self {
        let sectors = sectors.max(5);
        let stacks = stacks.max(5);

        let mut vertices = Vec::new();

        for i in 0..=stacks {
            let phi = PI * i as f32 / stacks as f32;
            let y = phi.cos() * radius;
            let r = phi.sin() * radius;

            for j in 0..=sectors {
                let tetha = 2.0 * PI * j as f32 / sectors as f32;
                let x = r * tetha.cos();
                let z = r * tetha.sin();

                vertices.push(Vertex {
                    position: Vec3::new(x, y, z),
                });
            }
        }

        let mut indices = Vec::new();
        let row = sectors + 1;

        for i in 0..stacks {
            for j in 0..sectors {
                let a = i * row + j;
                let b = a + row;

                indices.extend_from_slice(&[a, a + 1, b]);
                indices.extend_from_slice(&[a + 1, b + 1, b]);
            }
        }

        Mesh { vertices, indices }
    }

    pub fn draw(&self, framebuffer: &mut Framebuffer, model: Mat4, camera: &Camera) {
        let width = framebuffer.width;
        let height = framebuffer.height;
        let aspect_ratio = width as f32 / height as f32;
        let view_proj = camera.projection_matrix(aspect_ratio) * camera.view_matrix();

        let light_dir = Vec3::new(1.0, 3.0, 2.5); // light position

        for chunk in self.indices.chunks(3) {
            let i0 = chunk[0] as usize;
            let i1 = chunk[1] as usize;
            let i2 = chunk[2] as usize;

            let v0 = (model * Vec4::from((self.vertices[i0].position, 1.0))).xyz();
            let v1 = (model * Vec4::from((self.vertices[i1].position, 1.0))).xyz();
            let v2 = (model * Vec4::from((self.vertices[i2].position, 1.0))).xyz();

            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let n = edge1.cross(edge2);

            if n.length_squared() < 1e-12 {
                continue;
            }

            let normal = n.normalize();
            let view_dir = (camera.position - v0).normalize();
            if normal.dot(view_dir) <= 0.0 {
                continue;
            }

            let brightness = normal.dot(light_dir).max(0.0); // return 0 is value is negative
            let intensity = 0.2 + 0.8 * brightness;

            // 255.0 value can be change to change the color of the object
            let r = (255.0 * intensity) as u8;
            let g = (255.0 * intensity) as u8;
            let b = (255.0 * intensity) as u8;
            let shaded_color = Color::new(r, g, b);

            let p0 = project_point(v0, view_proj, width, height);
            let p1 = project_point(v1, view_proj, width, height);
            let p2 = project_point(v2, view_proj, width, height);

            if let (Some((x0, y0)), Some((x1, y1)), Some((x2, y2))) = (p0, p1, p2) {
                framebuffer.draw_triangle((x0, y0), (x1, y1), (x2, y2), shaded_color);
                framebuffer.draw_line(x0 as i32, y0 as i32, x1 as i32, y1 as i32, Color::BLACK);
                framebuffer.draw_line(x1 as i32, y1 as i32, x2 as i32, y2 as i32, Color::BLACK);
                framebuffer.draw_line(x2 as i32, y2 as i32, x0 as i32, y0 as i32, Color::BLACK);
            }
        }
    }
}

pub fn project_point(point: Vec3, mvp: Mat4, width: usize, height: usize) -> Option<(usize, usize)> {
    // convert 3D position (x, y, z) to homogeneous 4D coordinates (x, y, z, w=1.0)
    let point_4d = Vec4::from((point, 1.0));
    let clip_space = mvp * point_4d;

    if clip_space.w <= 0.0 {
        return None; // point behind the camera or on the camera "lens" is not rendered
    }

    // perspective divide: divide (x, y, z) by distance w to obtain NDC [-1.0, +1.0]
    let ndc = clip_space.xyz() / clip_space.w;

    // map NDC range [-1.0, +1.0] to pixel coord [0, width/heigh]
    let x_pixel = ((ndc.x + 1.0) / 2.0) * width as f32;
    let y_pixel = ((1.0 - ndc.y) / 2.0) * height as f32; // invert Y (pixel 0 is top-left)

    let x = x_pixel as usize;
    let y = y_pixel as usize;

    // check if pixel inside window dimension
    if x < width && y < height {
        Some((x, y))
    } else {
        None
    }
}
