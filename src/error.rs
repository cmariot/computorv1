/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   error.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/04 20:50:18 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/09 11:38:54 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


use std::process;


pub fn error(message: &str) {

    // Print an error message and exit the program

    let red = "\x1b[31m";
    let reset = "\x1b[0m";

    eprintln!("{red}{message}{reset}");
    process::exit(1);

}

pub struct Error {
    is_error: bool,
    message: String,
}

impl Error {

    pub fn new() -> Error {
        Error {
            is_error: false,
            message: String::new(),
        }
    }

    pub fn set_error(&mut self, message: &str) {
        self.is_error = true;
        self.message = message.to_string();
    }

    pub fn get_error(&self) -> String {
        self.message.clone()
    }

    pub fn is_error(&self) -> bool {
        self.is_error
    }

    pub fn print_error(&self) {
        if self.is_error {
            error(&self.message);
        }
    }

}