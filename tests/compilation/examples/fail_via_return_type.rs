#![feature(try_blocks)]

use std::num::ParseIntError;

#[derive(Debug)]
struct Error1;

#[derive(Debug)]
struct Error2;

impl From<ParseIntError> for Error1 {
    fn from(_: ParseIntError) -> Self {
        Self
    }
}

impl From<ParseIntError> for Error2 {
    fn from(_: ParseIntError) -> Self {
        Self
    }
}

impl From<Error1> for Error2 {
    fn from(_: Error1) -> Self {
        Self
    }
}

impl From<Error2> for Error1 {
    fn from(_: Error2) -> Self {
        Self
    }
}

fn err1(s: &str) -> Result<i32, Error1> {
    Ok(s.parse()?)
}

fn err2(s: &str) -> Result<i32, Error2> {
    Ok(s.parse()?)
}

fn heterogeneous_via_return_type() -> Result<(), Error1> {
    let x = try { err1("1")? + err2("2")? };
    let y = try { err2("1")? + err1("2")? };
    assert_eq!(x.unwrap(), y.unwrap());
    Ok(())
}

fn main() {}
