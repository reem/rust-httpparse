#![cfg_attr(test, deny(warnings))]
//#![deny(missing_docs)]

//! # httpparse
//!
//! A chunks-based, asynchronous HTTP parser.
//!

extern crate hyper;

use std::default::Default;

use hyper::header::Headers;
use hyper::status::StatusCode as Status;

use self::Next::{Continue, Break};

pub enum Next { Continue, Break }

pub trait Chunks {
    fn chunk<F>(self, F) where F: for<'a> FnOnce(&'a [u8], Self) -> Next + 'static;
}

pub trait Parser: Default + Sized + 'static {
    type Next: Parser<Error=Self::Error, Out=Self::Out>;
    type Error;
    type Out;

    fn update(self, &[u8]) -> ParserResult<Self>;

    fn parse<C, F>(self, src: C, cb: F)
    where F: FnOnce(Result<Self::Out, Self::Error>) + 'static,
          C: Chunks {
        src.chunk(move |chunk, rest| {
            match self.update(chunk) {
                ParserResult::Next(next) => next.parse(rest, cb),
                ParserResult::Error(err) => { cb(Err(err)); return Next::Break },
                ParserResult::Out(val) => { cb(Ok(val)); return Next::Break },
                ParserResult::Continue(slf) => slf.parse(rest, cb)
            };

            Next::Continue
        })
    }
}

pub enum ParserResult<P: Parser> {
    Next(P::Next),
    Error(P::Error),
    Out(P::Out),
    Continue(P)
}

