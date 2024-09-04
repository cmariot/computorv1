/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/02 22:36:19 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/04 10:15:05 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


use core::str;
use std::env;
use std::process;
use std::collections::BTreeMap;


fn error(message: &str) {

    // Print an error message and exit the program

    eprintln!("{message}");
    process::exit(1);
}


fn parse_argument() -> String {

    // Check if the number of arguments is 1
    // Return the equation

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error("Usage: ./computorv1 \"equation\"");
    }
    args[1].clone()
}


fn split_equation(equation: &String) -> (&str, &str) {

    // Split the polynomial equation by the '='
    // The right side will be multiplied by -1

    let sides: Vec<&str> = equation.split('=').collect();
    if sides.len() != 2 {
        error("Error: Equation must contain an '=' sign");
    }
    let left_side = sides[0].trim();
    let right_side = sides[1].trim();
    (left_side, right_side)
}


fn is_a_sign(string: &str) -> bool {

    // Return true if the string is "-" or "+"

    if string == "+" || string == "-" {
        return true;
    }
    return false;
}


fn is_a_number(string: &str) -> bool {

    // Return true if the string is a number

    let number = string.parse::<f64>();
    if number.is_ok() {
        return true;
    }
    return false;
}


fn is_multiplication_sign(string: &str) -> bool {

    // Return true if the string is "*"

    if string == "*" {
        return true;
    }
    return false;
}


fn contains(string: &str, character: char) -> bool {

    // Return true if the string contains the character

    for c in string.chars() {
        if c == character {
            return true;
        }
    }
    return false;
}


fn parse_degree(string: &str) -> i32 {

    // Return the degree of the term

    if string == "X" {
        return 1;
    } else {
        let parts: Vec<&str> = string.split('^').collect();
        if parts.len() != 2 || parts[0] != "X" {
            error("Error: Invalid term {string}");
        }
        let result = parts[1].parse::<i32>();
        if result.is_ok() {
            let degree = result.unwrap();
            return degree;
        } else {
            error("Invalid equation");
            -1
        }
    }
}


fn insert_coefficient(
    coefficients: &mut BTreeMap<i32, f64>,
    degree: &mut i32,
    coefficient: &mut f64,
    sign: &mut f64,
    side_sign: f64
) {

    // Ajouter le coefficient à la liste

    let mut coefficient = *coefficient * side_sign * *sign;
    if coefficients.contains_key(&degree) {
        let current_coefficient = coefficients.get(&degree).unwrap();
        coefficient += current_coefficient;
    }
    coefficients.insert(*degree, coefficient);
}


fn store_coefficients(side: &str, side_sign: f64, coefficients: &mut BTreeMap<i32, f64>) {

    // Retrieve the sign, the coefficient and the degree of each polynomial part of an equation
    // Insert them in a BTreeMap

    let mut sign = 1.0;
    let mut coefficient = 1.0;
    let mut degree;

    let side_splitted = side.split_whitespace();
    let nb_terms = side_splitted.count();
    let mut i = 0;

    while i < nb_terms {
        let term = side.split_whitespace().nth(i).unwrap();
        if is_a_sign(term) {
            sign = if term == "+" { 1.0 } else { -1.0 };
            i += 1;
            continue;
        } else if is_a_number(term) {
            coefficient = term.parse::<f64>().unwrap();
            // If there is no term or the next term is a sign, the degree is 0
            if i + 1 == nb_terms || is_a_sign(side.split_whitespace().nth(i + 1).unwrap()) {
                degree = 0;
                insert_coefficient(coefficients, &mut degree, &mut coefficient, &mut sign, side_sign);
                coefficient = 1.0;
                sign = 1.0;
            }
            i += 1;
            continue;
        } else if is_multiplication_sign(term) {
            i += 1;
            continue;
        } else if contains(term, 'X') {
            degree = parse_degree(term);
            insert_coefficient(coefficients, &mut degree, &mut coefficient, &mut sign, side_sign);
            coefficient = 1.0;
            sign = 1.0;
        } else {
            error("Error: Invalid term {term}");
        }
        i += 1;
    }
}


