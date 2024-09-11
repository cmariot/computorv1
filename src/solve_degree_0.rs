use crate::{color, Term};
use std::collections::BTreeMap;
use crate::get_coefficient::get_coefficient_in_terms;


fn solve_header(c: f64) {

    color("cyan", "Equation resolution:\n");
    println!("The polynomial degree is 0");
    println!("The equation is: {c} = 0");

}


pub fn solve_degree_0(terms: &BTreeMap<i32, Term>) {

    // Equation of the form: c*x^0 = 0
    // If c = 0, all real numbers are solutions
    // If c != 0, no solution

    let c = get_coefficient_in_terms(&0, terms);

    solve_header(c);

    if c == 0.0 {
        println!("All real numbers are solutions.");
        println!("There is an infinity of X values that satisfy the equation.");
    } else {
        println!("It is impossible to solve.");
        println!("There is no X value that satisfies the equation.");
    }

}