use std::fmt::{Display, Write};

use casey::lower;
use istd::index_map;
use lexer::{Lexer, TokenKind};
use parser::{Parser, BlockId};
use codegen::ToBytecode;
use errors::{CompilerError, Error, ErrorBuilder};

pub mod lexer;
pub mod parser;
pub mod semantic_anal;
pub mod codegen;
mod errors;
pub mod disassembler;


#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Debug, Hash)]
pub struct SymbolIndex(usize);

#[derive(Debug)]
pub struct SymbolMap {
    vec: Vec<String>,
}


impl SymbolMap {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
        }
    }


    pub fn push(&mut self, val: String) -> SymbolIndex {
        if let Some(v) = self.vec.iter().enumerate().find(|x| x.1 == &val) {
            return SymbolIndex(v.0);
        }

        self.vec.push(val);
        SymbolIndex(self.vec.len() - 1)
    }


    pub fn find(&self, str: &str) -> Option<SymbolIndex> {
        self.vec.iter().enumerate().find(|x| x.1 == str).map(|x| SymbolIndex(x.0)) 
    }


    pub fn get(&self, index: SymbolIndex) -> &str {
        self.vec.get(index.0).unwrap()
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SourceRange {
    start: usize,
    end: usize,
}


impl SourceRange {
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start <= end, "start: {start} end: {end}");
        Self { start, end }
    }


    pub fn with(self, other: Self) -> Self {
        Self::new(self.start, other.end)
    }
}


impl Display for SourceRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}



#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(SymbolIndex),
    Bool(bool),
    Empty,
}


impl Literal {
    fn pretty_print(&self, handle: &mut impl Write, symbol_map: &SymbolMap) {
        let _ = write!(handle, "lit(");

        let _ = match self {
            Literal::Integer(v) => write!(handle, "{}", v),
            Literal::Float  (v) => write!(handle, "{}", v),
            Literal::String (v) => write!(handle, "{}", &symbol_map.get(*v)),
            Literal::Bool   (v) => write!(handle, "{}", v),
            Literal::Empty      => write!(handle, "()"),
        };
        
        let _ = write!(handle, ")");
    }
}


pub trait PrettyPrint {
    fn pretty_print(&self, symbol_map: &SymbolMap) -> String;
}


macro_rules! operators {
    ($($indx: literal $name: ident ( $(($lit: ident $ty: ty))* ),)*) => {
        #[derive(Debug, PartialEq, Clone, Copy)]
        #[repr(u8)]
        pub enum OperatorToken {
            $($name,)+
        }

        impl Lexer<'_> {
            pub fn operator_token(str: &str) -> Option<OperatorToken> {
                match str {
                    $(lower!(stringify!($name)) => Some(OperatorToken::$name),)+
                    _ => None
                }
            }
        }


        #[derive(Debug)]
        pub struct Operator {
            kind: OperatorKind,
            source_range: SourceRange,
        }
        

        #[derive(Debug, Clone)]
        pub enum OperatorKind {
            $($name ($($ty,)*),)*
        }


        impl Parser<'_> {
            fn operator(&mut self) -> Result<Operator, Error> {
                match self.current_kind() {
                    $(TokenKind::Operator(OperatorToken::$name) => {
                        let start = self.current_token().source_range.start;
                        let kind = OperatorKind::$name(
                            $({ self.advance(); self.$lit()? },)*
                        );

                        Ok(Operator { kind, source_range: SourceRange::new(start, self.current_token().source_range.end) })
                    },)+
                    _ => Err(CompilerError::new(self.file, "unknown operator")
                            .highlight(self.current_token().source_range)
                            .build())
                }
            }
        }


        impl std::fmt::Display for OperatorToken {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                write!(f, "{}", match self {
                    $(Self::$name => lower!(stringify!($name)),)+
                })
            }
        }


        impl OperatorKind {
            fn as_bytecode(&self) -> u8 {
                match self {
                    $(Self::$name(..) => consts::$name,)*
                }
            }


            #[deny(unreachable_patterns)]
            fn ___validation() {
                match 0 {
                    $($indx => (),)+

                    _ => ()
                }
            }
        }


        #[allow(non_upper_case_globals)]
        mod consts {
            $(pub const $name : u8 = $indx;)*
        }
    }
}


