# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2024/09/04 10:21:35 by cmariot           #+#    #+#              #
#    Updated: 2024/09/04 10:22:33 by cmariot          ###   ########.fr        #
#                                                                              #
# **************************************************************************** #


NAME = computorv1


# Build and run the executable
run:
	cargo run "="


# Build the project
build:
	cargo build


# Build the project with optimisations
opti:
	cargo build --release


# Build a project without producing a binary to check for errors
check:
	cargo check


# Remove the target directory
clean:
	cargo clean
