use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};

pub fn project_point(
    point: Vec3,
    mvp: Mat4,
    width: usize,
    height: usize,
) -> Option<(f32, f32, f32)> {
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

    // check if pixel inside window dimension
    Some((x_pixel, y_pixel, clip_space.w))
}

// signed area of the parallelogram (a->b, a->c)
pub fn edge_function(a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> f32 {
    (c.0 - a.0) * (b.1 - a.1) - (c.1 - a.1) * (b.0 - a.0)
}

pub fn bounding_sphere(points: &[Vec3]) -> (Vec3, f32) {
    if points.is_empty() {
        return (Vec3::ZERO, 0.0);
    }
    let centroid: Vec3 = points.iter().copied().sum::<Vec3>() / points.len() as f32;
    let radius = points
        .iter()
        .map(|p| (*p - centroid).length())
        .fold(0.0_f32, f32::max);
    (centroid, radius)
}
