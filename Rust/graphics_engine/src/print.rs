use core::fmt;
use super::syscalls::{write, STDOUT};

const BUF_SIZE: usize = 256;

struct Writer {
  buf: [u8; BUF_SIZE],
  len: usize,
}
impl Writer {
  // Initialize Writer
  pub const fn new() -> Self{
    Self{
      buf: [0;BUF_SIZE],
      len: 0,
    }
  }
  // Flush the buffer to the OS
  pub fn flush(&mut self) {
    if self.len != 0 {
      write(STDOUT, self.buf.as_ptr(), self.len);
      self.len = 0;
    }
  }
}

// Implement the Write trait for formatted output
impl fmt::Write for Writer {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    for byte in s.as_bytes() {
      if self.len == BUF_SIZE {
        self.flush();
      }
    
      self.buf[self.len] = *byte;
      self.len += 1;
    }
    
    Ok(())
  }
}

/// Standard print function
pub fn _print(args: fmt::Arguments, new_line: bool) {
  use core::fmt::Write;
  
  let mut writer: Writer = Writer::new();
  
  writer.write_fmt(args).unwrap();
  
  if new_line {
    writer.write_str("\n\r").unwrap();
  }
  
  writer.flush();
}

/// Print macro
#[macro_export]
macro_rules! print {
  ($($arg:tt)*) => {{
    $crate::print::_print(format_args!($($arg)*), false);
  }};
}

/// Println macro
#[macro_export]
macro_rules! println {
  ($($arg:tt)*) => {{
    $crate::print::_print(format_args!($($arg)*), true);
  }};
}
