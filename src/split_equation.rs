/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   split_equation.rs                                  :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/04 20:56:03 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/05 15:12:37 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


use crate::error;


fn character_position(string: &str, character: char) -> usize {

    // Return the position of the character in the string
    // If the character is not found, an error is raised
    // If the character is found multiple times, an error is raised
    // If the character is at the beginning or the end of the string, an error is raised

    let bytes: &[u8] = string.as_bytes();
    let mut position: usize = 0;
    let mut count: usize = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == character as u8 {
            count += 1;
            position = i;
            if count > 1 {
                error("Error: Invalid equation, multiple '=' signs");
            }
        }
    }

    if count == 0 {
        error("Error: Invalid equation, no '=' sign");
    } else if position == 0 || position == string.len() - 1 {
        error("Error: Invalid equation, '=' sign at the beginning or the end of the equation");
    }
    position

}


pub fn split_equation(equation: &String) -> (&str, &str) {

    // Use the character_position function to find the position of the '=' sign
    // Slice the equation in two parts : left side and right side
    // The right side will be multiplied by -1

    let equal_position = character_position(equation, '=');

    let left_side: &str = &equation[..equal_position];
    let right_side: &str = &equation[equal_position + 1..];

    (left_side, right_side)

}
