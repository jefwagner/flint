use crate::utils::*;
use crate::flint::Flint;

macro_rules! impl_monotonic {
    ($func:ident) => {
        pub fn $func(self) -> Flint {
            Flint { a: pb(pb(self.a.$func())), b: na(na(self.b.$func())) }
        }
    }
}

macro_rules! impl_logfunc {
    ($func:ident, $min:expr) => {
        pub fn $func(self) -> Flint {
            if self.b < $min {
                Flint { a: f64::NAN, b: f64::NAN }
            } else if self.a < $min {
                Flint { a: f64::NEG_INFINITY, b: na(self.b.$func()) }
            } else {
                Flint {
                    a: pb(self.a.$func()),
                    b: na(self.b.$func()),
                }
            }
        }
    }
}

impl Flint {
    // Queries for special values
    pub fn is_nan(&self) -> bool {
        self.a.is_nan() || self.b.is_nan()
    }

    pub fn is_infinite(&self) -> bool {
        self.a.is_infinite() || self.b.is_infinite()
    }

    pub fn is_finite(&self) -> bool {
        self.a.is_finite() && self.b.is_finite()
    }

    pub fn nonzero(&self) -> bool {
        *self != 0
    }

    // General power functions
    pub fn powf<T: Into<Flint>>(self, other: T) -> Flint {
        let other = Into::<Flint>::into(other);
        let (a, b) = fminmax4(
            self.a.powf(other.a),
            self.a.powf(other.b),
            self.b.powf(other.a),
            self.b.powf(other.b),
        );
        if a.is_nan() || b.is_nan() {
            Flint { a: f64::NAN, b: f64::NAN }
        } else {
            Flint { a: pb(pb(a)), b: na(na(b)) }
        }
    }

    pub fn abs(&self) -> Flint {
        if self.b < 0.0 {
            -(*self)
        } else if self.a < 0.0 {
            Flint {
                a: 0.0,
                b: if -self.a > self.b { -self.a } else { self.b }
            }
        } else {
            *self
        }
    }

    pub fn sqrt(&self) -> Flint {
        if self.b < 0.0 {
            Flint { a: f64::NAN, b: f64::NAN }
        } else if self.a < 0.0 {
            Flint { a: 0.0, b: na(self.b.sqrt()) }
        } else {
            Flint { a: pb(self.a.sqrt()), b: na(self.b.sqrt()) }
        }
    }

    impl_monotonic!(cbrt);

    pub fn hypot<T: Into<Flint>>(self, other: T) -> Flint {
        let other = Into::<Flint>::into(other);
        // get values that give min and max outputs with respect to self
        let (f1a, f1b) = if self.a < 0.0 {
            if self.b < 0.0 {
                (self.b, self.a)
            } else {
                (0.0, if -self.a > self.b { -self.a } else { self.b })
            }
        } else {
            (self.a, self.b)
        };
        // get values that give min and max outputs with repect to other
        let (f2a, f2b) = if other.a < 0.0 {
            if other.b < 0.0 {
                (other.b, other.a)
            } else {
                (0.0, if -other.a > other.b { -other.a } else { other.b })
            }
        } else {
            (other.a, other.b)
        };
        let a = f1a.hypot(f2a);
        let a = if a == 0.0 { 0.0 } else { pb(pb(a)) };
        Flint { a: a, b: na(na(f1b.hypot(f2b))) }
    }

    // Exponential functions
    impl_monotonic!(exp);
    impl_monotonic!(exp2);
    impl_monotonic!(exp_m1);

    // Log functions
    pub fn log<T: Into<Flint>>(self, other: T) -> Flint {
        let other = Into::<Flint>::into(other);
        let fnan = Flint { a: f64::NAN, b: f64::NAN };
        if self.b < 0.0 {
            fnan
        } else if self.a < 0.0 {
            let (_, b) = fsort2( self.b.log(other.a), self.b.log(other.b) );
            if b.is_nan() {
                fnan
            } else {
                Flint { a: 0.0, b: na(na(b)) }
            }
        } else {
            let (a, b) = fminmax4(
                self.a.log(other.a),
                self.a.log(other.b),
                self.b.log(other.a),
                self.b.log(other.b),
            );
            if a.is_nan() || b.is_nan() {
                Flint { a: f64::NAN, b: f64::NAN }
            } else {
                Flint { a: pb(pb(a)), b: na(na(b)) }
            }
        }
    }

