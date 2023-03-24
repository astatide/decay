use std::collections::HashMap;
use std::ops::Sub;
use uuid::Uuid;
use crate::dynamics::ff::ForceField;
use crate::dynamics::ff::Elements;

pub trait HasMass {
    fn set_mass(&mut self, mass: f32);
}

pub trait HasPhysics {
    fn set_position(&mut self, pos: Vec<f64>);
    fn set_velocity(&mut self, vel: Vec<f64>);
    fn get_position(&self) -> &Vec<f64>;
    fn get_velocity(&self) -> &Vec<f64>;
    fn get_relevant_neighbors(&self) -> Option<&Vec<String>>;
}

pub trait IsSpatial {
    fn generate_spatial_coordinates(nDim: u32);
    fn set_position(pos: Vec<f64>);
}
pub trait HasCharge {
    fn force(&self);
    fn set_charge(&self, charge: f32);
}

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
        return None;
    }
}

impl HasMass for Particle {
    fn set_mass(&mut self, mass: f32) {
        self.mass = mass;
    }
}

impl HasCharge for Particle {
    fn force(&self) {
        
    }
    fn set_charge(&self, charge: f32) {

    }
}