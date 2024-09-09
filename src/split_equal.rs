/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   split_equal.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/04 20:56:03 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/09 15:40:19 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


fn character_position(string: &str, character: char) -> Result<usize, &'static str> {

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
                return Err("Error: Invalid equation, multiple '=' signs");
            }
        }
    }

    if count == 0 {
        return Err("Error: Invalid equation, no '=' sign");
    } else if position == 0 || position == string.len() - 1 {
        return Err("Error: Invalid equation, '=' sign at the beginning or the end of the equation");
    }
    Ok(position)

}


pub fn split_equal(equation: &String) -> Result<(&str, &str), &'static str> {

    // Use the character_position function to find the position of the '=' sign
    // Slice the equation in two parts : left side and right side
    // The right side will be multiplied by -1

    let equal_position = match character_position(equation, '=') {
        Ok(position) => position,
        Err(error) => {
            return Err(error);
        }
    };

    let left_side: &str = &equation[..equal_position];
    let right_side: &str = &equation[equal_position + 1..];

    Ok((left_side, right_side))

}