    impl_logfunc!(ln, 0.0);
    impl_logfunc!(log10, 0.0);
    impl_logfunc!(log2, 0.0);
    impl_logfunc!(ln_1p, -1.0);

    // Trig functions
    pub fn sin(&self) -> Flint {
        let tau = std::f64::consts::TAU;
        let pi_2 = std::f64::consts::FRAC_PI_2;
        let n = (self.a/tau).floor();
        let da = self.a - n*tau;
        let db = self.b - n*tau;
        let (mut sa, mut sb) = fsort2(self.a.sin(), self.b.sin());
        sa = pb(pb(sa));
        sb = na(na(sb));
        if da <= pi_2 && db > pi_2 {
            sb = 1.0;
        } else if da <= 3.0*pi_2 {
            if db > 3.0*pi_2 {
                sa = -1.0;
            } 
            if db > 5.0*pi_2 {
                sb = 1.0;
            }
        } else {
            if db > 5.0*pi_2 {
                sb = 1.0;
            }
            if db > 7.0*pi_2 {
                sa = -1.0;
            }
        }
        Flint { a: sa, b: sb }
    }

    pub fn cos(&self) -> Flint {
        let tau = std::f64::consts::TAU;
        let pi = std::f64::consts::PI;
        let n = (self.a/tau).floor();
        let da = self.a - n*tau;
        let db = self.b - n*tau;
        let (mut ca, mut cb) = fsort2(self.a.cos(), self.b.cos());
        ca = pb(pb(ca));
        cb = na(na(cb));
        if da <= pi && db >= pi {
            ca = -1.0;
            if db > tau {
                cb = 1.0;
            }
        } else {
            if db > tau {
                cb = 1.0;
            }
            if db > 3.0*pi {
                ca = -1.0;
            }
        }
        Flint { a: ca, b: cb }
    }

    pub fn tan(&self) -> Flint {
        let pi = std::f64::consts::PI;
        let ta = self.a.tan();
        let tb = self.b.tan();
        if ta > tb || self.width() > pi {
            Flint { a: f64::INFINITY, b: f64::NEG_INFINITY }
        } else {
            Flint { a: pb(pb(ta)), b: na(na(tb)) }
        }
    }

    // Inverse trig functions
    pub fn asin(&self) -> Flint {
        let pi_2 = std::f64::consts::FRAC_PI_2;
        if self.b < -1.0 || self.a > 1.0 {
            // whole interval out of domain? NAN
            Flint { a: f64::NAN, b: f64::NAN }
        } else {
            // lower interval out of domain?
            let a = if self.a < -1.0 {
                -pi_2
            } else {
                pb(pb(self.a.asin()))
            };
            let b = if self.b > 1.0 {
                pi_2
            } else {
                na(na(self.b.asin()))
            };
            Flint { a, b }
        }
    }

    pub fn acos(&self) -> Flint {
        let pi = std::f64::consts::PI;
        if self.b < -1.0 || self.a > 1.0 {
            // whole interval out of domain? NAN
            Flint { a: f64::NAN, b: f64::NAN }
        } else {
            // lower interval out of domain?
            let b = if self.a < -1.0 {
                pi
            } else {
                na(na(self.a.acos()))
            };
            let a = if self.b > 1.0 {
                0.0
            } else {
                pb(pb(self.a.acos()))
            };
            Flint { a, b }
        }
    }

    impl_monotonic!(atan);

