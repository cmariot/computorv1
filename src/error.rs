/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   error.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/04 20:50:18 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/11 17:32:45 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


use std::process;


pub fn error(message: String) {

    // Print an error message and exit the program

    let red = "\x1b[31m";
    let reset = "\x1b[0m";

    eprintln!("{red}{message}{reset}");
    process::exit(1);

}
