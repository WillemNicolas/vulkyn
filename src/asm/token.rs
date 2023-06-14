

#[derive(Debug)]
pub enum TokenType {
    EOF,
    UINT(usize),
    INT(isize),
    FLOAT(f64),
    CHAR(char),
    READ,
    SREAD,
    WRITE,
    SWRITE,
    ALLOC,
    FREE,
    SFREE,

    /* REGISTER */
    R1,
    R2,
    R3,
    R4,
    Ts,
    Bs,
    He,
    Fl,
    Li,
    Ni,
    /* MEMORY ACCESS */
    PUSH,
    POP,
    SCOPY,
    SMOVE,
    RCOPY,
    RMOVE,
    RWRITE,
    LOAD,
    LOADB,

    /* OPERATOR */
    // +
    ADD,
    RADD,
    // -
    MINUS,
    RMINUS,
    // *
    MUL,
    RMUL,
    // /
    DIV,
    RDIV,
    // % 
    MOD,
    RMOD,

    // &
    BAND,
    RBAND,
    // |
    BOR,
    RBOR,
    // ^
    BXOR,
    RBXOR,
    // >>
    RSHIFT,
    RRSHIFT,
    // <<
    LSHIFT,
    RLSHIFT,


    // == 
    EQUAL,
    REQUAL,
    // !=
    DIFF,
    RDIFF,
    // ! 
    NOT,
    RNOT,
    // &&
    AND,
    RAND,
    // ||
    OR,
    ROR,

    // <
    LESS,
    RLESS,
    // <=
    ELESS,
    RELESS,
    // >
    GREAT,
    RGREAT,
    // >=
    EGREAT,
    REGREAT,

    F2I,
    F2U,
    F2B,
    F2C,
    RF2I,
    RF2U,
    RF2B,
    RF2C,

    I2F,
    I2U,
    I2B,
    I2C,
    RI2F,
    RI2U,
    RI2B,
    RI2C,

    U2I,
    U2F,
    U2C,
    U2B,
    RU2I,
    RU2F,
    RU2C,
    RU2B,

    C2I,
    C2F,
    C2U,
    C2B,
    RC2I,
    RC2F,
    RC2U,
    RC2B,

    B2I,
    B2F,
    B2U,
    B2C,
    RB2I,
    RB2F,
    RB2U,
    RB2C,

    DMP,
    RDMP,

    /* FLOW */
    EXIT,
    NOP,
    LABEL(String),
    GO,
    GOIF,
    RGOIF,
    CALL,
    RET,
    SCALL,
    CALLP,
    SCALLP,
    RCALL,
    RCALLP,
    /* OTHER */
    O_SBR,
    C_SBR,
    BAR
}

