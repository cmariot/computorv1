/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   irreducible_fraction.rs                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/11 10:20:57 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/11 14:26:20 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::print::color;


// Fonction pour trouver l'approximation rationnelle d'un float
fn approx_fraction(mut num: f64, mut denom: f64) -> (i64, i64) {

    // Tant que le numérateur ou le dénominateur n'est pas un entier
    while num.fract() != 0.0 || denom.fract() != 0.0 {
        num *= 10.0;
        denom *= 10.0;
    }

    (num.round() as i64, denom.round() as i64)
}

// Fonction pour calculer le plus grand commun diviseur (GCD) avec l'algorithme d'Euclide
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs() // Retourne le GCD absolu
}

// Fonction pour simplifier la fraction
fn simplify_fraction(num: i64, denom: i64) -> (i64, i64) {
    let divisor = gcd(num, denom);
    (num / divisor, denom / divisor)
}

// Fonction principale qui prend deux floats et affiche la fraction irréductible
pub fn display_fraction(num: f64, denom: f64) {

    if denom == 0.0 {return ;}
    let (num_scaled, denom_scaled) = approx_fraction(num, denom);
    let (num_simple, denom_simple) = simplify_fraction(num_scaled, denom_scaled);

    print!("{num_simple} / {denom_simple}");
}


pub fn print_solution(solution: f64, numerator: f64, denominator: f64, solution_number: i8) {

    // Print the solution of the equation

    if solution_number == 0 {
        color("cyan", "Solution:\n");
    } else {
        color("cyan", &format!("Solution {solution_number}:\n").to_string());
    }
    if solution.fract() == 0.0 {
        println!("X = {}", solution);
    }
    else {
        print!("X = ");
        display_fraction(numerator, denominator);
        println!(" ≈ {}", solution);
    }

}