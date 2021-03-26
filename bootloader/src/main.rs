#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(lang_items)]

mod efi;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "efiapi" fn efi_main(_image_handle: efi::EFIHandle, system_table: &mut efi::EFISystemTable) -> efi::EFIStatus {
    use core::fmt::Write;

    system_table.con_out.clear_screen();
    write!(system_table.con_out, "Hello, Tachibana World\n").unwrap();
    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() { loop {} }