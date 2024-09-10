/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/05 09:38:52 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/10 10:20:40 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod split_equal;
use split_equal::split_equal;

use std::collections::BTreeMap;
mod term;
use term::Term;

mod print_reduced_form;
use print_reduced_form::print_reduced_form;

mod print_polynomial_degree;
use print_polynomial_degree::get_polynomial_degree;

mod resolve;
use resolve::resolve_degree_0;
use resolve::resolve_degree_1;
use resolve::resolve_degree_2;

mod error;
use error::error;

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


fn parse_term_sign(term: &String, i: &mut usize, coefficient: &mut f64) {
    // Parse the sign of the coefficient
    let mut sign = 1.0;
    while *i < term.len() {
        if term.chars().nth(*i).unwrap() == '-' {
            sign *= -1.0;
        } else if term.chars().nth(*i).unwrap() == '+' {
            sign *= 1.0;
        } else {
            break;
        }
        *i += 1;
    }
    *coefficient *= sign;
}


fn parse_term_coefficient(term: &String, i: &mut usize, coefficient: &mut f64) -> Result<(), &'static str> {

    let numeric_value_start = *i;
    while *i < term.len() {
        if term.chars().nth(*i).unwrap().is_numeric() || term.chars().nth(*i).unwrap() == '.' {
            *i += 1;
        } else {
            break;
        }
    }
    let numeric_value_end = *i;

    if numeric_value_start == numeric_value_end && *i + 1 <= term.len() && term[*i..*i + 1].to_string() == "X" {
        *coefficient = 1.0;
        return Ok(());
    }

    *coefficient = match term[numeric_value_start..numeric_value_end].parse::<f64>() {
        Ok(numeric_value) => *coefficient * numeric_value,
        Err(_) => {return Err("Parsing error: Invalid numeric value");}
    };

    if *coefficient == -0.0 {
        *coefficient = 0.0;
    }

    Ok(())
}


fn parse_degree(term: &String, i: &mut usize, terms: &mut BTreeMap<i32, f64>, coefficient: f64, degree: &mut i32) -> Result<(), &'static str> {

    // Parse the '*X^1' part of the terms

    let mut degree_part = term[*i..].to_string();

    if degree_part.len() == 0 {
        return Ok(());
    }

    // Skip '*' character if it is present and not at the end of the term
    if degree_part.chars().nth(0).unwrap() == '*' {
        *i += 1;
        if *i == term.len() {
            return Err("Parsing error: Unexpected end of term after '*'");
        }
    }

    // Check if there is the 'X' part
    degree_part = term[*i..].to_string();
    if degree_part.chars().nth(0).unwrap() == 'X' {
        *degree = 1;
        *i += 1;
        if *i == term.len() {
            terms.insert(*degree, coefficient as f64);
        return Ok(());
    }
    } else {
        return Err("Parsing error: Invalid term, missing 'X'");
    }

    // Check if there is the '^' part
    if term.chars().nth(*i).unwrap() != '^' {
        return Err("Parsing error: Invalid term, missing '^'");
    } else {
        *i += 1;
        if *i == term.len() {
            return Err("Parsing error: Unexpected end of term after '^'");
        }
    }

    // Parse the degree part
    degree_part = term[*i..].to_string();
    *degree = match degree_part.parse::<i32>() {
        Ok(degree) => degree,
        Err(_) => {
            return Err("Parsing error: Invalid degree value");
        }
    };

    Ok(())

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

    if sign == 1.0 {
    } else {
        print!(" =");
    }

    let mut first_term= if sign == 1.0 {true} else {false};

    for term in terms_vector {

        // Check if a term is empty or contains only a sign
        if term.is_empty() {
            return Err("Parsing error: Unexpected empty term");
        } else if term == "+" {
            return Err("Parsing error: Unexpected '+' sign");
        } else if term == "-" {
            return Err("Parsing error: Unexpected '-' sign");
        }

        let mut i = 0;
        let mut coefficient = sign;
        let mut degree = 0;

        parse_term_sign(&term, &mut i, &mut coefficient);
        parse_term_coefficient(&term, &mut i, &mut coefficient, )?;

        let _ = match parse_degree(&term, &mut i, &mut terms, coefficient, &mut degree) {
            Ok(()) => (),
            Err(error) => {
                return Err(error);
            }
        };

        // debug
        println!("Term : {}", term);
        println!("Coefficient : {}", coefficient);
        println!("Degree : {}", degree);

        let _term_to_print = Term::new(coefficient, degree, first_term, true);

        println!("\n");

        terms.insert(degree, coefficient as f64);
        first_term = false;

    }

    Ok(terms)

}

fn join_terms(terms: &mut BTreeMap<i32, Term>, left_terms: &BTreeMap<i32, f64>, right_terms: &BTreeMap<i32, f64>) {

    // Insert the right side terms in the left side terms

    let side_terms = [left_terms, right_terms];
    let mut first_term = true;

    for side_term in side_terms {
        for (degree, coefficient) in side_term.iter() {
            if first_term {
                first_term = false;
            }

            // Insert or append to the value if already present
            if terms.contains_key(degree) {
                println!("Degree {} is already present", degree);
                let previous_term = terms.get(degree).unwrap();
                let mut new_term = Term::new(*coefficient, *degree, first_term, false);
                new_term.update_coefficient(previous_term.coefficient + *coefficient);
                terms.insert(*degree, new_term);
            } else {
                println!("Degree {} is not present", degree);
                terms.insert(*degree, Term::new(*coefficient, *degree, first_term, false));

            }

        }
    }

}

fn parsing(equation: String) -> Result<BTreeMap<i32, Term>, &'static str> {

    let mut terms: BTreeMap<i32, Term> = BTreeMap::new();

    let (left_side, right_side) = match split_equal(&equation) {
        Ok((left_side, right_side)) => (left_side, right_side),
        Err(error) => {
            return Err(error);
        }
    };

    color("cyan", "Equation parsing:\n");

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

    println!("\n");

    join_terms(&mut terms, &left_terms, &right_terms);

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

    print_reduced_form(&terms);

    let polynomial_degree: i32 = get_polynomial_degree(&terms);

    match polynomial_degree {
        0 => resolve_degree_0(&terms),
        1 => resolve_degree_1(&terms),
        2 => resolve_degree_2(&terms),
        _ => error("Error: Invalid polynomial degree"),
    }

    Ok(0)

}
