// use crate::particle::ForceField;
use decay::run;
use legion::ff::ForceField; // have to add and use the trait!
use legion::ff;

mod legion;
// mod particle;
// mod ff;
// mod integrator;

fn main() {
    pollster::block_on(decay::run());
    println!("Hello, world!");
    // let _sin = ff::SIN { description: "SIN".to_string() };
    // println!("{:?}", _sin.atom(ff::Elements::H(0)));
}