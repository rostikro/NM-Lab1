fn phi(x: f64) -> f64 {
    (-18.0 / x.powi(2)) - (9.0 / x) + 8.0
}

fn phi_derivative(x: f64) -> f64 {
    (36.0 + 9.0 * x) / x.powi(3)
}

pub fn iterations_count(a: f64, b: f64, precision: f64) -> f64 {
    let q = phi_derivative(a);
    (((b - a) / ((1.0 - q) * precision)).ln() / (1.0 / q).ln()).floor() + 1.0
}

pub fn simple_iteration_method(a: f64, b: f64, precision: f64) -> Vec<Vec<f64>> {
    let mut result = vec![];

    let mut x = (a + b) / 2.0;
    let mut x_prev = f64::MAX;

    let c = if phi_derivative(a) > 0.5 {
        precision * 0.1
    } else {
        precision * (1.0 - phi_derivative(a)) / phi_derivative(a)
    };

    let mut i = 1.0;
    while (x_prev - x).abs() > c {
        let mut r = vec![];
        r.push(i);
        r.push(x);
        r.push(phi(x) - x);
        result.push(r);

        i = i + 1.0;
        x_prev = x;

        x = phi(x);
    }

    result.push(vec![i, x, phi(x) - x]);

    result
}