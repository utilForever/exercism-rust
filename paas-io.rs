use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    read_calls: usize,
    read_bytes: usize,
    wrapped: R,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {
            read_calls: 0,
            read_bytes: 0,
            wrapped,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.read_bytes
    }

    pub fn reads(&self) -> usize {
        self.read_calls
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes = self.wrapped.read(buf)?;
        self.read_calls += 1;
        self.read_bytes += bytes;

        Ok(bytes)
    }
}

pub struct WriteStats<W> {
    write_calls: usize,
    write_bytes: usize,
    wrapped: W,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            write_calls: 0,
            write_bytes: 0,
            wrapped,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.write_bytes
    }

    pub fn writes(&self) -> usize {
        self.write_calls
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes = self.wrapped.write(buf)?;
        self.write_calls += 1;
        self.write_bytes += bytes;

        Ok(bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
