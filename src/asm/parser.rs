use std::{iter::Peekable, collections::HashMap};

use crate::vm::{vm::{Instruction, Register,Either}, memory::Word};

use super::lexer::Token;

use core::slice::Iter;

pub struct Parser {
    tokens : Vec<Token>,
    labels : HashMap<String,usize>,
    identifier : Vec<(usize,String)>,
    number_instructions : usize,
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

    pub fn run(&mut self) -> Result<Vec<Instruction>,()>{
        let mut cursor :usize= 0;
        if self.tokens.is_empty() {
            return Err(());
        }

        return self.parse();
    }
    fn parse(&mut self) -> Result<Vec<Instruction>,()>{
        let mut tokens = self.tokens.iter().peekable();
        let mut res : Vec<Instruction> = Vec::new();
        loop {
            let peek = tokens.next();
            if peek.is_none() {
                break;
            }
            let peek = peek.unwrap();
            match peek {
                Token::PUSH => {
                    let some_inst = Parser::rule_push(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                Token::SCOPY => {
                    let some_inst = Parser::rule_scopy(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                Token::SMOVE => {
                    let some_inst = Parser::rule_smove(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                Token::RCOPY => {
                    let some_inst = Parser::rule_rcopy(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                Token::RMOVE => {
                    let some_inst = Parser::rule_rmove(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                Token::WRITE => {
                    let some_inst = Parser::rule_write(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                /* FLOW */
                Token::LABEL(label) => {
                    self.labels.insert(label.to_owned(),self.number_instructions);
                    res.push(Instruction::LABEL);
                    
                }
                Token::GO => {
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
                Token::GOIF => {
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
                Token::RGOIF => {
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
                Token::EOF => {
                    break;
                }
                _ => {
                    if let Ok(inst) = Parser::rule_binary_either_param(peek, &mut tokens) {
                        res.push(inst);
                    }else {
                        let Ok(inst) = Parser::rule_no_param(peek) else {
                            return Err(());
                        };
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
                                //dbg!((index,label));
                                *inst = Instruction::GO(*addr);
                            }
                            Instruction::GOIF(_) => {
                                *inst = Instruction::GOIF(*addr);
                            }
                            Instruction::RGOIF(_,reg) => {
                                *inst = Instruction::RGOIF(*addr,*reg);
                            }
                            _ => {
                                dbg!((index,label));
                                return Err(());
                            }
                        }
                    }
                    None => {
                        return Err(());
                    }
                }
            }else {
                return Err(());
            }
        }

        return Ok(res);
    }
    fn rule_float(tokens : &mut Peekable<Iter<Token>>) -> Result<f64,()>{
        if let Some(Token::FLOAT(num)) = tokens.peek() {
            tokens.next();
            return Ok(*num);
        }
        return Err(());
    }
    fn rule_char(tokens : &mut Peekable<Iter<Token>>) -> Result<char,()>{
        if let Some(Token::CHAR(char)) = tokens.peek() {
            tokens.next();
            return Ok(*char);
        }
        return Err(());
    }
    fn rule_int(tokens : &mut Peekable<Iter<Token>>) -> Result<isize,()>{
        if let Some(Token::INT(num)) = tokens.peek() {
            tokens.next();
            return Ok(*num);
        }
        return Err(());
    }
    fn rule_uint(tokens : &mut Peekable<Iter<Token>>) -> Result<usize,()>{
        if let Some(Token::UINT(num)) = tokens.peek() {
            tokens.next();
            return Ok(*num);
        }
        return Err(());
    }

    fn rule_word(tokens : &mut Peekable<Iter<Token>>) -> Result<Word,()>{
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
        return Err(());
    }
    
    
    fn rule_reg(tokens : &mut Peekable<Iter<Token>>) -> Result<Register,()>{
        if let Some(token) = tokens.peek() {
            match token {
                Token::R1 => {
                    tokens.next();
                    return Ok(Register::R1);
                }
                Token::R2 => {
                    tokens.next();
                    return Ok(Register::R2);
                }
                Token::R3 => {
                    tokens.next();
                    return Ok(Register::R3);
                }
                Token::R4 => {
                    tokens.next();
                    return Ok(Register::R3);
                }
                Token::He => {
                    tokens.next();
                    return Ok(Register::He);
                }
                Token::Fl => {
                    tokens.next();
                    return Ok(Register::Fl);
                }
                Token::Li => {
                    tokens.next();
                    return Ok(Register::Li);
                }
                Token::Ni => {
                    tokens.next();
                    return Ok(Register::Ni);
                }
                _ => {
                    return Err(());
                }
            }
        }
        return Err(());
    }
    
    fn rule_no_param(token : &Token) -> Result<Instruction,()>{
        match token {
            Token::POP => {
                return Ok(Instruction::POP);
            }
            /* OPERATION */
            Token::ADD => {
                return Ok(Instruction::ADD);
            }
            Token::MINUS => {
                return Ok(Instruction::MINUS);
            }
            Token::MUL => {
                return Ok(Instruction::MUL);
            }
            Token::DIV => {
                return Ok(Instruction::DIV);
            }
            Token::MOD => {
                return Ok(Instruction::MOD);
            }

            Token::BAND => {
                return Ok(Instruction::BAND);
            }
            Token::BOR => {
                return Ok(Instruction::BOR);
            }
            Token::BXOR => {
                return Ok(Instruction::BXOR);
            }
            Token::RSHIFT => {
                return Ok(Instruction::RSHIFT);
            }
            Token::LSHIFT => {
                return Ok(Instruction::LSHIFT);
            }
            Token::LESS => {
                return Ok(Instruction::LESS);
            }
            Token::ELESS => {
                return Ok(Instruction::ELESS);
            }
            Token::GREAT => {
                return Ok(Instruction::GREAT);
            }
            Token::EGREAT => {
                return Ok(Instruction::EGREAT);
            }
            Token::EQUAL => {
                return Ok(Instruction::EQUAL);
            }
            Token::DIFF => {
                return Ok(Instruction::DIFF);
            }
            Token::AND => {
                return Ok(Instruction::AND);
            }
            Token::OR => {
                return Ok(Instruction::OR);
            }
            /* FLOW */
            Token::EXIT => {
                return Ok(Instruction::EXIT);
            }
            Token::NOP => {
                return Ok(Instruction::NOP);
            }
            _ => {
                return Err(())
            }
        }
    }
     
    fn rule_binary_either_param(token : &Token,tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,()>{
        let some_x = Parser::rule_either(tokens);
        if some_x.is_err(){
            return Err(());
        }
        let some_y = Parser::rule_either(tokens);
        if some_y.is_err(){
            return Err(());
        }
        let x = some_x.unwrap();
        let y = some_y.unwrap();
        match token {
            /* OPERATION */
            Token::RADD => {
                return Ok(Instruction::RADD(x,y));
            }
            Token::RMINUS => {
                return Ok(Instruction::RMINUS(x,y));
            }
            Token::RMUL => {
                return Ok(Instruction::RMUL(x,y));
            }
            Token::RDIV => {
                return Ok(Instruction::RDIV(x,y));
            }
            Token::RMOD => {
                return Ok(Instruction::RMOD(x,y));
            }

            Token::RBAND => {
                return Ok(Instruction::RBAND(x,y));
            }
            Token::RBOR => {
                return Ok(Instruction::RBOR(x,y));
            }
            Token::RBXOR => {
                return Ok(Instruction::RBXOR(x,y));
            }
            Token::RRSHIFT => {
                return Ok(Instruction::RRSHIFT(x,y));
            }
            Token::RLSHIFT => {
                return Ok(Instruction::RLSHIFT(x,y));
            }
            Token::RLESS => {
                return Ok(Instruction::RLESS(x,y));
            }
            Token::RELESS => {
                return Ok(Instruction::RELESS(x,y));
            }
            Token::RGREAT => {
                return Ok(Instruction::RGREAT(x,y));
            }
            Token::REGREAT => {
                return Ok(Instruction::REGREAT(x,y));
            }
            Token::REQUAL => {
                return Ok(Instruction::REQUAL(x,y));
            }
            Token::RDIFF => {
                return Ok(Instruction::RDIFF(x,y));
            }
            Token::RAND => {
                return Ok(Instruction::RAND(x,y));
            }
            Token::ROR => {
                return Ok(Instruction::ROR(x,y));
            }
            _ => {
                return Err(())
            }
        }
    }

    fn rule_push(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,()>{
        if let Ok(word) = Parser::rule_word(tokens){
            return Ok(Instruction::PUSH(word));
        }
        return Err(());
    }
    fn rule_scopy(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,()>{
        if let Ok(reg) = Parser::rule_reg(tokens){
            return Ok(Instruction::SCOPY(reg));
        }
        return Err(());
    }
    fn rule_smove(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,()>{
        if let Ok(reg) = Parser::rule_reg(tokens){
            return Ok(Instruction::SMOVE(reg));
        }
        return Err(())
    }
    fn rule_rcopy(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,()>{
        let some_reg1 = Parser::rule_reg(tokens);
        if some_reg1.is_err() {
            return Err(());
        }
        let some_reg2 = Parser::rule_reg(tokens);
        if some_reg2.is_err() {
            return Err(());
        }
        let reg1 = some_reg1.unwrap();
        let reg2 = some_reg2.unwrap();
        return Ok(Instruction::RCOPY(reg1,reg2));
    }
    fn rule_rmove(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,()>{
        let some_reg1 = Parser::rule_reg(tokens);
        if some_reg1.is_err() {
            return Err(());
        }
        let some_reg2 = Parser::rule_reg(tokens);
        if some_reg2.is_err() {
            return Err(());
        }
        let reg1 = some_reg1.unwrap();
        let reg2 = some_reg2.unwrap();
        return Ok(Instruction::RMOVE(reg1,reg2));
    }
    fn rule_write(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,()>{
        let some_word = Parser::rule_word(tokens);
        if some_word.is_err() {
            return Err(());
        }
        let some_reg2 = Parser::rule_reg(tokens);
        if some_reg2.is_err() {
            return Err(());
        }
        let word = some_word.unwrap();
        let reg2 = some_reg2.unwrap();
        return Ok(Instruction::WRITE(word,reg2));
    }
    fn rule_either(tokens : &mut Peekable<Iter<Token>>) -> Result<Either<Word,Register>,()> {
        let reg = Parser::rule_reg(tokens);
        if reg.is_err() {
            let word = Parser::rule_word(tokens);
            if word.is_err(){
                return Err(());
            }
            tokens.next();
            return Ok(Either::Left(word.unwrap()));
        }
        tokens.next();
        Ok(Either::Right(reg.unwrap()))  
    }
    /* FLOW */
    fn rule_label(tokens : &mut Peekable<Iter<Token>>) -> Result<String,()>{
        if let Some(Token::LABEL(label)) = tokens.peek() {
            tokens.next();
            return Ok(label.to_owned());
        }
        return Err(());
    }
    fn rule_go(tokens : &mut Peekable<Iter<Token>>,labels : &HashMap<String,usize>) -> Result<Either<Instruction,(Instruction,String)>,()>{
        let some_label = Parser::rule_label(tokens);
        if some_label.is_err(){
            return Err(());
        }
        let label = some_label.unwrap();
        dbg!(&labels);
        if let Some(addr) = labels.get(&label) {
            return Ok(Either::Left(Instruction::GO(*addr)));
        }else {
            return Ok(Either::Right((Instruction::GO(0),label.to_owned())))
        }
    }
    fn rule_goif(tokens : &mut Peekable<Iter<Token>>,labels : &HashMap<String,usize>) -> Result<Either<Instruction,(Instruction,String)>,()>{
        let some_label = Parser::rule_label(tokens);
        if some_label.is_err(){
            return Err(());
        }
        let label = some_label.unwrap();
        if let Some(addr) = labels.get(&label) {
            return Ok(Either::Left(Instruction::GOIF(*addr)));
        }else {
            return Ok(Either::Right((Instruction::GOIF(0),label.to_owned())))
        }
    }
    fn rule_rgoif(tokens : &mut Peekable<Iter<Token>>,labels : &HashMap<String,usize>) -> Result<Either<Instruction,(Instruction,String)>,()>{
        let some_label = Parser::rule_label(tokens);
        if some_label.is_err(){
            return Err(());
        }
        let label = some_label.unwrap();
        let some_reg = Parser::rule_reg(tokens);
        if some_reg.is_err() {
            return Err(());
        }
        let reg = some_reg.unwrap();
        if let Some(addr) = labels.get(&label) {
            return Ok(Either::Left(Instruction::RGOIF(*addr,reg)));
        }else {
            return Ok(Either::Right((Instruction::RGOIF(0,reg),label.to_owned())))
        }
    }
}