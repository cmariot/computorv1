/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/05 09:38:52 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/11 17:34:45 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


mod error;
mod get_coefficient;
mod insert_term;
mod irreducible_fraction;
mod parse_coefficient;
mod parse_degree;
mod parse_sign;
mod parsing;
mod parsing_util;
mod print;
mod print_polynomial_degree;
mod print_reduced_form;
mod solve_degree_0;
mod solve_degree_1;
mod solve_degree_2;
mod split_equal;
mod store_polynomial;
mod term;

use error::error;
use print::color;
use print::header;
use print_polynomial_degree::get_polynomial_degree;
use print_reduced_form::print_reduced_form;
use solve_degree_0::solve_degree_0;
use solve_degree_1::solve_degree_1;
use solve_degree_2::solve_degree_2;
use term::Term;
use parsing::parsing;


pub fn run(equation: String) -> Result<i8, String> {

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
        0 => solve_degree_0(&terms),
        1 => solve_degree_1(&terms),
        2 => solve_degree_2(&terms),
        _ => error("Error: Invalid polynomial degree".to_string()),
    }

    Ok(0)

}
