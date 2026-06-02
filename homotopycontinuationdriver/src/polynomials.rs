use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

const RECIPROCAL_EPSILON: f64 = 1e-30;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ComplexNumber {
    pub real: f64,
    pub imag: f64,
}

impl ComplexNumber {
    pub const ZERO: Self = Self::new(0.0, 0.0);
    pub const ONE: Self = Self::new(1.0, 0.0);

    pub const fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    pub fn norm(self) -> f64 {
        self.norm_squared().sqrt()
    }

    pub fn norm_squared(self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }

    pub fn reciprocal(self) -> Option<Self> {
        let denominator = self.norm_squared();
        if !denominator.is_finite() || denominator <= RECIPROCAL_EPSILON {
            None
        } else {
            Some(Self {
                real: self.real / denominator,
                imag: -self.imag / denominator,
            })
        }
    }

    pub fn approx_eq(self, other: Self, tolerance: f64) -> bool {
        (self - other).norm() <= tolerance
    }
}

impl Add for ComplexNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl Sub for ComplexNumber {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl Mul for ComplexNumber {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

impl Div for ComplexNumber {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self * other
            .reciprocal()
            .expect("attempted to divide by a near-zero complex number")
    }
}

impl Neg for ComplexNumber {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

impl Mul<f64> for ComplexNumber {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            real: self.real * scalar,
            imag: self.imag * scalar,
        }
    }
}

impl Mul<ComplexNumber> for f64 {
    type Output = ComplexNumber;

    fn mul(self, value: ComplexNumber) -> ComplexNumber {
        value * self
    }
}

impl From<f64> for ComplexNumber {
    fn from(value: f64) -> Self {
        Self::new(value, 0.0)
    }
}

impl fmt::Display for ComplexNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imag == 0.0 {
            write!(f, "{}", self.real)
        } else if self.real == 0.0 {
            write!(f, "{}i", self.imag)
        } else if self.imag < 0.0 {
            write!(f, "{} - {}i", self.real, -self.imag)
        } else {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Polynomial {
    pub coefficients: Vec<ComplexNumber>,
}

impl Polynomial {
    pub fn new<T: Into<ComplexNumber>>(coeffs: Vec<T>) -> Self {
        let mut polynomial = Self {
            coefficients: coeffs.into_iter().map(Into::into).collect(),
        };
        polynomial.trim();
        polynomial
    }

    pub fn from_complex(coeffs: Vec<ComplexNumber>) -> Self {
        let mut polynomial = Self {
            coefficients: coeffs,
        };
        polynomial.trim();
        polynomial
    }

    pub fn zero() -> Self {
        Self::new(vec![0.0])
    }

    pub fn degree(&self) -> usize {
        self.coefficients.len().saturating_sub(1)
    }

    pub fn leading_coefficient(&self) -> ComplexNumber {
        *self.coefficients.last().unwrap_or(&ComplexNumber::ZERO)
    }

    pub fn is_zero(&self) -> bool {
        self.coefficients
            .iter()
            .all(|coefficient| coefficient.norm() <= 1e-14)
    }

    pub fn derivative(&self) -> Self {
        if self.coefficients.len() <= 1 {
            return Self::zero();
        }

        let coefficients = self
            .coefficients
            .iter()
            .enumerate()
            .skip(1)
            .map(|(power, coefficient)| *coefficient * power as f64)
            .collect();
        Self::from_complex(coefficients)
    }

    pub fn evaluate(&self, z: ComplexNumber) -> ComplexNumber {
        self.coefficients
            .iter()
            .rev()
            .fold(ComplexNumber::ZERO, |accumulator, coefficient| {
                accumulator * z + *coefficient
            })
    }

    pub fn scale_complex(&self, scalar: ComplexNumber) -> Self {
        Self::from_complex(
            self.coefficients
                .iter()
                .map(|coefficient| *coefficient * scalar)
                .collect(),
        )
    }

    pub fn scale_real(&self, scalar: f64) -> Self {
        self.scale_complex(ComplexNumber::from(scalar))
    }

    pub fn start_system(degree: usize) -> Result<Self, PolynomialError> {
        if degree == 0 {
            return Err(PolynomialError::ConstantPolynomial);
        }

        let mut coefficients = vec![ComplexNumber::ZERO; degree + 1];
        coefficients[0] = ComplexNumber::new(-1.0, 0.0);
        coefficients[degree] = ComplexNumber::ONE;
        Ok(Self::from_complex(coefficients))
    }

    fn trim(&mut self) {
        while self.coefficients.len() > 1 {
            let last = *self.coefficients.last().expect("nonempty coefficients");
            if last.norm() > 1e-14 {
                break;
            }
            self.coefficients.pop();
        }

        if self.coefficients.is_empty() {
            self.coefficients.push(ComplexNumber::ZERO);
        }
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let len = self.coefficients.len().max(other.coefficients.len());
        let mut coefficients = Vec::with_capacity(len);
        for index in 0..len {
            let left = self
                .coefficients
                .get(index)
                .copied()
                .unwrap_or(ComplexNumber::ZERO);
            let right = other
                .coefficients
                .get(index)
                .copied()
                .unwrap_or(ComplexNumber::ZERO);
            coefficients.push(left + right);
        }
        Self::from_complex(coefficients)
    }
}

impl Sub for Polynomial {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let len = self.coefficients.len().max(other.coefficients.len());
        let mut coefficients = Vec::with_capacity(len);
        for index in 0..len {
            let left = self
                .coefficients
                .get(index)
                .copied()
                .unwrap_or(ComplexNumber::ZERO);
            let right = other
                .coefficients
                .get(index)
                .copied()
                .unwrap_or(ComplexNumber::ZERO);
            coefficients.push(left - right);
        }
        Self::from_complex(coefficients)
    }
}

