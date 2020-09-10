#![no_std]
#![no_main]
#![feature(asm)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn entry() -> () {
    // let mut _x: u64 = 3;

    static _hello1: &str = "Hello";
    static hello: char = 'h';

    let hello2 = "Hello World!\x0A\x0D\x00";
    let hello3: [u16; 15] = [
        0x0048, 0x0065, 0x006c, 0x006c, 0x006f, 0x0020, 0x0077, 0x006f, 0x0072, 0x006c, 0x0064,
        0x0021, 0x000a, 0x000d, 0x0000,
    ];
    // let hello = &hello1 as *const char;
    unsafe {
        asm!(
        "mov rcx, [rdx + 64]",
        "mov rax, [rcx + 8]",
        "mov rdx, rdi ",
        "sub rsp, 32",
        "call rax",
        "add rsp, 32",
        in("rdi") &hello3,
        );
    }
    // let hdr: u64;

    // unsafe {
    //     asm!("mov {0}, [rdx + 64]", out(reg) hdr);
    // }

    //print(hdr+8);
    //loop {}
}
