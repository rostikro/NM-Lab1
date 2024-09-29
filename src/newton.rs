fn f(x: f64) -> f64 {
    x.powi(3) - 5.0 * x.powi(2) - 4.0 * x + 20.0
}

fn f_derivative(x: f64) -> f64 {
    3.0 * x.powi(2) - 10.0 * x - 4.0
}

pub fn newton_method(a: f64, b: f64, precision: f64) -> Vec<Vec<f64>> {
    let mut result = vec![];

    let mut x = (a + b) / 2.0;
    let mut x_prev = f64::MAX;

    let f_d = f_derivative(x);

    let mut i = 1.0;
    while (x_prev - x).abs() > 0.1 * precision {
        let mut r = vec![];
        r.push(i);
        r.push(x);
        r.push(f(x));
        result.push(r);

        i = i + 1.0;
        x_prev = x;

        x = x - result.last().unwrap()[2] / f_d;
    }

    result.push(vec![i, x, f(x)]);

    result
}
