#![allow(dead_code)]

use std::io;

pub struct FailingWriter;

impl io::Write for FailingWriter {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        Err(io::Error::other("write failed"))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub struct FailAfterNWrites {
    remaining_successes: usize,
}

impl FailAfterNWrites {
    pub fn new(remaining_successes: usize) -> Self {
        Self {
            remaining_successes,
        }
    }
}

impl io::Write for FailAfterNWrites {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.remaining_successes == 0 {
            Err(io::Error::other("write failed"))
        } else {
            self.remaining_successes -= 1;
            Ok(buf.len())
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
