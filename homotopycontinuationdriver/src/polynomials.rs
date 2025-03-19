use core::f64;
use std::env::temp_dir;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Sub;

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

impl Div for ComplexNumber {
    type Output = ComplexNumber;

    fn div(self, other: ComplexNumber) -> ComplexNumber {
        let denominator = other.real.powi(2) + other.imag.powi(2);
        ComplexNumber {
            real: (self.real * other.real + self.imag * other.imag) / denominator,
            imag: (self.imag * other.real - self.real * other.imag) / denominator,
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

impl Sub for ComplexNumber {
    type Output = ComplexNumber;

    fn sub(self, other: ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            real: self.real - other.real,
            imag: self.imag - other.imag,
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
    pub fn evaluate(&self, z: &ComplexNumber) -> ComplexNumber {
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

/* -----------------------------------
Some functions that operate on Polynomials
-----------------------------------*/

// This is newtons method done without division, implemented according to [Blanchard, Chamberlin, 2023]
pub fn newton_corrector(h: Polynomial, point: ComplexNumber, tolerance: f64, initial_inverse_guess: ComplexNumber) -> ComplexNumber {
    let mut error = h.evaluate(&point).clone();
    let mut fixed_point = point.clone();
    let mut inverse_guess = initial_inverse_guess;
    while error.norm() > tolerance {
        let current_inverse_guess = inverse_guess.clone();

        // Break down the expression into smaller steps
        let derivative_at_fixed_point = h.derivative().evaluate(&fixed_point);
        let correction = ComplexNumber::new(2.0, 0.0) - current_inverse_guess.clone() * derivative_at_fixed_point;
        inverse_guess = current_inverse_guess * correction;


        fixed_point = fixed_point.clone() - h.evaluate(&fixed_point)*inverse_guess.clone();
        error = h.evaluate(&fixed_point);
    }
    fixed_point
}

pub fn roots_of_unity(n: i32) -> Vec<ComplexNumber> {
    let mut result = Vec::new();
    for i in 0..n {
        let angle = 2.0 * f64::consts::PI * i as f64 / n as f64;
        result.push(ComplexNumber::new(angle.cos(), angle.sin()));
    }
    result
}
