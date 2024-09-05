/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   error.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/09/04 20:50:18 by cmariot           #+#    #+#             */
/*   Updated: 2024/09/05 13:51:33 by cmariot          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::process;

pub fn error(message: &str) {

    // Print an error message and exit the program

    eprintln!("{message}");
    process::exit(1);

}
