#![feature(try_blocks)]
#![feature(try_blocks_heterogeneous)]

use std::num::ParseIntError;

#[derive(Debug)]
struct Error1;

#[derive(Debug)]
struct Error2;

#[derive(Debug)]
struct Error3;

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

impl From<ParseIntError> for Error3 {
    fn from(_: ParseIntError) -> Self {
        Self
    }
}

impl From<Error2> for Error1 {
    fn from(_: Error2) -> Self {
        Self
    }
}

impl From<Error3> for Error2 {
    fn from(_: Error3) -> Self {
        Self
    }
}

fn err2(s: &str) -> Result<i32, Error2> {
    Ok(s.parse()?)
}

fn err3(s: &str) -> Result<i32, Error3> {
    Ok(s.parse()?)
}

fn heterogeneous_into_missing() {
    let x = try bikeshed Result<_, Error1> { err3("1")? + err2("2")? };
    let y = try bikeshed Result<_, Error1> { err2("1")? + err3("2")? };
    assert_eq!(x.unwrap(), y.unwrap());
}

fn main() {}
