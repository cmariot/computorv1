/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   print.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/05 17:59:36 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/11 17:41:11 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


pub fn exponent(nb: i32) -> String {

    // Return the unicode string of the number passed in argument
    // example : 42 -> "⁴²"

    let mut exponent = String::new();
    let mut copy = nb;
    let is_negative = nb < 0;

    if is_negative {
        copy *= -1;
    }

    loop {

        let digit = copy % 10;
        copy /= 10;

        match digit {
            0 => exponent.push('⁰'),
            1 => exponent.push('¹'),
            2 => exponent.push('²'),
            3 => exponent.push('³'),
            4 => exponent.push('⁴'),
            5 => exponent.push('⁵'),
            6 => exponent.push('⁶'),
            7 => exponent.push('⁷'),
            8 => exponent.push('⁸'),
            9 => exponent.push('⁹'),
            _ => {print!("Error")}
        }
        if copy == 0 {
            break;
        }
    }

    if is_negative {
        exponent.push('⁻');
    }

    exponent.push('X');

    exponent = exponent.chars().rev().collect::<String>();
    exponent

}


pub fn color(color: &str, text: &str) {

    if color == "blue" {
        print!("\x1b[34m");
    } else if color == "red" {
        print!("\x1b[31m");
    } else if color == "green" {
        print!("\x1b[32m");
    } else if color == "yellow" {
        print!("\x1b[33m");
    } else if color == "magenta" {
        print!("\x1b[35m");
    } else if color == "cyan" {
        print!("\x1b[36m");
    } else if color == "white" {
        print!("\x1b[37m");
    } else if color == "black" {
        print!("\x1b[30m");
    }

    print!("{}{}", text, "\x1b[0m");

}


pub fn header() {

    // Print the header of the program and an introduction

    color("cyan", "\n/* ************************************************************************** */\n");
    color("cyan", "/*                                                                            */\n");
    color("cyan", "/*  computorV1                                                                */\n");
    color("cyan", "/*  Second degree polynomial equation solver                                  */\n");
    color("cyan", "/*                                                                            */\n");
    color("cyan", "/* ************************************************************************** */\n\n");

    color("cyan", "Introduction:\n");
    println!("An equation of the second degree is an equation of the following type:");
    println!("a × X² + b × X + c = 0");
    println!("Where a, b and c are real numbers and a ≠ 0.");
    println!("This program will find the X value(s) that satisfies the equation.\n");

}


