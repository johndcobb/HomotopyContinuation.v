use crate::Homotopy;
use crate::ComplexNumber;
use crate::Polynomial;

pub struct PathTracker {
    pub homotopy: Homotopy,
    pub start_points: Vec<ComplexNumber>,
    pub num_steps: i32,
    pub tolerance: f64,
    _current_points: Vec<ComplexNumber>,
    _current_step: i32,
    _delta_time: f64,
}

impl PathTracker {
    pub fn new (homotopy: Homotopy, start_points: Vec<ComplexNumber>, num_steps: i32, tolerance: f64) -> Self {
        let start_points_clone = start_points.clone();
        Self { homotopy, start_points, num_steps, tolerance, _current_points: start_points_clone, _current_step: 0, _delta_time: 1.0 / num_steps as f64 }
    }

    pub fn homotopy_at_step(&self, step: i32) -> Polynomial {
        let time = step as f64 / self.num_steps as f64;
        let h = self.homotopy.at_time(time);
        h
    }

    // this function will complete a single step of the path tracking algorithm.
    pub fn next(&mut self) -> Vec<ComplexNumber> {
        if self._current_step == self.num_steps {
            println!("Path tracking has already completed.");
            return self._current_points.clone();
        } else {   
            let h = self.homotopy_at_step(self._current_step+1);

            // My (stupid) prediction for the zeroes of h are my current points, which should be accurate if the time step is small enough. I now need to correct these points using Newton's method. I'll implement a version that does not require division to replicate what should be done on the FPGA.

            //TODO: Finish here.


            // increment current steps and return the current points
            self._current_step += 1;
            self._current_points.clone()
        }
    }
}