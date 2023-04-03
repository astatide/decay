use crate::Legion::ForceFields::SIN::{Elements, ForceField};
use crate::Legion::Topology::atom::Atomic;
use crate::Legion::Topology::particle::HasPhysics;
use crate::Legion::Topology::cell::ContainsParticles;
use num_traits::float::FloatCore;
use num_traits::real::Real;
use num_traits::{Float, Zero};
use uuid::Uuid;

pub trait Integrator<ParT, EleT, NumT: Float, VecT: IntoIterator<Item=NumT>> {
    fn integrate(&self, particle: &ParT, acc: VecT) -> (VecT, VecT, VecT);
    fn calculate_forces(
        &self,
        name: String,
        world: &impl ContainsParticles<ParT>,
        sin: &impl ForceField<EleT, NumT, VecT>,
    ) -> VecT;
}

pub enum IntegratorTypes {
    LeapfrogVelocityVerlet,
}

pub struct Leapfrog<NumT: Float> {
    pub id: String,
    pub integrator_type: IntegratorTypes,
    pub dt: NumT,
}

// The number has to support being subtracted!  See how we're doing it?
pub fn distance<ParT: HasPhysics<Vec<NumT>>, NumT: std::ops::Sub<Output = NumT>>(
    A: &ParT,
    B: &ParT,
) -> Vec<NumT>
where
    NumT: Copy,
{
    let mut r: Vec<NumT> = Vec::new();
    let other_ijk = B.get_position();
    let ijk = A.get_position();
    for i in 0..ijk.len() {
        r.push(ijk[i] - other_ijk[i]);
    }
    return r;
}

// specific implementation blah blah
impl<NumT> Leapfrog<NumT>
where
    NumT: Float
{
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            integrator_type: IntegratorTypes::LeapfrogVelocityVerlet,
            dt: Zero::zero(), // Should be 0.002 but hey.
        }
    }
}

// this is KIND of a specific implementation, but also not really.  Trying to make it as generic as possible, although I'm not sure this is the way, so to speak.
// We need to limit this to number types that have add!
// Doing a lot of limits here, which makes some sense as this is a rather specific function
impl<ParT: Atomic<Elements, NumT, Vec<NumT>>, NumT> Integrator<ParT, Elements, NumT, Vec<NumT>>
    for Leapfrog<NumT>
where
    NumT: FloatCore
        + std::ops::AddAssign
        + std::ops::Mul<f64, Output = NumT>
        + std::iter::Sum
        + rand::distributions::uniform::SampleUniform
        + Real
        + Float
{
    fn integrate(&self, atom: &ParT, mut acc: Vec<NumT>) -> (Vec<NumT>, Vec<NumT>, Vec<NumT>) {
        let mut pos = atom.get_position().clone();
        let mut vel = atom.get_velocity().clone();
        for i in 0..pos.len() {
            pos[i] += (vel[i] * (self.dt)) + (acc[i] * (FloatCore::powi(self.dt, 2) * 0.5));
        }
        for i in 0..vel.len() {
            vel[i] += (acc[i] * self.dt) * 0.5;
        }
        return (pos, vel, acc);
    }

    // this is _probably_ not the ideal way to like, do this, but I don't care at the moment lmao.
    fn calculate_forces(
        &self,
        name: String,
        world: &impl ContainsParticles<ParT>,
        sin: &impl ForceField<Elements, NumT, Vec<NumT>>,
    ) -> Vec<NumT> {
        let atoms = world.get_particles();
        let atom = &atoms[&name];
        let neighbors = atom.get_neighbors();
        let mut force_sum: Vec<NumT> =
            vec![<NumT as FloatCore>::min_positive_value(); atom.get_position().len()]; // use the vec macro to prefill with 0.

        for neighbor in neighbors.iter() {
            // get the actual atom
            let na = &atoms[neighbor];
            let pwi = sin.pairwise_interactions(atom.get_element(), na.get_element());
            let d = distance(atom, na);
            let r = FloatCore::abs(num_traits::Float::sqrt(d.iter().map(|&z| z * z).sum::<NumT>())); // wait, did this work?  Huh!  Crazy nifty.
            let r_ijk = d.iter().map(|&z| z / r).collect::<Vec<NumT>>(); // collect is what turns the iterator back in a vector, apparently.
                                                                        // Now!  Get the forces!
            let force = pwi(r);
            for (i, &z) in r_ijk.iter().enumerate() {
                force_sum[i] = force * z; // cast back, etc.
            }
        }
        return force_sum;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::Legion::ForceFields::SIN::{SIN, Elements, ForceField};
    use crate::Legion::Topology::atom::{HasElement, Atom, Connected};
    use crate::Legion::Topology::cell::{Cell};
    use std::collections::HashMap;

    #[test]
    fn test_create_integrator() {
        let integrator = Leapfrog::<f64>::new();
        matches!(integrator.integrator_type, IntegratorTypes::LeapfrogVelocityVerlet);
    }

    #[test]
    fn test_distance() {
        let SinFF = SIN::<Elements> {
            description: "SIN".to_string(),
            particle_type: Vec::new(),
        };
        let mut atomA = SinFF.atom(Elements::H(0));
        let mut atomB = SinFF.atom(Elements::H(0));
        let posA = vec![1.0, 1.0, 1.0];
        let posB = vec![0.0, 0.0, 0.0];
        atomA.set_position(posA);
        atomB.set_position(posB);
        let d = distance(&atomA, &atomB);
        assert_eq!(d, vec![1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_calculate_forces_and_integrate() {
        let integrator = Leapfrog::<f64>::new();
        let SinFF = SIN::<Elements> {
            description: "SIN".to_string(),
            particle_type: Vec::new(),
        };
        let mut atomA = SinFF.atom(Elements::H(0));
        let mut atomB = SinFF.atom(Elements::H(0));
        // set positions
        let posA = vec![1.0, 1.0, 1.0];
        let posB = vec![0.0, 0.0, 0.0];
        atomA.set_position(posA);
        atomB.set_position(posB);
        // set velocity
        let velA = vec![1.0, 1.0, 1.0];
        let velB = vec![0.0, 0.0, 0.0];
        atomA.set_velocity(velA);
        atomB.set_velocity(velB);
        // set neighbors
        atomA.set_neighbors(vec![atomB.id.clone()]);
        atomB.set_neighbors(vec![atomA.id.clone()]);
        let mut space_time = Cell::<Atom<Elements, f64, Vec<f64>>, f64>::new();
        let mut particles = HashMap::<String, Atom<Elements, f64, Vec<f64>>>::new();
        let name = atomA.id.clone();
        particles.insert(atomA.id.clone(), atomA);
        particles.insert(atomB.id.clone(), atomB);
        space_time.set_particles(particles);
        let acc = integrator.calculate_forces(name.clone(), &space_time, &SinFF);
        let (pos, vel, acc) = integrator.integrate(space_time.get_particles().get(&name).unwrap(), acc);
    }
}