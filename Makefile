# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                          :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: cmariot <cmariot@student.42.fr>            +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2024/09/04 10:21:35 by cmariot           #+#    #+#              #
#    Updated: 2024/09/05 10:27:27 by cmariot         ###   ########.fr        #
#                                                                              #
# **************************************************************************** #


NAME = computorv1


# Build the project with optimisations
$(NAME): build
	cargo build --release
	cp target/release/$(NAME) $(NAME)


# Build the project
build:
	cargo build
	cp target/debug/$(NAME) $(NAME)


# Build a project without producing a binary to check for errors
check:
	cargo check


# Remove the target directory
clean:
	cargo clean


# Remove the target directory and the binary
fclean: clean
	rm -f $(NAME)


# Remove the target directory and the binary and rebuild the project
re: fclean $(NAME)
