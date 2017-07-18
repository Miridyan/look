use gl::types::*;

pub struct Polygon {
    verts: Vec<GLfloat>,
    indices: Vec<i32>,
    position: [f32; 2],
}

pub struct Circle {
    rad: usize,
    position: [f32; 2],
}

pub enum ShapeProperties {
    AntiAlias(u8),
    ColorHex(u32),
    ColorRGBA(u8, u8, u8, u8),
    Radius(u32),
}

impl Polygon {
    pub fn new_rect(width: f32, height: f32, x_pos: i32, y_pos: i32) -> Polygon {
        Polygon {
            verts: vec![
                width, height, 0.0,     // Top Right
                width,    0.0, 0.0,     // Bottom Right
                  0.0,    0.0, 0.0,     // Bottom Left
                  0.0, height, 0.0,     // Top Left
            ],
            indices: vec![
                0, 1, 3,                // First Triangle
                1, 2, 3,                // Second Triangle
            ],
            position: [x_pos as f32, y_pos as f32],
        }
    }

    pub fn new_square(size: f32, x_pos: i32, y_pos: i32) -> Polygon {
        Polygon {
            verts: vec![
                size, size, 0.0,    // Top Right
                size,  0.0, 0.0,    // Bottom Right
                 0.0,  0.0, 0.0,    // Bottom Left
                 0.0, size, 0.0,    // Top Left
            ],
            indices: vec![
                0, 1, 3,    // First Triangle
                1, 2, 3,    // Second Triangle
            ],
            position: [x_pos as f32, y_pos as f32],
        }
    }

    // pub fn new_triangle(width: f32, height: f32, x_pos: i32, y_pos: i32) -> Polygon {
    //     Polygon {
    //         verts: vec![
    //
    //         ]
    //     }
    // }
}
