use std::fmt;

use crate::{ComplexNumber, Homotopy, NewtonError, Polynomial, newton_corrector, roots_of_unity};

const MAX_FPGA_DEGREE: usize = 3;

#[derive(Clone, Copy, Debug)]
pub struct TrackConfig {
    pub steps: usize,
    pub tolerance: f64,
    pub max_newton_iterations: usize,
}

impl Default for TrackConfig {
    fn default() -> Self {
        Self {
            steps: 64,
            tolerance: 1e-8,
            max_newton_iterations: 20,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrackedRoot {
    pub path_index: usize,
    pub start_point: ComplexNumber,
    pub root: ComplexNumber,
    pub residual: f64,
    pub total_newton_iterations: usize,
}

#[derive(Debug)]
pub enum TrackError {
    InvalidConfig(&'static str),
    ConstantPolynomial,
    DegreeTooHigh {
        degree: usize,
        max_degree: usize,
    },
    NewtonFailed {
        step: usize,
        path_index: usize,
        source: NewtonError,
    },
}

impl fmt::Display for TrackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfig(message) => write!(f, "invalid tracking config: {message}"),
            Self::ConstantPolynomial => write!(f, "cannot factor a constant polynomial"),
            Self::DegreeTooHigh { degree, max_degree } => write!(
                f,
                "degree {degree} exceeds the current FPGA-facing limit of {max_degree}"
            ),
            Self::NewtonFailed {
                step,
                path_index,
                source,
            } => write!(
                f,
                "Newton correction failed at homotopy step {step}, path {path_index}: {source}"
            ),
        }
    }
}

impl std::error::Error for TrackError {}

#[derive(Clone, Debug)]
pub struct PathTracker {
    pub homotopy: Homotopy,
    pub start_points: Vec<ComplexNumber>,
    pub config: TrackConfig,
    current_points: Vec<ComplexNumber>,
    current_step: usize,
    total_newton_iterations: Vec<usize>,
}

impl PathTracker {
    pub fn new(
        homotopy: Homotopy,
        start_points: Vec<ComplexNumber>,
        config: TrackConfig,
    ) -> Result<Self, TrackError> {
        validate_config(config)?;
        if start_points.is_empty() {
            return Err(TrackError::InvalidConfig(
                "at least one start point is required",
            ));
        }

        Ok(Self {
            homotopy,
            current_points: start_points.clone(),
            total_newton_iterations: vec![0; start_points.len()],
            start_points,
            config,
            current_step: 0,
        })
    }

    pub fn homotopy_at_step(&self, step: usize) -> Polynomial {
        let time = step as f64 / self.config.steps as f64;
        self.homotopy.at_time(time)
    }

    pub fn step(&mut self) -> Result<&[ComplexNumber], TrackError> {
        if self.current_step >= self.config.steps {
            return Ok(&self.current_points);
        }

        let next_step = self.current_step + 1;
        let polynomial = self.homotopy_at_step(next_step);
        let mut next_points = Vec::with_capacity(self.current_points.len());

        for (path_index, point) in self.current_points.iter().copied().enumerate() {
            let corrected = newton_corrector(
                &polynomial,
                point,
                self.config.tolerance,
                self.config.max_newton_iterations,
            )
            .map_err(|source| TrackError::NewtonFailed {
                step: next_step,
                path_index,
                source,
            })?;

            self.total_newton_iterations[path_index] += corrected.iterations;
            next_points.push(corrected.point);
        }

        self.current_points = next_points;
        self.current_step = next_step;
        Ok(&self.current_points)
    }

    pub fn track_all(&mut self) -> Result<Vec<TrackedRoot>, TrackError> {
        while self.current_step < self.config.steps {
            self.step()?;
        }

        Ok(self
            .current_points
            .iter()
            .copied()
            .enumerate()
            .map(|(path_index, root)| TrackedRoot {
                path_index,
                start_point: self.start_points[path_index],
                root,
                residual: self.homotopy.target.evaluate(root).norm(),
                total_newton_iterations: self.total_newton_iterations[path_index],
            })
            .collect())
    }
}

pub fn factor_polynomial(
    target: Polynomial,
    config: TrackConfig,
) -> Result<Vec<TrackedRoot>, TrackError> {
    validate_config(config)?;
    let degree = target.degree();
    if degree == 0 {
        return Err(TrackError::ConstantPolynomial);
    }
    if degree > MAX_FPGA_DEGREE {
        return Err(TrackError::DegreeTooHigh {
            degree,
            max_degree: MAX_FPGA_DEGREE,
        });
    }

    let start = Polynomial::start_system(degree).map_err(|_| TrackError::ConstantPolynomial)?;
    let start_points = roots_of_unity(degree);
    let homotopy = Homotopy::new(start, target);
    let mut tracker = PathTracker::new(homotopy, start_points, config)?;
    tracker.track_all()
}

fn validate_config(config: TrackConfig) -> Result<(), TrackError> {
    if config.steps == 0 {
        return Err(TrackError::InvalidConfig("steps must be positive"));
    }
    if config.tolerance <= 0.0 {
        return Err(TrackError::InvalidConfig("tolerance must be positive"));
    }
    if config.max_newton_iterations == 0 {
        return Err(TrackError::InvalidConfig(
            "max_newton_iterations must be positive",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_roots_include(roots: &[TrackedRoot], expected: ComplexNumber) {
        assert!(
            roots.iter().any(|root| root.root.approx_eq(expected, 1e-6)),
            "expected root {expected}, found {roots:?}"
        );
    }

    #[test]
    fn factors_z_cubed_minus_one() {
        let roots = factor_polynomial(
            Polynomial::new(vec![-1.0, 0.0, 0.0, 1.0]),
            TrackConfig::default(),
        )
        .unwrap();

        assert_eq!(roots.len(), 3);
        assert_roots_include(&roots, ComplexNumber::new(1.0, 0.0));
        assert_roots_include(&roots, ComplexNumber::new(-0.5, 3.0_f64.sqrt() / 2.0));
        assert_roots_include(&roots, ComplexNumber::new(-0.5, -3.0_f64.sqrt() / 2.0));
        for root in roots {
            assert!(root.residual <= 1e-7, "{root:?}");
        }
    }

    #[test]
    fn factors_cubic_with_known_real_roots() {
        let roots = factor_polynomial(
            Polynomial::new(vec![-6.0, 11.0, -6.0, 1.0]),
            TrackConfig::default(),
        )
        .unwrap();

        assert_roots_include(&roots, ComplexNumber::new(1.0, 0.0));
        assert_roots_include(&roots, ComplexNumber::new(2.0, 0.0));
        assert_roots_include(&roots, ComplexNumber::new(3.0, 0.0));
        for root in roots {
            assert!(root.residual <= 1e-7, "{root:?}");
        }
    }

    #[test]
    fn factors_current_demo_polynomial_with_small_residuals() {
        let roots = factor_polynomial(
            Polynomial::new(vec![1.0, 2.0, 3.0, 4.0]),
            TrackConfig::default(),
        )
        .unwrap();

        assert_eq!(roots.len(), 3);
        for root in roots {
            assert!(root.residual <= 1e-7, "{root:?}");
        }
    }
}
