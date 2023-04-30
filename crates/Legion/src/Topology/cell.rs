use num_traits::{Float, Zero, float::FloatCore};
use std::collections::HashMap;

// #[derive(Debug)]

pub trait ContainsParticles<ParT> {
    fn get_particles(&self) -> &HashMap<String, ParT>;
    fn get_mut_particles(&mut self) -> &mut HashMap<String, ParT>;
}

pub struct Cell<ParT, NumT> {
    particles: HashMap<String, ParT>,
    time: NumT,
    dimensions: u32,
}

impl<ParT> Cell<ParT, f32> {
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

impl<ParT, NumT> ContainsParticles<ParT> for Cell<ParT, NumT> {
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
    use crate::Legion::ForceFields::SIN::{Elements, ForceField, SIN};
    use crate::Legion::Topology::atom::{Atom, HasElement};

    #[test]
    fn test_create_cell() {
        let cell = Cell::<Atom<Elements, f64, Vec<f64>>, f64>::new();
        assert_eq!(cell.dimensions, 3);
        let SinFF = SIN::<Elements> {
            description: "SIN".to_string(),
            particle_type: Vec::new(),
        };
        assert_eq!(SinFF.description, "SIN".to_string());
    }

    #[test]
    fn test_get_and_set_particles() {
        let mut cell = Cell::<Atom<Elements, f64, Vec<f64>>, f64>::new();
        let SinFF = SIN::<Elements> {
            description: "SIN".to_string(),
            particle_type: Vec::new(),
        };
        let atom = SinFF.atom(Elements::H(0));
        let mut particles = HashMap::<String, Atom<Elements, f64, Vec<f64>>>::new();
        particles.insert(atom.id.clone(), atom);
        cell.set_particles(particles.clone());
        assert_eq!(*cell.get_particles(), particles);
    }
}
