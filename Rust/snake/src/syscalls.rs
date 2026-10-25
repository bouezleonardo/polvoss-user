use core::arch::asm;

pub type Pid = isize;

pub const STDIN: i32 = 0;
pub const STDOUT: i32 = 1;

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn exit(status: i32) -> ! {
    unsafe {
        asm!(
            "ecall",
            in("a0") status,
            in("a7") 1,
            options(noreturn)
        );
    }
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn fork() -> Pid {
    let ret: Pid;

    unsafe {
        asm!(
            "ecall",
            in("a7") 2,
            lateout("a0") ret,
        );
    }

    ret
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn waitpid(
    pid: Pid,
    status: *mut i32,
) -> Pid {
    let ret: Pid;

    unsafe {
        asm!(
            "ecall",
            in("a0") pid,
            in("a1") status,
            in("a7") 7,
            lateout("a0") ret,
        );
    }

    ret
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn sleep(ticks: u32) -> u32 {
    let ret: u32;

    unsafe {
        asm!(
            "ecall",
            in("a0") ticks,
            in("a7") 14,
            lateout("a0") ret,
        );
    }

    ret
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn read(
    fd: i32,
    buf: *const u8,
    count: usize,
) -> i32 {
    let ret: i32;

    unsafe {
        asm!(
            "ecall",
            in("a0") fd,
            in("a1") buf,
            in("a2") count,
            in("a7") 3,
            lateout("a0") ret,
        );
    }

    ret
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn write(
    fd: i32,
    buf: *const u8,
    count: usize,
) -> i32 {
    let ret: i32;

    unsafe {
        asm!(
            "ecall",
            in("a0") fd,
            in("a1") buf,
            in("a2") count,
            in("a7") 4,
            lateout("a0") ret,
        );
    }

    ret
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn open(
    path: *const u8,
    flags: i32,
) -> i32 {
    let ret: i32;

    unsafe {
        asm!(
            "ecall",
            in("a0") path,
            in("a1") flags,
            in("a7") 5,
            lateout("a0") ret,
        );
    }

    ret
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn uptime() -> u32 {
    let ret: u32;

    unsafe {
        asm!(
            "ecall",
            in("a7") 15,
            lateout("a0") ret,
        );
    }

    ret
}

pub fn getchar() -> i32 {
  let mut buf: [u8; 2] = [0;2];
  let bytes: i32 = read(STDIN, buf.as_mut_ptr(), 1);
  if bytes <= -1 {
    return -1;
  }
  buf[0] as i32
}
