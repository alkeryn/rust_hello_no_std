#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    const STDOUT: i32 = 1;
    const SYS_WRITE: i64 = 1;
    const SYS_EXIT: i64 = 60;

    let msg = b"hello world\n";

    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") SYS_WRITE,
            in("rdi") STDOUT,
            in("rsi") msg.as_ptr(),
            in("rdx") msg.len(),
            lateout("rax") _,
            lateout("rcx") _,
            lateout("r11") _,
        );

        core::arch::asm!(
            "syscall",
            in("rax") SYS_EXIT,
            in("rdi") 0,
            options(noreturn)
        );
    }
}
