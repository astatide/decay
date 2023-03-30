use super::particle::HasCharge;
use super::particle::HasMass;
use super::particle::HasPhysics;
use super::particle::IsSpatial;
use super::particle::Particle;
use crate::legion::sin::ff::Elements;
use crate::legion::sin::ff::ForceField;
use cgmath::num_traits::ToPrimitive;
use std::collections::HashMap;
use std::ops::Sub;
use uuid::Uuid;

pub trait HasElement<EleT> {
    fn get_element(&self) -> &EleT;
}

pub trait Connected<VecT: Iterator> {
    fn force(&self);
    fn get_neighbors(&self) -> &VecT;
    fn set_neighbors(&mut self, neighbors: VecT);
}

pub trait IsAtomic<EleT, VecT: Iterator>:
    HasPhysics<VecT> + HasElement<EleT> + Connected<Vec<String>>
{
}

// needs to implement Connected, Charge, Bonded, HasPhysics, IsSpatial
#[derive(Debug)]
pub struct Atom<EleT, NumT, VecT: Iterator<Item=NumT>> {
    pub element: EleT,
    pub id: String,
    pub neighbors: Vec<String>,
    pub mass: NumT,
    pub charge: NumT,
    pub position: VecT,
    pub velocity: VecT,
    pub acceleration: VecT,
}

impl<EleT, NumT> Atom<EleT, NumT, Vec<NumT>> {
    pub fn new(element: EleT, ff: &impl ForceField<EleT, NumT, Vec<NumT>>) -> Self {
        let mass = ff.mass(&element);
        let charge = ff.charge(&element);

        return Self {
            element: element,
            id: Uuid::new_v4().to_string(),
            neighbors: Vec::<String>::new(),
            mass: mass,
            charge: charge,
            position: Vec::<NumT>::new(),
            velocity: Vec::<NumT>::new(),
            acceleration: Vec::<NumT>::new(),
        };
    }
}

impl<EleT, NumT, VecT: Iterator<Item=NumT>> IsAtomic<EleT, VecT> for Atom<EleT, NumT, VecT> {}

impl<EleT, NumT, VecT: Iterator<Item=NumT>> HasElement<EleT> for Atom<EleT, NumT, VecT> {
    fn get_element(&self) -> &EleT {
        return &self.element;
    }
}

impl<EleT, NumT, VecT: Iterator<Item=NumT>> Connected<Vec<String>> for Atom<EleT, NumT, VecT> {
    fn force(&self) {}
    fn get_neighbors(&self) -> &Vec<String> {
        return &self.neighbors;
    }
    fn set_neighbors(&mut self, neighbors: Vec<String>) {
        self.neighbors = neighbors;
    }
}

impl<EleT, NumT, VecT: Iterator<Item=NumT>> HasMass<NumT> for Atom<EleT, NumT, VecT> {
    fn set_mass(&mut self, mass: NumT) {
        self.mass = mass;
    }
}

impl<EleT, NumT, VecT: Iterator<Item=NumT>> HasCharge<NumT> for Atom<EleT, NumT, VecT> {
    fn force(&self) {
        todo!()
    }
    fn set_charge(&self, charge: NumT) {
        todo!()
    }
}

impl<EleT> IsSpatial for Atom<EleT, f64, Vec<f64>> {
    fn generate_spatial_coordinates(&mut self, nDim: u32) {
        self.position = vec![0.0; nDim.try_into().unwrap()];
        self.velocity = vec![0.0; nDim.try_into().unwrap()];
        self.acceleration = vec![0.0; nDim.try_into().unwrap()];
    }
}

impl<EleT, NumT, VecT: Iterator<Item=NumT>> HasPhysics<VecT> for Atom<EleT, NumT, VecT> {
    fn set_position(&mut self, pos: VecT) {
        self.position = pos;
    }
    fn set_velocity(&mut self, vel: VecT) {
        self.velocity = vel;
    }
    fn set_acceleration(&mut self, acc: VecT) {
        self.acceleration = acc;
    }
    fn get_position(&self) -> &VecT {
        return &self.position;
    }
    fn get_velocity(&self) -> &VecT {
        return &self.velocity;
    }
    fn get_acceleration(&self) -> &VecT {
        return &self.acceleration;
    }
}

// needs to implement Bondable
struct Molecule {
    atoms: Vec<String>,
    neighbors: HashMap<String, Vec<String>>,
}