fn register(word:&str) -> Option<TokenType> {
    return match word.to_ascii_lowercase().as_str() {
        /* REGISTER */
        "r1" => Some(TokenType::R1),
        "r2" => Some(TokenType::R2),
        "r3" => Some(TokenType::R3),
        "r4" => Some(TokenType::R4),
        "ts" => Some(TokenType::Ts),
        "bs" => Some(TokenType::Bs),
        "he" => Some(TokenType::He),
        "fl" => Some(TokenType::Fl),
        "li" => Some(TokenType::Li),
        "ni" => Some(TokenType::Ni),
        _ => None
    }
} 
pub fn match_token_type(src : &str) -> Option<TokenType> {
    if let Ok(num) = src.parse::<usize>(){
        return Some(TokenType::UINT(num));
    }
    if src.starts_with("0x") {
        if let Ok(num) = hex::decode(&src[2..]){
            let mut bytes : [u8;8] = [0;8];
            if num.len() > 8 {
                return None;
            }
            for (i,byte) in num.iter().enumerate() {
                bytes[i] = *byte;
            }
            return Some(TokenType::UINT(usize::from_le_bytes(bytes)));
        }    
    }
    if let Ok(num) = src.parse::<isize>() {
        return Some(TokenType::INT(num));
    }
    if let Ok(num) = src.parse::<f64>() {
        return Some(TokenType::FLOAT(num));
    }
    if src.starts_with("'") && src.ends_with("'") && src.len() == 3 {
        return Some(TokenType::CHAR(src.chars().nth(1).unwrap()));
    }
    if src.starts_with("%") {
        return Some(TokenType::LABEL(src[1..].to_string()));
    }
    if let Some(reg) = register(src) {
        return Some(reg);
    }
    return match src.to_ascii_lowercase().as_str() {
        /* OTHER */
        "|" => Some(TokenType::BAR),
        "[" => Some(TokenType::O_SBR),
        "]" => Some(TokenType::C_SBR),
        /* MEMORY ACCESS */
        "rwrite" => Some(TokenType::RWRITE),
        "push" => Some(TokenType::PUSH),
        "pop" => Some(TokenType::POP),
        "scopy" => Some(TokenType::SCOPY),
        "smove" => Some(TokenType::SMOVE),
        "rcopy" => Some(TokenType::RCOPY),
        "rmove" => Some(TokenType::RMOVE),
        "load" => Some(TokenType::LOAD),
        "loadb" => Some(TokenType::LOADB),
        "read" => Some(TokenType::READ),
        "sread" => Some(TokenType::SREAD),
        "write" => Some(TokenType::WRITE),
        "swrite" => Some(TokenType::SWRITE),
        "alloc" => Some(TokenType::ALLOC),
        "free" => Some(TokenType::FREE),
        "sfree" => Some(TokenType::SFREE),
        /* OPERATOR */
        "add" => Some(TokenType::ADD),
        "radd" => Some(TokenType::RADD),
        "sub" => Some(TokenType::MINUS),
        "rsub" => Some(TokenType::RMINUS),
        "mul" => Some(TokenType::MUL),
        "rmul" => Some(TokenType::RMUL),
        "div" => Some(TokenType::DIV),
        "rdiv" => Some(TokenType::RDIV),
        "mod" => Some(TokenType::MOD),
        "rmod" => Some(TokenType::RMOD),

        "band" => Some(TokenType::BAND),
        "rband" => Some(TokenType::RBAND),

        "bor" => Some(TokenType::BOR),
        "rbor" => Some(TokenType::RBOR),
        "bxor" => Some(TokenType::BXOR),
        "rbxor" => Some(TokenType::RBXOR),
        "rsh" => Some(TokenType::RSHIFT),
        "rrsh" => Some(TokenType::RRSHIFT),
        "lsh" => Some(TokenType::LSHIFT),
        "rlsh" => Some(TokenType::RLSHIFT),
        "eq" => Some(TokenType::EQUAL),
        "req" => Some(TokenType::REQUAL),
        "neq" => Some(TokenType::DIFF),
        "rneq" => Some(TokenType::RDIFF),
        "not" => Some(TokenType::NOT),
        "rnot" => Some(TokenType::RNOT),
        "and" => Some(TokenType::AND),
        "rand" => Some(TokenType::RAND),
        "or" => Some(TokenType::OR),
        "ror" => Some(TokenType::ROR),
        "lt" => Some(TokenType::LESS),
        "rlt" => Some(TokenType::RLESS),
        "lte" => Some(TokenType::ELESS),
        "rlte" => Some(TokenType::RELESS),
        "gt" => Some(TokenType::GREAT),
        "rgt" => Some(TokenType::RGREAT),
        "gte" => Some(TokenType::EGREAT),
        "rgte" => Some(TokenType::REGREAT),
        "f2i" => Some(TokenType::F2I),
        "f2u" => Some(TokenType::F2U),
        "f2b" => Some(TokenType::F2B),
        "f2c" => Some(TokenType::F2C),
        "rf2i" => Some(TokenType::RF2I),
        "rf2u" => Some(TokenType::RF2U),
        "rf2b" => Some(TokenType::RF2B),
        "rf2c" => Some(TokenType::RF2C),
        "i2f" => Some(TokenType::I2F),
        "i2u" => Some(TokenType::I2U),
        "i2b" => Some(TokenType::I2B),
        "i2c" => Some(TokenType::I2C),
        "ri2f" => Some(TokenType::RI2F),
        "ri2u" => Some(TokenType::RI2U),
        "ri2b" => Some(TokenType::RI2B),
        "ri2c" => Some(TokenType::RI2C),
        "u2i" => Some(TokenType::U2I),
        "u2f" => Some(TokenType::U2F),
        "u2c" => Some(TokenType::U2C),
        "u2b" => Some(TokenType::U2B),
        "ru2i" => Some(TokenType::RU2I),
        "ru2f" => Some(TokenType::RU2F),
        "ru2c" => Some(TokenType::RU2C),
        "ru2b" => Some(TokenType::RU2B),
        "c2i" => Some(TokenType::C2I),
        "c2f" => Some(TokenType::C2F),
        "c2u" => Some(TokenType::C2U),
        "c2b" => Some(TokenType::C2B),
        "rc2i" => Some(TokenType::RC2I),
        "rc2f" => Some(TokenType::RC2F),
        "rc2u" => Some(TokenType::RC2U),
        "rc2b" => Some(TokenType::RC2B),
        "b2i" => Some(TokenType::B2I),
        "b2f" => Some(TokenType::B2F),
        "b2u" => Some(TokenType::B2U),
        "b2c" => Some(TokenType::B2C),
        "rb2i" => Some(TokenType::RB2I),
        "rb2f" => Some(TokenType::RB2F),
        "rb2u" => Some(TokenType::RB2U),
        "rb2c" => Some(TokenType::RB2C),
        "dmp" => Some(TokenType::DMP),
        "rdmp" => Some(TokenType::RDMP),
        /* FLOW */
        "exit" => Some(TokenType::EXIT),
        "nop" => Some(TokenType::NOP),
        "go" => Some(TokenType::GO),
        "goif" => Some(TokenType::GOIF),
        "rgoif" => Some(TokenType::RGOIF),
        "call" => Some(TokenType::CALL),
        "callp" => Some(TokenType::CALLP),
        "scall" => Some(TokenType::SCALL),
        "scallp" => Some(TokenType::SCALLP),
        "rcall" => Some(TokenType::RCALL),
        "rcallp" => Some(TokenType::RCALLP),
        "ret" => Some(TokenType::RET),
        _ => None
    }
}