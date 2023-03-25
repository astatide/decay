use std::collections::HashMap;
use std::ops::Sub;
use uuid::Uuid;
use crate::dynamics::ff::ForceField;
use crate::dynamics::ff::Elements;
use super::particle::HasCharge;
use super::particle::HasMass;
use super::particle::HasPhysics;
use super::particle::IsSpatial;
use super::particle::Particle;

pub trait HasElement<T> {
    fn get_element(&self) -> &T;
}

pub trait Connected {
    fn force(&self);
    fn get_neighbors(&self) -> &Vec<String>;
    // fn modify_bonded_list(&self, other: &impl Connected);
}

pub trait IsAtomic<T>: HasPhysics + HasElement<T> {}

// needs to implement Connected, Charge, Bonded, HasPhysics, IsSpatial
#[derive(Debug)]
pub struct Atom<T> {
    pub element: T,
    pub id: String,
    pub neighbors: Vec<String>,
    pub mass: f32,
    pub charge: f32,
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
    pub acceleration: Vec<f64>
}

impl<T> Atom<T> {
    pub fn new(element: T, ff: &impl ForceField<T>) -> Self {
        let mass = ff.mass(&element);
        let charge = ff.charge(&element);

        return Self {
            element: element,
            id: Uuid::new_v4().to_string(),
            neighbors: Vec::new(),
            mass: mass,
            charge: charge,
            position: Vec::new(),
            velocity: Vec::new(),
            acceleration: Vec::new()
        };
    }
}

impl<T> IsAtomic<T> for Atom<T> {}

impl<T> HasElement<T> for Atom<T> {
    fn get_element(&self) -> &T {
        return &self.element;
    }
}

impl<T> Connected for Atom<T> {
    fn force(&self) {
        
    }
    fn get_neighbors(&self) -> &Vec<String> {
        return &self.neighbors;
    }
    // fn modify_bonded_list(&self, other: &impl Connected) {
        
    // }
}

impl<T> HasMass for Atom<T> {
    fn set_mass(&mut self, mass: f32) {
        self.mass = mass;
    }
}

impl<T> HasCharge for Atom<T> {
    fn force(&self) {
        
    }
    fn set_charge(&self, charge: f32) {

    }
}

impl<T> IsSpatial for Atom<T> {
    fn generate_spatial_coordinates(&mut self, nDim: u32) {
        self.position = vec![0.0; nDim.try_into().unwrap()];
        self.velocity = vec![0.0; nDim.try_into().unwrap()];
        self.acceleration = vec![0.0; nDim.try_into().unwrap()];
    }
}

impl<T> HasPhysics for Atom<T> {
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
    fn get_acceleration(&self) -> &Vec<f64> {
        return &self.acceleration;
    }
    fn set_acceleration(&mut self, acc: Vec<f64>) {
        self.acceleration = acc;
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