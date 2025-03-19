use crate::polynomials::Polynomial;
use crate::polynomials::ComplexNumber;
use rand::Rng;

pub struct Homotopy {
    pub start: Polynomial,
    pub target: Polynomial,
    pub random_unit: ComplexNumber,
}

impl Homotopy {
    pub fn new(start: Polynomial, target: Polynomial) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..2.0*std::f64::consts::PI);
        let random_unit = ComplexNumber::new(angle.cos(), angle.sin());
        Self { start, target, random_unit }
    }
    
    pub fn time(&self, t: f64) -> Polynomial {
        self.start.clone()*(1.0-t) + self.random_unit.clone()*self.target.clone()*t
    }
}   