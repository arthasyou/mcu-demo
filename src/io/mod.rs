pub mod data;

pub struct IoDevice<const BUF: usize> {
    /// 输入缓冲区
    buffer: [u8; BUF],

    /// 实际读到的字节数
    len: usize,
}

impl<const BUF: usize> IoDevice<BUF> {
    /// 用户提供的外界读取函数（例如 USB read）
    pub fn poll_read(&mut self, read_fn: impl FnOnce(&mut [u8]) -> usize) -> Option<&[u8]> {
        let n = read_fn(&mut self.buffer);
        if n > 0 {
            self.len = n;
            Some(&self.buffer[.. n])
        } else {
            None
        }
    }

    /// 写数据：输出给外界（USB/UART）
    pub fn write(&mut self, write_fn: impl FnOnce(&[u8]), data: &[u8]) {
        write_fn(data)
    }
}
