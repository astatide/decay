// use crate::particle::ForceField;
use decay::run;
use Legion::ForceFields::SIN::ForceField; // have to add and use the trait!

mod Legion;

fn main() {
    pollster::block_on(decay::run());
    println!("Hello, world!");
    // let _sin = ff::SIN { description: "SIN".to_string() };
    // println!("{:?}", _sin.atom(ff::Elements::H(0)));
}