/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parse_sign.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/10 18:26:07 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/11 17:36:41 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


pub fn parse_sign(term: &String, i: &mut usize, sign: &mut f64) -> Result<i8, String> {

    // Parse the sign of the term

    if term == "+" {
        return Err("Parsing error: Unexpected '+' sign".to_string());
    } else if term == "-" {
        return Err("Parsing error: Unexpected '-' sign".to_string());
    }

    while *i < term.len() {
        let char = term.chars().nth(*i).unwrap();
        if char == '-' {
            *sign *= -1.0;
        } else if char != '+' {
            break;
        }
        *i += 1;
    }

    Ok(0)
}
