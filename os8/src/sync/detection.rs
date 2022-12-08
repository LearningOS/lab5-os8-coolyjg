//! Weak Bank Detector

use alloc::vec::Vec;

#[derive(Clone)]
pub struct ResourceList{
    pub avail: Vec<i32>,
    pub allocated: Vec<Vec<usize>>,
    pub need: Vec<Vec<usize>>,
}

impl ResourceList{
    pub fn new() -> Self{
        Self { avail: Vec::new(), allocated: Vec::new(), need: Vec::new()}
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
        debug!("{} alloc {}, size: {}", tid, rid, size);
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
        debug!("before alloc: allocated: {:?}", self.allocated[tid]);
        self.allocated[tid][rid] += size;
        self.avail[rid] -= size as i32;
        self.need[tid][rid] -= size;
        debug!("after alloc: allocated: {:?}", self.allocated[tid]);
        // let check = self.need[tid].iter().any(|n| *n !=0);
        // if !check{
        //     self.need[tid].clear();
        // }
    }

    pub fn is_enough(&self, rid: usize, size: usize) -> bool{
        if self.avail[rid] < size as i32{
            return false;
        }
        true
    }

    pub fn is_dead(&mut self, tid: usize, rid: usize, size: usize, task_set: Vec<bool>) -> bool{
        debug!("{} check deadlock, request {}", tid, rid);
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
        debug!("before {} add need: \navail: {:?}, allocated: {:?}, need: {:?}, task_set: {:?}", 
            tid, self.avail, self.allocated, self.need, task_set);
        self.need[tid][rid] = size;
        if tid != 0 && rid == 0{
            return false;
        }
        if self.is_enough(rid, size){
            return false;
        }
        let not_finish = task_set.iter().any(|t| *t == false);
        if !not_finish{
            return false;
        }
        let mut task_set = task_set;
        let allocated = self.allocated.clone();
        let mut avail = self.avail.clone();
        loop{
            let all_finish = task_set.iter().all(|t| *t == true);
            if all_finish{
                return false;
            }
            let mut cnt = 0;
            for i in 0..task_set.len(){
                if i >= self.need.len(){
                    break;
                }
                if task_set[i]{
                    continue;
                }
                let enough_i = self.need[i].iter().enumerate()
                    .all(|(idx, &t)| avail[idx] >= t as i32);
                if enough_i{
                    avail.iter_mut().enumerate()
                        .for_each(|(idx, n)| 
                        if idx < allocated[i].len(){
                            *n += allocated[i][idx] as i32;
                        }
                    );
                    task_set[i] = true;
                    cnt += 1;
                }
            }
            if cnt == 0{
                return true;
            }
        }
    }

    pub fn cycle(&mut self, rid: usize, size: usize, tid: usize){
        self.avail[rid] += size as i32;
        debug!("{} release {} size: {}", tid, rid, size);
        debug!("before release, allocated: {:?}", self.allocated[tid]);
        self.allocated[tid][rid] -= size;
        debug!("after release, allocated: {:?}", self.allocated[tid]);
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

    pub fn check_mutex(&mut self, tid: usize, mid: usize, task_set: Vec<bool>) -> bool{
        self.mutexes.is_dead(tid, mid, 1, task_set)
    }

    pub fn alloc_mutex(&mut self, tid: usize, mid: usize){
        self.mutexes.alloc_one(1, mid, tid);
    }

    pub fn check_semaphore(&mut self, tid: usize, sid: usize, task_set: Vec<bool>) -> bool{
        self.semes.is_dead(tid, sid, 1, task_set)
    }

    pub fn alloc_semaphore(&mut self, tid: usize, sid: usize){
        self.semes.alloc_one(1, sid, tid);
    }

}
