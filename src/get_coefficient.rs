/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   get_coefficient.rs                                 :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/11 15:31:16 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/11 15:31:20 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


use std::collections::BTreeMap;
use crate::term::Term;


pub fn get_coefficient_in_terms(degree: &i32, terms: &BTreeMap<i32, Term>) -> f64 {

    // Return the coefficient of the degree passed in arguments in terms

    let coefficient = terms.get(degree);

    if coefficient.is_none() {
        return 0.0;
    }
    coefficient.unwrap().coefficient

}
