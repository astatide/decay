use std::iter::Enumerate;
use std::ops::Sub;

use uuid::Uuid;
use crate::dynamics::ff::ForceField;
use crate::dynamics::atom::Atom;
use crate::dynamics::spaceTime::ContainsParticles;
use crate::dynamics::atom::HasElement;
use crate::dynamics::particle::HasPhysics;
use crate::dynamics::atom::Connected;

use super::particle;
use super::spaceTime::{ContainsAtomicParticles};
use super::atom::IsAtomic;

pub fn distance(posA: &Box<dyn IsAtomic>, posB: &Box<dyn IsAtomic>) -> Vec<f64> {
    let mut r: Vec<f64> = Vec::new();
    let other_ijk = posB.get_position();
    for (idim, dim) in posA.get_position().iter().enumerate() {
        r.push(*dim - other_ijk[idim]);
    }
    return r;
}

pub trait Integrator {
    fn integrate(&self, particle: &Box<dyn IsAtomic>, acc: Vec<f64>) -> (Vec<f64>, Vec<f64>);
    fn calculate_forces(&self, name: String, world: &impl ContainsParticles, sin: &impl ForceField) -> Vec<f64>;
    fn calculate_neighboring_forces(&self, name: String, world: &impl ContainsAtomicParticles, sin: &impl ForceField) -> Vec<f64>;
}

pub enum IntegratorTypes {
    LeapfrogVelocityVerlet
}

pub struct Leapfrog {
    pub id: String,
    pub integrator_type: IntegratorTypes,
    pub dt: f64
}

impl Leapfrog {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            integrator_type: IntegratorTypes::LeapfrogVelocityVerlet,
            dt: 0.002
        }
    }
}

impl Integrator for Leapfrog {
    fn integrate(&self, atom: &Box<dyn IsAtomic>, acc: Vec<f64>) -> (Vec<f64>, Vec<f64>){
        let mut pos = atom.get_position().clone();
        let mut vel = atom.get_velocity().clone();
        for i in 0..pos.len() {
            pos[i] += (vel[i]*self.dt) + (0.5*acc[i]*self.dt.powi(2));
        }
        for i in 0..vel.len() {
            vel[i] += acc[i]*self.dt*0.5;
        }
        return (pos, vel)
        // atom.set_position(pos);
        // atom.set_velocity(vel);
    }
    fn calculate_forces(&self, name: String, world: &impl ContainsParticles, sin: &impl ForceField) -> Vec<f64> {
        todo!()
    }
    // this is _probably_ not the ideal way to like, do this, but I don't care at the moment lmao.
    fn calculate_neighboring_forces(&self, name: String, world: &impl ContainsAtomicParticles, sin: &impl ForceField) -> Vec<f64> {
        let atoms = &world.get_particles();
        match atoms {
            Some(atomWorld) => {
                let atom = &atomWorld[&name];
                let neighbors = atom.get_relevant_neighbors();
                let mut force_sum: Vec<f64> = vec![0.0; atom.get_position().len()]; // use the vec macro to prefill with 0.
                match neighbors {
                    Some(neighborNames) => {
                        for neighbor in neighborNames.iter() {
                            // get the actual atom
                            let na = &atomWorld[neighbor];
                            let pwi = sin.pairwise_interactions(atom.get_element(), na.get_element());
                            let d = distance(atom, na);
                            let r = d.iter().map(|&z| z*z).sum::<f64>().sqrt(); // wait, did this work?  Huh!  Crazy nifty.
                            let r_ijk = d.iter().map(|&z| z / r).collect::<Vec<f64>>(); // collect is what turns the iterator back in a vector, apparently.
                            // Now!  Get the forces!
                            let force = pwi(r as f32);
                            for (i, z) in r_ijk.iter().enumerate() {
                                force_sum[i] = force as f64 * z; // cast back, etc.
                            }
                        }
                    }
                    None => ()
                }
                return force_sum;
            }
            None => Vec::<f64>::new()
        }
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