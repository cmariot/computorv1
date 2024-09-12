/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   store_polynomial.rs                                :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/10 18:05:58 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/11 17:38:02 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


use std::collections::BTreeMap;
use crate::term::Term;
use crate::parsing_util::replace_whitespace;
use crate::parsing_util::split_equation_terms;
use crate::insert_term::insert_term;
use crate::parse_sign::parse_sign;
use crate::parse_coefficient::parse_coefficient;
use crate::parse_degree::parse_degree;


fn parse_term(term: &String, sign: &mut f64, coefficient: &mut f64, degree: &mut i32) -> Result<(), String> {

    let mut i = 0;

    if term.is_empty() {
        return Err("Parsing error: Unexpected empty term".to_string());
    }
    else if let Err(error_msg) = parse_sign(&term, &mut i, sign) {
        return Err(error_msg);
    }
    else if let Err(error_msg) = parse_coefficient(&term, &mut i, coefficient) {
        return Err(error_msg);
    }
    else if let Err(error_msg) = parse_degree(&term, &mut i, degree) {
        return Err(error_msg);
    }

    Ok(())

}


pub fn store_polynomial(side: &str, side_sign: f64, terms: &mut BTreeMap<i32, Term>) -> Result<(), String> {

    let cleaned_side = replace_whitespace(&mut side.to_string(), "".to_string());
    let terms_vector = split_equation_terms(&cleaned_side, &['+', '-']);
    let mut first_term = true;

    if side_sign == -1.0 {
        print!(" = ");
    }

    for term in terms_vector {

        let mut sign = 1.0;
        let mut coefficient = 1.0;
        let mut degree = 0;

        if let Err(error_msg) = parse_term(&term, &mut sign, &mut coefficient, &mut degree) {
            return Err(error_msg);
        }
        insert_term(terms, &sign, &coefficient, &degree, &first_term, side_sign);
        first_term = false;

    }

    Ok(())

}