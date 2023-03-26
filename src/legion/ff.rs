use crate::legion::atom::Atom;

// it's useful to include the mass
#[derive(Debug)]
pub enum Elements {
    H(u32),
    C(u32),
    O(u32),
    X(u32)
}

pub trait ForceField<T> {
    fn mass(&self, element: &T) -> f32;
    fn charge(&self, element: &T) -> f32;
    fn atom(&self, element: T) -> Atom<T>;
    fn pairwise_interactions(&self, e1: &T, e2: &T) -> Box<dyn Fn(f32) -> f32>;
}

fn GenerateBasicPairwiseInteractions(k: f32, exp: f32) -> Box<dyn Fn(f32) -> f32> {
    // we're creating a very simple, almost silly interaction: some coefficient divided by the pairwise distance.
    // Frankly, it's mostly for just testing.  Also, we need to move k into the closure to ensure the lifetime is respected.
    return Box::new(move |r: f32| -> f32 { k / (r.powf(exp))})
}

pub struct SIN<T> {
    pub description: String,
    pub particle_type: Vec<T>
}

impl ForceField<Elements> for SIN<Elements> {
    fn atom(&self, element: Elements) -> Atom<Elements> {
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
    fn pairwise_interactions(&self, e1: &Elements, e2: &Elements) -> Box<dyn Fn(f32) -> f32> {
        match e1 {
            Elements::H(_) => GenerateBasicPairwiseInteractions(-1.0, 2.0),
            Elements::C(_) => GenerateBasicPairwiseInteractions(-1.0, 2.0),
            Elements::O(_) => GenerateBasicPairwiseInteractions(-1.0, 2.0),
            Elements::X(_) => GenerateBasicPairwiseInteractions(-1.0, 2.0)
        }
    }
}