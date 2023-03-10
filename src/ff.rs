use crate::particle::Atom;

// it's useful to include the mass
#[derive(Debug)]
pub enum Elements {
    H,
    C,
    O,
    X
}

pub trait ForceField {
    fn mass(&self, element: &Elements) -> f32;
    fn charge(&self, element: &Elements) -> f32;
    fn atom(&self, element: Elements) -> Atom;
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
            Elements::H => 1.0,
            Elements::C => 2.0,
            Elements::O => 3.0,
            Elements::X => 99.0
        }
    }
    fn charge(&self, element: &Elements) -> f32 {
        match element {
            Elements::H => 1.0,
            Elements::C => 2.0,
            Elements::O => 3.0,
            Elements::X => 99.0
        }
    }
}