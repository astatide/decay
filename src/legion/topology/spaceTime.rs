use std::collections::HashMap;
use crate::legion::sin::ff::{ForceField, Elements};

// #[derive(Debug)]

pub trait ContainsParticles<Par> {
    fn get_particles(&self) -> &HashMap<String, Par>;
    fn get_mut_particles(&mut self) -> &mut HashMap<String, Par>;
}
// pub trait ContainsAtomicParticles {
//     fn get_particles(&self) -> Option<&HashMap<String, Box<dyn IsAtomic>>>;
//     fn get_mut_particles(&mut self) -> Option<&mut HashMap<String, Box<dyn IsAtomic>>>;
// }

pub struct SpaceTime<Par, Num> {
    particles: HashMap<String, Par>,
    time: Num,
    dimensions: u32
}

impl<Par> SpaceTime<Par, f64> {
    pub fn new() -> Self {
        Self {
            particles: HashMap::<String, Par>::new(),
            time: 0.0,
            dimensions: 3,
        }
    }

    pub fn set_particles(&mut self, particles: HashMap<String, Par>) {
        self.particles = particles;
    }

    // pub fn set_atomic_particles(&mut self, particles: Option<HashMap<String, Box<dyn IsAtomic>>>) {
    //     self.atomic_particles = particles;
    // }
}

impl<Par, Num> ContainsParticles<Par> for SpaceTime<Par, Num> {
    fn get_mut_particles(&mut self) -> &mut HashMap<String, Par> {
        return &mut self.particles;
    }
    fn get_particles(&self) -> &HashMap<String, Par> {
        return &self.particles;
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