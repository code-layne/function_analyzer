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

    let quadratic_coefficients = vec![1.0, 3.0, 2.0];
    let quadratic_fx = Polynomial::new(quadratic_coefficients);

    println!("f(x) = {}", quadratic_fx.format_pretty());
    println!("Function degree: {}", quadratic_fx.degree());
    println!("Leading coefficient: {}", quadratic_fx.leading_coefficient());

    println!("Evaluate function for x=3: f(3) = {}", quadratic_fx.evaluate(3.0));
    println!("Fx y-intercept: f(0) = {}", quadratic_fx.y_intercept());
    println!("First derivative: f'(x) = {}", quadratic_fx.derivative().format_pretty());

    println!("Sum of two fxs = {}", quadratic_fx.add(&linear_fx).format_pretty());
    println!("Difference of two fxs: {}", quadratic_fx.subtract(&linear_fx).format_pretty());
}
