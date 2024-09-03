/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/02 22:36:19 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/03 15:01:43 by cmariot          ###   ########.fr       */
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
        error("Usage: ./computor \"equation\"");
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
         let degree = parts[1].parse::<i32>().unwrap();
         return degree;
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

    let mut first_term = true;

    print!("Reduced form: ");
    for (degree, coefficient) in coefficients.iter() {

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
            print!("* X^{}", degree);
        }

        first_term = false;
    }
    println!(" = 0");
}


fn print_polynomial_degree(coefficients: &BTreeMap<i32, f64>) -> i32 {

    // Print the polynomial degree

    let mut polynomial_degree = 0;
    for (degree, coefficient) in coefficients.iter() {
        if *coefficient != 0.0 {
            polynomial_degree = *degree;
        }
    }
    println!("Polynomial degree: {polynomial_degree}");
    polynomial_degree
}


fn check_polynomial_degree(polynomial_degree: i32) {

    // Check if the polynomial degree is valid

    if polynomial_degree < 0 {
        error("The polynomial degree is negative, I can't solve.");
    } else if polynomial_degree > 2 {
        error("The polynomial degree is stricly greater than 2, I can't solve.");
    }
}

fn main() {
    let equation: String = parse_argument();
    let (left_side, right_side) = split_equation(&equation);
    let mut coefficients: BTreeMap<i32, f64> = BTreeMap::new();

    store_coefficients(left_side, 1.0, &mut coefficients);
    store_coefficients(right_side, -1.0, &mut coefficients);

    for (degree, coefficient) in coefficients.iter() {
        println!("{}: {}", degree, coefficient);
    }
    print_reduced_form(&coefficients);
    let polynomial_degree = print_polynomial_degree(&coefficients);
    check_polynomial_degree(polynomial_degree);
}
