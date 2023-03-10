use crate::particle::Atom;

// it's useful to include the mass
#[derive(Debug)]
pub enum Elements {
    H(u32),
    C(u32),
    O(u32),
    X(u32)
}

pub trait ForceField {
    fn mass(&self, element: &Elements) -> f32;
    fn charge(&self, element: &Elements) -> f32;
    fn atom(&self, element: Elements) -> Atom;
    fn pairwise_interactions(&self, e1: &Elements, e2: &Elements) -> Box<dyn Fn(u32) -> u32>;
}

pub struct SIN {
    pub description: String
}

impl ForceField for SIN {
    fn atom(&self, element: Elements) -> Atom {
        Atom::new(element, self)
    }
    fn mass(&self, element: &Elements) -> f32 {
        match element {
            Elements::H(_) => 1.0,
            Elements::C(_) => 2.0,
            Elements::O(_) => 3.0,
            Elements::X(_) => 99.0
        }
    }
    fn charge(&self, element: &Elements) -> f32 {
        match element {
            Elements::H(_) => 1.0,
            Elements::C(_) => 2.0,
            Elements::O(_) => 3.0,
            Elements::X(_) => 99.0
        }
    }
    fn pairwise_interactions(&self, e1: &Elements, e2: &Elements) -> Box<dyn Fn(u32) -> u32> {
        match e1 {
            Elements::H(_) => Box::new(|x: u32| -> u32 { x + 1 }),
            Elements::C(_) => Box::new(|x: u32| -> u32 { x + 1 }),
            Elements::O(_) => Box::new(|x: u32| -> u32 { x + 1 }),
            Elements::X(_) => Box::new(|x: u32| -> u32 { x + 1 })
        }
    }
}