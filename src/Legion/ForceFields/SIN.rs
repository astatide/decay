use num_traits::{float::FloatCore, real::Real, Float};

use crate::Legion::Topology::atom::{Atom, AtomBuilder};

// it's useful to include the mass
#[derive(Debug, Clone, PartialEq)]
pub enum Elements {
    H(u32),
    C(u32),
    O(u32),
    X(u32),
}

pub trait ForceField<EleT, NumT: Float, VecT: IntoIterator<Item = NumT>> {
    fn mass(&self, element: &EleT) -> NumT;
    fn charge(&self, element: &EleT) -> NumT;
    fn atom(&self, element: EleT) -> Atom<EleT, NumT, VecT>;
    fn pairwise_interactions(&self, e1: &EleT, e2: &EleT) -> Box<dyn Fn(NumT) -> NumT>;
}

pub trait ParticleGenerator<ParT, EleT> {
    fn generate_particle(&self, element: EleT) -> ParT;
}

fn GenerateBasicPairwiseInteractions<NumT>(k: NumT, l: NumT, exp: NumT) -> Box<dyn Fn(NumT) -> NumT>
where
    NumT: FloatCore + Real + 'static,
{
    // we're creating a very simple, almost silly interaction: some coefficient divided by the pairwise distance.
    // Frankly, it's mostly for just testing.  Also, we need to move k into the closure to ensure the lifetime is respected.
    return Box::new(move |r: NumT| -> NumT {
        if r <= l {
            return <NumT as Real>::min_value(); // fake out for getting too close to the atomic radius.
        } else {
            return k / (r.powf(exp));
        }
    });
}

pub struct SIN<ParT> {
    pub description: String,
    pub particle_type: Vec<ParT>,
}

impl ParticleGenerator<Atom<Elements, f64, Vec<f64>>, Elements> for SIN<Elements> {
    fn generate_particle(&self, element: Elements) -> Atom<Elements, f64, Vec<f64>> {
        return self.atom(element);
    }
}

// very specific implementation!  Using elements, 64 bit floats, and the built in Vec type.
impl ForceField<Elements, f64, Vec<f64>> for SIN<Elements> {
    fn atom(&self, element: Elements) -> Atom<Elements, f64, Vec<f64>> {
        AtomBuilder::new()
            .element(element.clone())
            .charge(self.charge(&element))
            .mass(self.mass(&element))
            .build()
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
            Elements::H(_) => GenerateBasicPairwiseInteractions(10.0, 0.0, 2.0),
            Elements::C(_) => GenerateBasicPairwiseInteractions(10.0, 0.0, 2.0),
            Elements::O(_) => GenerateBasicPairwiseInteractions(10.0, 0.0, 2.0),
            Elements::X(_) => GenerateBasicPairwiseInteractions(10.0, 0.0, 2.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Legion::ForceFields::SIN::Elements;
    use crate::Legion::Topology::atom::HasElement;

    #[test]
    fn test_create_force_field() {
        let SinFF = SIN::<Elements> {
            description: "SIN".to_string(),
            particle_type: Vec::new(),
        };
        assert_eq!(SinFF.description, "SIN".to_string());
    }

    #[test]
    fn test_force_field_atom_builder() {
        let SinFF = SIN::<Elements> {
            description: "SIN".to_string(),
            particle_type: Vec::new(),
        };
        let atom = SinFF.atom(Elements::H(0));
        matches!(atom.get_element(), Elements::H(0));
    }
}
