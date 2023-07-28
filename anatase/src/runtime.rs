use std::ops::Div;

use crate::{VM, Code, bytecode, Data};


impl<const DEBUG: bool> VM<DEBUG> {
    pub fn run(&mut self) {
        macro_rules! arithmetic_operation {
            ($tt: tt, $tag: ident, $kind: ident) => { arithmetic_operation!($tt, $tag, $kind, $tag, $kind) };

            ($tt: tt, $tag: ident, $kind: ident, $exp_tag: ident, $exp: ident) => {
                {
                    let dst = self.current.next();
                    let lhs = self.current.next();
                    let rhs = self.current.next();

                    let lhs = self.stack.reg(lhs);
                    let rhs = self.stack.reg(rhs);

                    if DEBUG {
                        assert_eq!(lhs.tag, Data::$tag);
                        assert_eq!(rhs.tag, Data::$tag);
                    }

                    let result = Data::new(
                        Data::$exp_tag,
                        unsafe { crate::InnerData { $exp: lhs.inner.$kind $tt rhs.inner.$kind } },
                    );

                    self.stack.set_reg(dst, result);
                 }
            }
        }

        
        macro_rules! arithmetic_division_operation {
            ($tag: ident, $kind: ident, $zero: literal) => {
                {
                    let dst = self.current.next();
                    let lhs = self.current.next();
                    let rhs = self.current.next();

                    let lhs = self.stack.reg(lhs);
                    let rhs = self.stack.reg(rhs);


                    if DEBUG {
                        assert_eq!(lhs.tag, Data::$tag);
                        assert_eq!(rhs.tag, Data::$tag);
                    }

                    if unsafe { rhs.inner.$kind } == $zero {
                        panic!("division by zero")
                    }
                    

                    let result = Data::new(
                        Data::$tag,
                        unsafe { crate::InnerData { $kind: lhs.inner.$kind / rhs.inner.$kind } },
                    );

                    self.stack.set_reg(dst, result);
                }
            }
        }


        macro_rules! cast_instruction {
            ($tag: ident, $field: ident | $ty: ty, $target_tag: ident, $target_field: ident) => {{
                println!("cast");
                let dst = self.current.next();
                let val = self.current.next();

                let val = self.stack.reg(val);

                if DEBUG {
                    assert!(val.tag == Data::$tag);
                }
                
                let result = unsafe { val.inner.$field as $ty };

                self.stack.set_reg(dst, Data::new(
                    Data::$target_tag, 
                    crate::InnerData { $target_field: result } 
                ))
            }}
        }
        
        loop {
            let value = self.current.next();

            match value {
                bytecode::RETURN => {
                    let Some(current) = self.callstack.pop() else { break };

                    let ret_val = self.stack.reg(0);
                    let ret_reg = self.current.return_to;
                    let argc = self.current.argc;
                    
                    self.current = current;
                    self.stack.bottom = self.current.offset;

                    self.stack.set_reg(ret_reg, ret_val);
                    self.stack.pop(argc as usize);
                },


                bytecode::COPY => {
                    let dst = self.current.next();
                    let src = self.current.next();

                    let val = self.stack.reg(src);
                    self.stack.set_reg(dst, val);
                },


                bytecode::SWAP => {
                    let v1 = self.current.next();
                    let v2 = self.current.next();

                    let val1 = self.stack.reg(v1);
                    let val2 = self.stack.reg(v2);
                    self.stack.set_reg(v1, val2);
                    self.stack.set_reg(v2, val1);
                },


                bytecode::SET => {
                    let dst = self.current.next();
                    let val = self.current.read_as::<u16>();
                    let val = self.constants[val as usize];

                    self.stack.set_reg(dst, val);
                },


                bytecode::PUSH => {
                    let amount = self.current.next();
                    self.stack.push(amount as usize);
                }


                bytecode::POP => {
                    let amount = self.current.next();
                    self.stack.pop(amount as usize);
                }


                bytecode::PRINT => {
                    let reg = self.current.next();
                    let val = self.stack.reg(reg);
                    println!("print: {val:?}");
                }


                bytecode::JIF => {
                    let cond = self.current.next();
                    let yes = self.current.read_as::<u32>();
                    let no = self.current.read_as::<u32>();

                    let cond = self.stack.reg(cond);
                    let cond = unsafe { cond.inner.Bool };
                    

                    if cond {
                        self.current.jump(yes as usize);
                    } else {
                        self.current.jump(no as usize);
                    }
                }


                bytecode::IJIF => {
                    let cond = self.current.next();
                    let yes = self.current.read_as::<u32>();

                    let cond = self.stack.reg(cond);
                    let cond = unsafe { cond.inner.Bool };
                    

                    if cond {
                        self.current.jump(yes as usize);
                    }
                }


                bytecode::JNIF => {
                    let cond = self.current.next();
                    let yes = self.current.read_as::<u32>();
                    let no = self.current.read_as::<u32>();

                    let cond = self.stack.reg(cond);
                    let cond = unsafe { cond.inner.Bool };
                    

                    if !cond {
                        self.current.jump(yes as usize);
                    } else {
                        self.current.jump(no as usize);
                    }
                }


                bytecode::IJNIF => {
                    let cond = self.current.next();
                    let yes = self.current.read_as::<u32>();

                    let cond = self.stack.reg(cond);
                    let cond = unsafe { cond.inner.Bool };
                    

                    if !cond {
                        self.current.jump(yes as usize);
                    }
                }
                

                bytecode::JMP => {
                    let pos = self.current.read_as::<u32>();
                    self.current.jump(pos as usize);
                }


                bytecode::CALL => {
                    let dst = self.current.next();
                    let goto = self.current.read_as::<u32>();
                    let argc = self.current.next() as usize;

                    self.stack.push(argc + 1);

                    let temp = self.stack.top - argc - self.stack.bottom;
                    for v in 0..argc {
                        let reg = self.stack.reg(self.current.next());
                        self.stack.set_reg((temp + v) as u8, reg);
                    }

                    let code = Code::new(
                        unsafe { self.current.base.add(goto as usize) },
                        self.current.base,
                        self.current.top,
                        dst,
                        self.stack.top - argc - 1,
                        argc as u8,
                    );

                    self.callstack.push(std::mem::replace(&mut self.current, code));

                    self.stack.bottom = self.current.offset;

                }


                bytecode::ADDI => arithmetic_operation!(+, TAG_I64, I64),
                bytecode::ADDU => arithmetic_operation!(+, TAG_U64, U64),
                bytecode::ADDF => arithmetic_operation!(+, TAG_F64, F64),
                bytecode::SUBI => arithmetic_operation!(-, TAG_I64, I64),
                bytecode::SUBU => arithmetic_operation!(-, TAG_U64, U64),
                bytecode::SUBF => arithmetic_operation!(-, TAG_F64, F64),
                bytecode::MULI => arithmetic_operation!(*, TAG_I64, I64),
                bytecode::MULU => arithmetic_operation!(*, TAG_U64, U64),
                bytecode::MULF => arithmetic_operation!(*, TAG_F64, F64),
                bytecode::REMI => arithmetic_operation!(%, TAG_I64, I64),
                bytecode::REMU => arithmetic_operation!(%, TAG_U64, U64),
                bytecode::REMF => arithmetic_operation!(%, TAG_F64, F64),
                bytecode::LSI  => arithmetic_operation!(<<, TAG_I64, I64),
                bytecode::LSU  => arithmetic_operation!(<<, TAG_U64, U64),
                bytecode::RSI  => arithmetic_operation!(>>, TAG_I64, I64),
                bytecode::RSU  => arithmetic_operation!(>>, TAG_U64, U64),
                bytecode::DIVI => arithmetic_division_operation!(TAG_I64, I64,   0),
                bytecode::DIVU => arithmetic_division_operation!(TAG_U64, U64,   0),
                bytecode::DIVF => arithmetic_division_operation!(TAG_F64, F64, 0.0),


                bytecode::LTI => arithmetic_operation!(< , TAG_I64, I64, TAG_BOOL, Bool),
                bytecode::LTU => arithmetic_operation!(< , TAG_U64, U64, TAG_BOOL, Bool),
                bytecode::LTF => arithmetic_operation!(< , TAG_F64, F64, TAG_BOOL, Bool),
                bytecode::GTI => arithmetic_operation!(> , TAG_I64, I64, TAG_BOOL, Bool),
                bytecode::GTU => arithmetic_operation!(> , TAG_U64, U64, TAG_BOOL, Bool),
                bytecode::GTF => arithmetic_operation!(> , TAG_F64, F64, TAG_BOOL, Bool),
                bytecode::LEI => arithmetic_operation!(<=, TAG_I64, I64, TAG_BOOL, Bool),
                bytecode::LEU => arithmetic_operation!(<=, TAG_U64, U64, TAG_BOOL, Bool),
                bytecode::LEF => arithmetic_operation!(<=, TAG_F64, F64, TAG_BOOL, Bool),
                bytecode::GEI => arithmetic_operation!(>=, TAG_I64, I64, TAG_BOOL, Bool),
                bytecode::GEU => arithmetic_operation!(>=, TAG_U64, U64, TAG_BOOL, Bool),
                bytecode::GEF => arithmetic_operation!(>=, TAG_F64, F64, TAG_BOOL, Bool),
                bytecode::EQI => arithmetic_operation!(==, TAG_I64, I64, TAG_BOOL, Bool),
                bytecode::EQU => arithmetic_operation!(==, TAG_U64, U64, TAG_BOOL, Bool),
                bytecode::EQF => arithmetic_operation!(==, TAG_F64, F64, TAG_BOOL, Bool),
                bytecode::NEI => arithmetic_operation!(!=, TAG_I64, I64, TAG_BOOL, Bool),
                bytecode::NEU => arithmetic_operation!(!=, TAG_U64, U64, TAG_BOOL, Bool),
                bytecode::NEF => arithmetic_operation!(!=, TAG_F64, F64, TAG_BOOL, Bool),


                bytecode::CASTIU => cast_instruction!(TAG_I64, I64 | u64, TAG_U64, U64),
                bytecode::CASTIF => cast_instruction!(TAG_I64, I64 | f64, TAG_F64, F64),
                
                bytecode::CASTUI => cast_instruction!(TAG_U64, U64 | i64, TAG_I64, I64),
                bytecode::CASTUF => cast_instruction!(TAG_U64, U64 | f64, TAG_F64, F64),
                
                bytecode::CASTFI => cast_instruction!(TAG_F64, F64 | i64, TAG_I64, I64),
                bytecode::CASTFU => cast_instruction!(TAG_F64, F64 | i64, TAG_I64, I64),

                _ => panic!("unreachable {value}"),
            }
            
        }
    }

}


