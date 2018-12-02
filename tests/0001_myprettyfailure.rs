extern crate my_pretty_failure;
use my_pretty_failure::{myprettyfailure, myprettyfailure_option, MyPrettyFailurePrint};

// start copy from : https://github.com/rust-lang-nursery/failure/blob/master/examples/string_custom_error_pattern.rs
extern crate core;
extern crate failure;
use core::fmt::{self, Display};
use failure::{Backtrace, Context, Fail, ResultExt};
fn err1() -> Result<(), MyError> { Ok(err2().context("a long err1".to_string())?) }
fn err2() -> Result<(), MyError> { Ok(err3().context("a very long err2")?) }
fn err3() -> Result<(), MyError> { Ok(Err(MyError::from("an another deep err3"))?) }
#[derive(Debug)] pub struct MyError { inner: Context<String> }
impl Fail for MyError { fn cause(&self) -> Option<&Fail> { self.inner.cause()} fn backtrace(&self) -> Option<&Backtrace> {self.inner.backtrace()}}
impl Display for MyError {fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {Display::fmt(&self.inner, f)}}
impl From<&'static str> for MyError {fn from(msg: &'static str) -> MyError {MyError {inner: Context::new(msg.into()),}}}
impl From<Context<String>> for MyError {fn from(inner: Context<String>) -> MyError {MyError { inner }}}
impl From<Context<&'static str>> for MyError {fn from(inner: Context<&'static str>) -> MyError {MyError {inner: inner.map(|s| s.to_string()),}}}
// end copy from : https://github.com/rust-lang-nursery/failure/blob/master/examples/string_custom_error_pattern.rs

#[test]
fn myprettyfailure_0001_equal() {
    let err = err1().unwrap_err(); // your failure
    assert!(myprettyfailure(&err) == r#"🔥 error
---------------------------------------------------------
a long err1
---------------------------------------------------------
 ▶ caused by: a very long err2
  ▶ caused by: an another deep err3
---------------------------------------------------------"#);
}

#[test]
fn myprettyfailure_0002_option() {
    let err = err1().unwrap_err(); // your failure
    let result = myprettyfailure_option(MyPrettyFailurePrint {
        head: "🔔 error".to_string(),
        separator: "---------------------------------------------------------".to_string(),
        causedby: "context".to_string()
    }, &err);
    assert!(result == r#"🔔 error
---------------------------------------------------------
a long err1
---------------------------------------------------------
 ▶ context: a very long err2
  ▶ context: an another deep err3
---------------------------------------------------------"#);
}



