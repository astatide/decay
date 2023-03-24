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

pub trait ContainsParticles {
    fn get_particles(&self) -> Option<&HashMap<String, Box<dyn HasPhysics>>>;
}
pub trait ContainsAtomicParticles {
    fn get_particles(&self) -> Option<&HashMap<String, Box<dyn IsAtomic>>>;
}

struct SpaceTime {
    particles: Option<HashMap<String, Box<dyn HasPhysics>>>,
    atomic_particles: Option<HashMap<String, Box<dyn IsAtomic>>>,
    time: f64,
    dimensions: u32
}

impl ContainsParticles for SpaceTime {
    fn get_particles(&self) -> Option<&HashMap<String, Box<dyn HasPhysics>>> {
        return self.particles.as_ref();
    }
}

impl ContainsAtomicParticles for SpaceTime {
    fn get_particles(&self) -> Option<&HashMap<String, Box<dyn IsAtomic>>> {
        return self.atomic_particles.as_ref();
    }
}