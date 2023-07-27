use crate::{lexer::{Token, TokenKind, Keyword}, SourceRange, SymbolIndex, Operator, errors::{CompilerError, ErrorBuilder, Error}, Literal, SymbolMap};


#[derive(Debug)]
pub struct Function {
    pub name: SymbolIndex,
    pub body: Vec<Block>,
    pub entry: BlockId,
    pub declaration_range: SourceRange,
    pub argc: u8,
}


#[derive(Debug)]
pub struct Block {
    pub operators: Vec<Operator>,
    pub id: BlockId,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlockId(pub SymbolIndex);


pub struct Parser<'a> {
    tokens: Vec<Token>,
    index: usize,
    symbol_map: &'a SymbolMap,
    pub file: SymbolIndex,
}


pub fn parse(file: SymbolIndex, tokens: Vec<Token>, symbol_map: &SymbolMap) -> Result<Vec<Function>, Error> {
    let mut vec = vec![];
    let mut parser = Parser {
        tokens,
        index: 0,
        file,
        symbol_map,
    };


    loop {
        if parser.current_kind() == TokenKind::EndOfFile {
            break
        }

        let start_pos = parser.current_token().source_range.start;
        parser.expect(TokenKind::Keyword(Keyword::Fn))?;
        parser.advance();

        let name = parser.expect_identifier()?;
        parser.advance();

        parser.expect(TokenKind::SquigglyDash)?;
        parser.advance();

        let argc = parser.literal_u8()?;
        
        parser.advance();
        
        let start = parser.label()?;
        let declaration_range = SourceRange::new(start_pos, parser.current_token().source_range.end);
        parser.advance();

        let mut blocks = vec![];
        loop {
            if [TokenKind::EndOfFile, TokenKind::Keyword(Keyword::Fn)].contains(&parser.current_kind()) {
                break
            }

            let name = parser.label()?;
            parser.advance();

            let mut operators = vec![];
            loop {
                if [
                    TokenKind::EndOfFile, 
                    TokenKind::Keyword(Keyword::Fn),
                    TokenKind::DollarSign
                ].contains(&parser.current_kind()) {
                    break
                }

                operators.push(parser.operator()?);
                if [
                    TokenKind::EndOfFile, 
                ].contains(&parser.current_kind()) {
                    break
                }
                parser.advance();
            }

            blocks.push(Block {
                operators,
                id: name,
            })
            
        }

        vec.push(Function {
            name,
            body: blocks,
            entry: start,
            argc,
            declaration_range,
        })
    }

    Ok(vec)
}


macro_rules! literal_sized_int {
    ($ident: ident, $ty: ty) => {
        pub fn $ident(&mut self) -> Result<$ty, Error> {
            match <$ty>::try_from(self.literal_int()?) {
                Ok(v) => Ok(v),
                Err(_) => Err(CompilerError::new(self.file, concat!("value doesn't fit in a ", stringify!($ty)))
                        .highlight(self.current_token().source_range)
                        .build())
            }
        }
    }
}


impl Parser<'_> {
    pub fn advance(&mut self) -> &Token {
        self.index += 1;
        &self.tokens[self.index]
    }

    
    pub fn retreat(&mut self) -> &Token {
        self.index -= 1;
        &self.tokens[self.index]
    }


    pub fn current_token(&self) -> &Token {
        &self.tokens[self.index]
    }


    pub fn current_kind(&self) -> TokenKind {
        self.current_token().token_kind
    }


    pub fn peek_token(&self) -> &Token {
        &self.tokens[self.index]
    }


    pub fn peek_kind(&self) -> TokenKind {
        self.peek_token().token_kind
    }


    pub fn expect_identifier(&self) -> Result<SymbolIndex, Error> {
        if let TokenKind::Identifier(v) = self.current_kind() {
            return Ok(v)
        }

        Err(CompilerError::new(self.file, "expected identifier")
            .highlight(self.current_token().source_range)
            .build())
    }


    pub fn expect(&self, kind: TokenKind) -> Result<(), Error> {
        if self.current_kind() != kind {
            return Err(CompilerError::new(self.file, "unexpected value")
                                .highlight(self.current_token().source_range)
                                    .note(format!("expected {} found {}", kind.to_str(self.symbol_map), self.current_kind().to_str(self.symbol_map)))
                                .build())
        }

        Ok(())
    }


    pub fn label(&mut self) -> Result<BlockId, Error> {
        self.expect(TokenKind::DollarSign)?;
        self.advance();

        Ok(BlockId(self.expect_identifier()?))
    }


    pub fn reg(&mut self) -> Result<u8, Error> {
        self.expect(TokenKind::At)?;
        self.advance();

        self.literal_u8()
    }


    literal_sized_int!(literal_i8 , i8 );
    literal_sized_int!(literal_i16, i16);
    literal_sized_int!(literal_i32, i32);
    literal_sized_int!(literal_i64, i64);
    literal_sized_int!(literal_u8 , u8 );
    literal_sized_int!(literal_u16, u16);
    literal_sized_int!(literal_u32, u32);
    literal_sized_int!(literal_u64, u64);



    pub fn literal_int(&self) -> Result<i64, Error> {
        let TokenKind::Literal(Literal::Integer(v)) = self.current_kind()
        else { return Err(CompilerError::new(self.file, "expected an integer literal")
            .highlight(self.current_token().source_range)
            .build())
        };

        Ok(v)
    }


    pub fn literal(&self) -> Result<Literal, Error> {
        match self.current_kind() {
            TokenKind::Literal(v) => Ok(v),
            _ => Err(CompilerError::new(self.file, "expected a literal")
                .highlight(self.current_token().source_range)
                .build())
        }
    }


    pub fn reg_list(&mut self) -> Result<Vec<u8>, Error> {
        let mut vec = vec![];
        loop {
            if [
                TokenKind::DollarSign, 
                TokenKind::EndOfFile, 
                TokenKind::Keyword(Keyword::Fn)
            ].contains(&self.current_kind()) {
                break
            }

            if matches!(self.current_kind(), TokenKind::Operator(_)) {
                break
            }

            vec.push(self.reg()?);
            self.advance();
        }

        self.retreat();

        Ok(vec)
    }
}