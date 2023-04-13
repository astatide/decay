use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Deref};
use std::convert::From;
use num_traits::float::FloatCore;
#[macro_use]
extern crate decay_si_derive;

#[derive(SITypes)]
struct meter(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let kM = Kilometer;
        let hM = Hectometer;
        let M = Onemeter;
        assert_eq!(*(M(1.0) + kM(1.0)), 1.001); // returns kilometers
        assert_eq!(*(kM(1.0) + M(1.0)), 1001.0); // returns meters
        let mut d = kM(1.0);
        d += M(1.0);
        assert_eq!(*d, 1001.0); // returns meters
        assert_eq!(*(kM(1.0) + kM(1.0)), 2.0);
        assert_eq!(*(kM(1.0) + 1.0), 2.0);
        assert_eq!((*kM(1.0) + 1.0), 2.0);
        assert_eq!(Onemeter::from(kM(1.0) + 1.0), M(2000.0));
    }
}
