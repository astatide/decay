use std::collections::HashMap;
use num_traits::{Zero, Float};

// #[derive(Debug)]

pub trait ContainsParticles<ParT> {
    fn get_particles(&self) -> &HashMap<String, ParT>;
    fn get_mut_particles(&mut self) -> &mut HashMap<String, ParT>;
}

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
}

impl<ParT, NumT: Float> ContainsParticles<ParT> for SpaceTime<ParT, NumT> {
    fn get_mut_particles(&mut self) -> &mut HashMap<String, ParT> {
        return &mut self.particles;
    }
    fn get_particles(&self) -> &HashMap<String, ParT> {
        return &self.particles;
    }
}