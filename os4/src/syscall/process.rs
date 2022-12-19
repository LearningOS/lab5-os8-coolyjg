//! Process management syscalls

use crate::config::{MAX_SYSCALL_NUM, PAGE_SIZE};
use crate::mm::{VirtAddr, virtaddr2phyaddr};
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, 
    TaskStatus, get_cur_start_time, get_cur_syscall, mmap, munmap};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[derive(Clone, Copy)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}

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

// YOUR JOB: 引入虚地址后重写 sys_get_time
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    // let us = get_time_us();
    let virt_us: VirtAddr = (ts as usize).into();
    if let Some(pa) = virtaddr2phyaddr(virt_us){
        let us = get_time_us();
        let phy_ts = pa.0 as *mut TimeVal;
        unsafe{
            *phy_ts = TimeVal{
                sec: us / 1_000_000,
                usec: us % 1_000_000,
            };
        }
        0
    }else{
        -1
    }
}

// CLUE: 从 ch4 开始不再对调度算法进行测试~
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

// YOUR JOB: 扩展内核以实现 sys_mmap 和 sys_munmap
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    if start % 0x1000 != 0{
        return -1;
    }
    if port & !0x7 != 0 || port == 0{
        return -1;
    }
    let mut ll = len;
    if len % 0x1000 !=0 {
        ll = (len/PAGE_SIZE + 1) * PAGE_SIZE;
    }

    mmap(start, ll, port)
}

pub fn sys_munmap(start: usize, len: usize) -> isize {
    if start % PAGE_SIZE != 0 || len % PAGE_SIZE !=0{
        return -1;
    }
    munmap(start, len)
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let virt_ti: VirtAddr = (ti as usize).into();
    if let Some(pa) = virtaddr2phyaddr(virt_ti){
        let now = get_time_us();
        let start = get_cur_start_time();
        let st = get_cur_syscall();
        let phy_ti = pa.0 as *mut TaskInfo;
        unsafe {
            *phy_ti = TaskInfo{
                status: TaskStatus::Running,
                syscall_times: st,
                time: (now - start) / 1_000,
            };
        }
        0
    }else{
        -1
    }
}
