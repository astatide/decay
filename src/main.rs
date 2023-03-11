// use crate::particle::ForceField;
use crate::ff::ForceField; // have to add and use the trait!
use decay::run;

mod particle;
mod ff;
mod integrator;

fn main() {
    pollster::block_on(decay::run());
    println!("Hello, world!");
    let _sin = ff::SIN { description: "SIN".to_string() };
    println!("{:?}", _sin.atom(ff::Elements::H(0)));
}