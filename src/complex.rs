#![macro_use]

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;

/// Holds a complex number with 64 bit float parts.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex {
    re: f64,
    im: f64
}

impl Complex {
    /// Construct a new complex number with 64 bit float parts.
    pub fn new(re: f64, im: f64) -> Complex {
        Complex {
            re: re,
            im: im
        }
    }

    /// Compute the square of the norm/absolute value, i.e. _|z|^2_.
    pub fn norm_sqr(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    /// Zero in the complex plane, i.e. 0 + 0i.
    pub fn zero() -> Complex {
        Complex::new(0f64, 0f64)
    }

    /// One in the complex plane, i.e. 1 + 0i.
    pub fn one() -> Complex {
        Complex::new(1f64, 0f64)
    }
}

/// Implement standard complex addition.
impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex {
        Complex::new(self.re + rhs.re, self.im + rhs.im)
    }
}

/// Implement standard complex multiplication.
impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Complex {
        Complex::new(self.re * rhs.re - self.im * rhs.im, self.re * rhs.im + self.im * rhs.re)
    }
}

/// Implement standard complex addition assignment.
impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Complex) {
        *self = *self + rhs;
    }
}

/// Convenience macro for complex number construction.
macro_rules! c {
    ($re:expr, $im:expr) => {
        Complex::new($re, $im)
    };
}

#[test]
fn complex_test() {
    assert_eq!(c![4f64, 6f64], c![1f64, 2f64] + c![3f64, 4f64]);
    assert_eq!(c![-5f64, 10f64], c![1f64, 2f64] * c![3f64, 4f64]);
    assert_eq!(5f64, c![1f64, 2f64].norm_sqr());

    let mut z = c![1f64, 2f64];
    z += c![3f64, 4f64];
    assert_eq!(z, c![4f64, 6f64]);
}