/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   store_terms.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/04 21:04:00 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/05 00:07:31 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::error::error;
use crate::parsing_utils::{contains, is_a_number, is_a_sign, is_multiplication_sign};
use crate::Term;
use std::collections::BTreeMap;

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
) {
    // Insert the Term in the BTreeMap or append the coefficient if
    // already present

    let already_present = terms.get(&degree);

    if already_present.is_some() {
        let previous_term = already_present.unwrap();
        let new_coefficient = previous_term.coefficient + *coefficient * *sign * side_sign;
        let new_term = Term::new(new_coefficient, *degree);
        terms.insert(*degree, new_term);
    } else {
        let term = Term::new(*coefficient * *sign * side_sign, *degree);
        terms.insert(*degree, term);
    }
}

pub fn store_terms(side: &str, side_sign: f64, terms: &mut BTreeMap<i32, Term>) {
    // Retrieve the sign, the coefficient and the degree of each polynomial part of an equation
    // Insert them in a BTreeMap

    let mut sign: f64 = 1.0;
    let mut coefficient: f64 = 1.0;
    let mut degree: i32;

    let mut i: usize = 0;
    let side_splitted: std::str::SplitWhitespace<'_> = side.split_whitespace();
    let nb_terms: usize = side_splitted.count();

    while i < nb_terms {
        let term: &str = side.split_whitespace().nth(i).unwrap();

        if is_a_sign(term) {
            sign = if term == "+" { 1.0 } else { -1.0 };
        } else if is_a_number(term) {
            coefficient = term.parse::<f64>().unwrap();
            // If there is no term or the next term is a sign, the degree is 0
            if i + 1 == nb_terms || is_a_sign(side.split_whitespace().nth(i + 1).unwrap()) {
                degree = 0;
                insert_coefficient(terms, &mut degree, &mut coefficient, &mut sign, side_sign);
                coefficient = 0.0;
                sign = 1.0;
            }
        } else if is_multiplication_sign(term) {
            i += 1;
            continue;
        } else if contains(term, 'X') {
            degree = parse_degree(term);
            insert_coefficient(terms, &mut degree, &mut coefficient, &mut sign, side_sign);
            coefficient = 1.0;
            sign = 1.0;
        } else {
            error(&format!("Error: Invalid term {term}"));
        }
        i += 1;
    }
}
