/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   term.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/04 22:12:38 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/06 14:07:47 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


pub struct Term {

    pub coefficient: f64,
    pub degree: i32,
    pub first_term: bool,
    

}

use crate::print_reduced_form::{print_sign, print_coefficient, print_degree};

impl Term {

    // Associated function (no self argument)
    pub fn new(coefficient: f64, degree: i32, first_term: bool) -> Term {

        // Create a new Term and print it

        let term = Term {
            coefficient,
            degree,
            first_term,
        };

        term.print();

        term

    }

    // Method (self argument)
    pub fn print(&self) {

        let abs_coefficient = self.coefficient.abs();
        let mut first_term = self.first_term;

        print_sign(&mut first_term, self.coefficient);
        print_coefficient(self.degree, abs_coefficient);
        print_degree(&self.degree, abs_coefficient);

    }

    pub fn update_coefficient(&mut self, new_value: f64) {
        self.coefficient = new_value;
    }

}
