#[derive(Clone,Debug)]
pub enum Token {
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
}

#[derive(Debug)]
pub enum LexerError {
    IllegalInstruction,
}

pub struct Lexer {
    pub lexems : Vec<Token>
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            lexems : Vec::new()
        }
    }
    pub fn run(&mut self,src : &String) -> Option<LexerError>{
        let mut current_token = Token::NOP;
        for word in src.split_ascii_whitespace(){
            if let Ok(num) = word.parse::<usize>(){
                self.lexems.push(Token::UINT(num));
                continue;
            }
            if let Ok(num) = word.parse::<isize>(){
                self.lexems.push(Token::INT(num));
                continue;
            }
            if let Ok(num) = word.parse::<f64>(){
                self.lexems.push(Token::FLOAT(num));
                continue;
            }
            if word.starts_with("'") && word.ends_with("'") && word.len() == 3{
                self.lexems.push(Token::CHAR(word.chars().nth(1).unwrap()));
                continue;
            }
            if word.starts_with("%") {
                self.lexems.push(Token::LABEL(word[1..].to_string()));
                continue;
            }
            match word.to_ascii_lowercase().as_str() {
                /* REGISTER */
                "r1" => {
                    self.lexems.push(Token::R1);
                    continue;
                }
                "r2" => {
                    self.lexems.push(Token::R2);
                    continue;
                }
                "r3" => {
                    self.lexems.push(Token::R3);
                    continue;
                }
                "r4" => {
                    self.lexems.push(Token::R4);
                    continue;
                }
                "he" => {
                    self.lexems.push(Token::He);
                    continue;
                }
                "fl" => {
                    self.lexems.push(Token::Fl);
                    continue;
                }
                "li" => {
                    self.lexems.push(Token::Li);
                    continue;
                }
                "ni" => {
                    self.lexems.push(Token::Ni);
                    continue;
                }
                /* MEMORY ACCESS */
                "write" => {
                    self.lexems.push(Token::WRITE);
                    continue;
                }
                "push" => {
                    self.lexems.push(Token::PUSH);
                    continue;
                }
                "pop" => {
                    self.lexems.push(Token::POP);
                    continue;
                }
                "scopy" => {
                    self.lexems.push(Token::SCOPY);
                    continue;
                }
                "smove" => {
                    self.lexems.push(Token::SMOVE);
                    continue;
                }
                "rcopy" => {
                    self.lexems.push(Token::RCOPY);
                    continue;
                }
                "rmove" => {
                    self.lexems.push(Token::RMOVE);
                    continue;
                }
                /* OPERATOR */
                "add" => {
                    self.lexems.push(Token::ADD);
                    continue;
                }
                "radd" => {
                    self.lexems.push(Token::RADD);
                    continue;
                }
                "sub" => {
                    self.lexems.push(Token::MINUS);
                    continue;
                }
                "rsub" => {
                    self.lexems.push(Token::RMINUS);
                    continue;
                }
                "mul" => {
                    self.lexems.push(Token::MUL);
                    continue;
                }
                "rmul" => {
                    self.lexems.push(Token::RMUL);
                    continue;
                }
                "div" => {
                    self.lexems.push(Token::DIV);
                    continue;
                }
                "rdiv" => {
                    self.lexems.push(Token::RDIV);
                    continue;
                }
                "mod" => {
                    self.lexems.push(Token::MOD);
                    continue;
                }
                "rmod" => {
                    self.lexems.push(Token::RMOD);
                    continue;
                }

                "band" => {
                    self.lexems.push(Token::BAND);
                    continue;
                }
                "rband" => {
                    self.lexems.push(Token::RBAND);
                    continue;
                }

                "bor" => {
                    self.lexems.push(Token::BOR);
                    continue;
                }
                "rbor" => {
                    self.lexems.push(Token::RBOR);
                    continue;
                }
                "bxor" => {
                    self.lexems.push(Token::BXOR);
                    continue;
                }
                "rbxor" => {
                    self.lexems.push(Token::RBXOR);
                    continue;
                }
                "rsh" => {
                    self.lexems.push(Token::RSHIFT);
                    continue;
                }
                "rrsh" => {
                    self.lexems.push(Token::RRSHIFT);
                    continue;
                }
                "lsh" => {
                    self.lexems.push(Token::LSHIFT);
                    continue;
                }
                "rlsh" => {
                    self.lexems.push(Token::RLSHIFT);
                    continue;
                }
                "eq" => {
                    self.lexems.push(Token::EQUAL);
                    continue;
                }
                "req" => {
                    self.lexems.push(Token::REQUAL);
                    continue;
                }
                "neq" => {
                    self.lexems.push(Token::DIFF);
                    continue;
                }
                "rneq" => {
                    self.lexems.push(Token::RDIFF);
                    continue;
                }
                "not" => {
                    self.lexems.push(Token::NOT);
                    continue;
                }
                "rnot" => {
                    self.lexems.push(Token::RNOT);
                    continue;
                }
                "and" => {
                    self.lexems.push(Token::AND);
                    continue;
                }
                "rand" => {
                    self.lexems.push(Token::RAND);
                    continue;
                }
                "or" => {
                    self.lexems.push(Token::OR);
                    continue;
                }
                "ror" => {
                    self.lexems.push(Token::ROR);
                    continue;
                }
                "lt" => {
                    self.lexems.push(Token::LESS);
                    continue;
                }
                "rlt" => {
                    self.lexems.push(Token::RLESS);
                    continue;
                }
                "lte" => {
                    self.lexems.push(Token::ELESS);
                    continue;
                }
                "rlte" => {
                    self.lexems.push(Token::RELESS);
                    continue;
                }
                "gt" => {
                    self.lexems.push(Token::GREAT);
                    continue;
                }
                "rgt" => {
                    self.lexems.push(Token::RGREAT);
                    continue;
                }
                "gte" => {
                    self.lexems.push(Token::EGREAT);
                    continue;
                }
                "rgte" => {
                    self.lexems.push(Token::REGREAT);
                    continue;
                }
                /* FLOW */
                "exit" => {
                    self.lexems.push(Token::EXIT);
                    continue;
                }
                "nop" => {
                    self.lexems.push(Token::NOP);
                    continue;
                }
                "go" => {
                    self.lexems.push(Token::GO);
                    continue;
                }
                "goif" => {
                    self.lexems.push(Token::GOIF);
                    continue;
                }
                "rgoif" => {
                    self.lexems.push(Token::RGOIF);
                    continue;
                }
                _ => {
                    return Some(LexerError::IllegalInstruction);
                }
            }
        }
        self.lexems.push(Token::EOF);
        return None;
    }
}