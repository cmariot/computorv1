/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   insert_term.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/10 18:26:29 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/11 10:10:00 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


use std::collections::BTreeMap;
use crate::term::Term;


fn append_term(terms: & mut BTreeMap<i32, Term>, sign: &f64, coefficient: &f64, degree: & i32, first_term: & bool, side_sign: f64) {

    let mut term = Term::new (*coefficient * *sign, * degree, * first_term, true);

    term.update_coefficient(term.coefficient * side_sign);
    if term.coefficient == 0.0 {
      return;
    }
    terms.insert(* degree, term);

}


fn update_term(terms: & mut BTreeMap<i32, Term>, sign: &f64, coefficient: &f64, degree: & i32, first_term: & bool, side_sign: f64) {

    let previous_term = terms.get(degree).unwrap();
    let mut new_term = Term::new (*coefficient * *sign, *degree, *first_term, true);

    new_term.update_coefficient(new_term.coefficient * side_sign + previous_term.coefficient);
    if new_term.coefficient == 0.0 {
        terms.remove(degree);
        return;
    }
    terms.insert(* degree, new_term);

}


pub fn insert_term(terms: & mut BTreeMap<i32, Term>, sign: &f64, coefficient: &f64, degree: & i32, first_term: & bool, side_sign: f64) {

  // Insert or update the value if already present

  if !terms.contains_key(degree) {
    append_term(terms, sign, coefficient, degree, first_term, side_sign);
  } else {
    update_term(terms, sign, coefficient, degree, first_term, side_sign);
  }

}