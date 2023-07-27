pub const RETURN : u8 = 0;
pub const COPY   : u8 = 1;
pub const SWAP   : u8 = 2;
pub const SET    : u8 = 3;


pub const PUSH : u8 = 7;
pub const POP  : u8 = 8;


pub const JIF : u8 = 9;
pub const JNIF : u8 = 10;
pub const JMP : u8 = 11;

pub const CALL : u8 = 12;


pub const ADDI : u8 = 100;
pub const ADDU : u8 = 101;
pub const ADDF : u8 = 102;
pub const SUBI : u8 = 103;
pub const SUBU : u8 = 104;
pub const SUBF : u8 = 105;
pub const MULI : u8 = 106;
pub const MULU : u8 = 107;
pub const MULF : u8 = 108;
pub const DIVI : u8 = 109;
pub const DIVU : u8 = 110;
pub const DIVF : u8 = 111;
pub const REMI : u8 = 112;
pub const REMU : u8 = 113;
pub const REMF : u8 = 114;

pub const LTI  : u8 = 120;
pub const LTU  : u8 = 121;
pub const LTF  : u8 = 122;
pub const GTI  : u8 = 123;
pub const GTU  : u8 = 124;
pub const GTF  : u8 = 125;
pub const LEI  : u8 = 126;
pub const LEU  : u8 = 127;
pub const LEF  : u8 = 128;
pub const GEI  : u8 = 129;
pub const GEU  : u8 = 130;
pub const GEF  : u8 = 131;
pub const EQI  : u8 = 132;
pub const EQU  : u8 = 133;
pub const EQF  : u8 = 134;
pub const NEI  : u8 = 135;
pub const NEU  : u8 = 136;
pub const NEF  : u8 = 137;

pub const CASTIU : u8 = 140;
pub const CASTIF : u8 = 141;
pub const CASTUI : u8 = 142;
pub const CASTUF : u8 = 143;
pub const CASTFI : u8 = 144;
pub const CASTFU : u8 = 145;
