pub trait Chunks {
    type Next: Chunks;

    fn chunk<F>(self, F) where F: for<'a> FnOnce(&'a [u8], Option<Self::Next>) + 'static;
    fn attach(self, data: &[u8]) -> Fused<Self> { Fused(self, Some(data)) }
    fn fuse<'a>(self) -> Fused<'a, Self> { Fused(self, None) }
}

pub struct Fused<'a, C>(C, Option<&'a [u8]>) where C: Chunks;

impl<'a, C: Chunks> Chunks for Fused<'a, C> {
    type Next = C;

    fn chunk<F>(self, cb: F)
    where F: FnOnce(&[u8], Option<C>) + 'static {
        match self.1 {
            Some(data) => cb(data, Some(self.0)),
            None => self.0.chunk(cb)
        }
    }
}

