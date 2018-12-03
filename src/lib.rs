//! # my-pretty-failure
//!
//! [![Build Status](https://travis-ci.org/AlbanMinassian/my-pretty-failure.svg?branch=master)](https://travis-ci.org/AlbanMinassian/my-pretty-failure)
//! [![codecov](https://codecov.io/gh/AlbanMinassian/my-pretty-failure/branch/master/graph/badge.svg)](https://codecov.io/gh/AlbanMinassian/my-pretty-failure)
//! [![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
//! [![my-pretty-failure Latest Version](https://img.shields.io/crates/v/my-pretty-failure.svg)](https://crates.io/crates/my-pretty-failure)
//!
//! my-pretty-failure display [failure](https://github.com/rust-lang-nursery/failure) (and context) in an elegant way
//!
//! ## Example n°1
//!
//! With defaut option
//! ```rust
//! #[macro_use] extern crate failure;
//! extern crate my_pretty_failure;
//! use my_pretty_failure::myprettyfailure;
//! use failure::Fail;

//! fn main() {
//!     let err3 = format_err!("string error message 1");
//!     let err2 = err3.context(format_err!("string error message 2"));
//!     let err1 = err2.context(format_err!("string error message 1"));
//!     println!("{}", myprettyfailure(&err1));
//! }

//! ```
//!  console output
//! ```console

//! 🔥 error
//! ---------------------------------------------------------
//! a long err1
//! ---------------------------------------------------------
//!  ▶ caused by: a very long err2
//!   ▶ caused by: an another deep err3
//! ---------------------------------------------------------
//! ```
//!
//! ## Example n°2
//!
//! With your options
//! ```rust,ignore
//! #[macro_use] extern crate failure; use failure::Fail;
//! extern crate my_pretty_failure; use my_pretty_failure::{myprettyfailure_option, MyPrettyFailurePrint};
//! extern crate yansi; // or ansi_term, colored, term_painter ...

//! fn main() {
//!     let err3 = format_err!("string error message 1");
//!     let err2 = err3.context(format_err!("string error message 2"));
//!     let err1 = err2.context(format_err!("string error message 1"));
//!     println!("{}", myprettyfailure_option(MyPrettyFailurePrint {
//!         head: format!("🌈 my pretty {} catch an {}", yansi::Paint::white("superApp").bold(), yansi::Paint::red("error").bold()),
//!         separator: "****************************************".to_string(),
//!         causedby: "context".to_string(),
//!     }, &err1));
//! }

//! ```
//!  console output
//! ```console

//! 🔔 my pretty app catch an error
//! - - - - - - - - - - - - - - - - - - -
//! a long err1
//! - - - - - - - - - - - - - - - - - - -
//!  ▶ context: a very long err2
//!   ▶ context: an another deep err3
//! - - - - - - - - - - - - - - - - - - -
//! ```
//!

//! ## Links
//!
//! github: [https://github.com/AlbanMinassian/my-pretty-failure](https://github.com/AlbanMinassian/my-pretty-failure)

extern crate failure;
use failure::{Fail};

// -------------------------------------------------------------------
// struct
// -------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct MyPrettyFailurePrint {
  pub head: String,
  pub separator: String,
  pub causedby: String,
}

// -------------------------------------------------------------------
// print pretty failure message with your options
// -------------------------------------------------------------------
/// display [failure](https://github.com/rust-lang-nursery/failure) (and context) with *your* options in an elegant way
// -------------------------------------------------------------------
pub fn myprettyfailure_option(option: MyPrettyFailurePrint, fail: &Fail) -> String {
    let mut count_context = 0;
    let mut _indent = " ".to_string();
    let mut message: String = option.head;
    message.push_str("\n");
    message.push_str(&option.separator);
    message.push_str("\n");
    message.push_str(&fail.to_string());
    // message.push_str(format!("- {:?} - ", cause).as_str()); // <=== HOW DISPLAY string struct "Error<Xxxxx>" ? (is possible ?)
    // message.push_str(format!("- {:?} - ", cause).as_str()); // <=== HOW DISPLAY string enum "Error<Xxxxx>Kind" ? (is possible ?)
    message.push_str("\n");
    message.push_str(&option.separator);
    for cause in fail.iter_causes() {
        message.push_str("\n");
        message.push_str(&_indent); _indent.push_str(&" ".to_string());
        message.push_str("▶ ");
        // message.push_str(format!("- {:?} - ", cause).as_str()); // <=== HOW DISPLAY string struct "Error<Xxxxx>" ? (if exist ?, is possible ?)
        // message.push_str(format!("- {:?} - ", cause).as_str()); // <=== HOW DISPLAY string enum "Error<Xxxxx>Kind" ? (if not exist ?, is possible ?)
        message.push_str(&option.causedby);
        message.push_str(": ");
        message.push_str(&cause.to_string());
        count_context = count_context + 1;
    }
    if count_context != 0 {
      message.push_str("\n");
      message.push_str(&option.separator);
    }
    message
}

// -------------------------------------------------------------------
// print pretty failure message with default options
// -------------------------------------------------------------------
/// display [failure](https://github.com/rust-lang-nursery/failure) (and context) with *default* options in an elegant way
// -------------------------------------------------------------------
pub fn myprettyfailure(fail: &Fail) -> String {
    myprettyfailure_option(MyPrettyFailurePrint {
        head: "🔥 error".to_string(),
        separator: "---------------------------------------------------------".to_string(),
        causedby: "caused by".to_string()}, fail)
}

