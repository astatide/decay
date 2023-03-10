use std::collections::HashMap;

#[derive(Debug)]
struct Particle {
    mass: u32,
    charge: u32,
}

// // these aren't actually methods/functions I care about, they're just placeholder examples.
// impl Particle {
//     // associated function; think like a static method.
//     fn new() -> Self {
//         Self {
//             edges: BTreeSet::new(),
//         }
//     }
//     // instance method!
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

pub trait HasPhysics {
    fn calculate_forces(&self);
}

pub trait IsSpatial {
    fn generate_spatial_coordinates(nDim: u32);
}

pub trait Bonded {
    fn force(&self);
}

pub trait Charge {
    fn force(&self);
}

pub trait Bondable {
    fn modify_bonded_list(&self, other: &impl Bondable);
}

// needs to implement Bondable, Charge, Bonded, HasPhysics, IsSpatial
struct Atom {
    particle: Particle,
    element: Elements,
    id: String,
    neighbors: Vec<String>
}

// needs to implement Bondable
struct Molecule {
    atoms: Vec<String>,
    neighbors: HashMap<String, Vec<String>>
}

struct SpaceTime {
    particles: HashMap<String, Atom>,
    time: f64,
    dimensions: u32
}

enum Elements {
    H,
    C,
    O,
    X
}