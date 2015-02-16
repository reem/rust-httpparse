use {Chunks};
use chunks::Fused;

pub trait Parser: Sized + 'static {
    type Error;
    type Out;

    fn subparse<C, F>(self, src: C, cb: F)
    where F: FnOnce(Result<Self::Out, Self::Error>, Fused<C>) + 'static,
          C: Chunks;

    fn parse<C, F>(self, src: C, cb: F)
    where F: FnOnce(Result<Self::Out, Self::Error>) + 'static,
          C: Chunks {
        self.subparse(src, move |res, _| cb(res))
    }

    fn and<O: Parser<Error=Self::Error>>(self, other: O) -> And<Self, O> {
        And(self, other)
    }
}

pub struct And<T, U>(T, U);

impl<T, U> Parser for And<T, U>
where T: Parser<Error=<U as Parser>::Error>,
      U: Parser<Error=<T as Parser>::Error> {
    type Out = (<T as Parser>::Out, <U as Parser>::Out);
    type Error = <T as Parser>::Error;

    fn subparse<C, F>(self, src: C, cb: F)
    where F: FnOnce(Result<<And<T, U> as Parser>::Out,
                           <U as Parser>::Error>, Fused<C>) + 'static,
          C: Chunks {
        self.0.subparse(src, move |res, chunks| {
            match res {
                Ok(first) => self.1.subparse(chunks, move |res, chunks| {
                    match res {
                        Ok(second) => cb(Ok(first, second), chunks.fuse()),
                        e @ Err(..) => cb(e, chunks.fuse())
                    }
                }),
                e @ Err(..) => cb(e, chunks.fuse())
            }
        })
    }
}
