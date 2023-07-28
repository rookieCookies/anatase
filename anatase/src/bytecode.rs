pub const RETURN : u8 = 0;
pub const COPY   : u8 = 1;
pub const SWAP   : u8 = 2;
pub const SET    : u8 = 3;


pub const PUSH : u8 = 7;
pub const POP  : u8 = 8;


pub const JIF : u8 = 9;
pub const JNIF : u8 = 10;
pub const JMP : u8 = 11;
pub const IJIF : u8 = 12;
pub const IJNIF : u8 = 13;

pub const CALL : u8 = 50;


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
pub const LSI  : u8 = 115;
pub const LSU  : u8 = 116;
pub const RSI  : u8 = 118;
pub const RSU  : u8 = 119;


pub const LTI  : u8 = 130;
pub const LTU  : u8 = 131;
pub const LTF  : u8 = 132;
pub const GTI  : u8 = 133;
pub const GTU  : u8 = 134;
pub const GTF  : u8 = 135;
pub const LEI  : u8 = 136;
pub const LEU  : u8 = 137;
pub const LEF  : u8 = 138;
pub const GEI  : u8 = 139;
pub const GEU  : u8 = 140;
pub const GEF  : u8 = 141;
pub const EQI  : u8 = 142;
pub const EQU  : u8 = 143;
pub const EQF  : u8 = 144;
pub const NEI  : u8 = 145;
pub const NEU  : u8 = 146;
pub const NEF  : u8 = 147;

pub const CASTIU : u8 = 150;
pub const CASTIF : u8 = 151;
pub const CASTUI : u8 = 152;
pub const CASTUF : u8 = 153;
pub const CASTFI : u8 = 154;
pub const CASTFU : u8 = 155;


pub const PRINT : u8 = 255;
