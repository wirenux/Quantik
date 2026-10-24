use std::f32::consts::PI;

use glam::Vec3;

use super::{Mesh, Vertex};

impl Mesh {
    #[allow(dead_code)]
    pub fn generate_cube(size: f32) -> Self {
        let half = size / 2.0;

        let vertices = vec![
            Vertex {
                position: Vec3::new(-half, -half, -half),
            }, // 0 back-bottom-left
            Vertex {
                position: Vec3::new(half, -half, -half),
            }, // 1 back-bottom-right
            Vertex {
                position: Vec3::new(-half, half, -half),
            }, // 2 back-top-left
            Vertex {
                position: Vec3::new(half, half, -half),
            }, // 3 back-top-right
            Vertex {
                position: Vec3::new(-half, -half, half),
            }, // 4 front-bottom-left
            Vertex {
                position: Vec3::new(half, -half, half),
            }, // 5 front-bottom-right
            Vertex {
                position: Vec3::new(-half, half, half),
            }, // 6 front-top-left
            Vertex {
                position: Vec3::new(half, half, half),
            }, // 7 front-top-right
        ];

        let normals: Vec<Vec3> = vertices
            .iter()
            .map(|v| v.position.normalize_or_zero())
            .collect();

        let indices = vec![
            // Front face (z = +half)
            4, 5, 7, 7, 6, 4, // Back face (z = -half)
            1, 0, 2, 2, 3, 1, // Top face (y = +half)
            6, 7, 3, 3, 2, 6, // Bottom face (y = -half)
            0, 1, 5, 5, 4, 0, // Right face (x = +half)
            5, 1, 3, 3, 7, 5, // Left face (x = -half)
            0, 4, 6, 6, 2, 0,
        ];

        Mesh {
            vertices,
            normals,
            indices,
        }
    }

    pub fn generate_uv_sphere(radius: f32, sectors: u32, stacks: u32) -> Self {
        let sectors = sectors.max(5);
        let stacks = stacks.max(5);

        let mut vertices = Vec::new();
        let mut normals = Vec::new();

        for i in 0..=stacks {
            let phi = PI * i as f32 / stacks as f32;
            let y = phi.cos() * radius;
            let r = phi.sin() * radius;

            for j in 0..=sectors {
                let tetha = 2.0 * PI * j as f32 / sectors as f32;
                let x = r * tetha.cos();
                let z = r * tetha.sin();

                let pos = Vec3::new(x, y, z);
                vertices.push(Vertex { position: pos });
                normals.push(pos.normalize());
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

        Mesh {
            vertices,
            normals,
            indices,
        }
    }

    pub fn generate_cylinder(radius: f32, height: f32, segments: u32) -> Self {
        let segments = segments.max(3);
        let half_h = height / 2.0;

        let mut vertices = Vec::new();
        let mut normals = Vec::new();

        for j in 0..=segments {
            let theta = 2.0 * std::f32::consts::PI * j as f32 / segments as f32;
            let x = radius * theta.cos();
            let z = radius * theta.sin();

            let n = Vec3::new(theta.cos(), 0.0, theta.sin());

            // bottom vertex
            vertices.push(Vertex {
                position: Vec3::new(x, -half_h, z),
            });
            normals.push(n);
            // top vertex
            vertices.push(Vertex {
                position: Vec3::new(x, half_h, z),
            });
            normals.push(n);
        }

        let mut indices = Vec::new();

        for j in 0..segments {
            let bl = 2 * j;
            let tl = 2 * j + 1;
            let br = 2 * (j + 1);
            let tr = 2 * (j + 1) + 1;

            indices.extend_from_slice(&[bl, tr, br]);
            indices.extend_from_slice(&[bl, tl, tr]);
        }

        Mesh {
            vertices,
            normals,
            indices,
        }
    }
}
