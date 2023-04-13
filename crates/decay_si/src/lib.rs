use std::ops::{Add, Sub, Mul, Div, Deref};
use num_traits::float::FloatCore;
#[macro_use]
extern crate decay_si_derive;
// use num_traits::

// https://www.nist.gov/pml/owm/metric-si-prefixes
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum SI {
    Quetta = 30, // 11110
    Ronna = 27,  // 11011
    Yotta = 24,  // 11000
    Zetta = 21,  // 10101
    Exa = 18,    // 10010
    Peta = 15,   // 01111
    Tera = 12,   // 01100
    Giga = 9,    // 01001
    Mega = 6,    // 00110
    Kilo = 3,    // 00011
    Hecto = 2,   // 00010
    Deka = 1,    // 00001
    One = 0,     // 00000
    Deci = -1,
    Centi = -2,
    Milli = -3,
    Micro = -6,
    Nano = -9,
    Pico = -12,
    Femto = -15,
    Atto = -18,
    Zepto = -21,
    Yocto = -24,
    Ronto = -27,
    Quecto = -30,
}

impl Sub for SI {
    type Output = u32;

    fn sub(self, other: SI) -> Self::Output {
        (self as i32 - other as i32) as u32
    }
}

// trait SIDeref {
//     fn deref(&self);
// }

trait SITypes {
    type Target;
    fn base(&self) -> &Self::Target;
}

// #[derive(SIDeref)]

#[derive(SITypes)]
struct Meters<NumT>(NumT) where NumT: FloatCore + Add + Mul + Sub + Div;

#[derive(SITypes)]
struct Newtons<NumT>(NumT) where NumT: FloatCore + Add + Mul + Sub + Div;

// #[derive(SITypes)]
// struct MetersTest<NumT>(NumT);

// macro_rules! create_si {

// }

// impl<NumT> Deref for Meters<NumT> {
//     type Target = NumT;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl<NumT> Mul<SI> for Meters<NumT> {
//     type Output = Meters<NumT>;

//     fn mul(self, si: SI) -> Self::Output {
//         // put in a check for the significance here, but briefly assume we're fine.
//         self
//     }
// }

// impl<f32> Add for Meters<f32> {
//     type Output = Meters<f32>;

//     fn add(self, other: Meters<f32>) -> Self::Output {
//         if std::f32::DIGITS >= (self.1 - other.1) {
//             // there's enough significance to make it work.
//             self
//         }
//         else {
//             // not enough significance in the underlying float type to make a difference.
//             self
//         }
//     } 
// }

// impl<f32> Sub for Meters<f32> {
//     type Output = Meters<f32>;

//     fn sub(self, other: Meters<f32>) -> Self::Output {
//         if std::f32::DIGITS >= (self.1 - other.1) {
//             // there's enough significance to make it work.
//             self
//         }
//         else {
//             // not enough significance in the underlying float type to make a difference.
//             self
//         }
//     } 
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
        let km = KiloMeters::<f64>(1.0);
        let mm = HectoMeters::<f64>(1.0);
        let m = OneMeters::<f64>(1.0);
        // assert_eq!(*(km + mm), 11.0);
        assert_eq!(*(m + km), 1.001); // returns kilometers
        assert_eq!(*(km + m), 1001.0); // returns meters
        let gn = GigaNewtons::<f64>(0.0);
    }
}
