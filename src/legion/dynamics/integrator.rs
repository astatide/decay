use crate::legion::sin::ff::{Elements, ForceField};
use crate::legion::topology::atom::IsAtomic;
use crate::legion::topology::particle::HasPhysics;
use crate::legion::topology::spaceTime::ContainsParticles;
use cgmath::num_traits::ToPrimitive;
use num_traits::float::FloatCore;
use num_traits::real::Real;
use num_traits::Float;
use rand::prelude::Distribution;
use rand::Rng;
use uuid::Uuid;

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

pub trait Integrator<ParT, EleT, NumT, VecT: IntoIterator<Item=NumT>> {
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

pub struct Leapfrog<NumT> {
    pub id: String,
    pub integrator_type: IntegratorTypes,
    pub dt: NumT,
}

// specific implementation blah blah
impl Leapfrog<f64> {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            integrator_type: IntegratorTypes::LeapfrogVelocityVerlet,
            dt: 0.002,
        }
    }
}

// this is KIND of a specific implementation, but also not really.  Trying to make it as generic as possible, although I'm not sure this is the way, so to speak.
// We need to limit this to number types that have add!
// Doing a lot of limits here, which makes some sense as this is a rather specific function
impl<ParT: IsAtomic<Elements, NumT, Vec<NumT>>, NumT> Integrator<ParT, Elements, NumT, Vec<NumT>>
    for Leapfrog<NumT>
where
    NumT: FloatCore
        + std::ops::AddAssign
        + std::ops::Mul<f64, Output = NumT>
        + std::iter::Sum
        + rand::distributions::uniform::SampleUniform
        + Real,
{
    fn integrate(&self, atom: &ParT, mut acc: Vec<NumT>) -> (Vec<NumT>, Vec<NumT>, Vec<NumT>) {
        let mut pos = atom.get_position().clone();
        let mut vel = atom.get_velocity().clone();
        let mut oAcc = atom.get_acceleration().clone();
        for i in 0..pos.len() {
            acc[i] += oAcc[i];
        }
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
        let mut rng = rand::thread_rng();
        // let sign: rand::distributions::Uniform<NumT> = rand::distributions::Uniform::from(-1.0..1.1);
        // let applyJitter = true;
        let atom = &atoms[&name];
        let neighbors = atom.get_neighbors();
        let mut force_sum: Vec<NumT> =
            vec![<NumT as FloatCore>::min_positive_value(); atom.get_position().len()]; // use the vec macro to prefill with 0.

        for neighbor in neighbors.iter() {
            // get the actual atom
            let na = &atoms[neighbor];
            let pwi = sin.pairwise_interactions(atom.get_element(), na.get_element());
            let d = distance(atom, na);
            let r = FloatCore::abs(d.iter().map(|&z| z * z).sum::<NumT>().sqrt()); // wait, did this work?  Huh!  Crazy nifty.
            let r_ijk = d.iter().map(|&z| z / r).collect::<Vec<NumT>>(); // collect is what turns the iterator back in a vector, apparently.
                                                                        // Now!  Get the forces!
            let force = pwi(r);
            for (i, &z) in r_ijk.iter().enumerate() {
                force_sum[i] = force * z; // cast back, etc.
            }
        }

        // if applyJitter {
        //     for (_i, z) in force_sum.iter_mut().enumerate() {
        //         *z += (rng.gen_range(0.0..10000.0)/10.0).to_f64().unwrap() * sign.sample(&mut rng).to_f64().unwrap();
        //     }
        // }
        return force_sum;
    }
}

// stolen from Legion, which I also wrote so that's fine.
/*
class integrator : functionBase {
  var dt: real;
  proc this(ref atom: Particles.Atom, acc: LinAlg.vector) {
    // leapfrog / verlet
    atom.pos += (atom.vel*dt) + (0.5*acc*dt**2);
    atom.vel += (acc*dt*0.5);
  }
  // uggghhhhh _apparently_ if we don't use this, it calls the superclass method, regardless of the arguments.  Blagh.
  //proc this(ref atom: Particles.Atom, acc: LinAlg.vector) { this.__internal__(atom, acc); }
}

class dampingForce : functionBase {
  var dampingStrength: real = 0.5;
  proc this(ref atom: Particles.Atom) {
    // bullshit damping force.
    atom.vel *= dampingStrength;
  }
} */
