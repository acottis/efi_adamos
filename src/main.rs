#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

//------------------------------------

mod uefi;

#[no_mangle]
fn entry(_image: uefi::EfiHandle, system_table: uefi::EfiSystemTable) -> uefi::EfiStatus {
    
    //uefi::init(system_table);
    
    let stdout: &mut uefi::EfiSimpleTextOutputProtocol = unsafe { &mut *(system_table.con_out) };
    // Clears Screen
    unsafe { (stdout.clear_screen)(stdout) };

    // let vendor = system_table.firmware_vendor;
    print(stdout, "Vendor: ");
    printchar16(stdout, system_table.firmware_vendor);
    print(stdout, "\n");


    let rbits: u16 = system_table.firmware_revision as u16;
    let lbits: u16 = (system_table.firmware_revision >> 16) as u16;

    printchar16(stdout, &rbits);
    printchar16(stdout, &lbits);


    const EFI_PAGE_SIZE:u64 = 0x1000;


    // println(stdout, "\nFuck you");
    // println(stdout, "Fuck you");
    // println(stdout, "Fuck you lotss");

    //let exit: &mut uefi::EfiBootServices = unsafe { &mut *(system_table.boot_services) };

    // unsafe { (stdout.reset)(stdout, false) };
    //let test = unsafe { (exit.exit_boot_services)(image, 0xEEEE)};
    // if test != 0{
    //     println(stdout, "Error")
    // }
    print(stdout, "\n");
    uefi::EfiStatus::Unsupported
}

fn println(stdout: &mut uefi::EfiSimpleTextOutputProtocol, string: &str) -> () {
    let string = string.as_bytes();
    let mut str_buffer = [0u16; 64];

    for i in 0..string.len() {
        str_buffer[i] = string[i] as u16;
    }
    str_buffer[string.len()] = '\n' as u16;
    str_buffer[string.len() + 1] = 0x000D;
    unsafe {
        (stdout.output_string)(stdout, str_buffer.as_ptr());
    };
}

fn print(stdout: &mut uefi::EfiSimpleTextOutputProtocol, string: &str) -> () {
    let string = string.as_bytes();
    let mut str_buffer = [0u16; 64];

    for i in 0..string.len() {
        str_buffer[i] = string[i] as u16;
    }
    let result = unsafe {
        (stdout.output_string)(stdout, str_buffer.as_ptr())
    };
    if result != 0 {
        println(stdout, "Error printing");
    }
}

fn printchar16(stdout: &mut uefi::EfiSimpleTextOutputProtocol, char16: *const u16) -> () {
    let result = unsafe { (stdout.output_string)(stdout, char16) };
    if result != 0{
        println(stdout, "Error while printing char16");
    }
}

// fn conout() -> () {
//     const NULLBYTE: u16 = 0x000A;

//     static HELLO: [u16; 13] = [
//         'h' as u16, 0x0065, 0x006c, 0x006c, 0x006f, 0x0020, 0x0077, 0x006f, 0x0072, 0x006c, 0x0064,
//         0x0021, NULLBYTE,
//     ];

//     // let hello = "Hello";

//     unsafe {
//         asm!(
//         "mov rcx, [rdx + 64]",
//         "mov rax, [rcx + 8]",
//         "mov rdx, rdi",
//         //"sub rsp, 32",
//         "call rax",
//         //"add rsp, 32",
//         in("rdi") &HELLO,
//         );
//     }
// }