operators!(
    0 Ret (),
    1 Cpy ((reg u8) (reg u8)),
    2 Swap ((reg u8) (reg u8)),
    3 Set ((reg u8) (literal Literal)),


    7 Push((literal_u8 u8)),
    8 Pop((literal_u8 u8)),


    9 Jif((reg u8) (label BlockId) (label BlockId)),
    10 JNif((reg u8) (label BlockId) (label BlockId)),
    11 Jmp((label BlockId)),
    12 IJif((reg u8) (label BlockId)),
    13 IJNif((reg u8) (label BlockId)),
    
    50 Call((reg u8) (expect_identifier SymbolIndex) (reg_list Vec<u8>)),
    

    100 AddI ((reg u8) (reg u8) (reg u8)),
    101 AddU ((reg u8) (reg u8) (reg u8)),
    102 AddF ((reg u8) (reg u8) (reg u8)),
    103 SubI ((reg u8) (reg u8) (reg u8)),
    104 SubU ((reg u8) (reg u8) (reg u8)),
    105 SubF ((reg u8) (reg u8) (reg u8)),
    106 MulI ((reg u8) (reg u8) (reg u8)),
    107 MulU ((reg u8) (reg u8) (reg u8)),
    108 MulF ((reg u8) (reg u8) (reg u8)),
    109 DivI ((reg u8) (reg u8) (reg u8)),
    110 DivU ((reg u8) (reg u8) (reg u8)),
    111 DivF ((reg u8) (reg u8) (reg u8)),
    112 RemI ((reg u8) (reg u8) (reg u8)),
    113 RemU ((reg u8) (reg u8) (reg u8)),
    114 RemF ((reg u8) (reg u8) (reg u8)),
    115 LsI ((reg u8) (reg u8) (reg u8)),
    116 LsU ((reg u8) (reg u8) (reg u8)),
    118 RsI ((reg u8) (reg u8) (reg u8)),
    119 RsU ((reg u8) (reg u8) (reg u8)),

    130 LtI  ((reg u8) (reg u8) (reg u8)),
    131 LtU  ((reg u8) (reg u8) (reg u8)),
    132 LtF  ((reg u8) (reg u8) (reg u8)),
    133 GtI  ((reg u8) (reg u8) (reg u8)),
    134 GtU  ((reg u8) (reg u8) (reg u8)),
    135 GtF  ((reg u8) (reg u8) (reg u8)),
    136 LeI  ((reg u8) (reg u8) (reg u8)),
    137 LeU  ((reg u8) (reg u8) (reg u8)),
    138 LeF  ((reg u8) (reg u8) (reg u8)),
    139 GeI  ((reg u8) (reg u8) (reg u8)),
    140 GeU  ((reg u8) (reg u8) (reg u8)),
    141 GeF  ((reg u8) (reg u8) (reg u8)),
    142 EqI  ((reg u8) (reg u8) (reg u8)),
    143 EqU  ((reg u8) (reg u8) (reg u8)),
    144 EqF  ((reg u8) (reg u8) (reg u8)),
    145 NeI  ((reg u8) (reg u8) (reg u8)),
    146 NeU  ((reg u8) (reg u8) (reg u8)),
    147 NeF  ((reg u8) (reg u8) (reg u8)),

    150 Cast_IU ((reg u8) (reg u8)),
    151 Cast_IF ((reg u8) (reg u8)),
    152 Cast_UI ((reg u8) (reg u8)),
    153 Cast_UF ((reg u8) (reg u8)),
    154 Cast_FI ((reg u8) (reg u8)),
    155 Cast_FU ((reg u8) (reg u8)),

    
    255 Print ((reg u8)),
);
