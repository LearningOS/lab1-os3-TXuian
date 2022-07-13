//! Process management syscalls

use alloc::sync::Arc;

use crate::config::{MAX_APP_NUM, MAX_SYSCALL_NUM};
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, TASK_MANAGER, record_time, get_current_task_info};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

// task info should be stored in TaskControlBlock
pub struct TaskInfo {
    status: TaskStatus,
    syscall_times: [u32; MAX_SYSCALL_NUM],
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// trigered by timer
/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    record_time(us / 1_000);
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    // call task control block for info
    let (s, st, t) = get_current_task_info();
    unsafe{
        (*ti).status = s;
        (*ti).syscall_times = st;
        (*ti).time = t;
    }
    0
}
