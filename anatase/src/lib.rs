use std::{fmt::Debug, mem::size_of, borrow::BorrowMut};

mod runtime;
mod bytecode;
pub mod garbage_collector;


#[derive(Debug)]
pub struct VM<const DEBUG: bool> {
    pub stack: Stack<DEBUG>,
    pub callstack: Vec<Code<DEBUG>>,
    pub current: Code<DEBUG>,
    pub constants: Box<[Data]>,
}


#[derive(Debug)]
pub struct Stack<const DEBUG: bool> {
    values: Vec<Data>,
    bottom: usize,
    top:    usize,
}


impl<const DEBUG: bool> Stack<DEBUG> {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            values: Vec::with_capacity(cap),
            bottom: 0,
            top   : 0,
        }
    }


    #[inline(always)]
    pub fn reg(&self, reg: u8) -> Data {
        if DEBUG {
            assert!(self.bottom.checked_add(reg as usize).unwrap() <= self.top);
        }

        unsafe {
            *self.values.get_unchecked(self.bottom.wrapping_add(reg as usize))
        }
    }


    #[inline(always)]
    pub fn set_reg(&mut self, reg: u8, data: Data) {
        if DEBUG {            
            assert!(
                self.bottom.checked_add(reg as usize).unwrap() <= self.top,
                "target register: {reg}, bottom: {}, top: {}",
                self.bottom,
                self.top,
            );
        }

        unsafe {
            *self.values.get_unchecked_mut(self.bottom.wrapping_add(reg as usize)) = data;
        }
    }


    pub fn reg_ptr(&mut self, reg: u8) -> *const Data {
        unsafe { self.values.get_unchecked(self.bottom + reg as usize) }
    }


    #[inline(always)]
    fn pop(&mut self, amount: usize) {
        if DEBUG {
            assert!(self.top
                .checked_sub(amount)
                .is_some_and(|x| self.bottom <= x),
                "{} {} | {amount}", self.top, self.bottom,
            );
        }

        
        self.top -= amount;
    }


    #[inline(always)]
    fn push(&mut self, amount: usize) {
        if DEBUG {
            assert!(self.top
                .checked_add(amount)
                .is_some_and(|x| x < self.values.capacity())
            );
        }

        self.top += amount;
    }
}


#[derive(Clone, Copy)]
pub struct Data {
    tag: u64,
    inner: InnerData,
}


impl Data {
    #[inline(always)]
    fn new(tag: u64, inner: InnerData) -> Self { 
        Self { tag, inner } 
    }
    
    pub fn new_uninit() -> Self { Self::new(Self::TAG_UNINIT, InnerData { uninit: () })}
    #[inline(always)]
    pub fn new_i64(val: i64) -> Self { Self::new(Self::TAG_I64, InnerData { I64: val }) }
    #[inline(always)]
    pub fn new_u64(val: u64) -> Self { Self::new(Self::TAG_U64, InnerData { U64: val }) }
    #[inline(always)]
    pub fn new_f64(val: f64) -> Self { Self::new(Self::TAG_F64, InnerData { F64: val }) }
    #[inline(always)]
    pub fn new_bool(val: bool) -> Self { Self::new(Self::TAG_BOOL, InnerData { Bool: val }) }

    const TAG_UNINIT : u64 = 0;
    const TAG_I64 : u64 = 1;
    const TAG_U64 : u64 = 2;
    const TAG_F64 : u64 = 3;
    const TAG_BOOL : u64 = 4;
}


#[derive(Clone, Copy)]
union InnerData {
    I64: i64,
    U64: u64,
    F64: f64,
    Bool: bool,
    uninit: (),
}


#[derive(Debug)]
pub struct Code<const DEBUG: bool> {
    ptr: *const u8,
    base: *const u8,
    top: *const u8,

    return_to: u8,
    offset: usize,
    argc: u8,
}


impl<const DEBUG: bool> Code<DEBUG> {
    pub fn new(ptr: *const u8, base: *const u8, top: *const u8, return_to: u8, offset: usize, argc: u8) -> Self {
        let slf = Self { 
            ptr, base, return_to, offset, top, argc
        };

        slf.assert_ptr();
        slf
    }

    #[inline(always)]
    fn next(&mut self) -> u8 {
        unsafe {
            let data = *self.ptr;
            self.ptr = self.ptr.add(1);

            self.assert_ptr();
            
            data
        }
    }

    
    #[inline(always)]
    fn read_as<T>(&mut self) -> T {
        unsafe {
            let data = self.ptr.cast::<T>().read_unaligned();
            self.ptr = self.ptr.add(size_of::<T>());
            self.assert_ptr();
            data
        }
    }


    #[inline(always)]
    fn jump(&mut self, pos: usize) {
        unsafe {
            self.ptr = self.base.add(pos);
        }
        self.assert_ptr();
    }


    fn assert_ptr(&self) {
        if DEBUG {
            assert!(self.ptr >= self.base);
            assert!(self.ptr <= self.top);
        }
    }
}


impl Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            match self.tag {
                Self::TAG_I64 => write!(f, "int {:?}", self.inner.I64),
                Self::TAG_U64 => write!(f, "uint {:?}", self.inner.U64),
                Self::TAG_F64 => write!(f, "float {:?}", self.inner.F64),
                Self::TAG_BOOL => write!(f, "bool {:?}", self.inner.Bool),
                Self::TAG_UNINIT => write!(f, "uninit"),
                _ => panic!("unexpected type {}", self.tag),
            }
        }
    }
}
