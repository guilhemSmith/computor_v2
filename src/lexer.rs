/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lexer.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 16:50:34 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/19 16:00:45 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod token;
pub use token::Token;

use token::Expression;
use token::FunctionToken;
use token::Operator;
use token::Resolve;
use token::Value;
use token::Variable;

extern crate rustyline;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::arg_parse::Param;
use crate::computor_error::ComputorError;
use crate::Timer;

use std::rc::Rc;
use std::str::Chars;

const PROMPT: &str = "> ";

pub struct Lexer {
    verbose: bool,
    bench: bool,
    line: Editor<()>,
    last_ch: Option<char>,
    depth: i32,
}

impl Lexer {
    pub fn new(param: &Param) -> Self {
        Lexer {
            verbose: param.verbose(),
            bench: param.bench(),
            line: Editor::new(),
            last_ch: None,
            depth: 0,
        }
    }

    pub fn read_input(&mut self) -> Result<Vec<Rc<Token>>, ComputorError> {
        let readline = self.line.readline(PROMPT);
        match readline {
            Ok(line) => {
                self.line.add_history_entry(line.as_str());
                if self.verbose {
                    println!("[v:Lexer] - input read: '{}'", line);
                }
                if !self.bench {
                    self.lexe(line)
                } else {
                    let _timer = Timer::new("Lexer");
                    self.lexe(line)
                }
            }
            Err(ReadlineError::Interrupted) => Err(ComputorError::io_stop()),
            Err(ReadlineError::Eof) => Err(ComputorError::io_stop()),
            Err(err) => Err(ComputorError::io(&format!("{:?}", err))),
        }
    }

    fn lexe(&mut self, l: String) -> Result<Vec<Rc<Token>>, ComputorError> {
        let cleared = self.clear_input(l);
        let mut iter = cleared.chars();
        let tokens;
        self.depth = 0;
        self.last_ch = None;
        tokens = self.tokenize(&mut iter, false);
        if self.depth == 0 && iter.next() == None {
            return Ok(tokens);
        } else {
            return Err(ComputorError::invalid_input());
        }
    }

    fn tokenize(&mut self, chars: &mut Chars, fun: bool) -> Vec<Rc<Token>> {
        let mut tokens: Vec<Rc<Token>> = Vec::new();
        let mut cur = chars.next();
        loop {
            match cur {
                Some(ch) if ch.is_alphanumeric() => {
                    self.last_ch = Some(ch);
                    tokens.push(self.read_operand(chars, fun));
                }
                Some(ch) if ch == '?' => tokens.push(Rc::new(Resolve)),
                Some(ch) if ch == '(' => {
                    self.depth += 1;
                    let expr = Expression::new(self.tokenize(chars, false));
                    if expr.count() > 0 {
                        tokens.push(Rc::new(expr));
                    }
                }
                Some(ch) if ch == ')' => {
                    self.depth -= 1;
                    break;
                }
                Some(ch) if fun && ch == ',' => {
                    self.last_ch = Some(',');
                    return tokens;
                }
                Some(ch) => match Operator::new(ch) {
                    Ok(val) => tokens.push(Rc::new(val)),
                    Err(err) => tokens.push(Rc::new(err)),
                },
                None => break,
            }
            if self.last_ch == None {
                cur = chars.next();
            } else {
                cur = self.last_ch;
                self.last_ch = None;
            }
        }
        return tokens;
    }

    fn read_operand(&mut self, chars: &mut Chars, fun: bool) -> Rc<Token> {
        if self.last_ch.unwrap().is_digit(10) {
            self.read_val(chars)
        } else {
            self.read_id(chars, fun)
        }
    }

    fn read_val(&mut self, chars: &mut Chars) -> Rc<Token> {
        let mut raw = String::new();

        raw.push(self.last_ch.unwrap());
        loop {
            match chars.next() {
                Some(ch) if ch == '.' || ch == 'i' => raw.push(ch),
                Some(ch) if ch.is_digit(10) => raw.push(ch),
                Some(ch) => {
                    self.last_ch = Some(ch);
                    break;
                }
                None => {
                    self.last_ch = None;
                    break;
                }
            }
        }
        match Value::new(raw) {
            Ok(val) => Rc::new(val),
            Err(err) => Rc::new(err),
        }
    }

    fn read_id(&mut self, chars: &mut Chars, fun: bool) -> Rc<Token> {
        let mut raw = String::new();

        raw.push(self.last_ch.unwrap());
        loop {
            match chars.next() {
                Some(ch) if ch.is_alphanumeric() => raw.push(ch),
                Some(ch) if ch == '(' => {
                    self.depth += 1;
                    let mut param_lst: Vec<Vec<Rc<Token>>> = Vec::new();
                    param_lst.push(self.tokenize(chars, true));
                    while self.last_ch == Some(',') {
                        param_lst.push(self.tokenize(chars, true));
                    }
                    match FunctionToken::new(raw, param_lst) {
                        Ok(val) => return Rc::new(val),
                        Err(err) => return Rc::new(err),
                    };
                }
                Some(ch) => {
                    self.last_ch = Some(ch);
                    break;
                }
                None => {
                    self.last_ch = None;
                    break;
                }
            }
        }
        if !(raw.len() == 1 && raw.starts_with('i')) {
            match Variable::new(raw) {
                Ok(var) => Rc::new(var),
                Err(err) => Rc::new(err),
            }
        } else {
            match Value::new(raw) {
                Ok(val) => Rc::new(val),
                Err(err) => Rc::new(err),
            }
        }
    }

    fn clear_input(&self, raw_input: String) -> String {
        let mut cleared = String::new();
        let mut iter = raw_input.trim().split_whitespace();

        loop {
            match iter.next() {
                Some(word) => cleared.push_str(word),
                None => {
                    if self.verbose {
                        println!("[v:Lexer] - input cleared: '{}'", cleared);
                    }
                    return cleared;
                }
            };
        }
    }
}
