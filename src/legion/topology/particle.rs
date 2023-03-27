use crate::legion::sin::ff::{ForceField, Elements};

pub trait HasMass<Num> {
    fn set_mass(&mut self, mass: Num);
}

pub trait HasPhysics<VecType> {
    fn set_position(&mut self, pos: VecType);
    fn set_velocity(&mut self, vel: VecType);
    fn set_acceleration(&mut self, acc: VecType);
    fn get_position(&self) -> &VecType;
    fn get_velocity(&self) -> &VecType;
    fn get_acceleration(&self) -> &VecType;
}

pub trait IsSpatial {
    fn generate_spatial_coordinates(&mut self, nDim: u32);
}
pub trait HasCharge<Num> {
    fn force(&self);
    fn set_charge(&self, charge: Num);
}

#[derive(Debug)]
pub struct Particle<Num, Vec> {
    mass: Num,
    charge: Num,
    position: Vec,
    velocity: Vec,
    acceleration: Vec
}

// impl Particle {
//     pub fn new() -> Self {
//         Self {
//             mass: 0.0,
//             charge: 0.0,
//             position: Vec::new(),
//             velocity: Vec::new(),
//             acceleration: Vec::new()
//         }
//     }
// }

// impl HasPhysics for Particle {
//     fn get_position(&self) -> &Vec<f64> {
//         return &self.position;
//     }
//     fn set_position(&mut self, pos: Vec<f64>) {
//         self.position = pos;
//     }
//     fn get_velocity(&self) -> &Vec<f64> {
//         return &self.velocity;
//     }
//     fn set_velocity(&mut self, vel: Vec<f64>) {
//         self.velocity = vel;
//     }
//     fn get_relevant_neighbors(&self) -> Option<&Vec<String>> {
//         return None;
//     }
//     fn get_acceleration(&self) -> &Vec<f64> {
//         return &self.acceleration;
//     }
//     fn set_acceleration(&mut self, acc: Vec<f64>) {
//         self.acceleration = acc;
//     }
// }

// impl HasMass for Particle {
//     fn set_mass(&mut self, mass: f32) {
//         self.mass = mass;
//     }
// }

// impl HasCharge for Particle {
//     fn force(&self) {
        
//     }
//     fn set_charge(&self, charge: f32) {

//     }
// }