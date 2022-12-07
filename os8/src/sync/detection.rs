//! Weak Bank Detector

use alloc::vec::Vec;

#[derive(Clone)]
pub struct ResourceList{
    pub avail: Vec<i32>,
    pub allocated: Vec<Vec<usize>>,
    pub need: Vec<Vec<usize>>,
    pub task_id: Vec<usize>,
}

impl ResourceList{
    pub fn new() -> Self{
        Self { avail: Vec::new(), allocated: Vec::new(), need: Vec::new(), task_id: Vec::new()}
    }

    pub fn init_size(&mut self, size: usize, rid: usize){
        if rid >= self.avail.len(){
            let n = rid - self.avail.len();
            for _ in 0..n {
                self.avail.push(0);
            }
            self.avail.push(size as i32);
        }else{
            self.avail[rid] = size as i32;
        }
    }

    pub fn alloc_one(&mut self, size: usize, rid: usize, tid: usize){
        if tid >= self.allocated.len(){
            let n = tid - self.allocated.len();
            for _ in 0..=n{
                self.allocated.push(Vec::new());
            }
        }
        if rid >= self.allocated[tid].len(){
            let n = rid - self.allocated[tid].len();
            for _ in 0..=n{
                self.allocated[tid].push(0);
            }
        }
        self.allocated[tid][rid] = size;
        self.avail[rid] -= size as i32;
        self.need[tid][rid] -= size;
    }

    pub fn is_enough(&self, rid: usize, size: usize) -> bool{
        if self.avail[rid] < size as i32{
            return false;
        }
        true
    }

    pub fn is_dead(&mut self, tid: usize, rid: usize, size: usize) -> bool{
        if tid >= self.need.len(){
            let n = tid - self.need.len();
            for _ in 0..=n{
                self.need.push(Vec::new());
            }
        }
        if rid >= self.need[tid].len(){
            let n = rid - self.need[tid].len();
            for _ in 0..=n{
                self.need[tid].push(0);
            }
        }
        self.need[tid][rid] = size;
        if !self.task_id.contains(&tid){
            self.task_id.push(tid);
        }
        if self.is_enough(rid, size){
            return false;
        }
        let mut flag = false;
        for &t in self.task_id.iter(){
            if self.need[t].len() == 0{
                flag = true;
                break;
            }else{
                let mut f = true;
                for j in 0..self.need[t].len(){
                    if self.need[t][j]!=0 && !self.is_enough(j, self.need[t][j]){
                        f = false;
                        break;
                    }
                }
                if f {
                    flag = true;
                    break;
                }
            }
        }
        !flag
    }

    pub fn cycle(&mut self, rid: usize, size: usize, tid: usize){
        self.avail[rid] += size as i32;
        self.allocated[tid][rid] -= size;
    }
}

#[derive(Clone)]
pub struct Detector{
    pub mutexes: ResourceList,
    pub semes: ResourceList,
}

impl Detector{
    pub fn new() -> Self{
        Self{
            mutexes: ResourceList::new(),
            semes: ResourceList::new(),
        }
    }

    pub fn create_mutex(&mut self, mid: usize){
        self.mutexes.init_size(1, mid);
    }

    pub fn cycle_mutex(&mut self, tid: usize, mid: usize){
        self.mutexes.cycle(mid, 1, tid);
    }

    pub fn create_sem(&mut self, sid: usize, size: usize){
        self.semes.init_size(size, sid);
    }

    pub fn cycle_sem(&mut self, tid: usize, sid: usize){
        self.semes.cycle(sid, 1, tid)
    }

    pub fn check_mutex(&mut self, tid: usize, mid: usize) -> isize{
        if self.mutexes.is_dead(tid, mid, 1){
            return -0xdead;
        }
        0
    }

    pub fn alloc_mutex(&mut self, tid: usize, mid: usize){
        self.mutexes.alloc_one(1, mid, tid);
    }

    pub fn check_semaphore(&mut self, tid: usize, sid: usize) -> isize{
        if self.semes.is_dead(tid, sid, 1){
            return -0xdead;
        }
        0
    }

    pub fn alloc_semaphore(&mut self, tid: usize, sid: usize){
        self.semes.alloc_one(1, sid, tid);
    }

}
