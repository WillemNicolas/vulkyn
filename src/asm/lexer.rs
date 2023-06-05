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
    ADD,
    MINUS,
    MUL,
    DIV,
    MOD,

    RADD,
    RMINUS,
    RMUL,
    RDIV,
    RMOD,

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
                "sub" => {
                    self.lexems.push(Token::MINUS);
                    continue;
                }
                "mul" => {
                    self.lexems.push(Token::MUL);
                    continue;
                }
                "div" => {
                    self.lexems.push(Token::DIV);
                    continue;
                }
                "mod" => {
                    self.lexems.push(Token::MOD);
                    continue;
                }
                "radd" => {
                    self.lexems.push(Token::RADD);
                    continue;
                }
                "rsub" => {
                    self.lexems.push(Token::RMINUS);
                    continue;
                }
                "rmul" => {
                    self.lexems.push(Token::RMUL);
                    continue;
                }
                "rdiv" => {
                    self.lexems.push(Token::RDIV);
                    continue;
                }
                "rmod" => {
                    self.lexems.push(Token::RMOD);
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