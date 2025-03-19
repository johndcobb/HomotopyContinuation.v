mod polynomials;
use polynomials::roots_of_unity;
use polynomials::Polynomial;
use polynomials::ComplexNumber;
use polynomials::newton_corrector;

mod homotopy;
use homotopy::Homotopy;

mod pathtracking;
use pathtracking::PathTracker;

fn main() {
    path_track_example();
}

fn basic_example() {
    let f = Polynomial::new(vec![1.0, 2.0, 3.0, 4.0]);
    let g = Polynomial::new(vec![-1.0, 0.0, 0.0, 1.0]);

    println!("f(z) = {}", &f);
    println!("g(z) = {}", &g);


    println!("f(2) = {}", &f.evaluate(&2.0.into()));
    println!("g(1) = {}", &g.evaluate(&1.0.into()));

    println!("f'(z) = {}", &f.derivative());

    let h = Homotopy::new(g,f);
    println!("h(z,0) = {}", &h.at_time(0.0));
    println!("h(z,0.5) = {}", &h.at_time(0.5));
    println!("h(z,1) = {}", &h.at_time(1.0));
}

fn path_track_example() {
    let f = Polynomial::new(vec![1.0, 2.0, 3.0, 4.0]);
    let g = Polynomial::new(vec![-1.0, 0.0, 0.0, 1.0]);
    let h = Homotopy::new(g,f);
    let start_points = roots_of_unity(3);
    let tolerance = 0.01;
    let mut path_tracker = PathTracker::new(h, start_points.clone(), 10, tolerance);


    let f1 = path_tracker.homotopy_at_step(1);
    let point = &start_points[0];
    let y_0 = ComplexNumber::new(1.0, 0.0) / f1.derivative().evaluate(&point);
    println!("f1(z) = {}", &f1);
    println!("point = {}", &point);
    println!("y_0 = {}", &y_0);
    let next_point = newton_corrector(f1, point.clone(), tolerance, y_0);
    println!("next_point = {}", &next_point);
}
