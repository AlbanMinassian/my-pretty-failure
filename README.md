# my-pretty-failure #

[![Build Status](https://travis-ci.org/AlbanMinassian/my-pretty-failure.svg?branch=master)](https://travis-ci.org/AlbanMinassian/my-pretty-failure)
[![codecov](https://codecov.io/gh/AlbanMinassian/my-pretty-failure/branch/master/graph/badge.svg)](https://codecov.io/gh/AlbanMinassian/my-pretty-failure)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![my-pretty-failure Latest Version](https://img.shields.io/crates/v/my-pretty-failure.svg)](https://crates.io/crates/my-pretty-failure)

my-pretty-failure display [failure](https://github.com/rust-lang-nursery/failure) (and context) in an elegant way

## Usage ##

``Cargo.toml``

```rust
[dependencies]
my-pretty-failure = "0.1.0"
```

## Usage n°1

With defaut option

```rust
extern crate my_pretty_failure;
use my_pretty_failure::myprettyfailure;

fn main() {
    let err = err1().unwrap_err(); // your failure
    println!("{}", myprettyfailure(&err)); // or panic!
}
```
 console output
```console

🔥 error
---------------------------------------------------------
a long err1
---------------------------------------------------------
 ▶ caused by: a very long err2
  ▶ caused by: an another deep err3
---------------------------------------------------------
```

## Usage n°2

With your option

```rust
extern crate my_pretty_failure;
use my_pretty_failure::{myprettyfailure_option, MyPrettyFailurePrint};
extern crate yansi; // or ansi_term, colored ...

fn main() {
    let err = err1().unwrap_err(); // your failure
    println!("{}", myprettyfailure_option(MyPrettyFailurePrint {
        head: format!("🔔 my pretty app catch an {}", yansi::Paint::red("error")),
        separator: "- - - - - - - - - - - - - - - - - - -".to_string(),
        causedby: "context".to_string(),
    }, &err));
}
```
 console output
```console

🔔 my pretty app catch an error
- - - - - - - - - - - - - - - - - - -
a long err1
- - - - - - - - - - - - - - - - - - -
 ▶ context: a very long err2
  ▶ context: an another deep err3
- - - - - - - - - - - - - - - - - - -
```

## Other example ##

``cargo run --example helpme``

## Links ##

- [documentation my-pretty-failure (docs.rs)](https://docs.rs/my-pretty-failure)

## License ##

Copyright © 2018, [Alban Minassian](https://github.com/AlbanMinassian)

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

The Software is provided “as is”, without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders X be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the Software.
Except as contained in this notice, the name of the [Alban Minassian](https://github.com/AlbanMinassian) shall not be used in advertising or otherwise to promote the sale, use or other dealings in this Software without prior written authorization from the [Alban Minassian](https://github.com/AlbanMinassian).
