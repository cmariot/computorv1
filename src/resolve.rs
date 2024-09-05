/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   resolve.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/04 21:00:32 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/05 10:18:44 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::Term;
use std::collections::BTreeMap;

fn get_coefficient_in_terms(degree: &i32, terms: &BTreeMap<i32, Term>) -> f64 {
    // Return the coefficient of the degree passed in arguments in terms

    let coefficient = terms.get(degree);

    if coefficient.is_none() {
        return 0.0;
    }
    coefficient.unwrap().coefficient
}

pub fn resolve_degree_0(terms: &BTreeMap<i32, Term>) {
    // Equation of the form: c*x^0 = 0
    // If c = 0, all real numbers are solutions
    // If c != 0, no solution

    let c = get_coefficient_in_terms(&0, terms);

    if c == 0.0 {
        println!("All real numbers are solutions");
    } else {
        println!("No solution");
    }
}

pub fn resolve_degree_1(terms: &BTreeMap<i32, Term>) {
    // Equation of the form: b*x^1 + c*x^0 = 0
    // The solution is -c / b

    let b = get_coefficient_in_terms(&1, terms);
    let c = get_coefficient_in_terms(&0, terms);
    let solution = -c / b;

    println!("The solution is: {solution}");
}

pub fn resolve_degree_2(terms: &BTreeMap<i32, Term>) {
    // Equation of the form: a*x^2 + b*x^1 + c*x^0 = 0

    let a: f64 = get_coefficient_in_terms(&2, terms);
    let b: f64 = get_coefficient_in_terms(&1, terms);
    let c: f64 = get_coefficient_in_terms(&0, terms);

    let discriminant: f64 = b.powi(2) - 4.0 * a * c;

    if discriminant == 0.0 {
        let solution: f64 = -b / (2.0 * a);
        println!("Discriminant is 0 -> 1 solution");
        println!("Solution: {solution}");
    } else if discriminant > 0.0 {
        let solution1: f64 = (-b + discriminant.sqrt()) / (2.0 * a);
        let solution2: f64 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Discriminant is positive -> 2 real solutions");
        println!("Solution 1: {solution1}");
        println!("Solution 2: {solution2}");
    } else {
        println!("Discriminant is negative -> 2 complex solutions");
        // TODO: compute the 2 complex solutions
    }
}
