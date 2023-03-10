use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct Particle {
    mass: f32,
    charge: f32,
    position: Vec<f64>,
    velocity: Vec<f64>
}

impl Particle {
    fn new() -> Self {
        Self {
            mass: 0.0,
            charge: 0.0,
            position: Vec::new(),
            velocity: Vec::new()
        }
    }
}

impl HasPhysics for Particle {
    fn calculate_forces(&self) {

    }
}

impl HasMass for Particle {
    fn set_mass(&mut self, mass: f32) {
        self.mass = mass;
    }
}

impl HasCharge for Particle {
    fn set_charge(&mut self, charge: f32) {
        self.charge = charge;
    }
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

pub trait HasMass {
    fn set_mass(&mut self, mass: f32);
}

pub trait HasCharge {
    fn set_charge(&mut self, charge: f32);
}

pub trait HasPhysics {
    fn calculate_forces(&self);
}

pub trait IsSpatial {
    fn generate_spatial_coordinates(nDim: u32);
    fn set_position(pos: Vec<f64>);
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
#[derive(Debug)]
pub struct Atom {
    pub particle: Particle,
    pub element: Elements,
    pub id: String,
    pub neighbors: Vec<String>
}

impl Atom {
    fn new(element: Elements, ff: &impl ForceField) -> Self {
        let mut atom = Self {
            particle: Particle::new(),
            element: element,
            id: Uuid::new_v4().to_string(),
            neighbors: Vec::new()
        };
        atom.particle.set_mass(ff.mass(&atom.element));
        atom.particle.set_charge(ff.charge(&atom.element));
        return atom;
    }
}

impl HasPhysics for Atom {
    fn calculate_forces(&self) {
        self.particle.calculate_forces();
    }
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

// it's useful to include the mass
#[derive(Debug)]
pub enum Elements {
    H,
    C,
    O,
    X
}

pub trait ForceField {
    fn mass(&self, element: &Elements) -> f32;
    fn charge(&self, element: &Elements) -> f32;
    fn atom(&self, element: Elements) -> Atom;
}

pub struct SIN {
    pub description: String
}

impl ForceField for SIN {
    fn atom(&self, element: Elements) -> Atom {
        Atom::new(element, self)
    }
    fn mass(&self, element: &Elements) -> f32 {
        match element {
            Elements::H => 1.0,
            Elements::C => 2.0,
            Elements::O => 3.0,
            Elements::X => 99.0
        }
    }
    fn charge(&self, element: &Elements) -> f32 {
        match element {
            Elements::H => 1.0,
            Elements::C => 2.0,
            Elements::O => 3.0,
            Elements::X => 99.0
        }
    }
}

// stolen from Legion, which I also wrote so that's fine.
/*
class integrator : functionBase {
  var dt: real;
  proc this(ref atom: Particles.Atom, acc: LinAlg.vector) {
    // leapfrog / verlet
    atom.pos += (atom.vel*dt) + (0.5*acc*dt**2);
    atom.vel += (acc*dt*0.5);
  }
  // uggghhhhh _apparently_ if we don't use this, it calls the superclass method, regardless of the arguments.  Blagh.
  //proc this(ref atom: Particles.Atom, acc: LinAlg.vector) { this.__internal__(atom, acc); }
}

class dampingForce : functionBase {
  var dampingStrength: real = 0.5;
  proc this(ref atom: Particles.Atom) {
    // bullshit damping force.
    atom.vel *= dampingStrength;
  }
} */