impl Mul<f64> for Polynomial {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        self.scale_real(scalar)
    }
}

impl Mul<ComplexNumber> for Polynomial {
    type Output = Self;

    fn mul(self, scalar: ComplexNumber) -> Self {
        self.scale_complex(scalar)
    }
}

impl Mul<Polynomial> for ComplexNumber {
    type Output = Polynomial;

    fn mul(self, polynomial: Polynomial) -> Polynomial {
        polynomial.scale_complex(self)
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }

        let mut first = true;
        for (power, coefficient) in self.coefficients.iter().enumerate() {
            if coefficient.norm() <= 1e-14 {
                continue;
            }

            if !first {
                write!(f, " + ")?;
            }
            first = false;

            if power == 0 {
                write!(f, "{coefficient}")?;
            } else if power == 1 {
                write!(f, "({coefficient})*z")?;
            } else {
                write!(f, "({coefficient})*z^{power}")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PolynomialError {
    ConstantPolynomial,
}

impl fmt::Display for PolynomialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConstantPolynomial => write!(f, "constant polynomials do not have path starts"),
        }
    }
}

impl std::error::Error for PolynomialError {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NewtonResult {
    pub point: ComplexNumber,
    pub residual: f64,
    pub iterations: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NewtonError {
    SingularDerivative {
        iteration: usize,
        point: ComplexNumber,
        derivative: ComplexNumber,
    },
    DidNotConverge {
        iterations: usize,
        point: ComplexNumber,
        residual: f64,
    },
}

impl fmt::Display for NewtonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SingularDerivative {
                iteration,
                point,
                derivative,
            } => write!(
                f,
                "singular derivative during Newton correction at iteration {iteration}, point {point}, derivative {derivative}"
            ),
            Self::DidNotConverge {
                iterations,
                point,
                residual,
            } => write!(
                f,
                "Newton correction did not converge after {iterations} iterations; point {point}, residual {residual}"
            ),
        }
    }
}

impl std::error::Error for NewtonError {}

pub fn newton_corrector(
    polynomial: &Polynomial,
    starting_point: ComplexNumber,
    tolerance: f64,
    max_iterations: usize,
) -> Result<NewtonResult, NewtonError> {
    let mut point = starting_point;

    for iteration in 0..=max_iterations {
        let residual_value = polynomial.evaluate(point);
        let residual = residual_value.norm();
        if residual <= tolerance {
            return Ok(NewtonResult {
                point,
                residual,
                iterations: iteration,
            });
        }

        if iteration == max_iterations {
            break;
        }

        let derivative = polynomial.derivative().evaluate(point);
        let inverse_guess = derivative
            .reciprocal()
            .ok_or(NewtonError::SingularDerivative {
                iteration,
                point,
                derivative,
            })?;

        point = point - residual_value * inverse_guess;
    }

    let residual = polynomial.evaluate(point).norm();
    Err(NewtonError::DidNotConverge {
        iterations: max_iterations,
        point,
        residual,
    })
}

pub fn roots_of_unity(n: usize) -> Vec<ComplexNumber> {
    (0..n)
        .map(|index| {
            let angle = 2.0 * std::f64::consts::PI * index as f64 / n as f64;
            ComplexNumber::new(angle.cos(), angle.sin())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complex_arithmetic_and_reciprocal_work() {
        let a = ComplexNumber::new(3.0, 2.0);
        let b = ComplexNumber::new(1.0, -4.0);

        assert!((a + b).approx_eq(ComplexNumber::new(4.0, -2.0), 1e-12));
        assert!((a - b).approx_eq(ComplexNumber::new(2.0, 6.0), 1e-12));
        assert!((a * b).approx_eq(ComplexNumber::new(11.0, -10.0), 1e-12));
        assert!((a * a.reciprocal().unwrap()).approx_eq(ComplexNumber::ONE, 1e-12));
    }

    #[test]
    fn polynomial_addition_pads_mismatched_degrees() {
        let a = Polynomial::new(vec![1.0, 2.0, 3.0]);
        let b = Polynomial::new(vec![4.0, 5.0]);
        let sum = a + b;

        assert_eq!(sum.coefficients.len(), 3);
        assert!(sum.coefficients[0].approx_eq(ComplexNumber::new(5.0, 0.0), 1e-12));
        assert!(sum.coefficients[1].approx_eq(ComplexNumber::new(7.0, 0.0), 1e-12));
        assert!(sum.coefficients[2].approx_eq(ComplexNumber::new(3.0, 0.0), 1e-12));
    }

    #[test]
    fn horner_and_derivative_match_known_values() {
        let polynomial = Polynomial::new(vec![1.0, 2.0, 3.0]);
        let z = ComplexNumber::new(2.0, 0.0);
        assert!(
            polynomial
                .evaluate(z)
                .approx_eq(ComplexNumber::new(17.0, 0.0), 1e-12)
        );

        let derivative = polynomial.derivative();
        assert!(
            derivative
                .evaluate(z)
                .approx_eq(ComplexNumber::new(14.0, 0.0), 1e-12)
        );
    }

    #[test]
    fn newton_corrector_converges_to_square_root() {
        let polynomial = Polynomial::new(vec![-1.0, 0.0, 1.0]);
        let result =
            newton_corrector(&polynomial, ComplexNumber::new(0.8, 0.1), 1e-10, 20).unwrap();

        assert!(result.point.approx_eq(ComplexNumber::ONE, 1e-8));
        assert!(result.residual <= 1e-10);
    }
}
