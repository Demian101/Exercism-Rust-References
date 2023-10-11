use std::io::{Read, Result, Write};

pub struct ReadStats<R>(R, usize, usize);
pub struct WriteStats<W>(W, usize, usize);

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {  ReadStats(wrapped, 0, 0) }
    pub fn get_ref(&self) -> &R {  &self.0 }
    pub fn bytes_through(&self) -> usize { self.1  }
    pub fn reads(&self) -> usize {  self.2 }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let res = self.0.read(buf)?;
        self.1 += res;
        self.2 += 1;
        
        Ok(res)
    }
}


impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {  WriteStats(wrapped, 0, 0) }
    pub fn get_ref(&self) -> &W {  &self.0 }
    pub fn bytes_through(&self) -> usize {  self.1 }
    pub fn writes(&self) -> usize { self.2 }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let res = self.0.write(buf)?;
        self.1 += res;
        self.2 += 1;
        
        Ok(res)
    }
    // 确保: 所有写入的数据都已刷新到底层 Write Object
    fn flush(&mut self) -> Result<()> {
        self.0.flush()?;
        Ok(())
    }
}