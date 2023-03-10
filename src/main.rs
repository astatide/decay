use crate::particle::ForceField;

mod particle;

fn main() {
    println!("Hello, world!");
    let _sin = particle::SIN { description: "SIN".to_string() };
    println!("{:?}", _sin.atom(particle::Elements::H));
}