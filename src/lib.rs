/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/05 09:38:52 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/11 11:36:10 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


mod term;
use term::Term;

mod store_polynomial;

mod print_reduced_form;
use print_reduced_form::print_reduced_form;

mod print_polynomial_degree;
use print_polynomial_degree::get_polynomial_degree;

mod get_coefficient;

mod solve_degree_0;
use solve_degree_0::solve_degree_0;

mod solve_degree_1;
use solve_degree_1::solve_degree_1;

mod solve_degree_2;
use solve_degree_2::solve_degree_2;

mod error;
use error::error;

mod print;
use print::header;
use print::color;

mod parsing;
use parsing::parsing;

mod parsing_util;

mod insert_term;

mod parse_sign;
mod parse_coefficient;
mod parse_degree;

mod split_equal;

mod irreducible_fraction;


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
        0 => solve_degree_0(&terms),
        1 => solve_degree_1(&terms),
        2 => solve_degree_2(&terms),
        _ => error("Error: Invalid polynomial degree"),
    }

    Ok(0)

}
