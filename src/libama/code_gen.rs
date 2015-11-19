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

use rust;
use rust::Token as rtok;
use rust::{TokenAndSpan, Span};

pub fn generate_rust_code<'a>(cx: &'a rust::ExtCtxt<'a>, tokens: Vec<TokenAndSpan>)
 -> Box<rust::MacResult>
{
  let reader = Box::new(TokenAndSpanArray::new(
    &cx.parse_sess().span_diagnostic,
    tokens));
  let mut parser = rust::Parser::new(cx.parse_sess(), cx.cfg(), reader);
  let expr = parser.parse_expr_panic();
  cx.parse_sess.span_diagnostic.handler.note(
    &rust::expr_to_string(&expr));
  rust::MacEager::expr(expr)
}

/// TokenAndSpanArray is used to feed the parser with tokens.
struct TokenAndSpanArray<'a>
{
  sp_diag: &'a rust::SpanHandler,
  tokens: Vec<TokenAndSpan>,
  current_idx: usize
}

impl<'a> TokenAndSpanArray<'a> {
  fn new(sp_diag: &'a rust::SpanHandler, tokens: Vec<TokenAndSpan>)
    -> TokenAndSpanArray<'a>
  {
    TokenAndSpanArray {
      sp_diag: sp_diag,
      tokens: tokens,
      current_idx: 0
    }
  }

  fn current(&self) -> TokenAndSpan {
    self.tokens[self.current_idx].clone()
  }

  fn current_span(&self) -> Span {
    self.current().sp
  }
}

impl<'a> rust::lexer::Reader for TokenAndSpanArray<'a> {
  fn is_eof(&self) -> bool {
    self.current().tok == rtok::Eof
  }

  fn next_token(&mut self) -> TokenAndSpan {
    let cur = self.current();
    self.current_idx = self.current_idx + 1;
    cur
  }

  fn fatal(&self, m: &str) -> rust::FatalError {
    self.sp_diag.span_fatal(self.current_span(), m)
  }

  fn err(&self, m: &str) {
    self.sp_diag.span_err(self.current_span(), m);
  }

  fn peek(&self) -> TokenAndSpan {
    self.current()
  }
}
