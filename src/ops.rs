use std::ops::{
    Neg,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
};

use crate::utils::*;
use crate::flint::Flint;

impl Neg for Flint {
    type Output = Flint;
    fn neg(self) -> Flint {
        Flint { a: -self.b, b: -self.a }
    }
}

impl<T: Into<Flint>> Add<T> for Flint {
    type Output = Flint;
    fn add(self, rhs: T) -> Flint {
        let rhs = Into::<Flint>::into(rhs);
        Flint {
            a: pb(self.a + rhs.a),
            b: na(self.b + rhs.b),
        }
    }
}

impl<T: Into<Flint>> AddAssign<T> for Flint {
    fn add_assign(&mut self, rhs: T) {
        let rhs = Into::<Flint>::into(rhs);
        self.a = pb(self.a + rhs.a);
        self.b = pb(self.b + rhs.b);
    }
}

impl<T: Into<Flint>> Sub<T> for Flint {
    type Output = Flint;
    fn sub(self, rhs: T) -> Flint {
        let rhs = Into::<Flint>::into(rhs);
        Flint {
            a: pb(self.a - rhs.b),
            b: na(self.b - rhs.a),
        }
    }
}

impl<T: Into<Flint>> SubAssign<T> for Flint {
    fn sub_assign(&mut self, rhs: T) {
        let rhs = Into::<Flint>::into(rhs);
        self.a = pb(self.a - rhs.b);
        self.b = na(self.b - rhs.a);
    }
}

impl<T: Into<Flint>> Mul<T> for Flint {
    type Output = Flint;
    fn mul(self, rhs: T) -> Flint {
        let rhs = Into::<Flint>::into(rhs);
        let (a, b) = fminmax4(
            self.a * rhs.a,
            self.a * rhs.b,
            self.b * rhs.a,
            self.b * rhs.b,
        );
        Flint { a: pb(a), b: na(b) }
    }
}

impl<T: Into<Flint>> MulAssign<T> for Flint {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = Into::<Flint>::into(rhs);
        let (a, b) = fminmax4(
            self.a * rhs.a,
            self.a * rhs.b,
            self.b * rhs.a,
            self.b * rhs.b,
        );
        self.a = pb(a);
        self.b = na(b);
    }
}

impl<T: Into<Flint>> Div<T> for Flint {
    type Output = Flint;
    fn div(self, rhs: T) -> Flint {
        let rhs = Into::<Flint>::into(rhs);
        let (a, b) = fminmax4(
            self.a / rhs.a,
            self.a / rhs.b,
            self.b / rhs.a,
            self.b / rhs.b,
        );
        Flint { a: pb(a), b: na(b) }
    }
}

impl<T: Into<Flint>> DivAssign<T> for Flint {
    fn div_assign(&mut self, rhs: T) {
        let rhs = Into::<Flint>::into(rhs);
        let (a, b) = fminmax4(
            self.a / rhs.a,
            self.a / rhs.b,
            self.b / rhs.a,
            self.b / rhs.b,
        );
        self.a = pb(a);
        self.b = na(b);
    }
}

macro_rules! impl_arithmatic_ops {
    ($other_type:ty) => {
        impl Add<Flint> for $other_type {
            type Output = Flint;
            fn add(self, rhs: Flint) -> Flint {
                let f = Flint::from(self);
                Flint {
                    a: pb(f.a + rhs.a),
                    b: na(f.b + rhs.b),
                }
            }
        }

        impl Sub<Flint> for $other_type {
            type Output = Flint;
            fn sub(self, rhs: Flint) -> Flint {
                let f = Flint::from(self);
                Flint {
                    a: pb(f.a - rhs.b),
                    b: na(f.b - rhs.a),
                }
            }
        }

        impl Mul<Flint> for $other_type {
            type Output = Flint;
            fn mul(self, rhs: Flint) -> Flint {
                let f = Flint::from(self);
                let (a, b) = fminmax4(
                    f.a * rhs.a,
                    f.a * rhs.b,
                    f.b * rhs.a,
                    f.b * rhs.b,
                );
                Flint { a: pb(a), b: na(b) }
            }
        }

        impl Div<Flint> for $other_type {
            type Output = Flint;
            fn div(self, rhs: Flint) -> Flint {
                let f = Flint::from(self);
                let (a, b) = fminmax4(
                    f.a / rhs.a,
                    f.a / rhs.b,
                    f.b / rhs.a,
                    f.b / rhs.b,
                );
                Flint { a: pb(a), b: na(b) }
            }
        }
    }
}

