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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Legion::ForceFields::SIN::{SIN, Elements, ForceField};
    use crate::Legion::Topology::atom::{HasElement, Atom};

    #[test]
    fn test_create_space_time() {
        let space_time = SpaceTime::<Atom<Elements, f64, Vec<f64>>, f64>::new();
        assert_eq!(space_time.dimensions, 3);
        let SinFF = SIN::<Elements> {
            description: "SIN".to_string(),
            particle_type: Vec::new(),
        };
        assert_eq!(SinFF.description, "SIN".to_string());
    }

    #[test]
    fn test_set_particles() {
        let mut space_time = SpaceTime::<Atom<Elements, f64, Vec<f64>>, f64>::new();
        let SinFF = SIN::<Elements> {
            description: "SIN".to_string(),
            particle_type: Vec::new(),
        };
        let atom = SinFF.atom(Elements::H(0));
        let mut particles = HashMap::<String, Atom<Elements, f64, Vec<f64>>>::new();
        particles.insert(atom.id.clone(), atom);
        space_time.set_particles(particles.clone());
        assert_eq!(*space_time.get_particles(), particles);
    }

    #[test]
    fn test_force_field_atom_builder() {

        // matches!(atom.get_element(), Elements::H(0));
    }
}