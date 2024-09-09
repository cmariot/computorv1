/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/05 09:38:52 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/09 18:25:35 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod split_equal;
use split_equal::split_equal;

use std::collections::BTreeMap;
mod term;
use term::Term;

mod store_terms;
use store_terms::store_terms;

mod print_reduced_form;
use print_reduced_form::print_reduced_form;

mod print_polynomial_degree;
use print_polynomial_degree::print_polynomial_degree;
// use print_reduced_form::color;

mod resolve;
use resolve::resolve_degree_0;
use resolve::resolve_degree_1;
use resolve::resolve_degree_2;

mod error;
use error::error;

mod parsing_utils;

mod print;
use print::header;
use print::color;


fn split_start_inclusive(str: &str, delimiters: &[char]) -> Result<Vec<String>, &'static str> {

    // Split a string in a Vector of &str
    // The delimiters are included in the result at the beginning of each element

    let mut result: Vec<String> = Vec::new();
    let mut word = String::new();

    for c in str.chars() {
        if delimiters.contains(&c) {
            if !word.is_empty() {
                result.push(word.clone());
                word = String::new();
            }
            word.push(c);
        } else {
            word.push(c);
        }
    }
    if !word.is_empty() {
        result.push(word.clone());
    }
    Ok(result)
}



fn store_polynomial(side: &str, sign: f64) -> Result<BTreeMap<i32, f64>, &'static str> {

    let mut terms = BTreeMap::new();
    let cleaned_side = side.replace(" ", "");
    let delimiters = ['+', '-'];

    let terms_vector = match split_start_inclusive(&cleaned_side, &delimiters) {
        Ok(terms_vector) => terms_vector,
        Err(error) => {
            return Err(error);
        }
    };

    for term in terms_vector {

        // Check if a term is empty or contains only a sign
        if term.is_empty() {
            return Err("Parsing error: Unexpected empty term");
        } else if term == "+" {
            return Err("Parsing error: Unexpected '+' sign");
        } else if term == "-" {
            return Err("Parsing error: Unexpected '-' sign");
        }

        // Parse the sign of the coefficient
        let mut coefficient = sign;
        let mut i = 0;
        while i < term.len() {
            if term.chars().nth(i).unwrap() == '-' {
                coefficient *= -1.0;
            } else if term.chars().nth(i).unwrap() == '+' {
                coefficient *= 1.0;
            } else {
                break;
            }
            i += 1;
        }

        // Parse the numeric value of the coefficient
        let numeric_value_start = i;
        while i < term.len() {
            if term.chars().nth(i).unwrap().is_numeric() || term.chars().nth(i).unwrap() == '.' {
                i += 1;
            } else {
                break;
            }
        }
        let numeric_value_end = i;
        let coefficient = match term[numeric_value_start..numeric_value_end].parse::<f64>() {
            Ok(numeric_value) => coefficient * numeric_value,
            Err(_) => {
                return Err("Parsing error: Invalid numeric value");
            }
        };

        // Check if the coefficient is not 0
        if coefficient == 0.0 {
            continue;
        }

        let mut degree = 0;

        // Check if the term is a number
        if i == term.len() {
            println!("Term: {}", term);
            println!("Coefficient: {}", coefficient);
            println!();
            terms.insert(degree, coefficient as f64);
            continue;
        }

        // Check if there is the '*X' part
        let mut degree_part = term[i..].to_string();

        // Skip '*' character if it is present
        if degree_part.chars().nth(0).unwrap() == '*' {
            i += 1;
            if i == term.len() {
                return Err("Parsing error: Unexpected end of term after '*'");
            }
        }

        // Check if there is the 'X' part
        degree_part = term[i..].to_string();
        if degree_part.chars().nth(0).unwrap() != 'X' {

            return Err("Parsing error: Invalid term, missing 'X'");
        } else {
            i += 1;
        }

        // Check if there is the '^' part
        if i == term.len() {
            degree = 0;
            println!("Term: {}", term);
            println!("Coefficient: {}", coefficient);
            println!("Degree: {}", degree);
            println!();
            terms.insert(degree, coefficient as f64);

            // store_terms(terms, &mut degree, &mut coefficient, 1.0, 1.0, true);

            continue;
        }
        if term.chars().nth(i).unwrap() != '^' {
            return Err("Parsing error: Invalid term, missing '^'");
        } else {
            i += 1;
        }
        if i == term.len() {
            return Err("Parsing error: Unexpected end of term after '^'");
        }

        // Parse the degree part
        degree_part = term[i..].to_string();
        let degree = match degree_part.parse::<i32>() {
            Ok(degree) => degree,
            Err(_) => {
                return Err("Parsing error: Invalid degree value");
            }
        };

        println!("Term: {}", term);
        println!("Coefficient: {}", coefficient);
        println!("Degree part: {}", degree_part);
        println!("Degree: {}", degree);
        println!();
        terms.insert(degree, coefficient as f64);

        // store_terms(terms, &mut degree, &mut coefficient, 1.0, 1.0, true);

    }

    Ok(terms)
}


fn parsing(equation: String) -> Result<BTreeMap<i32, Term>, &'static str> {

    let mut terms: BTreeMap<i32, Term> = BTreeMap::new();

    let (left_side, right_side) = match split_equal(&equation) {
        Ok((left_side, right_side)) => (left_side, right_side),
        Err(error) => {
            return Err(error);
        }
    };

    let left_terms = match store_polynomial(left_side, 1.0) {
        Ok(left_terms) => left_terms,
        Err(error) => {
            return Err(error);
        }
    };

    let right_terms = match store_polynomial(right_side, -1.0) {
        Ok(right_terms) => right_terms,
        Err(error) => {
            return Err(error);
        }
    };

    // Insert the right side terms in the left side terms
    let mut first_term = true;
    for (degree, coefficient) in right_terms.iter() {
        let term = terms.entry(*degree).or_insert(Term::new(0.0, *degree, first_term));
        if first_term {
            first_term = false;
        }
        term.coefficient += *coefficient;
    }
    for (degree, coefficient) in left_terms.iter() {
        let term = terms.entry(*degree).or_insert(Term::new(0.0, *degree, first_term));
        if first_term {
            first_term = false;
        }
        term.coefficient += *coefficient;
    }

    Ok(terms)

}

pub fn run(equation: String) -> Result<i8, &'static str> {

    // Solve a polynomial equation of degree 0, 1 or 2
    // The equation must be passed as an argument
    // Example: cargo run "1 * X^0 + 1 * X^1 + 1 * X^2 = 0"
    // Find the X value(s) that satisfies the equation

    header();

    let terms = match parsing(equation) {
        Ok(polynomial) => polynomial,
        Err(error) => {
            return Err(error);
        }
    };

    // let left_side_error = store_terms(&left_side, 1.0, &mut terms)?;
    // let right_side_error = store_terms(&right_side, -1.0, &mut terms)?;
    // if left_side_error || right_side_error {
        // return Err("Error: Invalid equation");
    // }

    let mut polynomial_degree: i32 = 0;
    print_reduced_form(&terms);
    print_polynomial_degree(&terms, &mut polynomial_degree);

    match polynomial_degree {
        0 => resolve_degree_0(&terms),
        1 => resolve_degree_1(&terms),
        2 => resolve_degree_2(&terms),
        _ => error("Error: Invalid polynomial degree"),
    }

    Ok(0)

}
