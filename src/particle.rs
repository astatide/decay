use std::collections::HashMap;
use uuid::Uuid;
use crate::ff::ForceField;
use crate::ff::Elements;

#[derive(Debug)]
pub struct Particle {
    mass: f32,
    charge: f32,
    position: Vec<f64>,
    velocity: Vec<f64>
}

impl Particle {
    pub fn new() -> Self {
        Self {
            mass: 0.0,
            charge: 0.0,
            position: Vec::new(),
            velocity: Vec::new()
        }
    }
}

impl HasPhysics for Particle {
    fn get_position(&self) -> &Vec<f64> {
        return &self.position;
    }
    fn set_position(mut self, pos: Vec<f64>) {
        self.position = pos;
    }
    fn get_velocity(&self) -> &Vec<f64> {
        return &self.velocity;
    }
    fn set_velocity(mut self, vel: Vec<f64>) {
        self.velocity = vel;
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

pub trait HasMass {
    fn set_mass(&mut self, mass: f32);
}

pub trait HasCharge {
    fn set_charge(&mut self, charge: f32);
}

pub trait HasElement {
    fn get_element(&self) -> &Elements;
}

pub trait HasPhysics {
    fn set_position(self, pos: Vec<f64>);
    fn set_velocity(self, vel: Vec<f64>);
    fn get_position(&self) -> &Vec<f64>;
    fn get_velocity(&self) -> &Vec<f64>;
}

pub trait IsSpatial {
    fn generate_spatial_coordinates(nDim: u32);
    fn set_position(pos: Vec<f64>);
}

pub trait Connected {
    fn force(&self);
    fn get_neighbors(&self) -> &Vec<String>;
    fn modify_bonded_list(&self, other: &impl Connected);
}

pub trait Charge {
    fn force(&self);
}

// needs to implement Connected, Charge, Bonded, HasPhysics, IsSpatial
#[derive(Debug)]
pub struct Atom {
    pub particle: Particle,
    pub element: Elements,
    pub id: String,
    pub neighbors: Vec<String>
}

impl Atom {
    pub fn new(element: Elements, ff: &impl ForceField) -> Self {
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
impl HasElement for Atom {
    fn get_element(&self) -> &Elements {
        return &self.element;
    }
}

impl Connected for Atom {
    fn force(&self) {
        
    }
    fn get_neighbors(&self) -> &Vec<String> {
        return &self.neighbors;
    }
    fn modify_bonded_list(&self, other: &impl Connected) {
        
    }
}
impl HasPhysics for Atom {
    fn get_position(&self) -> &Vec<f64> {
        return self.particle.get_position();
    }
    fn set_position(self, pos: Vec<f64>) {
        self.particle.set_position(pos);
    }
    fn get_velocity(&self) -> &Vec<f64> {
        return self.particle.get_velocity();
    }
    fn set_velocity(self, vel: Vec<f64>) {
        self.particle.set_velocity(vel);
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

impl ContainsParticles for SpaceTime {
    fn get_particles(&self) -> &HashMap<String, Atom> {
        return &self.particles;
    }
}

pub trait ContainsParticles {
    fn get_particles(&self) -> &HashMap<String, Atom>;
}