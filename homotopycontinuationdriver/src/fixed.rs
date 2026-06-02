use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::ops::{Add, Mul, Sub};
use std::path::Path;

use crate::{ComplexNumber, Polynomial};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FixedQ(i16);

impl FixedQ {
    pub const FRAC_BITS: i32 = 10;
    pub const SCALE: i32 = 1 << Self::FRAC_BITS;
    pub const MIN_RAW: i32 = i16::MIN as i32;
    pub const MAX_RAW: i32 = i16::MAX as i32;

    pub const fn from_raw(raw: i16) -> Self {
        Self(raw)
    }

    pub fn from_f64(value: f64) -> Self {
        Self::from_i64((value * Self::SCALE as f64).round() as i64)
    }

    pub const fn raw(self) -> i16 {
        self.0
    }

    pub fn to_f64(self) -> f64 {
        self.0 as f64 / Self::SCALE as f64
    }

    pub fn from_i64(value: i64) -> Self {
        Self(value.clamp(Self::MIN_RAW as i64, Self::MAX_RAW as i64) as i16)
    }

    fn rounded_shift(value: i64) -> i64 {
        let half = 1_i64 << (Self::FRAC_BITS - 1);
        if value >= 0 {
            (value + half) >> Self::FRAC_BITS
        } else {
            -(((-value) + half) >> Self::FRAC_BITS)
        }
    }
}

impl Add for FixedQ {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::from_i64(self.0 as i64 + other.0 as i64)
    }
}

impl Sub for FixedQ {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::from_i64(self.0 as i64 - other.0 as i64)
    }
}

impl Mul for FixedQ {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let product = self.0 as i64 * other.0 as i64;
        Self::from_i64(Self::rounded_shift(product))
    }
}

impl fmt::Display for FixedQ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.to_f64(), self.raw())
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FixedComplex {
    pub real: FixedQ,
    pub imag: FixedQ,
}

impl FixedComplex {
    pub const ZERO: Self = Self::new(FixedQ::from_raw(0), FixedQ::from_raw(0));
    pub const TWO: Self = Self::new(
        FixedQ::from_raw((2 * FixedQ::SCALE) as i16),
        FixedQ::from_raw(0),
    );

    pub const fn new(real: FixedQ, imag: FixedQ) -> Self {
        Self { real, imag }
    }

    pub fn from_complex(value: ComplexNumber) -> Self {
        Self {
            real: FixedQ::from_f64(value.real),
            imag: FixedQ::from_f64(value.imag),
        }
    }

    pub fn from_f64(real: f64, imag: f64) -> Self {
        Self::from_complex(ComplexNumber::new(real, imag))
    }

    pub fn to_complex(self) -> ComplexNumber {
        ComplexNumber::new(self.real.to_f64(), self.imag.to_f64())
    }
}

impl Add for FixedComplex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl Sub for FixedComplex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl Mul for FixedComplex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let ac = self.real * other.real;
        let bd = self.imag * other.imag;
        let ad = self.real * other.imag;
        let bc = self.imag * other.real;
        Self {
            real: ac - bd,
            imag: ad + bc,
        }
    }
}

#[derive(Clone, Debug)]
struct CoreVector {
    index: usize,
    fixed_a: FixedQ,
    fixed_b: FixedQ,
    complex_a: FixedComplex,
    complex_b: FixedComplex,
    horner_z: FixedComplex,
    horner_coefficients: [FixedComplex; 4],
    newton_point: FixedComplex,
    newton_h_value: FixedComplex,
    newton_derivative: FixedComplex,
    newton_inverse_guess: FixedComplex,
}

