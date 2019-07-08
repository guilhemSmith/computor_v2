# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2019/06/26 17:36:42 by gsmith            #+#    #+#              #
#    Updated: 2019/07/08 20:08:52 by gsmith           ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

.PHONY: all re clean fclean test

NAME		= computorv2
DEBUG_BIN	= target/debug/$(NAME)
RELEASE_BIN	= target/release/$(NAME)
SRC			= $(addprefix src/, \
				main.rs)

ifdef DEV
	COMPILED_BIN = $(DEBUG_BIN)
else
	COMPILED_BIN = $(RELEASE_BIN)
endif

all: $(NAME)

$(NAME): $(COMPILED_BIN)
	cp $(COMPILED_BIN) $(NAME)

$(RELEASE_BIN): $(SRC)
	cargo build --release

$(DEBUG_BIN): $(SRC)
	cargo build

test:
	cargo test --no-fail-fast

re: fclean all

clean:
	cargo clean

fclean: clean
	rm -f $(NAME)
