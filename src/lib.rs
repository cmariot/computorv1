/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/05 09:38:52 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/05 17:15:10 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod split_equation;
use split_equation::split_equation;

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


pub fn run(equation: String) -> i8 {

    // Solve a polynomial equation of degree 0, 1 or 2
    // The equation must be passed as an argument
    // Example: cargo run "1 * X^0 + 1 * X^1 + 1 * X^2 = 0"
    // Find the X value(s) that satisfies the equation

    header(equation.as_str());

    let (left_side, right_side) = split_equation(&equation);
    let mut terms: BTreeMap<i32, Term> = BTreeMap::new();
    let mut polynomial_degree: i32 = 0;

    store_terms(left_side, 1.0, &mut terms);
    store_terms(right_side, -1.0, &mut terms);

    print_reduced_form(&terms);
    print_polynomial_degree(&terms, &mut polynomial_degree);

    match polynomial_degree {
        0 => resolve_degree_0(&terms),
        1 => resolve_degree_1(&terms),
        2 => resolve_degree_2(&terms),
        _ => error("Error: Invalid polynomial degree"),
    }

    0

}
