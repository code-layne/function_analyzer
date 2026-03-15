use crate::polynomials::Polynomial;

mod polynomials;

fn main() {
    let linear_coefficients = vec![2.0,3.0];
    let linear_fx = Polynomial::new(linear_coefficients);

    println!("f(x) = {}", linear_fx.format_pretty());
    println!("Function degree: {}", linear_fx.degree());
    println!("Leading coefficient: {}", linear_fx.leading_coefficient());

    println!("Evaluate function for x=3: f(3) = {}", linear_fx.evaluate(3.0));
    println!("Fx y-intercept: f(0) = {}", linear_fx.y_intercept());
    println!("First derivative: f'(x) = {}", linear_fx.derivative().format_pretty());
}
