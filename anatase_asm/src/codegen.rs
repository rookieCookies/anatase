use std::collections::HashMap;

use crate::{SymbolMap, parser::Function, consts::{Ret}, Literal, OperatorKind};

pub fn codegen(symbol_map: &SymbolMap, functions: &[Function]) -> (Vec<Literal>, Vec<u8>) {
    let mut bytecode = Vec::new();
    let mut constants = Vec::new();
    
    let mut function_starts = HashMap::with_capacity(functions.len());
    let mut function_calls = Vec::new();


    {
        let main = symbol_map.find("main").unwrap();
        bytecode.push(OperatorKind::Call(0, main, vec![]).as_bytecode());
        bytecode.push(0);
        
        function_calls.push((main, bytecode.len()));
        bytecode.push(0);
        bytecode.push(0);
        bytecode.push(0);
        bytecode.push(0);


        bytecode.push(OperatorKind::Ret().as_bytecode());

        let temp : [u8; 0] = [];
        temp.as_slice().to_bytes(&mut bytecode);
    }
    
    
    let mut block_starts_cache = HashMap::new();
    for f in functions {
        block_starts_cache.clear();
        let block_starts = &mut block_starts_cache;
        let mut jumps = Vec::new();

        let temp = function_starts.insert(f.name, bytecode.len());
        assert!(temp.is_none());

        bytecode.push(OperatorKind::Jmp(f.entry).as_bytecode());
        jumps.push((OperatorKind::Jmp(f.entry), bytecode.len()));
        bytecode.push(0);
        bytecode.push(0);
        bytecode.push(0);
        bytecode.push(0);

        for b in &f.body {
            block_starts.insert(b.id, bytecode.len());
            for o in &b.operators {
                bytecode.push(o.kind.as_bytecode());
                println!("{o:?}");
                match o.kind {
                    crate::OperatorKind::Ret() => bytecode.push(Ret),

                    
                    | crate::OperatorKind::Push(v)
                    | crate::OperatorKind::Pop(v) => {
                        v.to_bytes(&mut bytecode);
                    },


                    crate::OperatorKind::Set(dst, val) => {
                        let index = constants.iter().enumerate().find(|x| x.1 == &val);
                        let index = match index {
                            Some(v) => v.0,
                            None => {
                                constants.push(val);
                                constants.len()-1
                            },
                        };

                        dst.to_bytes(&mut bytecode);
                        u16::try_from(index).expect("too many constants").to_bytes(&mut bytecode);
                    }
                    

                    | crate::OperatorKind::Cast_IU(v1, v2)
                    | crate::OperatorKind::Cast_IF(v1, v2)
                    | crate::OperatorKind::Cast_UI(v1, v2)
                    | crate::OperatorKind::Cast_UF(v1, v2)
                    | crate::OperatorKind::Cast_FI(v1, v2)
                    | crate::OperatorKind::Cast_FU(v1, v2)
                    | crate::OperatorKind::Cpy(v1, v2)
                    | crate::OperatorKind::Swap(v1, v2)
                     => {
                        v1.to_bytes(&mut bytecode);
                        v2.to_bytes(&mut bytecode);
                    },



                    
                    | crate::OperatorKind::Jif(v, ..)
                    | crate::OperatorKind::JNif(v, ..)
                     => {
                        v.to_bytes(&mut bytecode);
                        jumps.push((o.kind.clone(), bytecode.len()));
                        bytecode.push(0);
                        bytecode.push(0);
                        bytecode.push(0);
                        bytecode.push(0);
                        bytecode.push(0);
                        bytecode.push(0);
                        bytecode.push(0);
                        bytecode.push(0);
                    },

                    crate::OperatorKind::Jmp(_) => {
                        jumps.push((o.kind.clone(), bytecode.len()));
                        bytecode.push(0);
                        bytecode.push(0);
                        bytecode.push(0);
                        bytecode.push(0);
                    },

                    
                    crate::OperatorKind::Call(dst, func, ref args) => {
                        dst.to_bytes(&mut bytecode);

                        function_calls.push((func, bytecode.len()));
                        bytecode.push(0);
                        bytecode.push(0);
                        bytecode.push(0);
                        bytecode.push(0);

                        args.as_slice().to_bytes(&mut bytecode);
                        
                    },


                    | crate::OperatorKind::LtI  (v1, v2, v3)
                    | crate::OperatorKind::LtU  (v1, v2, v3)
                    | crate::OperatorKind::LtF  (v1, v2, v3)
                    | crate::OperatorKind::GtI  (v1, v2, v3)
                    | crate::OperatorKind::GtU  (v1, v2, v3)
                    | crate::OperatorKind::GtF  (v1, v2, v3)
                    | crate::OperatorKind::LeI  (v1, v2, v3)
                    | crate::OperatorKind::LeU  (v1, v2, v3)
                    | crate::OperatorKind::LeF  (v1, v2, v3)
                    | crate::OperatorKind::GeI  (v1, v2, v3)
                    | crate::OperatorKind::GeU  (v1, v2, v3)
                    | crate::OperatorKind::GeF  (v1, v2, v3)
                    | crate::OperatorKind::EqI  (v1, v2, v3)
                    | crate::OperatorKind::EqU  (v1, v2, v3)
                    | crate::OperatorKind::EqF  (v1, v2, v3)
                    | crate::OperatorKind::NeI  (v1, v2, v3)
                    | crate::OperatorKind::NeU  (v1, v2, v3)
                    | crate::OperatorKind::NeF  (v1, v2, v3)
                    | crate::OperatorKind::AddI (v1, v2, v3)
                    | crate::OperatorKind::AddU (v1, v2, v3)
                    | crate::OperatorKind::AddF (v1, v2, v3)
                    | crate::OperatorKind::SubI (v1, v2, v3)
                    | crate::OperatorKind::SubU (v1, v2, v3)
                    | crate::OperatorKind::SubF (v1, v2, v3)
                    | crate::OperatorKind::DivI (v1, v2, v3)
                    | crate::OperatorKind::DivU (v1, v2, v3)
                    | crate::OperatorKind::DivF (v1, v2, v3)
                    | crate::OperatorKind::MulI (v1, v2, v3)
                    | crate::OperatorKind::MulU (v1, v2, v3)
                    | crate::OperatorKind::MulF (v1, v2, v3)
                    | crate::OperatorKind::RemI (v1, v2, v3)
                    | crate::OperatorKind::RemU (v1, v2, v3)
                    | crate::OperatorKind::RemF (v1, v2, v3)
                     => {
                        v1.to_bytes(&mut bytecode);
                        v2.to_bytes(&mut bytecode);
                        v3.to_bytes(&mut bytecode);
                    }
                }
            }

        }

        for (i, j) in jumps.iter().enumerate() {
            println!("{}", i);
            match j.0 {
                | crate::OperatorKind::Jif(_, yes, no)
                | crate::OperatorKind::JNif(_, yes, no) => {
                    let index = block_starts.get(&yes).unwrap();
                    let index = u32::try_from(*index).expect("index too big");
                    let index : [u8; 4] = index.to_le_bytes();
                    bytecode[j.1 + 0] = index[0];
                    bytecode[j.1 + 1] = index[1];
                    bytecode[j.1 + 2] = index[2];
                    bytecode[j.1 + 3] = index[3];


                    let index = block_starts.get(&no).unwrap();
                    let index = u32::try_from(*index).expect("index too big");
                    let index : [u8; 4] = index.to_le_bytes();
                    bytecode[j.1 + 4] = index[0];
                    bytecode[j.1 + 5] = index[1];
                    bytecode[j.1 + 6] = index[2];
                    bytecode[j.1 + 7] = index[3];
                },


                crate::OperatorKind::Jmp(v) => {
                    println!("{}", symbol_map.get(v.0));
                    let index = block_starts.get(&v).unwrap();
                    let index = u32::try_from(*index).expect("index too big");
                    let index : [u8; 4] = index.to_le_bytes();
                    assert_eq!(bytecode[j.1..j.1+4], [0; 4]);
                    bytecode[j.1 + 0] = index[0];
                    bytecode[j.1 + 1] = index[1];
                    bytecode[j.1 + 2] = index[2];
                    bytecode[j.1 + 3] = index[3];

                }

                _ => unreachable!()
            }
        }
    }


    for func in function_calls {
        let index = function_starts.get(&func.0).unwrap();
        let index = u32::try_from(*index).expect("index too big");
        let index : [u8; 4] = index.to_le_bytes();
        assert_eq!(bytecode[func.1..func.1+4], [0; 4]);
        bytecode[func.1 + 0] = index[0];
        bytecode[func.1 + 1] = index[1];
        bytecode[func.1 + 2] = index[2];
        bytecode[func.1 + 3] = index[3];
    }

    println!("{bytecode:?}");
    (constants, bytecode)
}


