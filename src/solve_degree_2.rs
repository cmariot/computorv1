/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   solve_degree_2.rs                                  :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/12 08:58:36 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/12 09:19:22 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


use crate::{color, Term};
use std::collections::BTreeMap;
use crate::get_coefficient::get_coefficient_in_terms;
use crate::irreducible_fraction::{display_fraction, print_solution};


fn print_header(a: f64, b: f64, c: f64, discriminant: f64) {

    color("cyan", "Equation resolution:\n");
    println!("To solve the equation, we need to find the value of X that satisfies:");
    println!("a × X² + b × X + c = 0");
    println!("a × (X² + b/a × X + c/a) = 0");
    println!("X² + b/a × X + c/a = 0");
    println!("X² + 2 × (b/2a × X) + c/a = 0");
    println!("(X + (b / 2 × a))² -  b² / 4a² + c/a = 0");
    println!("(X + (b / 2 × a))² =  b² / 4a² - c/a");
    println!("(X + (b / 2 × a))² =  b² - 4ac / 4a²");
    println!("Based on this equation we can see that the left part is always positive or null");
    println!("The right part of the equation sign depend of the  b² - 4ac part sign");
    println!("We call  b² - 4ac the discriminant");
    println!("Discriminant : b² - 4 × ac -> {}² - 4 × {} × {} = {}\n", b, a, c, discriminant);

}


fn print_complex_solution(real_solution: f64, real_numerator: f64, real_denominator: f64, complex_solution: f64, complex_numerator: f64, complex_denominator: f64,solution_number: i8) {

    let sign = if solution_number == 1 {"+"} else {"-"};

    color("cyan", &format!("Solution {solution_number}:\n").to_string());
    if real_solution.fract() != 0.0 || complex_solution.fract() != 0.0 {
        print!("X = ");
        if real_solution.fract() != 0.0 {
            display_fraction(real_numerator, real_denominator);
        } else {
            print!("{real_solution}");
        }
        print!(" {sign} i * ");
        if complex_solution.fract() != 0.0 {
            display_fraction(complex_numerator, complex_denominator);
        } else {
            print!("{complex_solution}");
        }
        println!(" ≈ {} {sign} i * {}", real_solution, complex_solution);
    }
    else {
        println!("X = {} {sign} i * {}", real_solution, complex_solution);
    }

}


pub fn solve_degree_2(terms: &BTreeMap<i32, Term>) {

    // Equation of the form: a*x^2 + b*x^1 + c*x^0 = 0

    let a: f64 = get_coefficient_in_terms(&2, terms);
    let b: f64 = get_coefficient_in_terms(&1, terms);
    let c: f64 = get_coefficient_in_terms(&0, terms);
    let discriminant: f64 = b.powi(2) - 4.0 * a * c;

    print_header(a, b, c, discriminant);

    if discriminant == 0.0 {

        let solution: f64 = -b / (2.0 * a);

        color("cyan", "The discriminant is null\n");
        println!("We can isolate X in the previous equation and find the only solution :");
        println!("X = - b / (2 × a)");
        print_solution(solution, -b, 2.0 * a, 0);

    } else if discriminant > 0.0 {

        let solution1: f64 = (-b + discriminant.sqrt()) / (2.0 * a);
        let solution2: f64 = (-b - discriminant.sqrt()) / (2.0 * a);

        color("cyan", "The discriminant is positive\n");
        println!("We can isolate x in the previous equation and find the two solutions :");
        println!("X = (- b ± √(b² - 4 × a × c)) / (2 × a)");
        print_solution(solution1, -b + discriminant.sqrt(), 2.0 * a, 1);
        print_solution(solution2, -b - discriminant.sqrt(), 2.0 * a, 2);

    } else {

        let real_numerator = -b;
        let real_denominator = 2.0 * a;
        let real_solution: f64 = real_numerator / real_denominator;

        let complex_numerator = (-discriminant).sqrt();
        let complex_denominator = 2.0 * a;
        let complex_solution: f64 = complex_numerator / complex_denominator;

        color("cyan", "The discriminant is negative\n");
        println!("We can isolate x in the previous equation and find the two complex solutions :");
        println!("X = - b / (2 × a) ± i √(-discriminant) / (2 × a)");
        print_complex_solution(real_solution, real_numerator, real_denominator, complex_solution, complex_numerator, complex_denominator, 1);
        print_complex_solution(real_solution, real_numerator, real_denominator, complex_solution, complex_numerator, complex_denominator, 2);

    }

}
