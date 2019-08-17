# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2019/06/26 17:36:42 by gsmith            #+#    #+#              #
#    Updated: 2019/08/17 10:53:07 by gsmith           ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

.PHONY: all re clean fclean test

NAME		= computorv2
DEBUG_BIN	= target/debug/$(NAME)
RELEASE_BIN	= target/release/$(NAME)
SRC			= $(addprefix src/, \
				arg_parse.rs \
				computor_error.rs \
				main.rs \
				memory.rs \
				$(addprefix memory/, \
					function.rs \
					variable.rs) \
				lexer.rs \
				$(addprefix lexer/, \
					token.rs \
					token/expression.rs \
					token/function.rs \
					token/operator.rs \
					token/lexer_error.rs \
					token/resolve.rs \
					token/value.rs \
					token/variable.rs) \
				timer.rs \
				types.rs \
				$(addprefix types/, \
					imaginary.rs \
					rational.rs) \
)

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
