/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   matrix.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/09/23 13:51:19 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/28 13:43:14 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{super::Lexer, LexerError, Token};
use crate::computor::{Computed, ComputorError, TreeResult};
use crate::memory::{Extension, Memory, Value};
use crate::parser::{Parser, TokenTree};
use crate::types::{Matrix, MatrixError};

use std::any::Any;
use std::fmt;

type Tokenized = Vec<Box<dyn Token>>;

pub struct MatrixToken {
    width: u32,
    height: u32,
    tokens: Vec<Tokenized>,
}

impl MatrixToken {
    pub fn new(
        lexer: &mut Lexer,
        raw: String,
    ) -> Result<MatrixToken, LexerError> {
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
                return Ok(MatrixToken {
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
        Ok(MatrixToken {
            width,
            height,
            tokens,
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn consume_tokens(&mut self) -> Vec<Tokenized> {
        let mut export: Vec<Tokenized> = Vec::new();
        std::mem::swap(&mut self.tokens, &mut export);
        return export;
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
    let depth = lexer.depth();
    for raw_cell in raw_cells {
        match lexer.lexe(String::from(raw_cell)) {
            Err(_) => {
                lexer.set_depth(depth);
                return Err(MatrixError::InvalidVal(String::from(raw_cell)));
            }
            Ok(tokens) => row.push(tokens),
        };
    }
    lexer.set_depth(depth);
    return Ok(row);
}

impl fmt::Display for MatrixToken {
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

impl fmt::Debug for MatrixToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[mat_tok:{}]", self)
    }
}

impl Token for MatrixToken {
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

pub struct MatrixTree {
    width: u32,
    height: u32,
    trees: Vec<Box<dyn TokenTree>>,
}

impl MatrixTree {
    pub fn new(
        parser: &Parser,
        width: u32,
        height: u32,
        tokens: Vec<Tokenized>,
    ) -> Result<Self, LexerError> {
        let mut mat = MatrixTree {
            width,
            height,
            trees: Vec::new(),
        };
        for cell in tokens {
            let cell_str = super::display_token(&cell);
            match parser.parse_tokens(cell) {
                None => {
                    return Err(LexerError::InvalidMat(
                        MatrixError::InvalidVal(cell_str),
                    ))
                }
                Some(tree) => mat.trees.push(tree),
            };
        }
        return Ok(mat);
    }

    pub fn trees_mut(&mut self) -> &mut Vec<Box<dyn TokenTree>> {
        &mut self.trees
    }
}

impl fmt::Display for MatrixTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut print = String::new();
        print.push('[');
        let mut i: u32 = 0;
        let mut j: u32 = 0;
        for cell in self.trees.iter() {
            if i == 0 {
                print.push('[');
            }
            i += 1;
            print.push('{');
            print += &cell.to_string();
            print.push('}');
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

impl fmt::Debug for MatrixTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[mat_tree:{}]", self)
    }
}

impl Token for MatrixTree {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_result(
        &self,
        mem: &Memory,
        ext: Option<&mut Extension>,
    ) -> TreeResult {
        let mut mat = Matrix::new(self.width, self.height);
        match ext {
            None => {
                for cell in self.trees.iter() {
                    match cell.compute(mem, None)? {
                        Computed::ValIm(val) => mat.push(val),
                        Computed::VarCall(_, val) => match val {
                            Value::Im(val) => mat.push(val),
                            Value::Mat(_) => {
                                return Err(ComputorError::matrix_val())
                            }
                        },
                        _ => return Err(ComputorError::matrix_val()),
                    };
                }
            }
            Some(extend) => {
                for cell in self.trees.iter() {
                    match cell.compute(mem, Some(&mut extend.clone()))? {
                        Computed::ValIm(val) => mat.push(val),
                        Computed::VarCall(_, val) => match val {
                            Value::Im(val) => mat.push(val),
                            Value::Mat(_) => {
                                return Err(ComputorError::matrix_val())
                            }
                        },
                        _ => return Err(ComputorError::matrix_val()),
                    };
                }
            }
        };
        return Ok(Computed::ValMat(mat));
    }
}

pub struct MatrixComp {
    mat: Matrix,
}

impl MatrixComp {
    pub fn new(mat: Matrix) -> Self {
        MatrixComp { mat }
    }
}

impl fmt::Display for MatrixComp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.mat)
    }
}

impl fmt::Debug for MatrixComp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[mat_comp:{}]", self)
    }
}

impl Token for MatrixComp {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_result(&self, _: &Memory, _: Option<&mut Extension>) -> TreeResult {
        Ok(Computed::ValMat(self.mat.clone()))
    }
}
