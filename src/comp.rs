use std::cmp::{PartialEq, PartialOrd, Ordering};

use crate::flint::Flint;

impl<T: Into<Flint> + Copy> PartialEq<T> for Flint {
    fn eq(&self, other: &T) -> bool {
        let other = Into::<Flint>::into(*other);
        self.b >= other.a && self.a <= other.b
    }
}

impl<T: Into<Flint> + Copy> PartialOrd<T> for Flint {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        let other = Into::<Flint>::into(*other);
        if self.a > other.b {
            Some(Ordering::Greater)
        } else if self.b < other.a {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}
 
macro_rules! impl_partial_comp {
    ($other_type:ty) => {
        impl PartialEq<Flint> for $other_type {
            fn eq(&self, other: &Flint) -> bool {
                let f = Flint::from(*self);
                f.b >= other.a && f.a <= other.b
            }
        }        

        impl PartialOrd<Flint> for $other_type {
            fn partial_cmp(&self, other: &Flint) -> Option<Ordering> {
                let f = Flint::from(*self);
                if f.a > other.b {
                    Some(Ordering::Greater)
                } else if f.b < other.a {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Equal)
                }
            }
        }
    }
}

impl_partial_comp!(f32);
impl_partial_comp!(f64);
impl_partial_comp!(i8);
impl_partial_comp!(i16);
impl_partial_comp!(i32);
impl_partial_comp!(u8);
impl_partial_comp!(u16);
impl_partial_comp!(u32);
impl_partial_comp!(i64);
impl_partial_comp!(i128);
impl_partial_comp!(isize);
impl_partial_comp!(u64);
impl_partial_comp!(u128);
impl_partial_comp!(usize);

#[cfg(test)]
mod test {
    use crate::flint::Flint;

    #[test]
    fn test_eq_ne() {
        let f0 = Flint { a: 0.75, b: 1.25 };
        let f1 = Flint { a: 0.25, b: 0.50 };
        assert!(f0 != f1);
        let f1 = Flint { a: 0.5, b: 1.0} ;
        assert!(f0 == f1);
        let f1 = Flint { a: 0.875, b: 1.125} ;
        assert!(f0 == f1);
        let f1 = Flint { a: 1.0, b: 1.5} ;
        assert!(f0 == f1);
        let f1 = Flint { a: 1.5, b: 2.0} ;
        assert!(f0 != f1);
    }

    #[test]
    fn test_lt_gt() {
        let f0 = Flint { a: 0.75, b: 1.25 };
        let f1 = Flint { a: 0.25, b: 0.50 };
        assert!(f0 > f1);
        assert!(f0 >= f1);
        let f1 = Flint { a: 0.5, b: 1.0} ;
        assert!(f0 >= f1);
        assert!(f0 <= f1);
        let f1 = Flint { a: 0.875, b: 1.125} ;
        assert!(f0 >= f1);
        assert!(f0 <= f1);
        let f1 = Flint { a: 1.0, b: 1.5} ;
        assert!(f0 >= f1);
        assert!(f0 <= f1);
        let f1 = Flint { a: 1.5, b: 2.0} ;
        assert!(f0 < f1);
        assert!(f0 <= f1);
    }

    #[test]
    fn test_float_comp() {
        let f0 = Flint { a: 0.75, b: 1.25 };
        // float less
        assert!(0.5 != f0);
        assert!(f0 != 0.5);
        assert!(0.5 < f0);
        assert!(f0 > 0.5);
        assert!(0.5 <= f0);
        assert!(f0 >= 0.5);
        assert!(1.0 == f0);
        assert!(f0 == 1.0);
        assert!(1.0 >= f0);
        assert!(f0 >= 1.0);
        assert!(1.0 <= f0);
        assert!(f0 <= 1.0);
        assert!(1.5 != f0);
        assert!(f0 != 1.5);
        assert!(1.5 > f0);
        assert!(f0 < 1.5);
        assert!(1.5 >= f0);
        assert!(f0 <= 1.5);
    }

    #[test]
    fn test_int_comp() {
        let f0 = Flint { a: 0.75, b: 1.25 };
        assert!(0 != f0);
        assert!(f0 != 0);
        assert!(0 < f0);
        assert!(f0 > 0);
        assert!(0 <= f0);
        assert!(f0 >= 0);
        assert!(1 == f0);
        assert!(f0 == 1);
        assert!(1 >= f0);
        assert!(f0 >= 1);
        assert!(1 <= f0);
        assert!(f0 <= 1);
        assert!(2 != f0);
        assert!(f0 != 2);
        assert!(2 > f0);
        assert!(f0 < 2);
        assert!(2 >= f0);
        assert!(f0 <= 2);
    }


}
