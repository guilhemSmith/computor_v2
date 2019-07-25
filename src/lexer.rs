/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lexer.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 16:50:34 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/25 18:51:42 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod expression;
pub mod operand;
pub mod operator;

pub use expression::Expression;
pub use operand::Operand;
pub use operator::Operator;

pub enum Token {
	Expr(Expression),
	Orand(Operand),
	Orator(Operator),
	Invalid,
}

pub fn read_input() -> Expression {
	let tik = match Operand::new("125", true) {
		Ok(op) => Token::Orand(op),
		Err(_err) => Token::Invalid,
	};
	let tok = match Operator::new('+') {
		Ok(op) => Token::Orator(op),
		Err(_err) => Token::Invalid,
	};
	let res = Token::Expr(Expression {});
	match (tik, tok, res) {
		_ => Expression {},
	}
}
