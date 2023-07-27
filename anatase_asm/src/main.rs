use std::collections::HashMap;
use std::env::consts;
use std::path::Path;
use std::path::PathBuf;

use anatase_asm::SymbolMap;
use anatase_asm::PrettyPrint;
use anatase_asm::codegen::ToBytecode;
use anatase_asm::codegen::codegen;
use archiver::Packed;

fn main() {
    let mut symbol_map = SymbolMap::new();
    let file = symbol_map.push(String::from("test.an"));

    let data = std::fs::read_to_string("test.an").unwrap().replace('\r', "").replace('\t', "    ");
    let tokens = anatase_asm::lexer::lex(file, &data, &mut symbol_map);
    let tokens = match tokens {
        Ok(v) => {
            println!("{}", v.as_slice().pretty_print(&symbol_map));
            v
        },
        Err(e) => {
            println!("{}", e.build(&HashMap::from([(file, ("test.an".to_string(), data))])));
            return
        },
    };


    let instructions = anatase_asm::parser::parse(file, tokens, &symbol_map);
    
    let instructions = match instructions {
        Ok(v) => {
            println!("{:?}", v);
            v
        },
        Err(e) => {
            println!("{}", e.build(&HashMap::from([(file, ("test.an".to_string(), data))])));
            return
        },
    };


    if let Err(e) = anatase_asm::semantic_anal::analyze(file, &mut symbol_map, &instructions) {
        println!("{}", e.build(&HashMap::from([(file, ("test.an".to_string(), data))])));
        return
    }


    let codegen = codegen(&symbol_map, &instructions);


    let mut constant_bytes = vec![];
    for i in codegen.0 {
        match i {
            anatase_asm::Literal::Integer(v) => {
                constant_bytes.push(0);
                v.to_bytes(&mut constant_bytes)
            },

            anatase_asm::Literal::Float(v) => {
                constant_bytes.push(1);
                v.to_bytes(&mut constant_bytes)
            },

            anatase_asm::Literal::String(v) => {
                let str = symbol_map.get(v);
                let len : u64 = str.len().try_into().expect("string too big");

                constant_bytes.push(2);
                len.to_bytes(&mut constant_bytes);
                constant_bytes.extend_from_slice(str.as_bytes());
            },

            anatase_asm::Literal::Bool(v) => {
                let val = if v { 3 } else { 4 };
                constant_bytes.push(val);
            },

            anatase_asm::Literal::Empty => (),
        }
    }
    

    let bytes = Packed::new()
        .with(archiver::Data(constant_bytes))
        .with(archiver::Data(codegen.1))
        .as_bytes();


    let mut pathbuf = PathBuf::from(symbol_map.get(file));
    pathbuf.set_extension("anb");

    std::fs::write(pathbuf, bytes).unwrap();
}
