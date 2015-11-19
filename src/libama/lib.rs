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

#![feature(rustc_private, quote)]

extern crate syntax;

use rust::Token as rtok;
use rust::{TokenAndSpan, Span, token_to_string};
use token_flattener::*;
use code_gen::*;

use std::collections::hash_map::HashMap;

mod rust;
mod token_flattener;
mod code_gen;

pub struct Expander<'a>
{
  cx: &'a rust::ExtCtxt<'a>,
  tokens: Vec<TokenAndSpan>
}

impl<'a> Expander<'a>
{
  pub fn new(cx: &'a rust::ExtCtxt, tts: Vec<rust::TokenTree>)
    -> Expander<'a>
  {
    Expander{
      cx: cx,
      tokens: TokenFlattener::flatten(cx, tts)
    }
  }

  pub fn expand(mut self) -> Box<rust::MacResult + 'a> {
    self.replace_anonymous_macros();
    code_gen::generate_rust_code(self.cx, self.tokens)
  }

  fn start_of_anon_macro(&self, idx: usize, delim: rust::DelimToken) -> bool {
       idx + 1 < self.tokens.len()
    && self.tokens[idx].tok == rtok::Pound
    && self.tokens[idx + 1].tok == rtok::OpenDelim(delim)
  }

  fn span_between(&self, start_idx: usize, end_idx: usize) -> Span {
    let mut span = self.tokens[start_idx].sp;
    span.hi = self.tokens[end_idx].sp.hi;
    span
  }

  fn span_token(&self, tok: rtok, start_idx: usize, end_idx: usize) -> TokenAndSpan {
    TokenAndSpan {
      tok: tok,
      sp: self.span_between(start_idx, end_idx)
    }
  }

  fn replace_anonymous_macros(&mut self) {
    let mut idx = 0;
    let mut new_tokens = vec![];
    let delim = rust::DelimToken::Paren;
    while idx < self.tokens.len() {
      if self.start_of_anon_macro(idx, delim) {
        let pound_idx = idx;
        let open_brace_idx = idx + 1;
        let mut opened_braces = 1;
        idx = idx + 2;
        while idx < self.tokens.len()
         && (opened_braces != 1
         || self.tokens[idx].tok != rtok::CloseDelim(delim))
        {
          opened_braces = opened_braces +
            match self.tokens[idx].tok {
              rtok::CloseDelim(d) if d == delim => -1,
              rtok::OpenDelim(d) if d == delim => 1,
              _ => 0
            };
          idx = idx + 1;
        }
        if idx == self.tokens.len() || opened_braces != 1 {
          self.cx.span_fatal(self.tokens[open_brace_idx].sp,
            "unclosed delimiter of anynomous macro.");
        }
        let interpolated_tok = self.compile_anonymous_macro(pound_idx, idx);
        let tok_and_span = self.span_token(interpolated_tok, pound_idx, idx);
        new_tokens.push(tok_and_span);
      }
      else {
        new_tokens.push(self.tokens[idx].clone());
      }
      idx = idx + 1;
    }
    self.tokens = new_tokens;
  }

  fn compile_anonymous_macro(&self, start: usize, end: usize) -> rtok {
    let mut text = String::new();
    let span = self.span_between(start, end);
    let mut text_to_ident = HashMap::new();
    for idx in (start+2)..end {
      if let rust::Token::Ident(id, rust::IdentStyle::Plain) = self.tokens[idx].tok {
        text_to_ident.insert(format!("{}", id), id);
      }
      text.extend(token_to_string(&self.tokens[idx].tok).chars());
      text.push(' ');
    }
    // let code_gen = CodeGenerator::new(self.cx, text_to_ident, span);
    // let expr = code_gen.generate_expr(text);
    rtok::Interpolated(rust::Nonterminal::NtExpr(quote_expr!(self.cx, {})))
  }
}
