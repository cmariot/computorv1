/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parsing_util.rs                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/10 18:25:58 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/10 18:26:04 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


pub fn replace_whitespace(from: &mut String, by: String) -> String {

    // Replace all whitespaces in a string

    let mut result = String::new();
    for c in from.chars() {
        if c.is_whitespace() {
            result.push_str(&by);
        } else {
            result.push(c);
        }
    }
    result

}

pub fn split_equation_terms(str: &str, delimiters: &[char]) -> Vec<String> {

    // Split a string in a Vector of &str
    // The delimiters are included in the at the beginning of each element
    // "3 + 2" -> V[(('3'), ('+2'))]

    let mut result: Vec<String> = Vec::new();
    let mut word = String::new();

    for c in str.chars() {
        if delimiters.contains(&c) {
            if !word.is_empty() {
                result.push(word.clone());
                word = String::new();
            }
            word.push(c);
        } else {
            word.push(c);
        }
    }
    if !word.is_empty() {
        result.push(word.clone());
    }
    result
}