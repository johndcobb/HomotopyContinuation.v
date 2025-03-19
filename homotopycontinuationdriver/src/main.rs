mod polynomials;
use polynomials::Polynomial;
use polynomials::ComplexNumber;

mod homotopy;
use homotopy::Homotopy;

mod pathtracking;
use pathtracking::PathTracker;

fn main() {
    basic_example();
}

fn basic_example() {
    let f = Polynomial::new(vec![1.0, 2.0, 3.0, 4.0]);
    let g = Polynomial::new(vec![-1.0, 0.0, 0.0, 1.0]);

    println!("f(z) = {}", &f);
    println!("g(z) = {}", &g);


    println!("f(2) = {}", &f.evaluate(2.0.into()));
    println!("g(1) = {}", &g.evaluate(1.0.into()));

    println!("f'(z) = {}", &f.derivative());

    let h = Homotopy::new(f,g);
    println!("h(z,0) = {}", &h.at_time(0.0));
    println!("h(z,0.5) = {}", &h.at_time(0.5));
    println!("h(z,1) = {}", &h.at_time(1.0));
    println!("h.random_unit = {}", &h.random_unit);
}
