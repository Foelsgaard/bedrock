pub struct Buffer<'m> {
    mem: &'m mut [u8],
    start: usize,
    end: usize,
}

impl<'m> Buffer<'m> {
    #[inline]
    pub fn new(mem: &'m mut [u8]) -> Self {
        Self {
            mem,
            start: 0,
            end: 0,
        }
    }

    pub fn try_read_with<E>(
        &mut self,
        f: impl FnOnce(&[u8]) -> Result<usize, E>,
    ) -> Result<usize, E> {
        let n = f(&self.mem[self.start..self.end])?;
        assert!(self.start + n <= self.end);
        self.start += n;
        if self.start == self.end {
            self.start = 0;
            self.end = 0;
        }

        Ok(n)
    }

    pub fn read_with(&mut self, f: impl FnOnce(&[u8]) -> usize) -> usize {
        self.try_read_with::<()>(|buf| Ok(f(buf))).unwrap()
    }

    #[inline]
    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        self.read_with(|rbuf| {
            let n = buf.len().min(rbuf.len());
            buf[..n].copy_from_slice(&rbuf[..n]);
            n
        })
    }

    pub fn try_write_with<E>(
        &mut self,
        f: impl FnOnce(&mut [u8]) -> Result<usize, E>,
    ) -> Result<usize, E> {
        let n = f(&mut self.mem[self.end..])?;
        assert!(self.end + n <= self.mem.len());
        self.end += n;
        if self.start > 0 && self.end == self.mem.len() {
            self.mem.copy_within(self.start..self.end, 0);
            self.end -= self.start;
            self.start = 0;
        }

        Ok(n)
    }

    pub fn write_with(&mut self, f: impl FnOnce(&mut [u8]) -> usize) -> usize {
        self.try_write_with::<()>(|buf| Ok(f(buf))).unwrap()
    }

    #[inline]
    pub fn write(&mut self, buf: &[u8]) -> usize {
        self.write_with(|wbuf| {
            let n = buf.len().min(wbuf.len());
            wbuf[..n].copy_from_slice(&buf[..n]);
            n
        })
    }
}
