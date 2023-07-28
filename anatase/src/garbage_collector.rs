use std::{mem::size_of, cell::Cell, sync::{atomic::{AtomicBool, Ordering, AtomicUsize}, Arc}, thread, time::Duration};

use crate::Stack;


static IS_GC_RUNNING : AtomicBool = AtomicBool::new(false);
static GC_REQUESTED : AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
pub struct MemoryPool<const DEBUG: bool> {
    memory: Vec<Object>, // temporary
    free: AtomicUsize,
}


#[derive(Debug)]
pub struct Object {
    data: ObjectData,
    marked: u8,
}


#[derive(Debug)]
pub enum ObjectData {
    Data([u8; 32]),
    Free(usize),
}

impl<const DEBUG: bool> MemoryPool<DEBUG> {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            memory: (0..cap).map(|x| Object { data: ObjectData::Free(x+1), marked: 0 }).collect(),
            free: AtomicUsize::new(0),
        }
    }

    
    pub fn add(&self, obj: Object) -> *mut Object {
        if self.free.load(Ordering::SeqCst) >= self.memory.capacity() {
            GarbageCollector::<DEBUG>::run_gc();
            while IS_GC_RUNNING.load(Ordering::SeqCst) || GC_REQUESTED.load(Ordering::SeqCst) { std::hint::spin_loop() }

            if self.free.load(Ordering::SeqCst) >= self.memory.capacity() {
                panic!("out of memory")
            }
        }

        let ptr = self.memory.get(self.free.load(Ordering::SeqCst)).unwrap() as *const Object;
        let ptr = ptr as *mut Object;

        let old = std::mem::replace(unsafe {&mut *ptr}, obj);
        match old.data {
            ObjectData::Free(v) => self.free.store(v, Ordering::SeqCst),
            _ => panic!("replaced a not-freed-object")
        };

        ptr
    }
}


pub struct GarbageCollector<const DEBUG: bool> {
    
}


impl<const DEBUG: bool> GarbageCollector<DEBUG> {
    pub fn start(mem: Arc<MemoryPool<DEBUG>>, stack: SendPtr<Stack<DEBUG>>) -> ! {
        loop {
            if GC_REQUESTED.load(Ordering::SeqCst) {
                println!("run gc");
                IS_GC_RUNNING.store(true, Ordering::SeqCst);

                thread::sleep(Duration::from_secs(1));
                
                IS_GC_RUNNING.store(false, Ordering::SeqCst);
                GC_REQUESTED.store(false, Ordering::SeqCst);

            }
        }
    }


    pub fn run_gc() {
        GC_REQUESTED.store(true, Ordering::SeqCst)
    }
}


pub struct SendPtr<T>(pub *mut T);


unsafe impl<T: Send> Send for SendPtr<T> {}
unsafe impl Send for Object {}
