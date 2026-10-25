#![no_std]   // No standard crate
#![no_main]

mod graphics_2d;
mod screen;
mod snake;
mod syscalls;
mod print;

use core::panic::PanicInfo;
use syscalls::*;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Open console to read (STDIN)
    open("/console".as_ptr(), 1);

    // Open console to write (STDOUT)
    open("/console".as_ptr(), 2);
    
    let status: i32 = main();
    exit(status);
}

pub fn main() -> i32 {
    print!("\n\rStart game? (y/n): ");
    let mut key: [u8; 2] = [0;2];
    read(STDIN, key.as_mut_ptr(), 1);
	  
	  match key[0] {
	    b'y' => (),
	    b'n' => return 1,
	    _ => return -2,
	  };
    
    // Clear screen
    print!("\x1B[2J");
    
    // Disable canonical mode (raw mode)
    print!("\x1B[R");
	  
    // Snake game
    snake::main_loop();
    
    // Enable canonical mode
    print!("\x1B[N");
    
    0
}

/// Panic function
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  write(STDOUT, "\x1B[N".as_ptr(), 3);
  write(STDOUT, "\n\rGRAPHICS PANIC!".as_ptr(), 17); 
  exit(-1);
}
