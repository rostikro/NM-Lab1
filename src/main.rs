use comfy_table::Table;
use dialoguer::FuzzySelect;
use std::io;

use crate::newton::newton_method;
use crate::simple_iteration::{iterations_count, simple_iteration_method};

mod newton;
mod simple_iteration;

fn print_values(values: Vec<Vec<f64>>, header: Vec<&str>) {
    let mut table = Table::new();
    table.set_header(header);
    table.add_rows(values);
    println!("{table}");
}

fn main() {
    let items = vec![
        "Newton modified method: x^3 - 5x^2 - 4x + 20 = 0",
        "Simple iteration method: x^3 - 8x^2 + 9x + 18 = 0",
    ];

    let selection = FuzzySelect::new()
        .with_prompt("Choose method:")
        .items(&items)
        .interact()
        .unwrap();

    println!("Enter a desired precision (Press `Enter` to choose the default one: 0.001): ");

    let mut precision = 0.001;

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if !input.trim().is_empty() {
        precision = input.trim().parse().unwrap();
    }

    // Newton modified method
    if selection == 0 {
        // First root
        const A1: f64 = -2.5;
        const B1: f64 = -1.8;

        // Second root
        const A2: f64 = 1.8;
        const B2: f64 = 2.5;

        // Third root
        const A3: f64 = 4.7;
        const B3: f64 = 5.5;

        let values = newton_method(A1, B1, precision);

        println!("\nFirst root");
        print_values(values, vec!["n", "x_n", "f(x_n)"]);

        let values = newton_method(A2, B2, precision);

        println!("\nSecond root");
        print_values(values, vec!["n", "x_n", "f(x_n)"]);

        let values = newton_method(A3, B3, precision);

        println!("\nThird root");
        print_values(values,vec!["n", "x_n", "f(x_n)"]);
    } else {
        const A: f64 = 5.8;
        const B: f64 = 6.5;

        println!("Approx iterations: {}", iterations_count(A, B, precision));

        let values = simple_iteration_method(A, B, precision);

        print_values(values,vec!["n", "x_n", "phi(x_n) - x_n"]);
    }
}
