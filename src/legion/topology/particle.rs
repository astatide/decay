use crate::Legion::ForceFields::SIN::{Elements, ForceField};

pub trait HasMass<NumT> {
    fn set_mass(&mut self, mass: NumT);
}

pub trait HasPhysics<VecT> {
    fn set_position(&mut self, pos: VecT);
    fn set_velocity(&mut self, vel: VecT);
    fn set_acceleration(&mut self, acc: VecT);
    fn get_position(&self) -> &VecT;
    fn get_velocity(&self) -> &VecT;
    fn get_acceleration(&self) -> &VecT;
}

pub trait IsSpatial {
    fn generate_spatial_coordinates(&mut self, nDim: u32);
}
pub trait HasCharge<NumT> {
    fn force(&self);
    fn set_charge(&self, charge: NumT);
}

#[derive(Debug)]
pub struct Particle<NumT, VecT> {
    mass: NumT,
    charge: NumT,
    position: VecT,
    velocity: VecT,
    acceleration: VecT,
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
