use uuid::Uuid;
use crate::ff::ForceField;
use crate::particle::ContainsParticles;
use crate::particle::HasElement;
use crate::particle::HasPhysics;
use crate::particle::Connected;

pub trait Integrator {
    fn integrate(&self, particle: &impl HasPhysics);
    fn calculate_forces(&self, particle: &(impl HasPhysics + Connected + HasElement), world: &impl ContainsParticles, sin: &impl ForceField);
}

pub enum IntegratorTypes {
    LeapfrogVelocityVerlet
}

pub struct Leapfrog {
    pub id: String,
    pub integrator_type: IntegratorTypes
}

impl Leapfrog {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            integrator_type: IntegratorTypes::LeapfrogVelocityVerlet
        }
    }
}

impl Integrator for Leapfrog {
    fn integrate(&self, particle: &impl HasPhysics) {
       
    }
    fn calculate_forces(&self, particle: &(impl HasPhysics + Connected + HasElement), world: &impl ContainsParticles, sin: &impl ForceField) {
        let neighbors = particle.get_neighbors();
        for neighbor in neighbors.iter() {
            // get the actual atom
            let _na = world.get_particles().get(neighbor);
            match _na {
                Some(na) => {
                    let pwi = sin.pairwise_interactions(particle.get_element(), na.get_element());
                    pwi(1);
                },
                None => ()
            }
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