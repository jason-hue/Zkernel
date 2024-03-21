mod context;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Trap},
    stval, stvec,
};

use core::arch::global_asm;
pub use context::TrapContext;
global_asm!(include_str!("trap.S"));
pub fn init(){
    unsafe {
        extern "C" {
            fn __alltraps();
        }
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }
}