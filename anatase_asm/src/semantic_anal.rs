use std::collections::HashSet;

use crate::{errors::{Error, CompilerError, ErrorBuilder}, parser::Function, SymbolMap, SymbolIndex, Operator, OperatorKind};

pub fn analyze(file: SymbolIndex, symbol_table: &mut SymbolMap, functions: &[Function]) -> Result<(), Error> {
    let main_ident = symbol_table.push("main".to_string());

    let has_main = functions.iter().any(|x| x.name == main_ident);
    if !has_main {
        return Err(CompilerError::new(file, "no 'main' function defined").build())
    }


    let mut function_set = HashSet::with_capacity(functions.len());

    for f in functions {
        if !function_set.insert(f.name) {
            return Err(CompilerError::new(file, "function already defined")
                .highlight(f.declaration_range)
                    .note("this function is already defined earlier in the program".to_string())
                .build())
        }

        let has_entry = f.body.iter().any(|x| x.id == f.entry);
        if !has_entry {
            return Err(CompilerError::new(file, "entry block isn't defined")
                .highlight(f.declaration_range)
                .note(format!(
                    "entry block is defined as '${}' but it doesn't exist in the body", 
                    symbol_table.get(f.entry.0)))
                .build())
        }


        for block in &f.body {
            for o in &block.operators {
                if let OperatorKind::Call(_, name, ref args) = o.kind {
                    let function = functions.iter().find(|x| x.name == name);
                    let function = match function {
                        Some(v) => v,
                        None => return Err(CompilerError::new(file, "function isn't defined")
                            .highlight(o.source_range)
                            .build()),
                    };

                    if args.len() != function.argc as usize {
                        return Err(CompilerError::new(file, "differing argument counts")
                            .highlight(o.source_range)
                                .note(format!("the function expects {} but you gave {}", function.argc, args.len()))
                            .build())
                    }
                }
            }
        }
    }

    Ok(())
}
