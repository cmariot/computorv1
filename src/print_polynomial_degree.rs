/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   print_polynomial_degree.rs                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/04 21:09:12 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/05 08:56:37 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::error;
use crate::Term;
use std::collections::BTreeMap;

fn check_polynomial_degree(polynomial_degree: i32) {
    // Check if the polynomial degree is valid (between 0 and 2)

    if polynomial_degree < 0 {
        error("The polynomial degree is negative, I can't solve.");
    } else if polynomial_degree > 2 {
        error("The polynomial degree is stricly greater than 2, I can't solve.");
    }
}

pub fn print_polynomial_degree(terms: &BTreeMap<i32, Term>, polynomial_degree: &mut i32) {
    // Set and print the polynomial degree
    // The polynomial degree is the highest degree of the terms in the equation
    // Example : 3 * X^2 + 1 * X^1 + 1 * X^0 = 0 -> Polynomial degree: 2
    // Also check if the degree is >= 0 and < 3

    for (degree, term) in terms.iter() {
        if term.coefficient != 0.0 {
            *polynomial_degree = *degree;
        }
    }

    println!("Polynomial degree: {polynomial_degree}");

    check_polynomial_degree(*polynomial_degree);
}
