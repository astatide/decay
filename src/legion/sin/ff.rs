use num_traits::Float;

use crate::legion::topology::atom::Atom;

// it's useful to include the mass
#[derive(Debug)]
pub enum Elements {
    H(u32),
    C(u32),
    O(u32),
    X(u32),
}

pub trait ForceField<EleT, NumT, VecT> {
    fn mass(&self, element: &EleT) -> NumT;
    fn charge(&self, element: &EleT) -> NumT;
    fn atom(&self, element: EleT) -> Atom<EleT, NumT, VecT>;
    fn pairwise_interactions(&self, e1: &EleT, e2: &EleT) -> Box<dyn Fn(NumT) -> NumT>;
}

fn GenerateBasicPairwiseInteractions<NumT>(k: NumT, exp: NumT) -> Box<dyn Fn(NumT) -> NumT>
where
    NumT: Float + 'static,
{
    // we're creating a very simple, almost silly interaction: some coefficient divided by the pairwise distance.
    // Frankly, it's mostly for just testing.  Also, we need to move k into the closure to ensure the lifetime is respected.
    return Box::new(move |r: NumT| -> NumT { k / (r.powf(exp)) });
}

pub struct SIN<ParT> {
    pub description: String,
    pub particle_type: Vec<ParT>,
}

// very specific implementation!  Using elements, 64 bit floats, and the built in Vec type.
impl ForceField<Elements, f64, Vec<f64>> for SIN<Elements> {
    fn atom(&self, element: Elements) -> Atom<Elements, f64, Vec<f64>> {
        Atom::new(element, self)
    }
    fn mass(&self, element: &Elements) -> f64 {
        match element {
            Elements::H(_) => 1.0,
            Elements::C(_) => 2.0,
            Elements::O(_) => 3.0,
            Elements::X(_) => 99.0,
        }
    }
    fn charge(&self, element: &Elements) -> f64 {
        match element {
            Elements::H(_) => 1.0,
            Elements::C(_) => 2.0,
            Elements::O(_) => 3.0,
            Elements::X(_) => 99.0,
        }
    }
    fn pairwise_interactions(&self, e1: &Elements, e2: &Elements) -> Box<dyn Fn(f64) -> f64> {
        match e1 {
            Elements::H(_) => GenerateBasicPairwiseInteractions(100.0, 2.0),
            Elements::C(_) => GenerateBasicPairwiseInteractions(100.0, 2.0),
            Elements::O(_) => GenerateBasicPairwiseInteractions(100.0, 2.0),
            Elements::X(_) => GenerateBasicPairwiseInteractions(100.0, 2.0),
        }
    }
}
