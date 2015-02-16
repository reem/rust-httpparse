use {Parser, Chunks};

pub trait Update: Sized + 'static {
    type Error;
    type Out;
    type Next: Update<Error=Self::Error, Out=Self::Out>;

    fn update(self, &[u8]) -> ParserResult<Self>;
}

impl<U: Update> Parser for U {
    type Error = <Self as Update>::Error;
    type Out = <Self as Update>::Out;

    fn subparse<C, F>(self, src: C, cb: F)
    where F: FnOnce(Result<<Self as Parser>::Out, <Self as Parser>::Error>, C) + 'static,
          C: Chunks {
        src.chunk(move |chunk, rest| {
            match self.update(chunk) {
                ParserResult::Next(next) => next.subparse(rest, cb),
                ParserResult::Continue(slf) => slf.subparse(rest, cb),

                ParserResult::Error(err) => {
                    cb(Err(err), rest);
                },

                ParserResult::Out(val) => {
                    cb(Ok(val), rest);
                }
            };
        })
    }
}

pub enum ParserResult<P: Update> {
    Next(P::Next),
    Error(P::Error),
    Out(P::Out),
    Continue(P)
}

