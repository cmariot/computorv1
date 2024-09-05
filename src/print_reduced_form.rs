/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   print_reduced_form.rs                              :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/04 21:05:27 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/05 08:57:35 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::Term;
use std::collections::BTreeMap;

fn zero_equal_zero(terms: &BTreeMap<i32, Term>) -> bool {
    // Check if the reduced equation is 0 = 0

    for value in terms.values() {
        if value.coefficient != 0.0 {
            return false;
        }
    }
    true
}

fn print_sign(first_term: &mut bool, term: &Term) {
    // Print the sign of the coefficient

    if *first_term == false {
        if term.coefficient > 0.0 {
            print!(" + ");
        } else {
            print!(" - ");
        }
    } else if term.coefficient < 0.0 {
        print!("-");
    }
    *first_term = false;
}

fn print_coefficient(term: &Term, abs_coefficient: f64) {
    // Print the coefficient of the term

    if term.degree == 0 {
        print!("{}", abs_coefficient);
    } else if abs_coefficient != 1.0 {
        print!("{} ", abs_coefficient);
    }
}

fn print_degree(degree: &i32, abs_coefficient: f64) {
    // Print the degree of the term

    if *degree != 0 {
        if abs_coefficient != 1.0 {
            print!("* ")
        }
        print!("X^{}", *degree);
    }
}

pub fn print_reduced_form(terms: &BTreeMap<i32, Term>) {
    // Print the reduced form of the equation

    let blue = "\x1b[34m";
    let reset = "\x1b[0m";
    println!("{}Reduced form: {}{}", blue, reset, reset);

    // Check if the reduced equation is 0 = 0
    if zero_equal_zero(terms) {
        println!("0 = 0");
        return;
    }

    // For each term, print the sign, the coefficient and the degree
    let mut first_term: bool = true;
    for (degree, term) in terms.iter() {
        // Skip the term with a coefficient of 0
        if term.coefficient == 0.0 {
            continue;
        }

        let abs_coefficient: f64 = term.coefficient.abs();

        print_sign(&mut first_term, term);
        print_coefficient(term, abs_coefficient);
        print_degree(degree, abs_coefficient);
    }
    println!(" = 0");
}
