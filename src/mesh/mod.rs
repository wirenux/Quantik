mod draw;
mod primitives;

use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
}

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>, // table with all the position of all the point of the object
    pub normals: Vec<Vec3>,
    pub indices: Vec<u32>, // table with int group by 3, each group point to some position in vertices to create a triangle
}
