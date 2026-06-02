use crate::{ComplexNumber, Polynomial};

#[derive(Clone, Debug)]
pub struct Homotopy {
    pub start: Polynomial,
    pub target: Polynomial,
    pub gamma: ComplexNumber,
}

impl Homotopy {
    pub fn new(start: Polynomial, target: Polynomial) -> Self {
        Self::with_gamma(start, target, default_gamma())
    }

    pub fn with_gamma(start: Polynomial, target: Polynomial, gamma: ComplexNumber) -> Self {
        Self {
            start,
            target,
            gamma,
        }
    }

    pub fn at_time(&self, t: f64) -> Polynomial {
        self.start.clone() * (1.0 - t) + self.target.clone() * (self.gamma * t)
    }
}

pub fn default_gamma() -> ComplexNumber {
    let angle = std::f64::consts::PI / 7.0;
    ComplexNumber::new(angle.cos(), angle.sin())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn homotopy_endpoints_match_start_and_scaled_target() {
        let start = Polynomial::new(vec![-1.0, 0.0, 1.0]);
        let target = Polynomial::new(vec![2.0, -3.0, 1.0]);
        let gamma = ComplexNumber::new(0.0, 1.0);
        let homotopy = Homotopy::with_gamma(start.clone(), target.clone(), gamma);

        assert_eq!(homotopy.at_time(0.0), start);
        assert_eq!(homotopy.at_time(1.0), target * gamma);
    }
}
