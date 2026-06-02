pub mod fixed;
pub mod homotopy;
pub mod pathtracking;
pub mod polynomials;

pub use fixed::{FixedComplex, FixedQ, write_core_vectors};
pub use homotopy::Homotopy;
pub use pathtracking::{PathTracker, TrackConfig, TrackError, TrackedRoot, factor_polynomial};
pub use polynomials::{
    ComplexNumber, NewtonError, NewtonResult, Polynomial, newton_corrector, roots_of_unity,
};
