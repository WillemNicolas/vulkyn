use std::{iter::Peekable, collections::HashMap, borrow::Borrow};

use crate::vm::{vm::{Instruction,Either}, word::Word, register::Register};

use super::{lexer::Token, token::TokenType};

use core::slice::Iter;

pub struct Parser {
    tokens : Vec<Token>,
    labels : HashMap<String,usize>,
    identifier : Vec<(usize,String)>,
    number_instructions : usize,
}
#[derive(Debug)]
pub enum ParserError {
    RuleError(usize,usize),
    EmptyError
}

impl Parser {
    
    pub fn init(tokens : Vec<Token>) -> Self {
        Self{
            tokens,
            labels:HashMap::new(),
            identifier:Vec::new(),
            number_instructions:0,
        }
    }
    
    pub fn run(&mut self) -> Result<Vec<Instruction>,ParserError>{
        let mut cursor :usize= 0;
        if self.tokens.is_empty() {
            return Err(ParserError::EmptyError);
        }

        return self.parse();
    }
    fn parse(&mut self) -> Result<Vec<Instruction>,ParserError>{
        let mut tokens = self.tokens.iter().peekable();
        let mut res : Vec<Instruction> = Vec::new();
        loop {
            let peek = tokens.next();
            if peek.is_none() {
                break;
            }
            let peek = peek.unwrap();
            match &peek.token {
                TokenType::PUSH => {
                    let some_inst = Parser::rule_push(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                TokenType::SCOPY => {
                    let some_inst = Parser::rule_scopy(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                TokenType::SMOVE => {
                    let some_inst = Parser::rule_smove(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                TokenType::RCOPY => {
                    let some_inst = Parser::rule_rcopy(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                TokenType::RMOVE => {
                    let some_inst = Parser::rule_rmove(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                TokenType::RWRITE => {
                    let some_inst = Parser::rule_rwrite(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                TokenType::LOAD => {
                    let some_inst = Parser::rule_load(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                TokenType::LOADB => {
                    let some_inst = Parser::rule_loadb(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                TokenType::READ => {
                    let some_inst = Parser::rule_read(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                TokenType::SREAD => {
                    let some_inst = Parser::rule_sread(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                },
                TokenType::WRITE => {
                    let some_inst = Parser::rule_write(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                },
                TokenType::SWRITE => {
                    let some_inst = Parser::rule_swrite(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                },
                TokenType::ALLOC => {
                    let some_inst = Parser::rule_alloc(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                },
                TokenType::FREE => {
                    let some_inst = Parser::rule_free(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                },
                TokenType::SFREE => {
                    let some_inst = Parser::rule_sfree(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                },
                /* FLOW */
                TokenType::LABEL(label) => {
                    self.labels.insert(label.to_owned(),self.number_instructions);
                    res.push(Instruction::LABEL);
                    
                }
                TokenType::GO => {
                    let some_inst = Parser::rule_go(&mut tokens,&self.labels);
                    if let Ok(either) = some_inst {
                        match either {
                            Either::Left(inst) => {
                                res.push(inst);
                            }
                            Either::Right((inst,label)) => {
                                self.identifier.push((self.number_instructions,label));
                                res.push(inst);
                            }
                        }
                        
                    } 
                }
                TokenType::GOIF => {
                    let some_inst = Parser::rule_goif(&mut tokens,&self.labels);
                    if let Ok(either) = some_inst {
                        match either {
                            Either::Left(inst) => {
                                res.push(inst);
                            }
                            Either::Right((inst,label)) => {
                                self.identifier.push((self.number_instructions,label));
                                res.push(inst);
                            }
                        }
                        
                    } 
                }
                TokenType::RGOIF => {
                    let some_inst = Parser::rule_rgoif(&mut tokens,&self.labels);
                    if let Ok(either) = some_inst {
                        match either {
                            Either::Left(inst) => {
                                res.push(inst);
                            }
                            Either::Right((inst,label)) => {
                                self.identifier.push((self.number_instructions,label));
                                res.push(inst);
                            }
                        }
                        
                    } 
                }
                TokenType::CALL => {
                    let some_inst = Parser::rule_call(&mut tokens,&self.labels);
                    if let Ok(either) = some_inst {
                        match either {
                            Either::Left(inst) => {
                                res.push(inst);
                            }
                            Either::Right((inst,label)) => {
                                self.identifier.push((self.number_instructions,label));
                                res.push(inst);
                            }
                        }
                        
                    } 
                }
                TokenType::CALLP => {
                    let some_inst = Parser::rule_callp(&mut tokens,&self.labels);
                    if let Ok(either) = some_inst {
                        match either {
                            Either::Left(inst) => {
                                res.push(inst);
                            }
                            Either::Right((inst,label)) => {
                                self.identifier.push((self.number_instructions,label));
                                res.push(inst);
                            }
                        }
                        
                    } 
                }
                TokenType::SCALLP => {
                    let some_inst = Parser::rule_scallp(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                    } 
                }
                TokenType::RCALL => {
                    let some_inst = Parser::rule_rcall(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                    } 
                }
                TokenType::RCALLP => {
                    let some_inst = Parser::rule_rcallp(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                    } 
                }
                TokenType::RET => {
                    let some_inst = Parser::rule_ret(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                    } 
                }
                TokenType::EOF => {
                    break;
                }
                _ => {
                    if let Ok(inst) = Parser::rule_binary_either_param(peek, &mut tokens) {
                        res.push(inst);

                    }else if let Ok(inst) = Parser::rule_unary_either_param(peek, &mut tokens) {
                        res.push(inst);
                    }
                    else {
                        let inst = Parser::rule_no_param(peek)?;
                        res.push(inst);
                    }
                }
            }
            self.number_instructions += 1;
        }
        for (index,label) in &self.identifier {
            if let Some(addr) = self.labels.get(label){
                match res.get_mut(*index) {
                    Some(inst) => {
                        match inst {
                            Instruction::GO(_) => {
                                *inst = Instruction::GO(*addr);
                            }
                            Instruction::GOIF(_) => {
                                *inst = Instruction::GOIF(*addr);
                            }
                            Instruction::RGOIF(_,reg) => {
                                *inst = Instruction::RGOIF(*addr,*reg);
                            }
                            Instruction::CALL(_) => {
                                *inst = Instruction::CALL(*addr);
                            }
                            Instruction::CALLP(_,size) => {
                                *inst = Instruction::CALLP(*addr,*size);
                            }
                            _ => {
                                return Err(ParserError::EmptyError);
                            }
                        }
                    }
                    None => {
                        return Err(ParserError::EmptyError);
                    }
                }
            }else {
                return Err(ParserError::EmptyError);
            }
        }

        return Ok(res);
    }
    fn rule_float(tokens : &mut Peekable<Iter<Token>>) -> Result<f64,ParserError>{
        if let Some(token) = tokens.peek() {
            if let TokenType::FLOAT(num) = token.token {
                tokens.next();
                return Ok(num);
            }
            return Err(ParserError::RuleError(token.line, token.column))
        }
        return Err(ParserError::EmptyError);
    }
    fn rule_char(tokens : &mut Peekable<Iter<Token>>) -> Result<char,ParserError>{
        if let Some(token) = tokens.peek() {
            if let TokenType::CHAR(num) = token.token {
                tokens.next();
                return Ok(num);
            }
            return Err(ParserError::RuleError(token.line, token.column))
        }
        return Err(ParserError::EmptyError);
    }
    fn rule_int(tokens : &mut Peekable<Iter<Token>>) -> Result<isize,ParserError>{
        if let Some(token) = tokens.peek() {
            if let TokenType::INT(num) = token.token {
                tokens.next();
                return Ok(num);
            }
            return Err(ParserError::RuleError(token.line, token.column))
        }
        return Err(ParserError::EmptyError);
    }
    fn rule_uint(tokens : &mut Peekable<Iter<Token>>) -> Result<usize,ParserError>{
        if let Some(token) = tokens.peek() {
            if let TokenType::UINT(num) = token.token {
                tokens.next();
                return Ok(num);
            }
            return Err(ParserError::RuleError(token.line, token.column))
        }
        return Err(ParserError::EmptyError);
    }
    fn rule_addr_op(tokens : &mut Peekable<Iter<Token>>) -> Result<(Register,isize),ParserError>{
        let Some(token) = tokens.peek() else {
            return Err(ParserError::EmptyError);
        };
        if let TokenType::O_SBR = token.token{
            tokens.next();
        }else {
            return Err(ParserError::RuleError(token.line, token.column));
        }
        let reg = Parser::rule_reg(tokens)?;
        let Some(token) = tokens.peek() else {
            return Err(ParserError::EmptyError);
        };
        if let TokenType::C_SBR = token.token{
            tokens.next();
            return Ok((reg,0));
        }else if let TokenType::BAR = token.token{
            tokens.next();
        }else {
            return Err(ParserError::RuleError(token.line, token.column));
        }

        let num = {
            if let Ok(inum) = Parser::rule_int(tokens) {
                inum
            }else {
                Parser::rule_uint(tokens)? as isize
            }
        };
        let Some(token) = tokens.peek() else {
            return Err(ParserError::EmptyError);
        };
        if let TokenType::C_SBR = token.token{
            tokens.next();
        }else {
            return Err(ParserError::RuleError(token.line, token.column));
        }
        return Ok((reg,num));
    }
    fn rule_word(tokens : &mut Peekable<Iter<Token>>) -> Result<Word,ParserError>{
        if let Ok(num) = Parser::rule_uint(tokens){
            return Ok(Word::U64(num));
        }        
        if let Ok(num) = Parser::rule_int(tokens){
            return Ok(Word::I64(num));
        }      
        if let Ok(char) = Parser::rule_char(tokens){
            return Ok(Word::CHAR(char));
        }   
        if let Ok(num) = Parser::rule_float(tokens){
            return Ok(Word::F64(num));
        }
        return Err(ParserError::EmptyError);
    }
    
    
    fn rule_reg(tokens : &mut Peekable<Iter<Token>>) -> Result<Register,ParserError>{
        if let Some(token) = tokens.peek() {
            match token.token {
                TokenType::R1 => {
                    tokens.next();
                    return Ok(Register::R1);
                }
                TokenType::R2 => {
                    tokens.next();
                    return Ok(Register::R2);
                }
                TokenType::R3 => {
                    tokens.next();
                    return Ok(Register::R3);
                }
                TokenType::R4 => {
                    tokens.next();
                    return Ok(Register::R3);
                }
                TokenType::He => {
                    tokens.next();
                    return Ok(Register::He);
                }
                TokenType::Fl => {
                    tokens.next();
                    return Ok(Register::Fl);
                }
                TokenType::Li => {
                    tokens.next();
                    return Ok(Register::Li);
                }
                TokenType::Ni => {
                    tokens.next();
                    return Ok(Register::Ni);
                }
                _ => {
                    return Err(ParserError::RuleError(token.line, token.column));
                }
            }
        }
        return Err(ParserError::EmptyError);
    }
    
    fn rule_no_param(token : &Token) -> Result<Instruction,ParserError>{
        match token.token {
            TokenType::POP => {
                return Ok(Instruction::POP);
            }
            /* OPERATION */
            TokenType::ADD => {
                return Ok(Instruction::ADD);
            }
            TokenType::MINUS => {
                return Ok(Instruction::MINUS);
            }
            TokenType::MUL => {
                return Ok(Instruction::MUL);
            }
            TokenType::DIV => {
                return Ok(Instruction::DIV);
            }
            TokenType::MOD => {
                return Ok(Instruction::MOD);
            }

            TokenType::BAND => {
                return Ok(Instruction::BAND);
            }
            TokenType::BOR => {
                return Ok(Instruction::BOR);
            }
            TokenType::BXOR => {
                return Ok(Instruction::BXOR);
            }
            TokenType::RSHIFT => {
                return Ok(Instruction::RSHIFT);
            }
            TokenType::LSHIFT => {
                return Ok(Instruction::LSHIFT);
            }
            TokenType::LESS => {
                return Ok(Instruction::LESS);
            }
            TokenType::ELESS => {
                return Ok(Instruction::ELESS);
            }
            TokenType::GREAT => {
                return Ok(Instruction::GREAT);
            }
            TokenType::EGREAT => {
                return Ok(Instruction::EGREAT);
            }
            TokenType::EQUAL => {
                return Ok(Instruction::EQUAL);
            }
            TokenType::DIFF => {
                return Ok(Instruction::DIFF);
            }
            TokenType::AND => {
                return Ok(Instruction::AND);
            }
            TokenType::OR => {
                return Ok(Instruction::OR);
            }
            TokenType::NOT => {
                return Ok(Instruction::NOT);
            }
            TokenType::F2I => {
                return Ok(Instruction::F2I);
            }            
            TokenType::F2U => {
                return Ok(Instruction::F2U);
            }            
            TokenType::F2B => {
                return Ok(Instruction::F2B);
            }
            TokenType::F2C => {
                return Ok(Instruction::F2C);
            }
            TokenType::I2F => {
                return Ok(Instruction::I2F);
            }
            TokenType::I2U => {
                return Ok(Instruction::I2U);
            }
            TokenType::I2B => {
                return Ok(Instruction::I2B);
            }
            TokenType::I2C => {
                return Ok(Instruction::I2C);
            }
            TokenType::U2I => {
                return Ok(Instruction::U2I);
            }
            TokenType::U2F => {
                return Ok(Instruction::U2F);
            }
            TokenType::U2C => {
                return Ok(Instruction::U2C);
            }
            TokenType::U2B => {
                return Ok(Instruction::U2B);
            }
            TokenType::C2I => {
                return Ok(Instruction::C2I);
            }
            TokenType::C2F => {
                return Ok(Instruction::C2F);
            }
            TokenType::C2U => {
                return Ok(Instruction::C2U);
            }
            TokenType::C2B => {
                return Ok(Instruction::C2B);
            }
            TokenType::B2I => {
                return Ok(Instruction::B2I);
            }
            TokenType::B2F => {
                return Ok(Instruction::B2F);
            }
            TokenType::B2U => {
                return Ok(Instruction::B2U);
            }
            TokenType::B2C => {
                return Ok(Instruction::B2C);
            }
            TokenType::DMP => {
                return Ok(Instruction::DMP);
            }
            /* FLOW */
            TokenType::EXIT => {
                return Ok(Instruction::EXIT);
            }
            TokenType::NOP => {
                return Ok(Instruction::NOP);
            }
            TokenType::SCALL => {
                return Ok(Instruction::SCALL);
            }
            _ => {
                return Err(ParserError::RuleError(token.line, token.column));
            }
        }
    }

    fn rule_unary_either_param(token : &Token,tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,ParserError>{
        match token.token {
            /* OPERATION */
            TokenType::RNOT => {
                let x = Parser::rule_either(tokens)?;
                return Ok(Instruction::RNOT(x));
            }            
            TokenType::RF2I => {
                let x = Parser::rule_either(tokens)?;
                return Ok(Instruction::RF2I(x));
            }            
            TokenType::RF2U => {
                let x = Parser::rule_either(tokens)?;
                return Ok(Instruction::RF2U(x));
            }            
            TokenType::RF2B => {
                let x = Parser::rule_either(tokens)?;
                return Ok(Instruction::RF2B(x));
            }
            TokenType::RF2C => {
                let x = Parser::rule_either(tokens)?;
                return Ok(Instruction::RF2C(x));
            }
            TokenType::RI2F => {
                let x = Parser::rule_either(tokens)?;
                return Ok(Instruction::RI2F(x));
            }
            TokenType::RI2U => {
                let x = Parser::rule_either(tokens)?;
                return Ok(Instruction::RI2U(x));
            }
            TokenType::RI2B => {
                let x = Parser::rule_either(tokens)?;
                return Ok(Instruction::RI2B(x));
            }
            TokenType::RI2C => {
                let x = Parser::rule_either(tokens)?;
                return Ok(Instruction::RI2C(x));
            }
            TokenType::RU2I => {
                let x = Parser::rule_either(tokens)?;
                return Ok(Instruction::RU2I(x));
            }
            TokenType::RU2F => {
                let x = Parser::rule_either(tokens)?;
                return Ok(Instruction::RU2F(x));
            }
            TokenType::RU2C => {
                let x = Parser::rule_either(tokens)?;
                return Ok(Instruction::RU2C(x));
            }
            TokenType::RU2B => {
                let x = Parser::rule_either(tokens)?;
                return Ok(Instruction::RU2B(x));
            }
            TokenType::RC2I => {
                let x = Parser::rule_either(tokens)?;
                return Ok(Instruction::RC2I(x));
            }
            TokenType::RC2F => {
                let x = Parser::rule_either(tokens)?;
                return Ok(Instruction::RC2F(x));
            }
            TokenType::RC2U => {
                let x = Parser::rule_either(tokens)?;
                return Ok(Instruction::RC2U(x));
            }
            TokenType::RC2B => {
                let x = Parser::rule_either(tokens)?;
                return Ok(Instruction::RC2B(x));
            }
            TokenType::RB2I => {
                let x = Parser::rule_either(tokens)?;
                return Ok(Instruction::RB2I(x));
            }
            TokenType::RB2F => {
                let x = Parser::rule_either(tokens)?;
                return Ok(Instruction::RB2F(x));
            }
            TokenType::RB2U => {
                let x = Parser::rule_either(tokens)?;
                return Ok(Instruction::RB2U(x));
            }
            TokenType::RB2C => {
                let x = Parser::rule_either(tokens)?;
                return Ok(Instruction::RB2C(x));
            }
            TokenType::RDMP => {
                let x = Parser::rule_either(tokens)?;
                return Ok(Instruction::RDMP(x));
            }
            _ => {
                return Err(ParserError::RuleError(token.line, token.column));
            }
        }
    }
    fn binary_either_param(tokens : &mut Peekable<Iter<Token>>) -> Result<(Either<Word,Register>,Either<Word,Register>),ParserError>{
        let x = Parser::rule_either(tokens)?;
        let y = Parser::rule_either(tokens)?;
        return Ok((x,y));
    }
    fn rule_binary_either_param(token : &Token,tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,ParserError>{
        match token.token {
            /* OPERATION */
            TokenType::RADD => {
                let (x,y) = Parser::binary_either_param( tokens)?;
                return Ok(Instruction::RADD(x,y));
            }
            TokenType::RMINUS => {
                let (x,y) = Parser::binary_either_param( tokens)?;
                return Ok(Instruction::RMINUS(x,y));
            }
            TokenType::RMUL => {
                let (x,y) = Parser::binary_either_param( tokens)?;
                return Ok(Instruction::RMUL(x,y));
            }
            TokenType::RDIV => {
                let (x,y) = Parser::binary_either_param( tokens)?;
                return Ok(Instruction::RDIV(x,y));
            }
            TokenType::RMOD => {
                let (x,y) = Parser::binary_either_param( tokens)?;
                return Ok(Instruction::RMOD(x,y));
            }

            TokenType::RBAND => {
                let (x,y) = Parser::binary_either_param( tokens)?;
                return Ok(Instruction::RBAND(x,y));
            }
            TokenType::RBOR => {
                let (x,y) = Parser::binary_either_param( tokens)?;
                return Ok(Instruction::RBOR(x,y));
            }
            TokenType::RBXOR => {
                let (x,y) = Parser::binary_either_param( tokens)?;
                return Ok(Instruction::RBXOR(x,y));
            }
            TokenType::RRSHIFT => {
                let (x,y) = Parser::binary_either_param( tokens)?;
                return Ok(Instruction::RRSHIFT(x,y));
            }
            TokenType::RLSHIFT => {
                let (x,y) = Parser::binary_either_param( tokens)?;
                return Ok(Instruction::RLSHIFT(x,y));
            }
            TokenType::RLESS => {
                let (x,y) = Parser::binary_either_param( tokens)?;
                return Ok(Instruction::RLESS(x,y));
            }
            TokenType::RELESS => {
                let (x,y) = Parser::binary_either_param( tokens)?;
                return Ok(Instruction::RELESS(x,y));
            }
            TokenType::RGREAT => {
                let (x,y) = Parser::binary_either_param( tokens)?;
                return Ok(Instruction::RGREAT(x,y));
            }
            TokenType::REGREAT => {
                let (x,y) = Parser::binary_either_param( tokens)?;
                return Ok(Instruction::REGREAT(x,y));
            }
            TokenType::REQUAL => {
                let (x,y) = Parser::binary_either_param( tokens)?;
                return Ok(Instruction::REQUAL(x,y));
            }
            TokenType::RDIFF => {
                let (x,y) = Parser::binary_either_param( tokens)?;
                return Ok(Instruction::RDIFF(x,y));
            }
            TokenType::RAND => {
                let (x,y) = Parser::binary_either_param( tokens)?;
                return Ok(Instruction::RAND(x,y));
            }
            TokenType::ROR => {
                let (x,y) = Parser::binary_either_param( tokens)?;
                return Ok(Instruction::ROR(x,y));
            }
            _ => {
                return Err(ParserError::RuleError(token.line, token.column));
            }
        }

    }

    fn rule_push(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,ParserError>{
        let word = Parser::rule_either(tokens)?;
        return Ok(Instruction::PUSH(word));
    }
    fn rule_scopy(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,ParserError>{
        let reg = Parser::rule_reg(tokens)?;
        return Ok(Instruction::SCOPY(reg));
    }
    fn rule_smove(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,ParserError>{
        let reg = Parser::rule_reg(tokens)?;
        return Ok(Instruction::SMOVE(reg));
    }
    fn rule_rcopy(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,ParserError>{
        let reg1 = Parser::rule_reg(tokens)?;
        let reg2 = Parser::rule_reg(tokens)?;
        return Ok(Instruction::RCOPY(reg1,reg2));
    }
    fn rule_rmove(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,ParserError>{
        let reg1 = Parser::rule_reg(tokens)?;
        let reg2 = Parser::rule_reg(tokens)?;
        return Ok(Instruction::RMOVE(reg1,reg2));
    }
    fn rule_rwrite(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,ParserError>{
        let word = Parser::rule_word(tokens)?;
        let reg = Parser::rule_reg(tokens)?;
        return Ok(Instruction::RWRITE(word,reg));
    }
    fn rule_load(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,ParserError>{
        let addr_op = Parser::rule_addr_op(tokens)?;
        return Ok(Instruction::LOAD(addr_op));
    }
    fn rule_loadb(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,ParserError>{
        let addr_op = Parser::rule_addr_op(tokens)?;
        let size = Parser::rule_uint(tokens)?;
        return Ok(Instruction::LOADB(addr_op,size));
    }
    fn rule_read(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,ParserError>{
        let addr_op = Parser::rule_addr_op(tokens)?;
        let size = Parser::rule_uint(tokens)?;
        let offset = Parser::rule_uint(tokens)?;
        return Ok(Instruction::READ(addr_op,size,offset));
    }
    fn rule_sread(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,ParserError>{
        let size = Parser::rule_uint(tokens)?;
        let offset = Parser::rule_uint(tokens)?;
        return Ok(Instruction::SREAD(size,offset));
    }
    fn rule_write(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,ParserError>{
        let word = Parser::rule_word(tokens)?;
        let addr_op = Parser::rule_addr_op(tokens)?;
        return Ok(Instruction::WRITE(word,addr_op));
    }
    fn rule_swrite(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,ParserError>{
        return Ok(Instruction::SWRITE);
    }
    fn rule_alloc(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,ParserError>{
        let size = Parser::rule_uint(tokens)?;
        return Ok(Instruction::ALLOC(size));
    }
    fn rule_free(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,ParserError>{
        let addr_op = Parser::rule_addr_op(tokens)?;
        return Ok(Instruction::FREE(addr_op));
    }
    fn rule_sfree(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,ParserError>{
        return Ok(Instruction::SFREE);
    }
    fn rule_either(tokens : &mut Peekable<Iter<Token>>) -> Result<Either<Word,Register>,ParserError> {
        let reg = Parser::rule_reg(tokens);
        if reg.is_err() {
            let word = Parser::rule_word(tokens)?;
            return Ok(Either::Left(word));
        }
        Ok(Either::Right(reg.unwrap()))  
    }
    /* FLOW */
    fn rule_label(tokens : &mut Peekable<Iter<Token>>) -> Result<String,ParserError>{
        if let Some(token) = tokens.peek() {
            if let TokenType::LABEL(label) = &token.token{
                tokens.next();
                return Ok(label.to_owned());
            }
            return Err(ParserError::RuleError(token.line, token.column));
        }
        return Err(ParserError::EmptyError);
    }
    fn rule_go(tokens : &mut Peekable<Iter<Token>>,labels : &HashMap<String,usize>) -> Result<Either<Instruction,(Instruction,String)>,ParserError>{
        let label = Parser::rule_label(tokens)?;
        if let Some(addr) = labels.get(&label) {
            return Ok(Either::Left(Instruction::GO(*addr)));
        }else {
            return Ok(Either::Right((Instruction::GO(0),label.to_owned())))
        }
    }
    fn rule_goif(tokens : &mut Peekable<Iter<Token>>,labels : &HashMap<String,usize>) -> Result<Either<Instruction,(Instruction,String)>,ParserError>{
        let label = Parser::rule_label(tokens)?;
        if let Some(addr) = labels.get(&label) {
            return Ok(Either::Left(Instruction::GOIF(*addr)));
        }else {
            return Ok(Either::Right((Instruction::GOIF(0),label.to_owned())))
        }
    }
    fn rule_rgoif(tokens : &mut Peekable<Iter<Token>>,labels : &HashMap<String,usize>) -> Result<Either<Instruction,(Instruction,String)>,ParserError>{
        let label = Parser::rule_label(tokens)?;
        let reg = Parser::rule_reg(tokens)?;
        if let Some(addr) = labels.get(&label) {
            return Ok(Either::Left(Instruction::RGOIF(*addr,reg)));
        }else {
            return Ok(Either::Right((Instruction::RGOIF(0,reg),label.to_owned())))
        }
    }
    fn rule_call(tokens : &mut Peekable<Iter<Token>>,labels : &HashMap<String,usize>) -> Result<Either<Instruction,(Instruction,String)>,ParserError>{
        let label = Parser::rule_label(tokens)?;
        if let Some(addr) = labels.get(&label) {
            return Ok(Either::Left(Instruction::CALL(*addr)));
        }else {
            return Ok(Either::Right((Instruction::CALL(0),label.to_owned())))
        }
    }
    fn rule_callp(tokens : &mut Peekable<Iter<Token>>,labels : &HashMap<String,usize>) -> Result<Either<Instruction,(Instruction,String)>,ParserError>{
        let label = Parser::rule_label(tokens)?;
        let size = Parser::rule_uint(tokens)?;
        if let Some(addr) = labels.get(&label) {
            return Ok(Either::Left(Instruction::CALLP(*addr,size)));
        }else {
            return Ok(Either::Right((Instruction::CALLP(0,size),label.to_owned())))
        }
    }
    fn rule_scallp(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,ParserError>{
        let size = Parser::rule_uint(tokens)?;
        return Ok(Instruction::SCALLP(size));
    }
    fn rule_ret(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,ParserError>{
        let size = Parser::rule_uint(tokens)?;
        return Ok(Instruction::RET(size));
    }
    fn rule_rcall(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,ParserError>{
        let reg = Parser::rule_reg(tokens)?;
        return Ok(Instruction::RCALL(reg));
    }
    fn rule_rcallp(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,ParserError>{
        let reg = Parser::rule_reg(tokens)?;
        let size = Parser::rule_uint(tokens)?;
        return Ok(Instruction::RCALLP(reg,size));
    }
}