impl_arithmatic_ops!(f32);
impl_arithmatic_ops!(f64);
impl_arithmatic_ops!(i8);
impl_arithmatic_ops!(i16);
impl_arithmatic_ops!(i32);
impl_arithmatic_ops!(u8);
impl_arithmatic_ops!(u16);
impl_arithmatic_ops!(u32);
impl_arithmatic_ops!(i64);
impl_arithmatic_ops!(i128);
impl_arithmatic_ops!(isize);
impl_arithmatic_ops!(u64);
impl_arithmatic_ops!(u128);
impl_arithmatic_ops!(usize);

#[cfg(test)]
mod test {
    use crate::Flint;

    #[test]
    fn test_neg() {
        let p = Flint::from(1.0);
        let n = -p;
        assert_eq!(p.a, -n.b);
        assert_eq!(p.b, -n.a);
    }

    #[test]
    fn test_add() {
        // show classic failure of a float
        let a = 0.2;
        assert_ne!(0.6, a+a+a);
        // show working with a flint
        let a = Flint::from(0.2);
        assert_eq!(0.6, a+a+a);
        // test adding non-flint
        assert_eq!(0.4, a+0.2);
        assert_eq!(0.4, 0.2+a);
        // test in-place
        let mut a = 0.2;
        a += 0.2;
        a += 0.2;
        assert_ne!(0.6, a);
        let mut a = Flint::from(0.2);
        a += 0.2;
        a += 0.2;
        assert_eq!(0.6, a);
    }

    #[test]
    fn test_sub() {
        // show classic failure of a float
        let mut a = 10.0;
        for _ in 0..100 {
            a = a - 0.1;
        }
        // show working with a flint
        assert_ne!(0.0, a);
        let mut a = Flint::from(10.0);
        for _ in 0..100 {
            a = a - 0.1;
        }
        assert_eq!(0.0, a);
        // test subtracking non-floats
        let a = Flint::from(0.4);
        assert_eq!(0.2, a - 0.2 );
        let a = Flint::from(0.2);
        assert_eq!(0.2, 0.4 - a );
        // test in-place
        let mut a = 10.0;
        for _ in 0..100 {
            a -= 0.1;
        }
        assert_ne!(0.0, a);
        let mut a = Flint::from(10.0);
        for _ in 0..100 {
            a -= 0.1;
        }
        assert_eq!(0.0, a);
    }

    #[test]
    fn test_mul() {
        // show failure of float
        let a = 2.2; 
        assert_ne!(6.6, 3.0*a);
        // show working with flint
        let a = Flint::from(2.2);
        assert_eq!(6.6, a*3.0);
        // show working with non-float
        assert_eq!(6.6, 3*a);
        // show in-place
        let mut a = 2.2;
        a *= 3.0;
        assert_ne!(6.6, a);
        let mut a = Flint::from(2.2);
        a *= 3.0;
        assert_eq!(6.6, a);
    }

    #[test]
    fn test_div() {
        // show failure of float
        let a = 6.6;
        assert_ne!(2.2, a/3.0);
        // show working with flint
        let a = Flint::from(6.6);
        assert_eq!(2.2, a/3.0);
        // show with non-floats
        let a = Flint::from(3.0);
        assert_eq!(2.2, 6.6/a);
        // show in-place
        let mut a = 6.6;
        a /= 3.0;
        assert_ne!(2.2, a);
        let mut a = Flint::from(6.6);
        a /= 3.0;
        assert_eq!(2.2, a);
    }
}