impl CoreVector {
    fn to_line(&self) -> String {
        let fixed_add = self.fixed_a + self.fixed_b;
        let fixed_sub = self.fixed_a - self.fixed_b;
        let fixed_mul = self.fixed_a * self.fixed_b;

        let complex_add = self.complex_a + self.complex_b;
        let complex_sub = self.complex_a - self.complex_b;
        let complex_mul = self.complex_a * self.complex_b;

        let horner = horner_fixed(self.horner_coefficients, self.horner_z);
        let (inverse_next, point_next) = newton_step_fixed(
            self.newton_point,
            self.newton_h_value,
            self.newton_derivative,
            self.newton_inverse_guess,
        );

        let values = [
            self.index as i16,
            self.fixed_a.raw(),
            self.fixed_b.raw(),
            fixed_add.raw(),
            fixed_sub.raw(),
            fixed_mul.raw(),
            self.complex_a.real.raw(),
            self.complex_a.imag.raw(),
            self.complex_b.real.raw(),
            self.complex_b.imag.raw(),
            complex_add.real.raw(),
            complex_add.imag.raw(),
            complex_sub.real.raw(),
            complex_sub.imag.raw(),
            complex_mul.real.raw(),
            complex_mul.imag.raw(),
            self.horner_z.real.raw(),
            self.horner_z.imag.raw(),
            self.horner_coefficients[0].real.raw(),
            self.horner_coefficients[0].imag.raw(),
            self.horner_coefficients[1].real.raw(),
            self.horner_coefficients[1].imag.raw(),
            self.horner_coefficients[2].real.raw(),
            self.horner_coefficients[2].imag.raw(),
            self.horner_coefficients[3].real.raw(),
            self.horner_coefficients[3].imag.raw(),
            horner.real.raw(),
            horner.imag.raw(),
            self.newton_point.real.raw(),
            self.newton_point.imag.raw(),
            self.newton_h_value.real.raw(),
            self.newton_h_value.imag.raw(),
            self.newton_derivative.real.raw(),
            self.newton_derivative.imag.raw(),
            self.newton_inverse_guess.real.raw(),
            self.newton_inverse_guess.imag.raw(),
            inverse_next.real.raw(),
            inverse_next.imag.raw(),
            point_next.real.raw(),
            point_next.imag.raw(),
        ];

        values
            .iter()
            .map(i16::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    }
}

pub fn write_core_vectors(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = fs::File::create(path)?;
    for vector in core_vectors() {
        writeln!(file, "{}", vector.to_line())?;
    }
    Ok(())
}

fn core_vectors() -> Vec<CoreVector> {
    vec![
        CoreVector {
            index: 0,
            fixed_a: FixedQ::from_f64(0.75),
            fixed_b: FixedQ::from_f64(-1.25),
            complex_a: FixedComplex::from_f64(0.5, 0.25),
            complex_b: FixedComplex::from_f64(-0.75, 0.5),
            horner_z: FixedComplex::from_f64(0.5, -0.25),
            horner_coefficients: fixed_coefficients(&Polynomial::new(vec![1.0, 2.0, 3.0, 4.0])),
            newton_point: FixedComplex::from_f64(0.75, -0.1),
            newton_h_value: FixedComplex::from_f64(0.08, -0.03),
            newton_derivative: FixedComplex::from_f64(2.5, -0.5),
            newton_inverse_guess: FixedComplex::from_f64(0.38, 0.08),
        },
        CoreVector {
            index: 1,
            fixed_a: FixedQ::from_f64(4.0),
            fixed_b: FixedQ::from_f64(1.5),
            complex_a: FixedComplex::from_f64(1.25, -0.75),
            complex_b: FixedComplex::from_f64(0.5, 0.5),
            horner_z: FixedComplex::from_f64(-0.5, 0.75),
            horner_coefficients: fixed_coefficients(&Polynomial::new(vec![-6.0, 11.0, -6.0, 1.0])),
            newton_point: FixedComplex::from_f64(1.2, 0.05),
            newton_h_value: FixedComplex::from_f64(-0.04, 0.02),
            newton_derivative: FixedComplex::from_f64(1.5, 0.25),
            newton_inverse_guess: FixedComplex::from_f64(0.65, -0.1),
        },
        CoreVector {
            index: 2,
            fixed_a: FixedQ::from_f64(24.0),
            fixed_b: FixedQ::from_f64(16.0),
            complex_a: FixedComplex::from_f64(-1.5, 1.0),
            complex_b: FixedComplex::from_f64(-0.25, -0.75),
            horner_z: FixedComplex::from_f64(1.0, 0.5),
            horner_coefficients: [
                FixedComplex::from_f64(-1.0, 0.0),
                FixedComplex::from_f64(0.0, 0.0),
                FixedComplex::from_f64(0.0, 0.0),
                FixedComplex::from_f64(1.0, 0.0),
            ],
            newton_point: FixedComplex::from_f64(-0.55, 0.85),
            newton_h_value: FixedComplex::from_f64(0.03, 0.04),
            newton_derivative: FixedComplex::from_f64(-1.2, 1.8),
            newton_inverse_guess: FixedComplex::from_f64(-0.25, -0.38),
        },
        CoreVector {
            index: 3,
            fixed_a: FixedQ::from_f64(31.0),
            fixed_b: FixedQ::from_f64(4.0),
            complex_a: FixedComplex::from_f64(2.0, -1.0),
            complex_b: FixedComplex::from_f64(3.0, 0.25),
            horner_z: FixedComplex::from_f64(-1.0, -0.25),
            horner_coefficients: [
                FixedComplex::from_f64(0.5, -0.25),
                FixedComplex::from_f64(-1.25, 0.5),
                FixedComplex::from_f64(0.75, -0.75),
                FixedComplex::from_f64(0.25, 0.5),
            ],
            newton_point: FixedComplex::from_f64(0.1, -1.1),
            newton_h_value: FixedComplex::from_f64(-0.02, -0.05),
            newton_derivative: FixedComplex::from_f64(0.75, -2.0),
            newton_inverse_guess: FixedComplex::from_f64(0.15, 0.42),
        },
    ]
}

fn fixed_coefficients(polynomial: &Polynomial) -> [FixedComplex; 4] {
    let mut coefficients = [FixedComplex::ZERO; 4];
    for (index, coefficient) in polynomial.coefficients.iter().take(4).copied().enumerate() {
        coefficients[index] = FixedComplex::from_complex(coefficient);
    }
    coefficients
}

fn horner_fixed(coefficients: [FixedComplex; 4], z: FixedComplex) -> FixedComplex {
    let mut result = coefficients[3];
    result = result * z + coefficients[2];
    result = result * z + coefficients[1];
    result = result * z + coefficients[0];
    result
}

fn newton_step_fixed(
    point: FixedComplex,
    h_value: FixedComplex,
    derivative: FixedComplex,
    inverse_guess: FixedComplex,
) -> (FixedComplex, FixedComplex) {
    let inverse_next = inverse_guess * (FixedComplex::TWO - inverse_guess * derivative);
    let point_next = point - h_value * inverse_next;
    (inverse_next, point_next)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_q_saturates_addition_and_multiplication() {
        assert_eq!(
            (FixedQ::from_f64(31.0) + FixedQ::from_f64(4.0)).raw(),
            i16::MAX
        );
        assert_eq!(
            (FixedQ::from_f64(-31.0) - FixedQ::from_f64(4.0)).raw(),
            i16::MIN
        );
        assert_eq!(
            (FixedQ::from_f64(1.5) * FixedQ::from_f64(-2.0)).to_f64(),
            -3.0
        );
    }

    #[test]
    fn fixed_complex_multiplication_matches_complex_math() {
        let a = FixedComplex::from_f64(0.5, 0.25);
        let b = FixedComplex::from_f64(-0.75, 0.5);
        let result = (a * b).to_complex();
        let expected = ComplexNumber::new(0.5, 0.25) * ComplexNumber::new(-0.75, 0.5);

        assert!(result.approx_eq(expected, 1.0 / FixedQ::SCALE as f64));
    }

    #[test]
    fn vector_lines_have_expected_field_count() {
        for vector in core_vectors() {
            assert_eq!(vector.to_line().split_whitespace().count(), 40);
        }
    }
}
