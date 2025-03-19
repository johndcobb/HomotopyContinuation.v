use core::f64;
use std::ops::Add;
use std::ops::Mul;

/* -----------------------------------
Complex Numbers
-----------------------------------*/

#[derive(Clone)]
pub struct ComplexNumber {
    pub real: f64,
    pub imag: f64,
}

impl ComplexNumber {
    pub fn new(real: f64, imag: f64) -> Self {
        ComplexNumber { real, imag }
    }
    
    pub fn norm(&self) -> f64 {
        (self.real.powi(2) + self.imag.powi(2)).sqrt()
    }
}

impl Add for ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, other: ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl Mul for ComplexNumber {
    type Output = ComplexNumber;

    fn mul(self, other: ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

// This allows for us to convert floats to a complex number with zero imaginary part.
impl From<f64> for ComplexNumber {
    fn from(value: f64) -> Self {
        Self::new(value, 0.0)
    }
}

impl std::fmt::Display for ComplexNumber{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.imag != 0.0 && self.real != 0.0 {
            write!(f, "{} + {}i", self.real, self.imag)
        } else if self.imag == 0.0 {
            write!(f, "{}", self.real)
        } else {
            write!(f, "{}i", self.imag)
        }
    }
} 

/* -----------------------------------
Polynomials 
-----------------------------------*/

#[derive(Clone)]
pub struct Polynomial {
    pub coefficients: Vec<ComplexNumber>, // The coefficients of the polynomial in increasing order.
}

impl Polynomial {
    pub fn new<T: Into<ComplexNumber> + Copy>(coeffs: Vec<T>) -> Self {
        Self {
            coefficients: coeffs.into_iter().map(|c| c.into()).collect(),
        }
    }

     pub fn derivative(&self) -> Polynomial {
        Polynomial {
            coefficients: self
                .coefficients
                .iter()
                .enumerate()
                .filter_map(
                    |(i,c)
                    | if i == 0 {
                        None
                    } else {
                        Some(c.clone() * ComplexNumber::new((i) as f64, 0.0)
                    )
                })
                .collect(),
        }
    }

    // This evaluates the polynomial using Horner's method.
    pub fn evaluate(&self, z: ComplexNumber) -> ComplexNumber {
        let mut result = ComplexNumber::new(0.0, 0.0);
        for c in self.coefficients.iter().rev() {
            result = result * z.clone() + c.clone();
        }
        result
    }
}


// Operations for Polynomials
impl Add for Polynomial {
    type Output = Polynomial;

    fn add(self, other: Polynomial) -> Polynomial {
        Polynomial {
            coefficients: self
                .coefficients
                .iter()
                .zip(other.coefficients.iter())
                .map(|(a, b)| a.clone() + b.clone())
                .collect(),
        }
    }
}

impl Mul<f64> for Polynomial {
    type Output = Polynomial;

    fn mul(self, scalar: f64) -> Polynomial {
        Polynomial {
            coefficients: self
                .coefficients
                .iter()
                .map(|c| ComplexNumber::new(c.real * scalar, c.imag * scalar))
                .collect(),
        }
    }
}

impl Mul<ComplexNumber> for Polynomial {
    type Output = Polynomial;

    fn mul(self, z: ComplexNumber) -> Polynomial {
        let mut result = Polynomial::new(vec![0.0]);
        for c in self.coefficients.iter().rev() {
            result = Polynomial {
                coefficients: vec![c.clone() * z.clone()],
            } + result * z.clone();
        }
        result
    }
}

impl Mul<Polynomial> for ComplexNumber {
    type Output = Polynomial;

    fn mul(self, poly: Polynomial) -> Polynomial {
        Polynomial {
            coefficients: poly
                .coefficients
                .iter()
                .map(|c| ComplexNumber::new(
                    self.real * c.real - self.imag * c.imag,
                    self.real * c.imag + self.imag * c.real,
                ))
                .collect(),
        }
    }
}

impl std::fmt::Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut first = true;
        for (i, c) in self.coefficients.iter().enumerate() {
            if c.real != 0.0 || c.imag != 0.0 {
                if first {
                    first = false;
                } else {
                    write!(f, " + ")?;
                }
                if c.imag == 0.0 {
                    write!(f, "{}", c.real)?;
                } else if c.real == 0.0 {
                    write!(f, "{}i", c.imag)?;
                } else {
                    write!(f, "({} + {}i)", c.real, c.imag)?;
                }
                if i > 0 {
                    write!(f, "*z^{}", i)?;
                }
            }
        }
        Ok(())
    }
}
