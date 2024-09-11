/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parsing.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/10 18:25:51 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/11 09:58:03 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


use crate::term::Term;
use crate::print::color;
use crate::split_equal::split_equal;
use std::collections::BTreeMap;
use crate::store_polynomial::store_polynomial;


fn parsing_header(equation: &String) {
    color("cyan", "Equation parsing:\n");
    println!("The equation given as argument is:\n{}", equation);
    println!("It's equivalent to:")
}


fn parsing_footer() {
    println!("\n");
}


pub fn parsing(equation: String) -> Result<BTreeMap<i32, Term>, &'static str> {

    let mut terms: BTreeMap<i32, Term> = BTreeMap::new();

    let (left_side, right_side) = match split_equal(&equation) {
        Ok((left_side, right_side)) => (left_side, right_side),
        Err(error) => {return Err(error);}
    };

    parsing_header(&equation);

    if let Err(error_msg) = store_polynomial(left_side, 1.0, &mut terms) {
        return Err(error_msg);
    }
    if let Err(error_msg) = store_polynomial(right_side, -1.0, &mut terms) {
        return Err(error_msg);
    }

    parsing_footer();

    Ok(terms)

}