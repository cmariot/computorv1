/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parse_coefficient.rs                               :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/10 17:59:57 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/11 17:35:50 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


fn get_coefficient_index_end(i: &mut usize, term: &String) -> usize {

    // Increase i value while the string contains float characters

    while *i < term.len() {
        let char = term.chars().nth(*i).unwrap();
        if !char.is_numeric() && char != '.' {
            break;
        }
        *i += 1;
    }
    *i
}


fn no_coefficient(index_start: &usize, index_end: &usize, i: &usize, term: &String) -> bool {

    // Return true if the term is 'X' without a coefficient
    // Example : X, X^2 ...

    return index_start == index_end && *i + 1 <= term.len() && term[*i..*i + 1].to_string() == "X";
}


pub fn parse_coefficient(term: &String, i: &mut usize, coefficient: &mut f64) -> Result<(), String> {

    // The coefficient is the numeric value before the 'X' part of the term

    let index_start = *i;
    let index_end = get_coefficient_index_end(i, term);

    if no_coefficient(&index_start, &index_end, i, term) {
        return Ok(());
    }

    *coefficient *= match term[index_start..index_end].parse::<f64>() {
        Ok(numeric_value) => numeric_value,
        Err(_) => {return Err("Parsing error: Invalid numeric value in the coefficient".to_string());}
    };

    Ok(())

}