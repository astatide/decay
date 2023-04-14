use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Deref};
use std::convert::From;
use num_traits::float::FloatCore;
#[macro_use]
extern crate decay_si_derive;

#[derive(SITypes, SIDeref, PartialEq, Debug, Copy, Clone)]
struct Meter(f64);

#[derive(SITypes, SIDeref, PartialEq, Debug, Copy, Clone)]
struct Newtons(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let kM = KiloMeter;
        let hM = HectoMeter;
        let M = Meter;
        let Angstrom = AngMeter;
        assert_eq!(*(M(1.0) + kM(1.0)), 1.001); // returns kilometers
        assert_eq!(*(kM(1.0) + M(1.0)), 1001.0); // returns meters
        let mut d = kM(1.0); // 1.0
        d += 1.0; // 2.0
        assert_eq!(*d, 2.0);
        d += Meter(1.0); // does not convert to a meter now yay. 2.001!
        d += 1.0; // Now 3.001
        assert_eq!(*d, 3.001); // returns meters
        assert_eq!(*(kM(1.0) + kM(1.0)), 2.0);
        assert_eq!(*(kM(1.0) + 1.0), 2.0);
        assert_eq!((*kM(1.0) + 1.0), 2.0);
        assert_eq!(Meter::from(kM(1.0) + 1.0), M(2000.0));
    }
}
