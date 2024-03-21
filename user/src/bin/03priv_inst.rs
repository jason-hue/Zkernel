#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use core::arch::asm;

#[no_mangle]
fn main() -> i32 {
    println!("尝试在 U 模式下执行特权指令");
    println!("内核应该杀死这个应用程序！");
    unsafe {
        asm!("sret");
    }
    0
}
