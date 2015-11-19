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
use rust::{Ident, Span};
use std::collections::hash_map::HashMap;

pub struct Unquote
{
  pub text_to_ident: HashMap<String, Ident>,
  pub code: String,
  pub span: Span
}

pub trait Compiler
{
  fn compile_expr(&mut self, unquote: Unquote) -> rust::P<rust::Expr>;
  fn compile_block(&mut self, unquote: Unquote) -> rust::P<rust::Block>;
}
