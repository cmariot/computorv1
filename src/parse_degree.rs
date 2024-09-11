/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parse_degree.rs                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/10 18:26:12 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/11 10:05:05 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


pub fn parse_degree(term: &String, i: &mut usize, degree: &mut i32) -> Result<(), &'static str> {

    // Parse the '*X^1' part of the terms

    let mut degree_part = term[*i..].to_string();

    if degree_part.len() == 0 {
        return Ok(());
    }

    // Skip '*' character if it is present and not at the end of the term
    if degree_part.chars().nth(0).unwrap() == '*' {
        *i += 1;
        if *i == term.len() {
            return Err("Parsing error: Unexpected end of term after '*'");
        }
    }

    // Check if there is the 'X' part
    degree_part = term[*i..].to_string();
    if degree_part.chars().nth(0).unwrap() == 'X' {
        *degree = 1;
        *i += 1;
        if *i == term.len() {
            return Ok(());
        }
    } else {
        return Err("Parsing error: Invalid term, missing 'X'");
    }

    // Check if there is the '^' part
    if term.chars().nth(*i).unwrap() != '^' {
        return Err("Parsing error: Invalid term, missing '^'");
    } else {
        *i += 1;
        if *i == term.len() {
            return Err("Parsing error: Unexpected end of term after '^'");
        }
    }

    // Parse the degree part
    degree_part = term[*i..].to_string();
    *degree = match degree_part.parse::<i32>() {
        Ok(degree) => degree,
        Err(_) => {
            return Err("Parsing error: Invalid degree value");
        }
    };

    Ok(())

}