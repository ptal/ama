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
use rust::TokenAndSpan;

pub struct TokenFlattener<'a>
{
  rp: rust::Parser<'a>,
  tokens: Vec<TokenAndSpan>
}

impl<'a> TokenFlattener<'a>
{
  pub fn flatten(cx: &'a rust::ExtCtxt, tts: Vec<rust::TokenTree>)
   -> Vec<TokenAndSpan>
  {
    let mut flattener = TokenFlattener::new(cx, tts);
    flattener.flatten_tokens();
    flattener.tokens
  }

  fn new(cx: &'a rust::ExtCtxt, tts: Vec<rust::TokenTree>)
    -> TokenFlattener<'a>
  {
    TokenFlattener {
      rp: rust::new_parser_from_tts(cx.parse_sess(), cx.cfg(), tts),
      tokens: vec![]
    }
  }

  fn flatten_tokens(&mut self) {
    self.push_open_brace();
    loop {
      if self.rp.token == rtok::Eof {
        self.push_close_brace();
        self.push_current_tok();
        break;
      }
      self.push_current_tok();
      self.rp.bump();
    }
  }

  fn push_open_brace(&mut self) {
    self.push_tok(rtok::OpenDelim(rust::DelimToken::Brace));
  }

  fn push_close_brace(&mut self) {
    self.push_tok(rtok::CloseDelim(rust::DelimToken::Brace));
  }

  fn push_current_tok(&mut self) {
    let cur = self.token_and_span();
    self.tokens.push(cur);
  }

  fn token_and_span(&mut self) -> TokenAndSpan {
    TokenAndSpan {
      tok: self.rp.token.clone(),
      sp: self.rp.span
    }
  }

  fn push_tok(&mut self, tok: rtok) {
    self.tokens.push(TokenAndSpan {
      tok: tok,
      sp: self.rp.span
    })
  }
}
