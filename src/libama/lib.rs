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

use token_flattener::TokenFlattener;
use code_gen::generate_rust_code;
use quasiquote::Quasiquote;
use compiler::Compiler;

pub mod compiler;
mod rust;
mod token_flattener;
mod code_gen;
mod quasiquote;

pub fn compile_anonymous_macro<'a, 'b, C>(cx: &'a rust::ExtCtxt<'b>, tts: Vec<rust::TokenTree>,
  compiler: &mut C) -> Box<rust::MacResult + 'a> where
 C: Compiler
{
  let tokens = TokenFlattener::flatten(cx, tts);
  let tokens = Quasiquote::compile(cx, tokens, compiler);
  generate_rust_code(cx, tokens)
}