    pub fn atan2<T: Into<Flint>>(self, other: T) -> Flint {
        let tau = std::f64::consts::TAU;
        let pi = std::f64::consts::PI;
        let y = self;
        let x: Flint = other.into();
        let a: f64;
        let b: f64;
        if y.a > 0.0 {
            // y > 0
            if x.a > 0.0 {
                // x > 0 
                a = y.a.atan2(x.b);
                b = y.b.atan2(x.a);
            } else if x.b > 0.0 {
                // x = 0 (positive y axis)
                a = y.a.atan2(x.b);
                b = y.a.atan2(x.a);
            } else {
                // x > 0
                a = y.b.atan2(x.b);
                b = y.a.atan2(x.a);
            }
        } else if y.b > 0.0 {
            // y interval cross zero
            if x.a > 0.0 {
                // x > 0 (positive x axis)
                a = y.a.atan2(x.a);
                b = y.b.atan2(x.a);
            } else if x.b > 0.0 {
                // product of x and y intervals contain origin
                // The angle is undefined in this case, so we return a maximal
                // interval that contains the entire -pi to pi.
                a = -pi;
                b = pi;
            } else {
                // x < 0 (negative x axis)
                // this interval contains the jump of the output from -pi to pi,
                // so the 'greater' angle would be greater than pi, so it wraps
                // around to -pi, we want to move it back, so we add 2pi=tau
                a = y.b.atan2(x.b);
                b = y.a.atan2(x.b)+tau;
            }
        } else {
            // y < 0
            if x.a > 0.0 {
                // x > 0
                a = y.a.atan2(x.a);
                b = y.b.atan2(x.b);
            } else if x.b > 0.0 {
                // x = 0 (negative y axis)
                a = y.b.atan2(x.a);
                b = y.b.atan2(x.b);
            } else {
                // x < 0
                a = y.b.atan2(x.a);
                b = y.a.atan2(x.b);
            }
        }
        Flint { a: pb(pb(a)), b: na(na(b)) }
    }

    // Hyperbolic trig functions
    impl_monotonic!(sinh);

    pub fn cosh(&self) -> Flint {
        let (a, b) = fsort2(self.a.cosh(), self.b.cosh());
        let a = if self.a <= 0.0 && self.b >= 0.0 {
            1.0
        } else {
            pb(pb(a))
        };
        let b = na(na(b));
        Flint { a, b }
    }

    impl_monotonic!(tanh);

    // Inverse hyperbolic trig
    impl_monotonic!(asinh);

    pub fn acosh(&self) -> Flint {
        if self.b < 1.0 {
            Flint { a: f64::NAN, b: f64::NAN }
        } else if self.a < 1.0 {
            Flint { a: 0.0, b: na(na(self.b.acosh())) }
        } else {
            Flint { 
                a: pb(pb(self.a.acosh())),
                b: na(na(self.b.acosh())),
            }
        }
    }

