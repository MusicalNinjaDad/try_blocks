#![feature(try_blocks_heterogeneous)]

use std::{io, num};

enum PairError {
    IoError(Box<io::Error>),
    ParseError(Box<num::ParseIntError>),
}

impl From<io::Error> for PairError {
    fn from(e: io::Error) -> Self {
        Self::IoError(Box::new(e))
    }
}

impl From<num::ParseIntError> for PairError {
    fn from(e: num::ParseIntError) -> Self {
        Self::ParseError(Box::new(e))
    }
}

fn main() {
    let pair_result = try bikeshed Result<_, PairError> {
        let a = std::fs::read_to_string("hello")?;
        let b = std::fs::read_to_string("world")?;
        let c: i32 = b.parse()?;
        (a, c)
    };
}
