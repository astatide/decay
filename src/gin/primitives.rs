use super::vertex;

// 1 / sqRt 3: 0.57735026919
// 2 / sqRt 3: 1.15470053838
// 4 / sqRt 6: 1.63299316186

// 1 / sqrt(2): 0.70710678118
// / 2 = 0.35355339059

// Tetrahedron coordinates: (±1, 0, -1/√2), (0, ±1, 1/√2)

const SF: f32 = 2.0; 

pub const VERTICES: &[vertex::Vertex] = &[
    vertex::Vertex { position: [-0.5/SF, 0.0, -0.35355339/SF], color: [0.5, 0.0, 0.5] }, // A
    vertex::Vertex { position: [0.5/SF, 0.0, -0.35355339/SF], color: [0.5, 0.0, 0.5] }, // B
    vertex::Vertex { position: [0.0, -0.5/SF, 0.35355339/SF], color: [0.5, 0.0, 0.5] }, // C
    vertex::Vertex { position: [0.0, 0.5/SF, 0.35355339/SF], color: [0.5, 0.0, 0.5] } // D
];

// FUCK!  Apparently order DOES matter.  WHAT?!
pub const INDICES: &[u16] = &[
    1, 3, 2, // F 1
    3, 1, 0, // F 2
    2, 0, 1, // F 3
    0, 2, 3, // F 4
    // 0, 1, 2, // F3 but not
    // 0, 1, 3, // F 2 but not
    // 0, 2, 3, // F 4
    // 1, 2, 3 // F 1 but not
];

// we need to draw a set of coordinates for each face.
// wait, how the fuck does this magnet shit work.
// alright, I exhaustively did everything and it works.  Need to go back and read what I'm missing here re: whether... does order matter?  That's weird.
// pub const INDICES: &[u16] = &[
//     0, 1, 2,
//     0, 2, 1,
//     0, 1, 3,
//     0, 3, 1,
//     0, 3, 2,
//     0, 2, 3,
//     1, 2, 3,
//     1, 3, 2,
//     1, 0, 2,
//     1, 2, 0,
//     1, 3, 0,
//     1, 0, 3,
//     2, 0, 1,
//     2, 1, 0,
//     2, 0, 3,
//     2, 3, 0,
//     2, 1, 3,
//     2, 3, 1,
//     3, 2, 1,
//     3, 1, 2,
//     3, 0, 1,
//     3, 1, 0,
//     3, 0, 2,
//     3, 2, 0
// ];