/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/02 22:36:19 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/05 20:40:01 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use computorv1::run;
use std::env;
mod error;
use error::error;


// struct Arguments {
//     program: String,
//     equation: String
// }


pub fn parse_argument() -> String {

    // Check if the program received 1 argument
    // Return the argument as a String

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error("Usage: cargo run \"equation\"");
    }
    args[1].trim().to_string()

}


fn main() {

    let equation: String = parse_argument();

    if let Err(_) = run(equation) {
        error(&"Fatal error.");
    }

}
