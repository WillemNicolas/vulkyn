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
                Token::POP => {
                    let some_inst = Parser::rule_pop(&mut tokens);
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
                /* OPERATION */
                Token::ADD => {
                    let some_inst = Parser::rule_add(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                Token::MINUS => {
                    let some_inst = Parser::rule_minus(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                Token::MUL => {
                    let some_inst = Parser::rule_mul(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                Token::DIV => {
                    let some_inst = Parser::rule_div(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                Token::MOD => {
                    let some_inst = Parser::rule_mod(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                Token::RADD => {
                    let some_inst = Parser::rule_radd(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                Token::RMINUS => {
                    let some_inst = Parser::rule_rminus(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                Token::RMUL => {
                    let some_inst = Parser::rule_rmul(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                Token::RDIV => {
                    let some_inst = Parser::rule_rdiv(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                Token::RMOD => {
                    let some_inst = Parser::rule_rmod(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                /* FLOW */
                Token::EXIT => {
                    let some_inst = Parser::rule_exit(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
                Token::NOP => {
                    let some_inst = Parser::rule_nop(&mut tokens);
                    if let Ok(inst) = some_inst {
                        res.push(inst);
                        
                    } 
                }
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
                    return Err(());
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
    
    fn rule_push(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,()>{
        if let Ok(word) = Parser::rule_word(tokens){
            return Ok(Instruction::PUSH(word));
        }
        return Err(());
    }
    fn rule_pop(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,()>{
        return Ok(Instruction::POP);
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
    /* OPERATION */
    fn rule_add(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,()>{        
        return Ok(Instruction::ADD);
    }
    fn rule_minus(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,()>{        
        return Ok(Instruction::MINUS);
    }
    fn rule_mul(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,()>{        
        return Ok(Instruction::MUL);
    }
    fn rule_div(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,()>{        
        return Ok(Instruction::DIV);
    }
    fn rule_mod(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,()>{        
        return Ok(Instruction::MOD);
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
    fn rule_radd(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,()>{   
        let some_x = Parser::rule_either(tokens);
        if some_x.is_err(){
            return Err(());
        }
        let some_y = Parser::rule_either(tokens);
        if some_y.is_err(){
            return Err(());
        }
        return Ok(Instruction::RADD(some_x.unwrap(), some_y.unwrap()));
    }
    fn rule_rminus(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,()>{  
        let some_x = Parser::rule_either(tokens);
        if some_x.is_err(){
            return Err(());
        }
        let some_y = Parser::rule_either(tokens);
        if some_y.is_err(){
            return Err(());
        }
        return Ok(Instruction::RMINUS(some_x.unwrap(), some_y.unwrap()));
    }
    fn rule_rmul(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,()>{        
        let some_x = Parser::rule_either(tokens);
        if some_x.is_err(){
            return Err(());
        }
        let some_y = Parser::rule_either(tokens);
        if some_y.is_err(){
            return Err(());
        }
        return Ok(Instruction::RMUL(some_x.unwrap(), some_y.unwrap()));
    }
    fn rule_rdiv(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,()>{        
        let some_x = Parser::rule_either(tokens);
        if some_x.is_err(){
            return Err(());
        }
        let some_y = Parser::rule_either(tokens);
        if some_y.is_err(){
            return Err(());
        }
        return Ok(Instruction::RDIV(some_x.unwrap(), some_y.unwrap()));
    }
    fn rule_rmod(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,()>{        
        let some_x = Parser::rule_either(tokens);
        if some_x.is_err(){
            return Err(());
        }
        let some_y = Parser::rule_either(tokens);
        if some_y.is_err(){
            return Err(());
        }
        return Ok(Instruction::RMOD(some_x.unwrap(), some_y.unwrap()));
    }
    /* FLOW */
    fn rule_exit(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,()>{        
        return Ok(Instruction::EXIT);
    }
    fn rule_nop(tokens : &mut Peekable<Iter<Token>>) -> Result<Instruction,()>{
        return Ok(Instruction::NOP);
    }
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