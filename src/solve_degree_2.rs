use crate::{color, print, Term};
use std::collections::BTreeMap;
use crate::get_coefficient::get_coefficient_in_terms;
use crate::irreducible_fraction::{display_fraction, print_solution};


pub fn solve_degree_2(terms: &BTreeMap<i32, Term>) {

    // Equation of the form: a*x^2 + b*x^1 + c*x^0 = 0

    let a: f64 = get_coefficient_in_terms(&2, terms);
    let b: f64 = get_coefficient_in_terms(&1, terms);
    let c: f64 = get_coefficient_in_terms(&0, terms);

    let discriminant: f64 = b.powi(2) - 4.0 * a * c;

    color("cyan", "Equation resolution:\n");
    println!("To solve the equation, we need to find the value of X that satisfies:");
    println!("a × x² + b × x + c = 0");
    // println!("We can factorize the equation to find the solutions:");
    println!("a × (x² + b/a × x + c/a) = 0");
    // println!("This is equivalent to:");
    println!("x² + b/a × x + c/a = 0");
    println!("x² + 2 × (b/2a × x) + c/a = 0");
    // println!("x² + 2 × (b/2a × x) looks like the begin of the remarquable equation without the third part:");
    // println!("(alpha + beta)^2 = alpha² + 2 × alpha × beta + beta²");
    // println!("with alpha = x and beta = b / 2a");
    // println!("We can rewrite :");
    // println!("(x + (b / 2 × a))² as x² + 2 × (b/2a × x) + b² / 4a²");
    // println!("This is equivalent to :");
    println!("(x + (b / 2 × a))² -  b² / 4a² + c/a = 0");
    println!("(x + (b / 2 × a))² =  b² / 4a² - c/a");
    println!("(x + (b / 2 × a))² =  b² - 4ac / 4a²");
    println!("Based on this equation we can see that the left part is always positive or null");
    println!("The right part of the equation sign depend of the  b² - 4ac part sign");
    println!("We call  b² - 4ac the discriminant");
    println!("Discriminant : b² - 4 × ac -> {}² - 4 × {} × {} = {}\n", b, a, c, discriminant);

    if discriminant == 0.0 {
        let solution: f64 = -b / (2.0 * a);
        color("cyan", "The discriminant is null ;");
        println!("We can isolate x in the previous equation and find the only solution :");
        println!("x = - b / (2 × a)");
        println!("There is one real solution");
        print_solution(solution, -b, 2.0 * a, 0);
    } else if discriminant > 0.0 {
        let solution1: f64 = (-b + discriminant.sqrt()) / (2.0 * a);
        let solution2: f64 = (-b - discriminant.sqrt()) / (2.0 * a);
        color("cyan", "The discriminant is positive\n");
        println!("We can isolate x in the previous equation and find the two solutions :");
        println!("x = (- b ± √(b² - 4 × a × c)) / (2 × a)");
        println!("There are two real solutions\n");
        print_solution(solution1, -b + discriminant.sqrt(), 2.0 * a, 1);
        print_solution(solution2, -b - discriminant.sqrt(), 2.0 * a, 2);
    } else {
        println!("The discriminant is negative");
        println!("We can isolate x in the previous equation and find the two complex solutions :");
        println!("x = - b / (2 × a) ± i √(-discriminant) / (2 × a)");
        let real_part: f64 = -b / (2.0 * a);
        let imaginary_part: f64 = (-discriminant).sqrt() / (2.0 * a);
        println!("Solution 1: {real_part} + i{imaginary_part}");
        println!("Solution 2: {real_part} - i{imaginary_part}");
    }

}
