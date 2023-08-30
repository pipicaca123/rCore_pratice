#![no_std]
#![no_main]
#![feature(panic_info_message)]
#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

use core::arch::global_asm;
// TODO: modify with rust origin logger, not my console one.
// use log::*;
global_asm!(include_str!("entry.asm")); // first instruction in kernel

#[no_mangle] // tell compiler not to modify the name
pub fn rust_main() -> ! {
    clear_bss();
    println!("[kernel] Hello, world!");
    logging::init();
    get_kernel_base_info();
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
    // iterator [sbss_addr, sbss_addr + usize, ..., ebss_addr -usize]
}

fn get_kernel_base_info() {
    extern "C" {
        fn stext();
        fn etext();
        fn _start();
        fn sdata();
        fn edata();
        fn srodata();
        fn erodata();
        fn sbss();
        fn ebss();
    }
    info!(
        "[kernel]text range:[{:#x}, {:#x}], _start:{:#x}",
        stext as usize, etext as usize, _start as usize
    );
    info!(
        "[kernel] data range:[{:#x}, {:#x}]",
        sdata as usize, edata as usize
    );
    info!(
        "[kernel] rodata range:[{:#x}, {:#x}]",
        srodata as usize, erodata as usize
    );

    info!(
        "[kernel] bss range: [{:#x}, {:#x}]",
        sbss as usize, ebss as usize
    );
}
