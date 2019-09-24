/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   matrix.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/09/23 13:51:19 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/24 14:19:07 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{super::Lexer, LexerError, Token};
use crate::computor::{ComputorError, TreeResult};
use crate::memory::{Extension, Memory};
use crate::types::MatrixError;

use std::any::Any;
use std::fmt;

type Tokenized = Vec<Box<dyn Token>>;

pub struct MatrixUnparsed {
    width: u32,
    height: u32,
    tokens: Vec<Vec<Box<dyn Token>>>,
}

impl MatrixUnparsed {
    pub fn new(
        lexer: &mut Lexer,
        raw: String,
    ) -> Result<MatrixUnparsed, LexerError> {
        let width: u32;
        let mut height: u32 = 0;
        let mut tokens: Vec<Tokenized> = Vec::new();

        if !raw.starts_with("[") || !raw.ends_with("]") {
            return Err(LexerError::InvalidMat(MatrixError::InvalidFormat));
        }
        let mut raw_rows = raw[1..raw.len() - 1].split(";");
        match raw_rows.next() {
            Some(first_row) => {
                let mut first = match new_row(lexer, first_row) {
                    Err(err) => return Err(LexerError::InvalidMat(err)),
                    Ok(row) => row,
                };
                width = first.len() as u32;
                height += 1;
                tokens.append(&mut first);
            }
            None => {
                return Ok(MatrixUnparsed {
                    width: 0,
                    height: 0,
                    tokens,
                })
            }
        }
        for raw_row in raw_rows {
            let mut row = match new_row(lexer, raw_row) {
                Err(err) => return Err(LexerError::InvalidMat(err)),
                Ok(row) => row,
            };
            if row.len() as u32 != width {
                return Err(LexerError::InvalidMat(
                    MatrixError::IncoherentSizes,
                ));
            }
            height += 1;
            tokens.append(&mut row);
        }
        Ok(MatrixUnparsed {
            width,
            height,
            tokens,
        })
    }
}

fn new_row(
    lexer: &mut Lexer,
    raw_row: &str,
) -> Result<Vec<Tokenized>, MatrixError> {
    if !raw_row.starts_with("[") || !raw_row.ends_with("]") {
        return Err(MatrixError::InvalidFormat);
    }
    let mut row: Vec<Tokenized> = Vec::new();
    let raw_cells = raw_row[1..raw_row.len() - 1].split(",");
    for raw_cell in raw_cells {
        match lexer.lexe(String::from(raw_cell)) {
            Err(_) => {
                return Err(MatrixError::InvalidVal(String::from(raw_cell)))
            }
            Ok(tokens) => row.push(tokens),
        };
    }
    return Ok(row);
}

impl fmt::Display for MatrixUnparsed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut print = String::new();
        print.push('[');
        let mut i: u32 = 0;
        let mut j: u32 = 0;
        for cell in self.tokens.iter() {
            if i == 0 {
                print.push('[');
            }
            i += 1;
            print.push('"');
            print += &super::display_token(&cell);
            print.push('"');
            if i == self.width {
                print.push(']');
                i = 0;
                j += 1;
                if j < self.height {
                    print.push(';');
                }
            } else {
                print.push(',');
            }
        }
        print.push(']');
        write!(f, "{}", print)
    }
}

impl fmt::Debug for MatrixUnparsed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[mat_!pars:{}]", self)
    }
}

impl Token for MatrixUnparsed {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_result(
        &self,
        _mem: &Memory,
        _ext: Option<&mut Extension>,
    ) -> TreeResult {
        Err(ComputorError::unparsed_token(self))
    }
}
