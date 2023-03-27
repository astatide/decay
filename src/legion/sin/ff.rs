use num_traits::Float;

use crate::legion::topology::atom::Atom;

// it's useful to include the mass
#[derive(Debug)]
pub enum Elements {
    H(u32),
    C(u32),
    O(u32),
    X(u32)
}

pub trait ForceField<Ele, Num, Vec> {
    fn mass(&self, element: &Ele) -> Num;
    fn charge(&self, element: &Ele) -> Num;
    fn atom(&self, element: Ele) -> Atom<Ele, Num, Vec>;
    fn pairwise_interactions(&self, e1: &Ele, e2: &Ele) -> Box<dyn Fn(Num) -> Num>;
}

fn GenerateBasicPairwiseInteractions<Num>(k: Num, exp: Num) -> Box<dyn Fn(Num) -> Num> where Num: Float + 'static {
    // we're creating a very simple, almost silly interaction: some coefficient divided by the pairwise distance.
    // Frankly, it's mostly for just testing.  Also, we need to move k into the closure to ensure the lifetime is respected.
    return Box::new(move |r: Num| -> Num { k / (r.powf(exp))})
}

pub struct SIN<T> {
    pub description: String,
    pub particle_type: Vec<T>
}

impl ForceField<Elements, f64, Vec<f64>> for SIN<Elements> {
    fn atom(&self, element: Elements) -> Atom<Elements, f64, Vec<f64>> {
        Atom::new(element, self)
    }
    fn mass(&self, element: &Elements) -> f64 {
        match element {
            Elements::H(_) => 1.0,
            Elements::C(_) => 2.0,
            Elements::O(_) => 3.0,
            Elements::X(_) => 99.0
        }
    }
    fn charge(&self, element: &Elements) -> f64 {
        match element {
            Elements::H(_) => 1.0,
            Elements::C(_) => 2.0,
            Elements::O(_) => 3.0,
            Elements::X(_) => 99.0
        }
    }
    fn pairwise_interactions(&self, e1: &Elements, e2: &Elements) -> Box<dyn Fn(f64) -> f64> {
        match e1 {
            Elements::H(_) => GenerateBasicPairwiseInteractions(-1.0, 4.0),
            Elements::C(_) => GenerateBasicPairwiseInteractions(-1.0, 4.0),
            Elements::O(_) => GenerateBasicPairwiseInteractions(-1.0, 4.0),
            Elements::X(_) => GenerateBasicPairwiseInteractions(-1.0, 4.0)
        }
    }
}