    pub fn atanh(&self) -> Flint {
        if self.b < -1.0 || self.a > 1.0 {
            Flint { a: f64::NAN, b: f64::NAN }
        } else {
            let a = if self.a < -1.0 {
                f64::NEG_INFINITY
            } else {
                pb(pb(self.a.atanh()))
            };
            let b = if self.b > 1.0 {
                f64::INFINITY
            } else {
                na(na(self.b.atanh()))
            };
            Flint { a, b }
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_special_values() {
        let f = Flint::from(0.0);
        assert!(!f.nonzero());
        assert!(!f.is_nan());
        assert!(f.is_finite());
        assert!(!f.is_infinite());
        let f = Flint::from(1.0);
        assert!(f.nonzero());
        assert!(!f.is_nan());
        assert!(f.is_finite());
        assert!(!f.is_infinite());
        let f = Flint { a: 0.0, b: f64::INFINITY };
        assert!(!f.nonzero());
        assert!(!f.is_nan());
        assert!(!f.is_finite());
        assert!(f.is_infinite());
        let f = Flint { a: f64::NAN, b: f64::NAN };
        assert!(f.nonzero());
        assert!(f.is_nan());
        assert!(!f.is_finite());
        assert!(!f.is_infinite());
    }

    #[test]
    fn test_abs() {
        let x = Flint::from(-2);
        let y = x.abs();
        assert_eq!(x.width(), y.width());
        assert_eq!(2, y);
        let x = Flint::from(-1.5);
        let y = x.abs();
        assert_eq!(x.width(), y.width());
        assert_eq!(1.5, y);
        let x = Flint::from(1.5);
        let y = x.abs();
        assert_eq!(x.width(), y.width());
        assert_eq!(1.5, y);
        let x = Flint { a: -1.0, b: 1.0 };
        let y = x.abs();
        assert_eq!(0.0, y.a);
        assert_eq!(1.0, y.b);
    }

    #[test]
    fn test_pow() {
        let x = Flint::from(4);
        assert_eq!(16, x.powf(2));
        assert_eq!(2, x.powf(0.5));
        let x = Flint::from(-1);
        assert!(x.powf(0.5).is_nan());
    }

    #[test]
    fn test_sqrt() {
        let x = Flint::from(4);
        let y = x.sqrt();
        assert!( y.width() > 0.0 );
        assert_eq!(2, y);
        let x = Flint { a: -1.0, b: 4.0 };
        let y = x.sqrt();
        assert_eq!(0.0, y.a);
        assert!((y.b - 2.0) < 1.0e-10);
        let x = Flint::from(-14);
        let y = x.sqrt();
        assert!( y.is_nan() );
    }

    #[test]
    fn test_cbrt() {
        let x = Flint::from(8);
        let y = x.cbrt();
        assert!(y.width() > 0.0);
        assert_eq!(2, y);
    }

    #[test]
    fn test_hypot() {
        let a = Flint::from(0.6);
        let b = Flint::from(0.8);
        assert_eq!(1, a.hypot(b));
        assert_eq!(1, b.hypot(a));
        //
        assert_eq!(1, (-a).hypot(b));
        assert_eq!(1, a.hypot(-b));
        assert_eq!(1, (-a).hypot(-b));

        let a = Flint { a: -1.0, b: 1.0 };
        let c = a.hypot(1);
        assert_eq!(1, c);
        let sqrt2 = std::f64::consts::SQRT_2;
        assert!( c.a < 1.0 );
        assert!( (1.0 - c.a) < 1.0e-10 );
        assert!( c.b > sqrt2 );
        assert!( (c.b - sqrt2) < 1.0e-10 );
        let c = a.hypot(a);
        assert_eq!(0.0, c.a);
        assert!( c.b > sqrt2 );
        assert!( (c.b - sqrt2) < 1.0e-10 );
    }

    #[test]
    fn test_exp() {
        let x = Flint::from(1);
        assert_eq!(std::f64::consts::E, x.exp());
    }

    #[test]
    fn test_exp2() {
        let x = Flint::from(0.5);
        assert_eq!(std::f64::consts::SQRT_2, x.exp2());
    }

    #[test]
    fn test_exp_m1() {
        let x = Flint::from(1.0e-15);
        assert_eq!(1.0e-15, x.exp_m1());
    }

    #[test]
    fn test_ln() {
        let x = Flint::from(std::f64::consts::E);
        assert_eq!(1.0, x.ln());
        let x = Flint { a: -1.0e-8, b: 1.0e-8 };
        assert!(x.ln().is_infinite());
        let x = Flint::from(-1.0);
        assert!(x.ln().is_nan());
    }

    #[test]
    fn test_log10() {
        let x = Flint::from(1.0e-15);
        assert_eq!(-15, x.log10());
        let x = Flint { a: -1.0e-8, b: 1.0e-8 };
        assert!(x.log10().is_infinite());
        let x = Flint::from(-1.0);
        assert!(x.log10().is_nan());
    }

    #[test]
    fn test_log2() {
        let x = Flint::from( 1 << 25 );
        assert_eq!(25, x.log2());
        let x = Flint { a: -1.0e-8, b: 1.0e-8 };
        assert!(x.log2().is_infinite());
        let x = Flint::from(-1.0);
        assert!(x.log2().is_nan());
    }

    #[test]
    fn test_ln_1p() {
        let x = Flint::from(1.0e-15);
        assert_eq!(1.0e-15, x.ln_1p());
        let x = Flint::from(-1.0);
        assert!(x.ln_1p().is_infinite());
        let x = Flint::from(-2.0);
        assert!(x.ln_1p().is_nan());
    }

    #[test]
    fn test_log() {
        let x = Flint::from(9);
        assert_eq!(2, x.log(3));
        let x = Flint::from(-1);
        assert!(x.log(10).is_nan());
        let x = Flint { a: -0.1, b: 0.1 };
        assert!(x.log(-1).is_nan());
        assert_eq!(1, x.log(0.1));
        let x = Flint::from(1);
        assert!(x.log(-1).is_nan());
    }

    #[test]
    fn test_sin() {
        // test expected value
        let x = Flint::from(std::f64::consts::PI);
        assert_eq!(0, x.sin());
        // test monotonic increasing
        let x = Flint::from(0.5);
        let y = x.sin();
        assert!( y.b > y.a );
        // test monotonic decreasing
        let x = Flint::from(3.5);
        let y = x.sin();
        assert!( y.b > y.a );
        // ranging over a pi/2
        let x = Flint { a: 1.0, b: 2.0 };
        let y = x.sin();
        assert_eq!(1.0, y.b);
        assert!( (1.0f64.sin() - y.a) < 1.0e-10 );
        // ranging over 3pi/2
        let x = Flint { a: 4.0, b: 5.0 };
        let y = x.sin();
        assert_eq!(-1.0, y.a);
        assert!( (y.b - 4.0f64.sin()) < 1.0e-10 );
        // ranging over a 5pi/2
        let x = Flint { a: 6.0, b: 8.0 };
        let y = x.sin();
        assert_eq!(1.0, y.b);
        assert!( (6.0f64.sin() - y.a) < 1.0e-10 );
        // ranging over a 3pi/2 and 5pi/2
        let x = Flint { a: 4.0, b: 8.0 };
        let y = x.sin();
        assert_eq!(1.0, y.b);
        assert_eq!(-1.0, y.a);
        // ranging over a 3pi/2 and 5pi/2
        let x = Flint { a: 6.0, b: 11.0 };
        let y = x.sin();
        assert_eq!(1.0, y.b);
        assert_eq!(-1.0, y.a);
    }

    #[test]
    fn test_cos() {
        // test expected value
        let x = Flint::from(std::f64::consts::FRAC_PI_2);
        assert_eq!(0, x.cos());
        // test monotonic decreasing
        let x = Flint::from(1.5);
        let y = x.cos();
        assert!( y.a < y.b );
        // test monotonic increasing
        let x = Flint::from(4.5);
        let y = x.cos();
        assert!( y.a < y.b );
        // over minimum around pi
        let x = Flint {a: 3.0, b: 3.5};
        let y = x.cos();
        assert_eq!(-1.0, y.a);
        // over maximum round 0
        let x = Flint { a: -0.1, b: 0.1 };
        let y = x.cos();
        assert_eq!(1.0, y.b);
        // over maximum around 2pi
        let x = Flint { a: 6.2, b: 6.3 };
        let y = x.cos();
        assert_eq!(1.0, y.b);
        // over min and max
        let x = Flint { a: 3.1, b: 6.3 };
        let y = x.cos();
        assert_eq!(-1.0, y.a);
        assert_eq!(1.0, y.b);
        let x = Flint { a: 6.2, b: 9.5 };
        let y = x.cos();
        assert_eq!(-1.0, y.a);
        assert_eq!(1.0, y.b);
    }

    #[test]
    fn test_tan() {
        let x = Flint::from(std::f64::consts::FRAC_PI_4);
        let y = x.tan();
        assert_eq!(1, y);
        let x = Flint { a: 1.5, b: 1.6 };
        let y = x.tan();
        assert!(y.is_infinite());
    }

    #[test]
    fn test_asin() {
        let x = Flint::from(0.5);
        let y = x.asin();
        assert_eq!(std::f64::consts::FRAC_PI_6, y);
        let x = Flint { a: -1.1, b: -0.9 };
        let y = x.asin();
        assert_eq!(-std::f64::consts::FRAC_PI_2, y.a);
        let x = Flint { a: 0.9, b: 1.1 };
        let y = x.asin();
        assert_eq!(std::f64::consts::FRAC_PI_2, y.b);
        let x = Flint::from(-1.1);
        let y = x.asin();
        assert!(y.is_nan());
        let x = Flint::from(1.1);
        let y = x.asin();
        assert!(y.is_nan());
        let x = Flint { a: -1.1, b: 1.1 };
        let y = x.asin();
        assert_eq!(-std::f64::consts::FRAC_PI_2, y.a);
        assert_eq!(std::f64::consts::FRAC_PI_2, y.b);
    }

    #[test]
    fn test_acos() {
        let x = Flint::from(0.5);
        let y = x.acos();
        assert_eq!(std::f64::consts::FRAC_PI_3, y);
        let x = Flint { a: -1.1, b: -0.9 };
        let y = x.acos();
        assert_eq!(std::f64::consts::PI, y.b);
        let x = Flint { a: 0.9, b: 1.1 };
        let y = x.acos();
        assert_eq!(0.0, y.a);
        let x = Flint::from(-1.1);
        let y = x.acos();
        assert!(y.is_nan());
        let x = Flint::from(1.1);
        let y = x.acos();
        assert!(y.is_nan());
        let x = Flint { a: -1.1, b: 1.1 };
        let y = x.acos();
        assert_eq!(0.0, y.a);
        assert_eq!(std::f64::consts::PI, y.b);
    }

    #[test]
    fn test_atan() {
        let x = Flint::from(1);
        let y = x.atan();
        assert_eq!(std::f64::consts::FRAC_PI_4, y);
    }

    #[test]
    fn test_atan2() {
        // Check all quadrants and where intervals overlap axes
        let zero = Flint { a: -1.0e-8, b: 1.0e-8 };
        let one = Flint::from(1.0);
        // positive +x axis
        let z = zero.atan2(one);
        assert!( z.b > z.a ); // always confirm invariant is held
        assert_eq!(0, z);
        // quadrant I (+x, +y)
        let z = one.atan2(one);
        assert!( z.b > z.a );
        assert_eq!(std::f64::consts::FRAC_PI_4, z);
        // positive +y
        let z = one.atan2(zero);
        assert!( z.b > z.a );
        assert_eq!(std::f64::consts::FRAC_PI_2, z);
        // quadrant II (-x, +y)
        let z = one.atan2(-one);
        assert!( z.b > z.a );
        assert_eq!(3.0*std::f64::consts::FRAC_PI_4, z);
        // negative -x
        let z = zero.atan2(-one);
        assert!( z.b > z.a );
        assert_eq!(std::f64::consts::PI, z);
        // quadrant III (-x, -y)
        let z = (-one).atan2(-one);
        assert!( z.b > z.a );
        assert_eq!(-3.0*std::f64::consts::FRAC_PI_4, z);
        // negative -y
        let z = (-one).atan2(zero);
        assert!( z.b > z.a );
        assert_eq!(-std::f64::consts::FRAC_PI_2, z);
        // quadrant IV (+x, -y)
        let z = (-one).atan2(one);
        assert!( z.b > z.a );
        assert_eq!(-std::f64::consts::FRAC_PI_4, z);
        // test origin
        let z = zero.atan2(zero);
        assert!( std::f64::consts::TAU < z.width());
    }

    #[test]
    fn test_sinh() {
        let e = std::f64::consts::E;
        let res = 0.5*(e-1.0/e);
        let x = Flint::from(1);
        let y = x.sinh();
        assert_eq!(res, y);
        let y = (-x).sinh();
        assert_eq!(-res, y);
    }

    #[test]
    fn test_cosh() {
        let e = std::f64::consts::E;
        let res = 0.5*(e+1.0/e);
        let x = Flint::from(1);
        let y = x.cosh();
        assert_eq!(res, y);
        let y = (-x).cosh();
        assert_eq!(res, y);
        let zero = Flint { a: -1.0e-8, b: 1.0e-8 };
        let y = zero.cosh();
        assert_eq!(1.0, y.a);
        assert_eq!(1.0, y);
    }

    #[test]
    fn test_tanh() {
        let e = std::f64::consts::E;
        let res = (e-1.0/e)/(e+1.0/e);
        let x = Flint::from(1);
        assert_eq!(res, x.tanh());
        assert_eq!(-res, (-x).tanh());
    }

    #[test]
    fn test_asinh() {
        let e = std::f64::consts::E;
        let e = Flint::from(e);
        let x: Flint = 0.5*(e - 1/e);
        assert_eq!(1, x.asinh());
        assert_eq!(-1, -(x).asinh());
    }

    #[test]
    fn test_acosh() {
        let e = std::f64::consts::E;
        let e = Flint::from(e);
        let x: Flint = 0.5*(e + 1/e);
        assert_eq!(1, x.acosh());
        let x = Flint::from(1.0);
        assert_eq!(0, x.acosh());
        let x = Flint::from(0);
        assert!(x.acosh().is_nan());
    }

    #[test]
    fn test_atanh() {
        let e = std::f64::consts::E;
        let e = Flint::from(e);
        let x: Flint = (e - 1/e)/(e + 1/e);
        assert_eq!(1, x.atanh());
        let x = Flint::from(1.0);
        assert!(x.atanh().is_infinite());
        assert!((-x).atanh().is_infinite());
        let x = Flint::from(1.1);
        assert!(x.atanh().is_nan());
        assert!((-x).atanh().is_nan());
    }

}
