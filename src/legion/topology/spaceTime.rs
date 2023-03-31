use crate::legion::sin::ff::{Elements, ForceField};
use std::collections::HashMap;
use num_traits::{Zero, Float};

// #[derive(Debug)]

pub trait ContainsParticles<ParT> {
    fn get_particles(&self) -> &HashMap<String, ParT>;
    fn get_mut_particles(&mut self) -> &mut HashMap<String, ParT>;
}
// pub trait ContainsAtomicParticles {
//     fn get_particles(&self) -> Option<&HashMap<String, Box<dyn IsAtomic>>>;
//     fn get_mut_particles(&mut self) -> Option<&mut HashMap<String, Box<dyn IsAtomic>>>;
// }

pub struct SpaceTime<ParT, NumT: Float> {
    particles: HashMap<String, ParT>,
    time: NumT,
    dimensions: u32,
}

impl<ParT, NumT: Float> SpaceTime<ParT, NumT> {
    pub fn new() -> Self {
        Self {
            particles: HashMap::<String, ParT>::new(),
            time: Zero::zero(),
            dimensions: 3,
        }
    }

    pub fn set_particles(&mut self, particles: HashMap<String, ParT>) {
        self.particles = particles;
    }

    // pub fn set_atomic_particles(&mut self, particles: Option<HashMap<String, Box<dyn IsAtomic>>>) {
    //     self.atomic_particles = particles;
    // }
}

impl<ParT, NumT: Float> ContainsParticles<ParT> for SpaceTime<ParT, NumT> {
    fn get_mut_particles(&mut self) -> &mut HashMap<String, ParT> {
        return &mut self.particles;
    }
    fn get_particles(&self) -> &HashMap<String, ParT> {
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