pub trait ToBytecode {
    fn to_bytes(&self, vec: &mut Vec<u8>);
}


impl<T: ToBytecode> ToBytecode for &[T] {
    fn to_bytes(&self, vec: &mut Vec<u8>) {
        vec.push(self.len().try_into().expect("list is bigger than a u8"));
        for i in self.iter() {
            i.to_bytes(vec);
        }
    }
}


impl ToBytecode for u8 {
    fn to_bytes(&self, vec: &mut Vec<u8>) {
        vec.push(*self)
    }
}


impl ToBytecode for u16 {
    fn to_bytes(&self, vec: &mut Vec<u8>) {
        for i in self.to_le_bytes() {
            vec.push(i)
        }
    }
}


impl ToBytecode for u32 {
    fn to_bytes(&self, vec: &mut Vec<u8>) {
        for i in self.to_le_bytes() {
            vec.push(i)
        }
    }
}


impl ToBytecode for u64 {
    fn to_bytes(&self, vec: &mut Vec<u8>) {
        for i in self.to_le_bytes() {
            vec.push(i)
        }
    }
}


impl ToBytecode for i8 {
    fn to_bytes(&self, vec: &mut Vec<u8>) {
        vec.push(self.to_le_bytes()[0])
    }
}


impl ToBytecode for i16 {
    fn to_bytes(&self, vec: &mut Vec<u8>) {
        for i in self.to_le_bytes() {
            vec.push(i)
        }
    }
}


impl ToBytecode for i32 {
    fn to_bytes(&self, vec: &mut Vec<u8>) {
        for i in self.to_le_bytes() {
            vec.push(i)
        }
    }
}


impl ToBytecode for i64 {
    fn to_bytes(&self, vec: &mut Vec<u8>) {
        for i in self.to_le_bytes() {
            vec.push(i)
        }
    }
}


impl ToBytecode for f32 {
    fn to_bytes(&self, vec: &mut Vec<u8>) {
        for i in self.to_le_bytes() {
            vec.push(i)
        }
    }
}


impl ToBytecode for f64 {
    fn to_bytes(&self, vec: &mut Vec<u8>) {
        for i in self.to_le_bytes() {
            vec.push(i)
        }
    }
}
