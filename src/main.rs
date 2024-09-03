/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/02 22:36:19 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/03 10:40:19 by cmariot          ###   ########.fr       */
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


fn insert_coefficient(coefficients: &mut BTreeMap<i32, f64>, degree: i32, coefficient: f64, sign: f64, side_sign: f64) {

    // Ajouter le coefficient à la liste

    let mut coefficient = coefficient * side_sign * sign;
    if coefficients.contains_key(&degree) {
        let current_coefficient = coefficients.get(&degree).unwrap();
        coefficient += current_coefficient;
    }
    coefficients.insert(degree, coefficient);
}


fn store_coefficients(side: &str, side_sign: f64, coefficients: &mut BTreeMap<i32, f64>) {

    // Retrieve the sign, the coefficient and the degree of each polynomial part of an equation
    // Insert them in a BTreeMap

    let mut sign = 1.0;
    let mut coefficient = 0.0;
    let mut degree;

    for term in side.split_whitespace() {
        if is_a_sign(term) {
            sign = if term == "+" { 1.0 } else { -1.0 };
            continue;
        } else if is_a_number(term) {
            coefficient = term.parse::<f64>().unwrap();
            continue;
        } else if is_multiplication_sign(term) {
            continue;
        } else if contains(term, 'X') {
            degree = parse_degree(term);
            insert_coefficient(coefficients, degree, coefficient, sign, side_sign);
        } else {
            error("Error: Invalid term {term}");
        }
    }
}


fn main() {
    let equation: String = parse_argument();
    let (left_side, right_side) = split_equation(&equation);
    let mut coefficients: BTreeMap<i32, f64> = BTreeMap::new();

    store_coefficients(left_side, 1.0, &mut coefficients);
    store_coefficients(right_side, -1.0, &mut coefficients);

    print!("\nReduced form: ");
    let mut polynomial_degree = 0;
    for (degree, coefficient) in coefficients.iter() {

        if *coefficient == 0.0 {
            continue;
        }
        if polynomial_degree != 0 {
            if *coefficient > 0.0 {
                print!(" + ");
            } else {
                print!(" - ");
            }
        } else if *coefficient < 0.0 {
            print!("- ");
        }

        print!("{} X", coefficient.abs());

        print!("^{}", degree);

        if *coefficient != 0.0 {
            polynomial_degree = *degree;
        }
    }
    println!(" = 0");

    println!("Polynomial degree: {polynomial_degree}")


}
