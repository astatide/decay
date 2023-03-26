use crate::legion::sin::ff::{ForceField, Elements};

pub trait HasMass {
    fn set_mass(&mut self, mass: f32);
}

pub trait HasPhysics {
    fn set_position(&mut self, pos: Vec<f64>);
    fn set_velocity(&mut self, vel: Vec<f64>);
    fn set_acceleration(&mut self, acc: Vec<f64>);
    fn get_position(&self) -> &Vec<f64>;
    fn get_velocity(&self) -> &Vec<f64>;
    fn get_acceleration(&self) -> &Vec<f64>;
    fn get_relevant_neighbors(&self) -> Option<&Vec<String>>;
}

pub trait IsSpatial {
    fn generate_spatial_coordinates(&mut self, nDim: u32);
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
    velocity: Vec<f64>,
    acceleration: Vec<f64>
}

impl Particle {
    pub fn new() -> Self {
        Self {
            mass: 0.0,
            charge: 0.0,
            position: Vec::new(),
            velocity: Vec::new(),
            acceleration: Vec::new()
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
    fn get_acceleration(&self) -> &Vec<f64> {
        return &self.acceleration;
    }
    fn set_acceleration(&mut self, acc: Vec<f64>) {
        self.acceleration = acc;
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