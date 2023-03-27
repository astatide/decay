use std::collections::HashMap;
use std::ops::Sub;
use cgmath::num_traits::ToPrimitive;
use uuid::Uuid;
use crate::legion::sin::ff::ForceField;
use crate::legion::sin::ff::Elements;
use super::particle::HasCharge;
use super::particle::HasMass;
use super::particle::HasPhysics;
use super::particle::IsSpatial;
use super::particle::Particle;

pub trait HasElement<Ele> {
    fn get_element(&self) -> &Ele;
}

pub trait Connected<Vec> {
    fn force(&self);
    fn get_neighbors(&self) -> &Vec;
    fn set_neighbors(&mut self, neighbors: Vec);
}

pub trait IsAtomic<Ele, VecType>: HasPhysics<VecType> + HasElement<Ele> + Connected<Vec<String>> {}

// needs to implement Connected, Charge, Bonded, HasPhysics, IsSpatial
#[derive(Debug)]
pub struct Atom<Ele, Num, VecType> {
    pub element: Ele,
    pub id: String,
    pub neighbors: Vec<String>,
    pub mass: Num,
    pub charge: Num,
    pub position: VecType,
    pub velocity: VecType,
    pub acceleration: VecType
}

impl<Ele, Num> Atom<Ele, Num, Vec<Num>> {
    pub fn new(element: Ele, ff: &impl ForceField<Ele, Num, Vec<Num>>) -> Self {
        let mass = ff.mass(&element);
        let charge = ff.charge(&element);

        return Self {
            element: element,
            id: Uuid::new_v4().to_string(),
            neighbors: Vec::<String>::new(),
            mass: mass,
            charge: charge,
            position: Vec::<Num>::new(),
            velocity: Vec::<Num>::new(),
            acceleration: Vec::<Num>::new()
        };
    }
}

impl<Ele, Num, VecType> IsAtomic<Ele, VecType> for Atom<Ele, Num, VecType> {}

impl<Ele, Num, VecType> HasElement<Ele> for Atom<Ele, Num, VecType> {
    fn get_element(&self) -> &Ele {
        return &self.element;
    }
}

impl<Ele, Num, VecType> Connected<Vec<String>> for Atom<Ele, Num, VecType> {
    fn force(&self) {
        
    }
    fn get_neighbors(&self) -> &Vec<String> {
        return &self.neighbors;
    }
    fn set_neighbors(&mut self, neighbors: Vec<String>) {
        self.neighbors = neighbors;
    }
}

impl<Ele, Num, VecType> HasMass<Num> for Atom<Ele, Num, VecType> {
    fn set_mass(&mut self, mass: Num) {
        self.mass = mass;
    }
}

impl<Ele, Num, VecType> HasCharge<Num> for Atom<Ele, Num, VecType> {
    fn force(&self) {
        todo!()
    }
    fn set_charge(&self, charge: Num) {
        todo!()
    }
}

impl<Ele> IsSpatial for Atom<Ele, f64, Vec<f64>> {
    fn generate_spatial_coordinates(&mut self, nDim: u32) {
        self.position = vec![0.0; nDim.try_into().unwrap()];
        self.velocity = vec![0.0; nDim.try_into().unwrap()];
        self.acceleration = vec![0.0; nDim.try_into().unwrap()];
    }
}

impl<Ele, Num, VecType> HasPhysics<VecType> for Atom<Ele, Num, VecType> {
    fn get_position(&self) -> &VecType {
        return &self.position;
    }
    fn set_position(&mut self, pos: VecType) {
        self.position = pos;
    }
    fn get_velocity(&self) -> &VecType {
        return &self.velocity;
    }
    fn set_velocity(&mut self, vel: VecType) {
        self.velocity = vel;
    }
    fn get_acceleration(&self) -> &VecType {
        return &self.acceleration;
    }
    fn set_acceleration(&mut self, acc: VecType) {
        self.acceleration = acc;
    }
}

// needs to implement Bondable
struct Molecule {
    atoms: Vec<String>,
    neighbors: HashMap<String, Vec<String>>
}