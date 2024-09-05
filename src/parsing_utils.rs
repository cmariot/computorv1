/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parsing_utils.rs                                   :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/04 20:59:07 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/05 15:27:19 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


pub fn is_a_sign(string: &str) -> bool {

    // Return true if the string is "-" or "+"

    if string == "+" || string == "-" {
        return true;
    }
    false

}


pub fn is_a_number(string: &str) -> bool {

    // Return true if the string is a number

    let number = string.parse::<f64>();
    number.is_ok()

}


pub fn is_multiplication_sign(string: &str) -> bool {

    // Return true if the string is "*"

    if string == "*" {
        return true;
    }
    false

}


pub fn contains(string: &str, character: char) -> bool {

    // Return true if the string contains the character

    for c in string.chars() {
        if c == character {
            return true;
        }
    }
    false

}
