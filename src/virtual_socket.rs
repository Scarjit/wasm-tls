use std::io::{Read, Write};

pub struct VirtualSocket {
    read_buffer: Vec<u8>,
    write_buffer: Vec<u8>,
}

impl VirtualSocket {
    pub fn new() -> Self {
        Self {
            read_buffer: Vec::new(),
            write_buffer: Vec::new(),
        }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.read_buffer.extend_from_slice(data);
    }

    pub fn get_written_data(&mut self) -> Vec<u8> {
        let data = self.write_buffer.clone();
        self.write_buffer.clear();
        data
    }

    pub fn has_data_to_read(&self) -> bool {
        !self.read_buffer.is_empty()
    }
}

impl Read for VirtualSocket {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.read_buffer.is_empty() {
            return Ok(0);
        }

        let n = std::cmp::min(buf.len(), self.read_buffer.len());
        let data = self.read_buffer.drain(0..n).collect::<Vec<u8>>();
        buf[0..n].copy_from_slice(&data);
        Ok(n)
    }
}

impl Write for VirtualSocket {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write_buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
