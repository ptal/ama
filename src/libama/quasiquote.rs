// Copyright 2015 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use rust;
use rust::Token as rtok;
use rust::{TokenAndSpan, Span, token_to_string};
use compiler::*;

use std::collections::hash_map::HashMap;

pub struct Quasiquote<'a, 'b:'a, 'c, C> where C: 'c
{
  cx: &'a rust::ExtCtxt<'b>,
  compiler: &'c mut C,
  tokens: Vec<TokenAndSpan>,
  current_idx: usize,
  unquoted_tokens: Vec<TokenAndSpan>
}

impl<'a, 'b, 'c, C> Quasiquote<'a, 'b, 'c, C> where
  C: 'c + Compiler
{
  pub fn compile(cx: &'a rust::ExtCtxt<'b>, tokens: Vec<TokenAndSpan>, compiler: &'c mut C)
    -> Vec<TokenAndSpan>
  {
    let mut quasiquote = Quasiquote::new(cx, compiler, tokens);
    quasiquote.unquote_all();
    quasiquote.unquoted_tokens
  }

  fn new(cx: &'a rust::ExtCtxt<'b>, compiler: &'c mut C, tokens: Vec<TokenAndSpan>)
    -> Quasiquote<'a, 'b, 'c, C>
  {
    Quasiquote {
      cx: cx,
      compiler: compiler,
      tokens: tokens,
      current_idx: 0,
      unquoted_tokens: vec![]
    }
  }

  fn unquote_all(&mut self) {
    while !self.at_end() {
      match self.peek_unquote() {
        None => {
          let tok = self.token();
          self.unquoted_tokens.push(tok);
        }
        Some(d) => {
          self.unquote(d);
        }
      }
      self.bump(1);
    }
  }

  fn bump(&mut self, n: usize) {
    self.current_idx = self.current_idx + n;
  }

  fn at_end(&self) -> bool {
    self.current_idx >= self.tokens.len()
  }

  fn token(&self) -> TokenAndSpan {
    self.tokens[self.current_idx].clone()
  }

  fn peek_unquote(&self) -> Option<rust::DelimToken> {
    if self.current_idx + 1 < self.tokens.len()
      && self.tokens[self.current_idx].tok == rtok::Pound
    {
      match self.tokens[self.current_idx + 1].tok {
        rtok::OpenDelim(d@rust::DelimToken::Paren) |
        rtok::OpenDelim(d@rust::DelimToken::Brace) => Some(d),
        _ => None
      }
    }
    else {
      None
    }
  }

  fn unquote(&mut self, delim: rust::DelimToken) {
    let pound_idx = self.current_idx;
    let mut opened_delims = 1isize;
    self.bump(2);
    while !self.at_end()
      && self.still_in_quote(delim, opened_delims)
    {
      opened_delims = opened_delims + self.count_delim(delim);
      self.bump(1);
    }
    if self.at_end() || opened_delims != 1 {
      self.cx.span_fatal(self.tokens[pound_idx + 1].sp,
        "unclosed delimiter of anynomous macro.");
    }
    let unquote = self.make_unquote(pound_idx);
    self.compile_unquote(delim, unquote);
  }

  fn still_in_quote(&self, delim: rust::DelimToken,
    opened_delims: isize) -> bool
  {
    opened_delims != 1
    || self.token().tok != rtok::CloseDelim(delim)
  }

  fn count_delim(&self, delim: rust::DelimToken) -> isize {
    match self.token().tok {
      rtok::CloseDelim(d) if d == delim => -1,
      rtok::OpenDelim(d) if d == delim => 1,
      _ => 0
    }
  }

  fn compile_unquote(&mut self, delim: rust::DelimToken, unquote: Unquote) {
    let span = unquote.span;
    let non_terminal =
      match delim {
        rust::DelimToken::Paren => {
          rust::Nonterminal::NtExpr(self.compiler.compile_expr(unquote))
        }
        rust::DelimToken::Brace => {
          rust::Nonterminal::NtBlock(self.compiler.compile_block(unquote))
        }
        d => panic!("compile_unquote: unrecognized delimiter {:?}", d)
      };
    let interpolated_tok = rtok::Interpolated(non_terminal);
    let unquoted_tok = TokenAndSpan {
      tok: interpolated_tok,
      sp: span
    };
    self.unquoted_tokens.push(unquoted_tok);
  }

  fn make_unquote(&self, start_idx: usize) -> Unquote {
    let mut code = String::new();
    let mut text_to_ident = HashMap::new();
    for idx in (start_idx+2)..self.current_idx {
      if let rtok::Ident(id) = self.tokens[idx].tok {
        text_to_ident.insert(format!("{}", id), id);
      }
      code.extend(token_to_string(&self.tokens[idx].tok).chars());
      code.push(' ');
    }
    Unquote {
      text_to_ident: text_to_ident,
      code: code,
      span: self.span_from(start_idx)
    }
  }

  fn span_from(&self, start_idx: usize) -> Span {
    let mut span = self.tokens[start_idx].sp;
    span.hi = self.token().sp.hi;
    span
  }
}