fn print_reduced_form(coefficients: &BTreeMap<i32, f64>) {

    // Print the reduced form of the equation

    print!("Reduced form: ");


    // Check if the reduced equation is 0 = 0
    let mut all_zero = true;
    for value in coefficients.values() {
        if *value != 0.0 {
            all_zero = false;
            break
        }
    }
    if all_zero == true {
        println!("0 = 0");
        return
    }


    let mut first_term = true;

    for (degree, coefficient) in coefficients.iter().rev() {

        // Skip the terms with a coefficient of 0
        if *coefficient == 0.0 {
            continue;
        }

        // Print the sign
        if first_term == false {
            if *coefficient > 0.0 {
                print!(" + ");
            } else {
                print!(" - ");
            }
        } else if *coefficient < 0.0 {
            print!("-");
        }

        // Print the coefficient (if different from 1 or 0)
        let abs_coefficient = coefficient.abs();
        if abs_coefficient != 1.0 {
            print!("{} ", abs_coefficient);
        }

        // Print the degree

        if *degree != 0 {
            if abs_coefficient != 1.0 {
                print!("* ")
            }
            print!("X^{}", degree);
        }

        first_term = false;
    }
    println!(" = 0");
}


fn print_polynomial_degree(coefficients: &BTreeMap<i32, f64>, polynomial_degree: &mut i32) {

    // Set and print the polynomial degree
    // Also check if the degree is >= 0 and < 3

    for (degree, coefficient) in coefficients.iter() {
        if *coefficient != 0.0 {
            *polynomial_degree = *degree;
        }
    }

    println!("Polynomial degree: {polynomial_degree}");

    check_polynomial_degree(*polynomial_degree);
}


fn check_polynomial_degree(polynomial_degree: i32) {

    // Check if the polynomial degree is valid (between 0 and 2)

    if polynomial_degree < 0 {
        error("The polynomial degree is negative, I can't solve.");
    } else if polynomial_degree > 2 {
        error("The polynomial degree is stricly greater than 2, I can't solve.");
    }
}


fn resolve_degree_0(coefficients: &BTreeMap<i32, f64>) {

    if coefficients.contains_key(&0) && *coefficients.get(&0).unwrap() == 0.0 {
        println!("All real numbers are solutions");
    } else {
        println!("No solution");
    }

}


fn resolve_degree_1(coefficients: &BTreeMap<i32, f64>) {

    let solution = -coefficients.get(&0).unwrap() / coefficients.get(&1).unwrap();
    println!("The solution is: {solution}");

}


fn resolve_degree_2(coefficients: &BTreeMap<i32, f64>) {

    let a = if coefficients.contains_key(&2) { *coefficients.get(&2).unwrap() } else { 0.0 };
    let b = if coefficients.contains_key(&1) { *coefficients.get(&1).unwrap() } else { 0.0 };
    let c = if coefficients.contains_key(&0) { *coefficients.get(&0).unwrap() } else { 0.0 };

    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant == 0.0 {
        let solution = -b / (2.0 * a);
        println!("Discriminant is 0 -> 1 solution");
        println!("Solution: {solution}");
    } else if discriminant > 0.0 {
        let solution1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let solution2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Discriminant is positive -> 2 solution");
        println!("Solution 1: {solution1}");
        println!("Solution 2: {solution2}");
    } else {
        println!("Discriminant is negative -> no real solution");
    }

}


fn main() {

    let equation: String = parse_argument();
    let (left_side, right_side) = split_equation(&equation);
    let mut coefficients: BTreeMap<i32, f64> = BTreeMap::new();
    let mut polynomial_degree = 0;

    store_coefficients(left_side, 1.0, &mut coefficients);
    store_coefficients(right_side, -1.0, &mut coefficients);

    print_reduced_form(&coefficients);
    print_polynomial_degree(&coefficients, &mut polynomial_degree);

    match polynomial_degree {
        0 => resolve_degree_0(&coefficients),
        1 => resolve_degree_1(&coefficients),
        2 => resolve_degree_2(&coefficients),
        _ => println!("Polynomial degree: {polynomial_degree}")
    }

}
