/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   expression.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:28:47 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/17 15:48:51 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::Token;
use std::any::Any;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub struct Expression {
    tokens: Vec<Rc<Token>>,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", super::display_token(&self.tokens, " "))
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[exp:({})]", super::debug_token(&self.tokens, ""))
    }
}

impl Token for Expression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression {
    pub fn new(tokens: Vec<Rc<Token>>) -> Self {
        Expression { tokens: tokens }
    }

    pub fn count(&self) -> usize {
        self.tokens.len()
    }
}

// impl Expression {
//     pub fn new(tokens: LinkedList<Token>) -> Self {
//         Expression { tokens: tokens }
//     }

//     pub fn len(&self) -> usize {
//         self.tokens.len()
//     }

//     pub fn front(&self) -> Option<&Token> {
//         self.tokens.front()
//     }

//     pub fn check_errors(&self, verbose: bool) -> u32 {
//         let mut count = 0;
//         let mut iter = self.tokens.iter();
//         if verbose {
//             println!("[V:computor] - collecting errors in : {}", self);
//         }
//         loop {
//             match iter.next() {
//                 Some(tok) => match tok {
//                     Token::Invalid(err) => {
//                         count += 1;
//                         println!("{}", err);
//                     }
//                     Token::Expr(expr) => count += expr.check_errors(verbose),
//                     _ => {}
//                 },
//                 None => break,
//             }
//         }
//         return count;
//     }

//     pub fn compute(&self, verbose: bool) -> Result<Self, ComputorError> {
//         if verbose {
//             println!("[V:computor] - computing expression: {}", self);
//         }
//         let mut result = self.tokens.clone();
//         result = compute_expr(result, verbose)?;
//         if verbose {
//             println!(
//                 "[V:computor] - sub expressions computed: {}",
//                 super::tokens_to_string(&result)
//             );
//         }
//         result = compute_all(result, true, verbose)?;
//         if verbose {
//             println!(
//                 "[V:computor] - prior operations computed: {}",
//                 super::tokens_to_string(&result)
//             );
//         }
//         if result.len() > 1 {
//             result = compute_all(result, false, verbose)?;
//             if verbose {
//                 println!(
//                     "[V:computor] - remaining operations computed: {}",
//                     super::tokens_to_string(&result)
//                 );
//             }
//         }
//         if result.len() % 2 == 0 {
//             return Err(ComputorError::invalid_expr());
//         }
//         Ok(Expression { tokens: result })
//     }
// }

// fn compute_expr(
//     mut lst: LinkedList<Token>,
//     verbose: bool,
// ) -> Result<LinkedList<Token>, ComputorError> {
//     let mut result: LinkedList<Token> = LinkedList::new();
//     loop {
//         match lst.pop_front() {
//             Some(tok) => match tok {
//                 Token::Expr(exp) => {
//                     let computed = exp.compute(verbose)?;
//                     if computed.len() == 1 {
//                         match computed.front() {
//                             Some(tok_res) => result.push_back(tok_res.clone()),
//                             None => result.push_back(Token::Expr(computed)),
//                         };
//                     } else {
//                         result.push_back(Token::Expr(computed));
//                     }
//                 }
//                 _ => result.push_back(tok),
//             },
//             None => break,
//         }
//     }
//     return Ok(result);
// }

// fn compute_all(
//     mut lst: LinkedList<Token>,
//     prior: bool,
//     verbose: bool,
// ) -> Result<LinkedList<Token>, ComputorError> {
//     let mut result: LinkedList<Token> = LinkedList::new();
//     while lst.len() > 2 {
//         let mut reduced = compute_op(&mut lst, prior, verbose)?;
//         result.append(&mut reduced);
//     }
//     loop {
//         match lst.pop_front() {
//             Some(tok) => result.push_back(tok),
//             None => break,
//         };
//     }
//     Ok(result)
// }

// fn compute_op(
//     lst: &mut LinkedList<Token>,
//     prior: bool,
//     verbose: bool,
// ) -> Result<LinkedList<Token>, ComputorError> {
//     let remain = lst.split_off(3);
//     let lst_orand = (lst.pop_front(), lst.pop_back());
//     let orator = match lst.pop_front() {
//         Some(tok) => match tok {
//             Token::Orator(op) => op,
//             _ => return Err(ComputorError::invalid_expr()),
//         },
//         None => return Err(ComputorError::invalid_expr()),
//     };
//     *lst = remain;
//     let orands = (
//         match lst_orand.0 {
//             Some(tok) => tok,
//             None => return Err(ComputorError::invalid_expr()),
//         },
//         match lst_orand.1 {
//             Some(tok) => tok,
//             None => return Err(ComputorError::invalid_expr()),
//         },
//     );

//     let mut result: LinkedList<Token> = LinkedList::new();
//     if orator.prior() == prior {
//         let mut op_result = orator.exec(&orands.0, &orands.1, verbose)?;
//         match op_result.pop_back() {
//             Some(tok) => lst.push_front(tok),
//             None => lst.push_front(orands.1),
//         };
//     } else {
//         lst.push_front(orands.1);
//         result.push_front(Token::Orator(orator));
//         result.push_front(orands.0);
//     }
//     return Ok(result);
// }
