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
#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext)->&mut TrapContext{
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall)=>{
            cx.sepc+=4;
            cx.x[10]= crate::syscall::syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) =>{
            crate::println!("[kernel] 应用程序中的 PageFault，内核杀死了它。");
            crate::batch::run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            crate::println!("[kernel] IllegalInstruction in application, kernel killed it.");
            crate::batch::run_next_app();
        }
        _ => {
            panic!("Unsupported trap {:?}, stval = {:#x}!", scause.cause(), stval);
        }
    }
    cx
}