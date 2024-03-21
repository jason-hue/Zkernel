#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use riscv::register::sstatus::{self, SPP};

#[no_mangle]
fn main() -> i32 {
    println!("尝试在 U 模式下访问特权 CSR");
    println!("内核应该杀死这个应用程序！");
    unsafe {
        sstatus::set_spp(SPP::User);
    }
    0
}
