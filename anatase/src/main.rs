#![feature(iter_next_chunk)]

use std::{time::Instant, mem::size_of, env};

use anatase::{VM, Stack, Code, Data};
use archiver::Packed;

fn main() {
    let data = std::fs::read("test.anb").unwrap();
    let data = Packed::from_bytes(&data).unwrap();
    let mut data : Vec<_> = data.into();

    let bytecode = data.pop().unwrap();
    let constants = data.pop().unwrap();
    let constants = parse_constants(&constants.0);

    let mut vm = VM::<false> {
        stack: Stack::with_capacity(1_000_000 / size_of::<Data>()),
        callstack: Vec::with_capacity(128),
        current: Code::new(
            bytecode.0.as_ptr(),
            bytecode.0.as_ptr(),
            bytecode.0.last().unwrap() as *const u8,
            0,
            0,
            0,
        ),
        constants: constants.into(),
    };
    

    if let Ok(v) = env::var("ANATASE_WATCH_REG") {
        let delay : usize = env::var("ANATASE_WATCH_PERIOD").map(|x| x.parse().unwrap()).unwrap_or(200usize);

        let mut regs = vec![];
        for i in v.split(',') {
            let r : u8 = i.parse().unwrap();
            regs.push(vm.stack.reg_ptr(r) as usize);
        }

        let base = vm.stack.reg_ptr(0) as usize;
        std::thread::spawn(move || {
            let mut timer = Instant::now();
            loop {
                if timer.elapsed().as_millis() > delay as u128 {
                    timer = Instant::now();
                    for r in &regs {
                        println!("WATCH {}: {:?}", (*r - base) / size_of::<Data>(), unsafe { *(*r as *const Data) });
                        
                    }
                }
            }
        });
    }
    

    let timer = Instant::now();
    vm.run();
    let end = timer.elapsed();
    
    
    println!("finished in {}", end.as_secs_f64());
    println!("result is {:?}", vm.stack.reg(0));
}


fn parse_constants(bytes: &[u8]) -> Vec<Data> {
    let mut iter = bytes.iter().copied();
    let mut vec = vec![];

    while let Some(v) = iter.next() {
        match v {
            0 => {
                let bytes = iter.next_chunk::<8>().unwrap();
                let num = i64::from_le_bytes(bytes);
                vec.push(Data::new_i64(num));
            }
            1 => {
                let bytes = iter.next_chunk::<8>().unwrap();
                let num = f64::from_le_bytes(bytes);
                vec.push(Data::new_f64(num));
            }
            2 => {
                todo!()
            }

            3 => vec.push(Data::new_bool(true)),
            4 => vec.push(Data::new_bool(false)),

            _ => unreachable!(),
        }
    }

    vec
}