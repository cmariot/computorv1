/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   term.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/04 22:12:38 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/04 23:51:00 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub struct Term {
    pub coefficient: f64,
    pub degree: i32,
}

impl Term {
    // Associated function (no self argument)
    pub fn new(coefficient: f64, degree: i32) -> Term {
        Term {
            coefficient,
            degree,
        }
    }

    // Method (self argument)
    pub fn _print(&self) {
        println!("{} * X^{}", self.coefficient, self.degree);
    }
}
