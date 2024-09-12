/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/02 22:36:19 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/11 17:34:25 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use computorv1::run;
use std::env;
mod error;
use error::error;


pub fn parse_argument() -> String {

    // Check if the program received 1 argument
    // Return the argument as a String

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error("Error: Invalid number of arguments".to_string());
    }
    args[1].trim().to_string()

}


fn main() {

    let equation: String = parse_argument();

    if let Err(error_msg) = run(equation) {
        error(error_msg);
    }

}
