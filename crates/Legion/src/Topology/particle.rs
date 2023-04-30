pub trait HasMass<NumT> {
    fn set_mass(&mut self, mass: NumT);
}

pub trait HasPhysics<VecT> {
    fn set_position(&mut self, pos: VecT);
    fn set_velocity(&mut self, vel: VecT);
    fn set_acceleration(&mut self, acc: VecT);
    fn get_position(&self) -> &VecT;
    fn get_velocity(&self) -> &VecT;
    fn get_acceleration(&self) -> &VecT;
}

pub trait IsSpatial {
    fn generate_spatial_coordinates(&mut self, nDim: u32);
}
pub trait HasCharge<NumT> {
    fn force(&self);
    fn set_charge(&self, charge: NumT);
}

#[derive(Debug)]
pub struct Particle<NumT, VecT> {
    mass: NumT,
    charge: NumT,
    position: VecT,
    velocity: VecT,
    acceleration: VecT,
}
