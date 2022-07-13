//! Implementation of [`TaskContext`]

#[derive(Copy, Clone)]
#[repr(C)]
/// task context structure containing some registers
pub struct TaskContext {
    ra: usize,  // reg ra, ra stores where should process run after switch.
    sp: usize,  // reg sp
    s: [usize; 12], // data of regs
}

impl TaskContext {
    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }
    pub fn goto_restore(kstack_ptr: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}
