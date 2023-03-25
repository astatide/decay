use std::collections::HashMap;
use std::ops::Sub;
use uuid::Uuid;
use crate::dynamics::ff::ForceField;
use crate::dynamics::ff::Elements;

use super::atom::Atom;
use super::atom::IsAtomic;
use super::atom::HasElement;
use super::particle::HasPhysics;

// uh oh, this feels like bad rust code.
// // TODO: bleh, just get this running then sort out some of the nasty trait business here!
// pub trait IsAtom<Atom>: HasPhysics<Atom> + HasElement {}

// #[derive(Debug)]

pub trait ContainsParticles<T> {
    fn get_particles(&self) -> Option<&HashMap<String, Box<T>>>;
    fn get_mut_particles(&mut self) -> Option<&mut HashMap<String, Box<T>>>;
}
// pub trait ContainsAtomicParticles {
//     fn get_particles(&self) -> Option<&HashMap<String, Box<dyn IsAtomic>>>;
//     fn get_mut_particles(&mut self) -> Option<&mut HashMap<String, Box<dyn IsAtomic>>>;
// }

pub struct SpaceTime<T> {
    particles: Option<HashMap<String, Box<T>>>,
    time: f64,
    dimensions: u32
}

impl<T> SpaceTime<T> {
    pub fn new() -> Self {
        Self {
            particles: None,
            time: 0.0,
            dimensions: 3,
        }
    }

    pub fn set_particles(&mut self, particles: Option<HashMap<String, Box<T>>>) {
        self.particles = particles;
    }

    // pub fn set_atomic_particles(&mut self, particles: Option<HashMap<String, Box<dyn IsAtomic>>>) {
    //     self.atomic_particles = particles;
    // }
}

impl<T> ContainsParticles<T> for SpaceTime<T> {
    fn get_mut_particles(&mut self) -> Option<&mut HashMap<String, Box<T>>> {
        return self.particles.as_mut();
    }
    fn get_particles(&self) -> Option<&HashMap<String, Box<T>>> {
        return self.particles.as_ref();
    }
}

// impl ContainsAtomicParticles for SpaceTime {
//     fn get_mut_particles(&mut self) -> Option<&mut HashMap<String, Box<dyn IsAtomic>>> {
//         return self.atomic_particles.as_mut();
//     }
//     fn get_particles(&self) -> Option<&HashMap<String, Box<dyn IsAtomic>>> {
//         return self.atomic_particles.as_ref();
//     }
// }