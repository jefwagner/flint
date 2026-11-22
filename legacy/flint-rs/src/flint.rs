use std::fmt;
use float_next_after::NextAfter;

use crate::utils::{fsort2, na, pb};

/// A floating point interval
#[derive(Debug, Clone, Copy)]
pub struct Flint {
    /// lower endpoint of the interval
    pub a: f64,
    /// upper endpoitn of the interval
    pub b: f64,
}

impl From<Flint> for f64 {
    fn from(f: Flint) -> f64 { f.midpoint() }
}

impl From<f32> for Flint {
    fn from(f: f32) -> Flint {
        Flint {
            a: (f.next_after(f32::NEG_INFINITY) as f64),
            b: (f.next_after(f32::INFINITY) as f64),
        }
    }
}

impl From<f64> for Flint {
    fn from(f: f64) -> Flint {
        Flint { a: pb(f), b: na(f) }
    }
}

impl From<(f64, f64)> for Flint {
    fn from((a, b): (f64, f64)) -> Flint {
        let (a, b) = fsort2(a, b);
        Flint { a, b }
    } 
}

const MAX_F64_EXACT_INT: i64 = 9_007_199_254_740_992i64;

macro_rules! impl_small_int {
    ($sm_int:ty) => {
        impl From<$sm_int> for Flint {
            fn from(i: $sm_int) -> Flint {
                Flint { a: (i as f64), b: (i as f64) }
            }
        }        
    }
}

macro_rules! impl_large_int {
    ($lg_int:ty) => {
        impl From<$lg_int> for Flint {
            fn from(i: $lg_int) -> Flint {
                if i > (MAX_F64_EXACT_INT as $lg_int) || i < -(MAX_F64_EXACT_INT as $lg_int) {
                    Flint { a: pb((i as f64)), b: na((i as f64)) }
                } else {
                    Flint { a: (i as f64), b: (i as f64) }
                }
            }
        }
    }
}

macro_rules! impl_large_uint {
    ($lg_int:ty) => {
        impl From<$lg_int> for Flint {
            fn from(i: $lg_int) -> Flint {
                if i > (MAX_F64_EXACT_INT as $lg_int) {
                    Flint { a: pb((i as f64)), b: na((i as f64)) }
                } else {
                    Flint { a: (i as f64), b: (i as f64) }
                }
            }
        }
    }
}

impl_small_int!(i8);
impl_small_int!(i16);
impl_small_int!(i32);
impl_small_int!(u8);
impl_small_int!(u16);
impl_small_int!(u32);
impl_large_int!(i64);
impl_large_int!(i128);
impl_large_int!(isize);
impl_large_uint!(u64);
impl_large_uint!(u128);
impl_large_uint!(usize);

impl Flint {
    /// Get the midpoint of the interval
    pub fn midpoint(&self) -> f64 { 0.5*self.b + 0.5*self.a }
    /// Get the width of the interval
    pub fn width(&self) -> f64 { self.b - self.a }
}

impl fmt::Display for Flint {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = self.midpoint();
        let d = 0.5*self.width(); 
        write!(f, "{v}+/-{d}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_midpoint() {
        let f = Flint { a: 0.0, b: 1.0 };
        assert_eq!(0.5, f.midpoint());
    }

    #[test]
    fn test_width() {
        let f = Flint { a: 0.0, b: 1.0 };
        assert_eq!(1.0, f.width());
    }

    #[test]
    fn test_fmt() {
        let f = Flint { a: 0.9375 , b: 1.0625 };
        let x = format!("{f}");
        assert_eq!("1+/-0.0625", x);
    }

    #[test]
    fn test_to_float() {
        let f = Flint { a: 0.0, b: 1.0 };
        assert_eq!(0.5, f64::from(f));
    }

    #[test]
    fn test_from_float() {
        let two_ulp = 3.3306690738754696e-16;
        let f = Flint::from(1.0);
        assert!(f.a < 1.0 && 1.0 < f.b);
        assert_eq!(two_ulp, f.width());
        let two_ulp = 1.7881393432617188e-7;
        let f = Flint::from(1.0_f32);
        assert!(f.a < 1.0 && 1.0 < f.b);
        assert_eq!(two_ulp, f.width());
    }

    #[test]
    fn test_from_small_int() {
        let f = Flint::from(1_i8);
        assert_eq!(1.0, f.a);
        assert_eq!(1.0, f.b);
        assert_eq!(0.0, f.width());
        let f = Flint::from(1_i16);
        assert_eq!(1.0, f.a);
        assert_eq!(1.0, f.b);
        assert_eq!(0.0, f.width());
        let f = Flint::from(1_i32);
        assert_eq!(1.0, f.a);
        assert_eq!(1.0, f.b);
        assert_eq!(0.0, f.width());
        let f = Flint::from(1_u8);
        assert_eq!(1.0, f.a);
        assert_eq!(1.0, f.b);
        assert_eq!(0.0, f.width());
        let f = Flint::from(1_u16);
        assert_eq!(1.0, f.a);
        assert_eq!(1.0, f.b);
        assert_eq!(0.0, f.width());
        let f = Flint::from(1_u32);
        assert_eq!(1.0, f.a);
        assert_eq!(1.0, f.b);
        assert_eq!(0.0, f.width());
    }

    #[test]
    fn test_from_large_int() {
        let f = Flint::from(1_i64);
        assert_eq!(1.0, f.a);
        assert_eq!(1.0, f.b);
        assert_eq!(0.0, f.width());
        let f = Flint::from(1_i128);
        assert_eq!(1.0, f.a);
        assert_eq!(1.0, f.b);
        assert_eq!(0.0, f.width());
        let f = Flint::from(MAX_F64_EXACT_INT+1);
        assert_ne!(0.0, f.width());
        let f = Flint::from(-MAX_F64_EXACT_INT-1);
        assert_ne!(0.0, f.width());
        let f = Flint::from((MAX_F64_EXACT_INT+1) as i128);
        assert_ne!(0.0, f.width());
        let f = Flint::from((-MAX_F64_EXACT_INT-1) as i128);
        assert_ne!(0.0, f.width());
    }

    #[test]
    fn test_from_large_uint() {
        let f = Flint::from(1_u64);
        assert_eq!(1.0, f.a);
        assert_eq!(1.0, f.b);
        assert_eq!(0.0, f.width());
        let f = Flint::from(1_u128);
        assert_eq!(1.0, f.a);
        assert_eq!(1.0, f.b);
        assert_eq!(0.0, f.width());
        let f = Flint::from((MAX_F64_EXACT_INT+1) as u64);
        assert_ne!(0.0, f.width());
        let f = Flint::from((MAX_F64_EXACT_INT+1) as u128);
        assert_ne!(0.0, f.width());
    }

    #[test]
    fn test_from_pair() {
        let f = Flint::from((0.0, 1.0));
        assert_eq!(0.0, f.a);
        assert_eq!(1.0, f.b);
        assert_eq!(1.0, f.width());
        let f = Flint::from((1.0, 0.0));
        assert_eq!(0.0, f.a);
        assert_eq!(1.0, f.b);
        assert_eq!(1.0, f.width());
    }
}
