

#[derive(Debug)]
pub enum TokenType {
    EOF,
    UINT(usize),
    INT(isize),
    FLOAT(f64),
    CHAR(char),
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
    WRITE,
    SLOAD,
    SLOADB,

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
    dbg!(&src);
    if let Ok(num) = src.parse::<usize>(){
        return Some(TokenType::UINT(num));
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
        "write" => Some(TokenType::WRITE),
        "push" => Some(TokenType::PUSH),
        "pop" => Some(TokenType::POP),
        "scopy" => Some(TokenType::SCOPY),
        "smove" => Some(TokenType::SMOVE),
        "rcopy" => Some(TokenType::RCOPY),
        "rmove" => Some(TokenType::RMOVE),
        "sload" => Some(TokenType::SLOAD),
        "sloadb" => Some(TokenType::SLOADB),

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