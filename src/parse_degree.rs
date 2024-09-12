/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parse_degree.rs                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/10 18:26:12 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/12 08:38:32 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


fn no_degree_part(term: &String, i: &mut usize) -> bool {

    // Return true if the term is 'X' without a degree
    // Example : 3 * X, 1 * X ...

    return term[*i..].len() == 0;

}


fn skip_star(term: &String, i: &mut usize) -> Result<i32, String> {

    // Skip '*' character if it is present and not at the end of the term

    if term.chars().nth(*i).unwrap() == '*' {
        *i += 1;
        if *i == term.len() {
            return Err("Parsing error: Unexpected end of term after '*'".to_string());
        }
    }
    Ok(0)
}


fn check_x(term: &String, i: &mut usize, degree: &mut i32) -> Result<i8, String> {

    // Check if there is the 'X' part

    if term.chars().nth(*i).unwrap() == 'X' {
        *i += 1;
        if *i == term.len() {
            *degree = 1;
            return Ok(0);
        }
    } else {
        return Err("Parsing error: Invalid term, missing 'X'".to_string());
    }
    Ok(42)

}

fn check_exponent(term: &String, i: &mut usize) -> Result<(), String> {

    // Check if there is the '^' part

    if term.chars().nth(*i).unwrap() != '^' {
        return Err(format!("Parsing error: Invalid character after 'X' in the term {term}").to_string());
    } else {
        *i += 1;
        if *i == term.len() {
            return Err(format!("Parsing error: Invalid character after '^' in the term {term}").to_string());
        }
    }

    Ok(())
}


fn parse_exponent_degree(term: &String, i: &mut usize, degree: &mut i32) -> Result<(), String> {

    // Parse the exponent part of the term

    let degree_part = term[*i..].to_string();

    *degree = match degree_part.parse::<i32>() {
        Ok(degree) => degree,
        Err(_) => {
            return Err("Parsing error: Invalid degree value".to_string());
        }
    };

    Ok(())

}


pub fn parse_degree(term: &String, i: &mut usize, degree: &mut i32) -> Result<(), String> {

    // Parse the "* X^1" part of the terms

    if no_degree_part(term, i) {
        return Ok(());
    }
    if let Err(e) = skip_star(term, i) {
        return Err(e);
    }

    let x_ret = check_x(term, i, degree);
    if let Err(e) = x_ret {
        return Err(e);
    } else {
        let x_ret = x_ret.unwrap();
        if x_ret == 0 {
            return Ok(());
        }
    }

    if let Err(e) = check_exponent(term, i) {
        return Err(e);
    }
    if let Err(e) = parse_exponent_degree(term, i, degree) {
        return Err(e);
    }

    Ok(())

}