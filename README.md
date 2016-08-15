Anonymous Procedural Macro System
=================================

[![ptal on Travis CI][travis-image]][travis]

[travis-image]: https://travis-ci.org/ptal/ama.png
[travis]: https://travis-ci.org/ptal/ama


Compiled on the nightly channel of Rust. Use [rustup](www.rustup.rs) for managing compiler channels. Download the exact same version of the compiler used with `rustup override add nightly-2016-08-12`.

This library is used for anonymously escaping code inside Rust code and avoiding repeating `my_language!(code)` everywhere. This is a tool for people implementing procedural macros and trying to integrate their language into Rust. It uses an escape mechanism (the `#` symbol) to specify we enter the user-language world.

Example with the [pcp EDSL](https://github.com/ptal/pcp/tree/master/lang) (truncated and modified for clarity):

```
pcp! {
  // ...
  for _ in 0..n {
    let n: i32 = n as i32;
    queens.push(#(variables <- 0..n));
  }
  for i in 0..n-1 {
    for j in i + 1..n {
      let a = i as i32;
      let b = j as i32;
      #{
        constraints <- queens[i] + a != queens[j] + b;
        constraints <- queens[i] - a != queens[j] - b;
      }
    }
  }
  // ...
}
```

Traditional Rust code is in the macro `pcp!` but we easily escape our user-defined language with `#(code)` or `#{code}` depending on the nature of the generated code (expression or statements). Control will be given to the user-compiler (parameter of the main function `compile_anonymous_macro`) for the code inside `#` and the generated Rust code will be automatically inserted.

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above, without any additional terms or conditions.
