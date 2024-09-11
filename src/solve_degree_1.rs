use crate::{color, Term};
use std::collections::BTreeMap;
use crate::get_coefficient::get_coefficient_in_terms;
use crate::irreducible_fraction::print_solution;


fn solve_header(b: f64, c: f64) {

    color("cyan", "Equation resolution:\n");
    println!("To solve the equation, we need to find the value of X that satisfies:");
    println!("b × x + c = 0");
    println!("We can isolate X by moving c to the right side of the equation:");
    if c < 0.0 {
        println!("{} × X = {}", b, -c);
    } else {
        println!("{} × X = -{}", b, c);
    }
    println!("Then we divide by b to find the solution:");
    if c < 0.0 {
        println!("X = {} / {}", c, b);
    } else {
        println!("X = -{} / {}", c, b);
    }

}


pub fn solve_degree_1(terms: &BTreeMap<i32, Term>) {

    // Equation of the form: b*x^1 + c*x^0 = 0
    // The solution is -c / b

    let b = get_coefficient_in_terms(&1, terms);
    let c = get_coefficient_in_terms(&0, terms);

    solve_header(b, c);

    let solution = -c / b;

    print_solution(solution, -c, b, 0);

}