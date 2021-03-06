
use super::mesh::{MeshIndexed, MeshCompressed};

pub fn load_debug_color_cube() -> MeshCompressed {
    MeshCompressed { vertices: vec![
        // front
        -1.0, -1.0,  1.0, // 0
        1.0, -1.0,  1.0,  // 1
        1.0,  1.0,  1.0,  // 2
        -1.0,  1.0,  1.0, // 3
        // back
        -1.0, -1.0, -1.0, // 4
        -1.0,  1.0, -1.0, // 5
        1.0,  1.0, -1.0,  // 6
        1.0, -1.0, -1.0,  // 7
    ], colors: vec![
        // front
        0.583,  0.771,  0.014,  1.000,
        // back
        0.609,  0.115,  0.436,  1.000,
        // top
        0.327,  0.483,  0.844,  1.000,
        // bottom
        0.822,  0.569,  0.201,  1.000,
        // right
        0.535,  0.602,  0.923,  1.000,
        // left
        0.310,  0.747,  0.185,  1.000,
    ], v_indices: vec![
        0,  1,  2,      0,  2,  3,    // front
        4,  5,  6,      4,  6,  7,    // back
        5,  3,  2,      5,  2,  6,   // top
        4,  7,  1,      4,  1,  0,   // bottom
        7,  6,  2,      7,  2,  1,   // right
        4,  0,  3,      4,  3,  5,   // left
    ], c_indices: vec![
        0, 0,     // front
        1, 1,     // back
        2, 2,     // top
        3, 3,     // bottom
        4, 4,     // right
        5, 5,     // left
    ] }
}

pub fn load_debug_cube() -> MeshCompressed {
    MeshCompressed { vertices: vec![
        // front
        -1.0, -1.0,  1.0, // 0
        1.0, -1.0,  1.0,  // 1
        1.0,  1.0,  1.0,  // 2
        -1.0,  1.0,  1.0, // 3
        // back
        -1.0, -1.0, -1.0, // 4
        -1.0,  1.0, -1.0, // 5
        1.0,  1.0, -1.0,  // 6
        1.0, -1.0, -1.0,  // 7
    ], colors: vec![
        1.000,  1.000,  1.000,  1.000,
    ], v_indices: vec![
        0,  1,  2,      0,  2,  3,    // front
        4,  5,  6,      4,  6,  7,    // back
        5,  3,  2,      5,  2,  6,   // top
        4,  7,  1,      4,  1,  0,   // bottom
        7,  6,  2,      7,  2,  1,   // right
        4,  0,  3,      4,  3,  5,   // left
    ], c_indices: vec![
        0, 0,     // front
        0, 0,     // back
        0, 0,     // top
        0, 0,     // bottom
        0, 0,     // right
        0, 0,     // left
    ] }
}

pub fn load_debug_d20() -> MeshIndexed {
    let sqr5 = 5.0_f32.sqrt();
    let phi = (1.0 + sqr5) * 0.5;
    let golden_ratio = (10.0 + (2.0 * sqr5)).sqrt() / (4.0 * phi);
    let a = (1.0 / golden_ratio) * 0.5;
    let b = (1.0 / golden_ratio) / (2.0 * phi);
    MeshIndexed { vertices: vec![
        0.0,    b,   -a,
          b,    a,  0.0,
         -b,    a,  0.0,
        0.0,    b,    a,
        0.0,   -b,    a,
         -a,  0.0,    b,
        0.0,   -b,   -a,
          a,  0.0,   -b,
          a,  0.0,    b,
         -a,  0.0,   -b,
          b,   -a,  0.0,
         -b,   -a,  0.0,
    ], colors: vec![
        0.583,  0.771,  0.014,  1.000,
        0.609,  0.115,  0.436,  1.000,
        0.327,  0.483,  0.844,  1.000,
        0.822,  0.569,  0.201,  1.000,
        0.535,  0.602,  0.923,  1.000,
        0.310,  0.747,  0.185,  1.000,
        0.609,  0.771,  0.014,  1.000,
        0.327,  0.115,  0.436,  1.000,
        0.822,  0.483,  0.844,  1.000,
        0.535,  0.569,  0.201,  1.000,
        0.310,  0.602,  0.923,  1.000,
        0.583,  0.747,  0.185,  1.000,
        0.583,  0.115,  0.014,  1.000,
        0.609,  0.483,  0.436,  1.000,
        0.327,  0.569,  0.844,  1.000,
        0.822,  0.602,  0.201,  1.000,
        0.535,  0.747,  0.923,  1.000,
        0.310,  0.771,  0.185,  1.000,
        0.583,  0.115,  0.014,  1.000,
        0.609,  0.771,  0.436,  1.000,
    ], indices: vec![
        2,  1,  0,
        1,  2,  3,
        5,  4,  3,
        4,  8,  3,
        7,  6,  0,
        6,  9,  0,
        11,  10,  4,
        10,  11,  6,
        9,  5,  2,
        5,  9,  11,
        8,  7,  1,
        7,  8,  10,
        2,  5,  3,
        8,  1,  3,
        9,  2,  0,
        1,  7,  0,
        11,  9,  6,
        7,  10,  6,
        5,  11,  4,
        10,  8,  4,
    ] }
}