/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   print_polynomial_degree.rs                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/04 21:09:12 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/09 20:07:43 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


use crate::color;
use crate::error;
use crate::print::exponent;
use crate::Term;
use std::collections::BTreeMap;


fn check_polynomial_degree(polynomial_degree: i32, negative_polynomial_degree: &bool) {

    // Check if the polynomial degree is valid (between 0 and 2)

    if polynomial_degree < 0 {
        error("The polynomial degree is negative, I can't solve.");
    } else if polynomial_degree > 2 {
        error("The polynomial degree is stricly greater than 2, I can't solve.");
    } else if *negative_polynomial_degree {
        error("An equation degree is negative, I can't solve.");
    }

}

pub fn get_polynomial_degree(terms: &BTreeMap<i32, Term>) -> i32 {

    // Set and print the polynomial degree
    // The polynomial degree is the highest degree of the terms in the equation
    // Example : 3 * X^2 + 1 * X^1 + 1 * X^0 = 0 -> Polynomial degree: 2
    // Also check if the degree is >= 0 and < 3

    let mut polynomial_degree = 0;
    let mut negative_polynomial_degree = false;

    color("cyan", "Polynomial degree:\n");
    println!("The polynomial degree is the highest degree of the terms in the equation");
    println!("Here are the terms of the equation:");

    for (degree, term) in terms.iter().rev() {

        if *degree > polynomial_degree {
            polynomial_degree = *degree;
        }

        println!("{} × {}", term.coefficient, exponent(*degree));

        if *degree < 0 {
            negative_polynomial_degree = true;
        }

    }

    println!("The polynomial degree is {}\n", polynomial_degree);

    check_polynomial_degree(polynomial_degree, &negative_polynomial_degree);

    polynomial_degree

}
