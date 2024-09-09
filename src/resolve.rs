/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   resolve.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/04 21:00:32 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/09 10:18:35 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


use crate::{color, Term};
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

    color("cyan", "Equation resolution:\n");
    println!("The polynomial degree is 0");
    println!("The equation is: {c} = 0");

    if c == 0.0 {
        println!("All real numbers are solutions.");
        println!("There is an infinity of X values that satisfy the equation.");
    } else {
        println!("It is impossible to solve.");
        println!("There is no X value that satisfies the equation.");
    }

}


pub fn resolve_degree_1(terms: &BTreeMap<i32, Term>) {

    // Equation of the form: b*x^1 + c*x^0 = 0
    // The solution is -c / b

    let b = get_coefficient_in_terms(&1, terms);
    let c = get_coefficient_in_terms(&0, terms);
    let solution = -c / b;

    color("cyan", "Equation resolution:\n");
    println!("To solve the equation, we need to find the value of X that satisfies:");
    println!("b × x + c = 0");
    println!("We can isolate X by moving c to the right side of the equation:");
    if c < 0.0 {
        println!("{} × X = {}", b, -c);
    } else {
        println!("{} × X = -{}", b, c);
    }
    println!("Then we divide by b to find the solution:");
    if c < 0.0 {
        println!("X = {} / {}", c, b);
    } else {
        println!("X = -{} / {}", c, b);
    }
    println!("The solution is: {solution}");

}


pub fn resolve_degree_2(terms: &BTreeMap<i32, Term>) {

    // Equation of the form: a*x^2 + b*x^1 + c*x^0 = 0

    let a: f64 = get_coefficient_in_terms(&2, terms);
    let b: f64 = get_coefficient_in_terms(&1, terms);
    let c: f64 = get_coefficient_in_terms(&0, terms);

    let discriminant: f64 = b.powi(2) - 4.0 * a * c;

    color("cyan", "Equation resolution:\n");

    println!("To solve the equation, we need to find the value of X that satisfies:");
    println!("a × x² + b × x + c = 0");
    // println!("We can factorize the equation to find the solutions:");
    println!("a × (x² + b/a × x + c/a) = 0");
    // println!("This is equivalent to:");
    println!("x² + b/a × x + c/a = 0");
    println!("x² + 2 × (b/2a × x) + c/a = 0");
    // println!("x² + 2 × (b/2a × x) looks like the begin of the remarquable equation without the third part:");
    // println!("(alpha + beta)^2 = alpha² + 2 × alpha × beta + beta²");
    // println!("with alpha = x and beta = b / 2a");
    // println!("We can rewrite :");
    // println!("(x + (b / 2 × a))² as x² + 2 × (b/2a × x) + b² / 4a²");
    // println!("This is equivalent to :");
    println!("(x + (b / 2 × a))² -  b² / 4a² + c/a = 0");
    println!("(x + (b / 2 × a))² =  b² / 4a² - c/a");
    println!("(x + (b / 2 × a))² =  b² - 4ac / 4a²");
    println!("Based on this equation we can see that the left part is always positive or null");
    println!("The right part of the equation sign depend of the  b² - 4ac part sign");
    println!("We call  b² - 4ac the discriminant");
    println!("Discriminant : b² - 4 × ac -> {}² - 4 × {} × {} = {}\n", b, a, c, discriminant);

    if discriminant == 0.0 {
        let solution: f64 = -b / (2.0 * a);
        color("cyan", "The discriminant is null ;");
        println!("We can isolate x in the previous equation and find the only solution :");
        println!("x = - b / (2 × a)");
        println!(" There is one real solution");
        println!("Solution: {solution}");
    } else if discriminant > 0.0 {
        let solution1: f64 = (-b + discriminant.sqrt()) / (2.0 * a);
        let solution2: f64 = (-b - discriminant.sqrt()) / (2.0 * a);
        color("cyan", "The discriminant is positive ;");
        println!(" There are two real solutions");
        // println!("Discriminant is positive -> 2 real solutions");
        color("cyan", "Solution 1: ");
        println!("{solution1}");
        color("cyan", "Solution 2: ");
        println!("{solution2}");
    } else {
        println!("Discriminant is negative -> 2 complex solutions");
        let real_part: f64 = -b / (2.0 * a);
        let imaginary_part: f64 = (-discriminant).sqrt() / (2.0 * a);
        println!("Solution 1: {real_part} + i{imaginary_part}");
        println!("Solution 2: {real_part} - i{imaginary_part}");
    }

}
