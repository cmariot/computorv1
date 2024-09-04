/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/02 22:36:19 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/04 18:29:04 by cmariot          ###   ########.fr       */
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


fn character_position(string: &str, character: char) -> usize {

    // Return the position of the character in the string
    // If the character is not found, an error is raised
    // If the character is found multiple times, an error is raised
    // If the character is at the beginning or the end of the string, an error is raised

    let bytes = string.as_bytes();
    let mut position: usize = 0;
    let mut count: usize = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == character as u8 {
            count += 1;
            position = i;
            if count > 1 {
                error("Error: Invalid equation, multiple '=' signs");
            }
        }
    }

    if count == 0 {
        error("Error: Invalid equation, no '=' sign");
    } else if position == 0 || position == string.len() - 1 {
        error("Error: Invalid equation, '=' sign at the beginning or the end of the equation");
    }
    position

}


fn split_equation(equation: &String) -> (&str, &str) {

    // Split the polynomial equation by the '='
    // The right side will be multiplied by -1

    let equal_position = character_position(equation, '=');

    let left_side = &equation[..equal_position];
    let right_side = &equation[equal_position + 1..];

    (left_side, right_side)

}


struct Term {
    coefficient: f64,
    degree: i32
}


impl Term {

    fn new(coefficient: f64, degree: i32) -> Term {
        Term { coefficient, degree }
    }

    fn get_value(&self) -> f64 {
        self.coefficient
    }

    fn _print(&self) {
        println!("{} * X^{}", self.coefficient, self.degree);
    }

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
    terms: &mut BTreeMap<i32, Term>,
    degree: &mut i32,
    coefficient: &mut f64,
    sign: &mut f64,
    side_sign: f64
) {

    // Insert the Term in the BTreeMap

    if terms.contains_key(&degree) {

        let previous_term = terms.get(&degree).unwrap();

        let new_coefficient = previous_term.coefficient + *coefficient * *sign * side_sign;
        let new_term = Term::new(new_coefficient, *degree);

        terms.insert(*degree, new_term);
    }
    else {

        let term = Term::new(*coefficient * *sign * side_sign, *degree);
        terms.insert(*degree, term);

    }

}


fn store_terms(side: &str, side_sign: f64, terms: &mut BTreeMap<i32, Term>) {

    // Retrieve the sign, the coefficient and the degree of each polynomial part of an equation
    // Insert them in a BTreeMap

    let mut sign = 1.0;
    let mut coefficient = 0.0;
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
                print!("HERE coefficient: {coefficient}// ");
                degree = 0;
                insert_coefficient(terms, &mut degree, &mut coefficient, &mut sign, side_sign);
                coefficient = 0.0;
                sign = 1.0;
            }
            i += 1;
            continue;
        } else if is_multiplication_sign(term) {
            i += 1;
            continue;
        } else if contains(term, 'X') {
            degree = parse_degree(term);
            insert_coefficient(terms, &mut degree, &mut coefficient, &mut sign, side_sign);
            coefficient = 0.0;
            sign = 1.0;
        } else {
            error("Error: Invalid term {term}");
        }
        i += 1;
    }
}


fn print_reduced_form(terms: &BTreeMap<i32, Term>) {

    // Print the reduced form of the equation

    print!("Reduced form: ");


    // Check if the reduced equation is 0 = 0
    let mut all_zero = true;
    for value in terms.values() {
        if value.coefficient != 0.0 {
            all_zero = false;
            break
        }
    }
    if all_zero == true {
        println!("0 = 0");
        return
    }


    let mut first_term = true;

    for (degree, term) in terms.iter() {

        // Skip the terms with a coefficient of 0
        if term.coefficient == 0.0 {
            continue;
        }

        // Print the sign
        if first_term == false {
            if term.coefficient > 0.0 {
                print!(" + ");
            } else {
                print!(" - ");
            }
        } else if term.coefficient < 0.0 {
            print!("-");
        }

        // Print the coefficient (if different from 1)
        let abs_coefficient = term.coefficient.abs();
            if term.degree == 0 {
                print!("{}", abs_coefficient);
            } else if abs_coefficient != 1.0 {
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


fn print_polynomial_degree(terms: &BTreeMap<i32, Term>, polynomial_degree: &mut i32) {

    // Set and print the polynomial degree
    // Also check if the degree is >= 0 and < 3

    for (degree, term) in terms.iter() {
        if term.coefficient != 0.0 {
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


fn resolve_degree_0(terms: &BTreeMap<i32, Term>) {

    // Equation of the form: c*x^0 = 0
    // If c = 0, all real numbers are solutions
    // If c != 0, no solution

    let c = if terms.contains_key(&0) { terms.get(&0).unwrap().get_value() } else { 0.0 };

    if c == 0.0 {
        println!("All real numbers are solutions");
    } else {
        println!("No solution");
    }

}


fn resolve_degree_1(terms: &BTreeMap<i32, Term>) {

    // Equation of the form: b*x^1 + c*x^0 = 0
    // The solution is -c / b

    let b = if terms.contains_key(&1) { terms.get(&1).unwrap().get_value() } else { 0.0 };
    let c = if terms.contains_key(&0) { terms.get(&0).unwrap().get_value() } else { 0.0 };

    let solution = -c / b;
    println!("The solution is: {solution}");

}


fn resolve_degree_2(terms: &BTreeMap<i32, Term>) {

    // Equation of the form: a*x^2 + b*x^1 + c*x^0 = 0
    
    let a = if terms.contains_key(&2) { terms.get(&2).unwrap().get_value() } else { 0.0 };
    let b = if terms.contains_key(&1) { terms.get(&1).unwrap().get_value() } else { 0.0 };
    let c = if terms.contains_key(&0) { terms.get(&0).unwrap().get_value() } else { 0.0 };

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
    let mut terms: BTreeMap<i32, Term> = BTreeMap::new();
    let mut polynomial_degree = 0;

    store_terms(left_side, 1.0, &mut terms);
    store_terms(right_side, -1.0, &mut terms);

    print_reduced_form(&terms);
    print_polynomial_degree(&terms, &mut polynomial_degree);

    match polynomial_degree {
        0 => resolve_degree_0(&terms),
        1 => resolve_degree_1(&terms),
        2 => resolve_degree_2(&terms),
        _ => println!("Polynomial degree: {polynomial_degree}")
    }

}
