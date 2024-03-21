use core::arch::asm;
use lazy_static::lazy_static;
use crate::println;
use crate::sync::UPSafeCell;
const MAX_APP_NUM: usize = 16;
const USER_STACK_OFFSET: usize = 4096*2;
const KERNEL_STACK_SIZE: usize = 4096*2;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x200000;
struct AppManager{
    num_app: usize,
    current_app: usize,
    app_start: [usize;MAX_APP_NUM+1],
}
#[repr(align(4096))]
struct KernelStack{
    data: [u8;KERNEL_STACK_SIZE],
}
#[repr(align(4096))]
struct UserStack{
    data: [u8;KERNEL_STACK_SIZE],
}
impl UserStack{
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
}
static KERNEL_STACK: KernelStack = KernelStack{
    data: [0;KERNEL_STACK_SIZE],
};
static USER_STACK: UserStack = UserStack{
    data: [0;KERNEL_STACK_SIZE],
};
lazy_static!{
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe{
        UPSafeCell::new({
            extern "C"{
                fn _num_app();
            }
            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = num_app_ptr.read_volatile();
            let mut app_start: [usize;MAX_APP_NUM+1] = [0;MAX_APP_NUM+1];
            let app_start_raw: &[usize] =  core::slice::from_raw_parts(
            num_app_ptr.add(1), num_app + 1);

            app_start[..=num_app].copy_from_slice(app_start_raw);
            AppManager {
                num_app,
                current_app: 0,
                app_start,
            }
        })};
}
impl AppManager{
    unsafe fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app {
            panic!("All applications completed!");
        }
        println!("[kernel] Loading app_{}", app_id);
        // clear app area
        core::slice::from_raw_parts_mut(
            APP_BASE_ADDRESS as *mut u8,
            APP_SIZE_LIMIT
        ).fill(0);
        let app_src = core::slice::from_raw_parts(
            self.app_start[app_id] as *const u8,
            self.app_start[app_id + 1] - self.app_start[app_id]
        );
        let app_dst = core::slice::from_raw_parts_mut(
            APP_BASE_ADDRESS as *mut u8,
            app_src.len()
        );
        app_dst.copy_from_slice(app_src);
        // memory fence about fetching the instruction memory
        asm!("fence.i");
    }
}