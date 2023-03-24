use std::collections::HashMap;
use std::ops::Sub;
use uuid::Uuid;
use crate::dynamics::ff::ForceField;
use crate::dynamics::ff::Elements;
use super::particle::HasCharge;
use super::particle::HasMass;
use super::particle::HasPhysics;
use super::particle::Particle;

pub trait HasElement {
    fn get_element(&self) -> &Elements;
}

pub trait Connected {
    fn force(&self);
    fn get_neighbors(&self) -> &Vec<String>;
    // fn modify_bonded_list(&self, other: &impl Connected);
}

pub trait IsAtomic: HasPhysics + HasElement {}

// needs to implement Connected, Charge, Bonded, HasPhysics, IsSpatial
#[derive(Debug)]
pub struct Atom {
    pub element: Elements,
    pub id: String,
    pub neighbors: Vec<String>,
    pub mass: f32,
    pub charge: f32,
    pub position: Vec<f64>,
    pub velocity: Vec<f64>
}

impl Atom {
    pub fn new(element: Elements, ff: &impl ForceField) -> Self {
        let mass = ff.mass(&element);
        let charge = ff.charge(&element);

        return Self {
            element: element,
            id: Uuid::new_v4().to_string(),
            neighbors: Vec::new(),
            mass: mass,
            charge: charge,
            position: Vec::new(),
            velocity: Vec::new()
        };
    }
}

impl IsAtomic for Atom {}

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
    // fn modify_bonded_list(&self, other: &impl Connected) {
        
    // }
}

impl HasMass for Atom {
    fn set_mass(&mut self, mass: f32) {
        self.mass = mass;
    }
}

impl HasCharge for Atom {
    fn force(&self) {
        
    }
    fn set_charge(&self, charge: f32) {

    }
}

impl HasPhysics for Atom {
    fn get_position(&self) -> &Vec<f64> {
        return &self.position;
    }
    fn set_position(&mut self, pos: Vec<f64>) {
        self.position = pos;
    }
    fn get_velocity(&self) -> &Vec<f64> {
        return &self.velocity;
    }
    fn set_velocity(&mut self, vel: Vec<f64>) {
        self.velocity = vel;
    }
    fn get_relevant_neighbors(&self) -> Option<&Vec<String>> {
        return Some(self.get_neighbors());
    }
}

// needs to implement Bondable
struct Molecule {
    atoms: Vec<String>,
    neighbors: HashMap<String, Vec<String>>
}