use std::iter::Enumerate;
use std::ops::Sub;

use uuid::Uuid;
use crate::ff::ForceField;
use crate::particle::ContainsParticles;
use crate::particle::HasElement;
use crate::particle::HasParticle;
use crate::particle::HasPhysics;
use crate::particle::Connected;
use crate::particle::Particle;

pub trait Integrator<T> {
    fn integrate(&self, particle: &mut impl HasPhysics<T>, acc: Vec<f64>);
    fn calculate_forces(&self, particle: &(impl HasPhysics<T> + Connected + HasElement), world: &impl ContainsParticles, sin: &impl ForceField) -> Vec<f64>;
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

impl Integrator<Particle> for Leapfrog {
    fn integrate(&self, particle: &mut impl HasPhysics<Particle>, acc: Vec<f64>) {
        let mut pos = particle.get_position().clone();
        let mut vel = particle.get_velocity().clone();
        for i in 0..pos.len() {
            pos[i] += (vel[i]*self.dt) + (0.5*acc[i]*self.dt.powi(2));
        }
        for i in 0..vel.len() {
            vel[i] += acc[i]*self.dt*0.5;
        }
        particle.set_position(pos);
        particle.set_velocity(vel);
    }

    // this is _probably_ not the ideal way to like, do this, but I don't care at the moment lmao.
    fn calculate_forces(&self, particle: &(impl HasPhysics<Particle> + Connected + HasElement), world: &impl ContainsParticles, sin: &impl ForceField) -> Vec<f64> {
        let neighbors = particle.get_neighbors();
        let mut force_sum: Vec<f64> = vec![0.0; particle.get_position().len()]; // use the vec macro to prefill with 0.
        for neighbor in neighbors.iter() {
            // get the actual atom
            let _na = world.get_particles().get(neighbor);
            match _na {
                Some(na) => {
                    let pwi = sin.pairwise_interactions(particle.get_element(), na.get_element());
                    let d = particle.distance(na.get_particle());
                    let r = d.iter().map(|&z| z*z).sum::<f64>().sqrt(); // wait, did this work?  Huh!  Crazy nifty.
                    let r_ijk = d.iter().map(|&z| z / r).collect::<Vec<f64>>(); // collect is what turns the iterator back in a vector, apparently.
                    // Now!  Get the forces!
                    let force = pwi(r as f32);
                    for (i, z) in r_ijk.iter().enumerate() {
                        force_sum[i] = force as f64 * z; // cast back, etc.
                    }
                },
                None => ()
            }
        }
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