pub struct ComplexNumber {
    pub real: f64,
    pub imag: f64,
}

impl ComplexNumber {
    pub fn new(real: f64, imag: f64) -> Self {
        ComplexNumber { real, imag }
    }
}

impl From<f64> for ComplexNumber {
    fn from(value: f64) -> Self {
        Self::new(value, 0.0)
    }
}
pub struct Polynomial {
    pub coefficients: Vec<ComplexNumber>,
}

impl Polynomial {
    pub fn new<T: Into<ComplexNumber> + Copy>(coeffs: Vec<T>) -> Self {
        Self {
            coefficients: coeffs.into_iter().map(|c| c.into()).collect(),
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
                    write!(f, "{} + {}i", c.real, c.imag)?;
                }
                if i < self.coefficients.len() - 1 {
                    write!(f, "*z^{}", self.coefficients.len() - i - 1)?;
                }
            }
        }
        Ok(())
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