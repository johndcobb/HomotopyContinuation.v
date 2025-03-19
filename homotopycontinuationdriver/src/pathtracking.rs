use crate::Homotopy;
use crate::ComplexNumber;

pub struct PathTracker {
    pub homotopy: Homotopy,
    pub start_points: Vec<ComplexNumber>,
    pub time_steps: i32,
    pub tolerance: f64,
}

impl PathTracker {
    pub fn new (homotopy: Homotopy, start_points: Vec<ComplexNumber>, time_steps: i32, tolerance: f64) -> Self {
        Self { homotopy, start_points, time_steps, tolerance }
    }
}