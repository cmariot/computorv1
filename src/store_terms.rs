/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   store_terms.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/04 21:04:00 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/09 12:11:17 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::error::error;
use crate::parsing_utils::{contains, is_a_number, is_a_sign, is_multiplication_sign};
use crate::Term;
use std::collections::BTreeMap;
use crate::print::color;


fn parse_degree(string: &str) -> i32 {

    // Return the degree of the term

    if string == "X" {
        return 1;
    } else {
        let parts: Vec<&str> = string.split('^').collect();
        if parts.len() != 2 || parts[0] != "X" {
            error(&format!("Error: Invalid term {string}"));
        }
        let degree = parts[1].parse::<i32>();
        if degree.is_err() {
            error(&format!("Error: Invalid term {string}"));
        }
        degree.unwrap()
    }

}


fn insert_coefficient(
    terms: &mut BTreeMap<i32, Term>,
    degree: &mut i32,
    coefficient: &mut f64,
    sign: &mut f64,
    side_sign: f64,
    first_term: bool,
) {

    // Insert the Term in the BTreeMap or append the coefficient if
    // already present

    let already_present = terms.get(&degree);

    if already_present.is_some() {
        let previous_term: &Term = already_present.unwrap();
        if *coefficient == 0.0 {
            let mut new_term = Term::new(0.0, *degree, first_term);
            new_term.update_coefficient(previous_term.coefficient);
            terms.insert(*degree, new_term);
            return;
        }
        let mut new_term: Term = Term::new(*coefficient * *sign, *degree, first_term);
        new_term.update_coefficient(new_term.coefficient * side_sign + previous_term.coefficient);
        terms.insert(*degree, new_term);
    } else {
        if *coefficient == 0.0 {
            let term = Term::new(0.0, *degree, first_term);
            terms.insert(*degree, term);
            return;
        }
        if side_sign == -1.0 {
            let mut term = Term::new(*coefficient * *sign, *degree, first_term);
            term.update_coefficient(term.coefficient * side_sign);
            terms.insert(*degree, term);
        } else {
            let term = Term::new(*coefficient * *sign * side_sign, *degree, first_term);
            terms.insert(*degree, term);
        }
    }

}


fn reset_vars(coefficient: &mut f64, degree: &mut i32, sign: &mut f64, first_term: &mut bool) {

    // Reset the coefficient, the degree, the sign and the first_term variables

    *coefficient = 1.0;
    *degree = 0;
    *sign = 1.0;
    *first_term = false;

}


pub fn store_terms(side: &str, side_sign: f64, terms: &mut BTreeMap<i32, Term>) -> Result<bool, &'static str> {

    // Retrieve the sign, the coefficient and the degree of each polynomial part of an equation
    // Insert them in a BTreeMap

    let mut sign: f64 = 1.0;
    let mut coefficient: f64 = 1.0;
    let mut degree: i32;

    let mut i: usize = 0;
    let side_splitted: std::str::SplitWhitespace<'_> = side.split_whitespace();
    let nb_terms: usize = side_splitted.count();
    let mut first_term = true;
    let mut term_complete = false;

    if side_sign == 1.0 {
        color("cyan", "Equation:\n");
        println!("Here is the equation given as input: ");
    } else {
        print!("{} ", " =");
    }

    while i < nb_terms {

        let term: &str = side.split_whitespace().nth(i).unwrap();

        if is_a_sign(term) {

            sign = if term == "+" { 1.0 } else { -1.0 };

        } else if is_a_number(term) {

            coefficient = term.parse::<f64>().unwrap();
            // If there is no term or the next term is a sign, the degree is 0
            if i + 1 == nb_terms || is_a_sign(side.split_whitespace().nth(i + 1).unwrap()) {
                degree = 0;
                insert_coefficient(terms, &mut degree, &mut coefficient, &mut sign, side_sign, first_term);
                reset_vars(&mut coefficient, &mut degree, &mut sign, &mut first_term);
            }

        } else if is_multiplication_sign(term) {

            i += 1;
            continue;

        } else if contains(term, 'X') {

            degree = parse_degree(term);
            insert_coefficient(terms, &mut degree, &mut coefficient, &mut sign, side_sign, first_term);
            reset_vars(&mut coefficient, &mut degree, &mut sign, &mut first_term);

        } else {

            return Err("Error: Invalid equation");
            
        }
        i += 1;
    }

    if side_sign == -1.0 {
        println!();
    }

    return Ok(false)

}
