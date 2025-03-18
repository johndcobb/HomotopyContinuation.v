mod polynomials;
use polynomials::Polynomial;
use polynomials::ComplexNumber;

fn main() {
    let z = ComplexNumber::new(1.0, 2.0);
    let p = Polynomial::new(vec![1.0,2.0,3.0]);

    println!("z = {}", z);
    println!("p = {}", p);
}
