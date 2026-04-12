//! A playground was linked in one of the tickets somewhere ... can't find it again, so here's a new
//! link to same contents as it was still persisted in my browser's local storage
//! https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=b34a769829732813491fabdb97d8a409

#![feature(try_blocks_heterogeneous)]

use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct MyError;

impl Error for MyError {}
impl Display for MyError {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

fn foo(x: std::io::Result<i32>, y: Result<i32, MyError>) {
    // This would error
    // dbg!(try { x? + y? });

    // But this works
    let _ = dbg!(try bikeshed anyhow::Result<_> { x? + y? });
}

#[test]
fn ok() {
    foo(Ok(1), Ok(2));
}

#[test]
fn err() {
    foo(Err(std::io::Error::last_os_error()), Ok(2));
}
