/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   term.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/04 22:12:38 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/05 18:10:50 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


use crate::print::exponent;


pub struct Term {

    pub coefficient: f64,
    pub degree: i32,

}


impl Term {

    // Associated function (no self argument)
    pub fn new(coefficient: f64, degree: i32) -> Term {

        let sign = if coefficient < 0.0 { "- " } else { "+ " };
        print!("{}{} * {} ", sign, coefficient, exponent(degree));

        Term {
            coefficient,
            degree,
        }

    }


    // Method (self argument)
    pub fn print(&self) {

        println!("{} * X^{}", self.coefficient, self.degree);

    }

}
