extern crate core;
extern crate failure;
extern crate my_pretty_failure;
use my_pretty_failure::{myprettyfailure};
use failure::{Backtrace, Context, Fail};
use core::fmt::Display;
use std::fmt;

// -----------------------------------------
// helpme
// -----------------------------------------
fn main() {
    let err3 = ErrTest3Error::from(ErrTest3ErrorKind::ErrKind3); // println!("{:?}", err2);
    let err2 = err3.context(ErrTest2Error::from(ErrTest2ErrorKind::ErrKind2));
    let err1 = err2.context(ErrTest1Error::from(ErrTest1ErrorKind::ErrKind1));
    println!("{}", myprettyfailure(&err1));
}

// ------------------------------------------------------------------------------------
// ErrTest1
// ------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct ErrTest1Error {
    pub inner: Context<ErrTest1ErrorKind>,
}

#[derive(Fail, Debug, Clone, Eq, PartialEq)]
pub enum ErrTest1ErrorKind {
    ErrKind1,
}

impl fmt::Display for ErrTest1ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrTest1ErrorKind::ErrKind1 => { write!(f, "error message kind 1") }
        }
    }
}

impl Fail for ErrTest1Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }
    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for ErrTest1Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl ErrTest1Error {
    pub fn kind(&self) -> &ErrTest1ErrorKind {
        &*self.inner.get_context()
    }
}

impl From<ErrTest1ErrorKind> for ErrTest1Error {
    fn from(kind: ErrTest1ErrorKind) -> ErrTest1Error {
        ErrTest1Error { inner: Context::new(kind)}
    }
}

// ------------------------------------------------------------------------------------
// ErrTest2
// ------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct ErrTest2Error {
    pub inner: Context<ErrTest2ErrorKind>,
}

#[derive(Fail, Debug, Clone, Eq, PartialEq)]
pub enum ErrTest2ErrorKind {
    ErrKind2,
}

impl fmt::Display for ErrTest2ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrTest2ErrorKind::ErrKind2 => { write!(f, "error message kind 2") }
        }
    }
}

impl Fail for ErrTest2Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }
    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for ErrTest2Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl ErrTest2Error {
    pub fn kind(&self) -> &ErrTest2ErrorKind {
        &*self.inner.get_context()
    }
}

impl From<ErrTest2ErrorKind> for ErrTest2Error {
    fn from(kind: ErrTest2ErrorKind) -> ErrTest2Error {
        ErrTest2Error { inner: Context::new(kind)}
    }
}

// ------------------------------------------------------------------------------------
// ErrTest3
// ------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct ErrTest3Error {
    pub inner: Context<ErrTest3ErrorKind>,
}

#[derive(Fail, Debug, Clone, Eq, PartialEq)]
pub enum ErrTest3ErrorKind {
    ErrKind3,
}

impl fmt::Display for ErrTest3ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrTest3ErrorKind::ErrKind3 => { write!(f, "error message kind 3") }
        }
    }
}

impl Fail for ErrTest3Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }
    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for ErrTest3Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl ErrTest3Error {
    pub fn kind(&self) -> &ErrTest3ErrorKind {
        &*self.inner.get_context()
    }
}

impl From<ErrTest3ErrorKind> for ErrTest3Error {
    fn from(kind: ErrTest3ErrorKind) -> ErrTest3Error {
        ErrTest3Error { inner: Context::new(kind)}
    }
}
