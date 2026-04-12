use function_analyzer::functions::polynomial::Polynomial;

fn main() {
    let linear_coefficients = vec![2.0, 3.0];
    let linear_fx = Polynomial::new(linear_coefficients);

    println!("f(x) = {}", linear_fx.format_pretty());
    println!("Function degree: {}", linear_fx.degree());
    println!("Leading coefficient: {}", linear_fx.leading_coefficient());

    println!(
        "Evaluate function for x=3: f(3) = {}",
        linear_fx.evaluate(3.0)
    );
    println!("Fx y-intercept: f(0) = {}", linear_fx.y_intercept());
    println!(
        "First derivative: f'(x) = {}",
        linear_fx.derivative().format_pretty()
    );

    let quadratic_coefficients = vec![1.0, 3.0, 2.0];
    let quadratic_fx = Polynomial::new(quadratic_coefficients);

    println!("f(x) = {}", quadratic_fx.format_pretty());
    println!("Function degree: {}", quadratic_fx.degree());
    println!(
        "Leading coefficient: {}",
        quadratic_fx.leading_coefficient()
    );

    println!(
        "Evaluate function for x=3: f(3) = {}",
        quadratic_fx.evaluate(3.0)
    );
    println!("Fx y-intercept: f(0) = {}", quadratic_fx.y_intercept());
    println!(
        "First derivative: f'(x) = {}",
        quadratic_fx.derivative().format_pretty()
    );

    println!(
        "Sum of two fxs = {}",
        quadratic_fx.add(&linear_fx).format_pretty()
    );
    println!(
        "Difference of two fxs: {}",
        quadratic_fx.subtract(&linear_fx).format_pretty()
    );

    let binomial1 = Polynomial::new(vec![1.0, 2.0]);
    let binomial2 = Polynomial::new(vec![2.0, -3.0]);
    println!(
        "Product of {} and {} = {}",
        binomial1.format_pretty(),
        binomial2.format_pretty(),
        binomial1.multiply(&binomial2).format_pretty()
    );

    println!(
        "Quotient of {} and {}",
        quadratic_fx.format_pretty(),
        binomial1.format_pretty()
    );
    match quadratic_fx.divide(&binomial1) {
        Ok((quotient, remainder)) => {
            println!("Quotient: {}", quotient.format_pretty());
            println!("Remainder: {}", remainder.format_pretty());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    let binomial1 = Polynomial::new(vec![1.0, 2.0]);
    let trinomial = Polynomial::new(vec![1.0, 5.0, 6.0]);
    println!(
        "Trinomial: {} divided by binomial: {} = ",
        trinomial.format_pretty(),
        binomial1.format_pretty()
    );
    match quadratic_fx.divide(&binomial1) {
        Ok((quotient, remainder)) => {
            println!("Quotient: {}", quotient.format_pretty());
            println!("Remainder: {}", remainder.format_pretty());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
