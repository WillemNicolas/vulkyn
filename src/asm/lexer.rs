use super::token::{TokenType, match_token_type};


#[derive(Debug)]
pub struct Token {
    pub token : TokenType,
    pub line : usize,
    pub column : usize,
}
#[derive(Debug)]
pub enum LexerError {
    UnrecognizedToken(usize,usize,String)
}

fn eat_whitespace(src : &str, cursor : usize) -> usize {
    let Some((idx,_)) = src[cursor..].char_indices().find(|(_,char)| {
        return *char != ' ';
    }) else {
        return src.len();
    };
    return cursor + idx;
}

fn until_new_line(src : &str, cursor : usize) -> usize {
    let Some((idx,_)) = src[cursor..].char_indices().find(|(_,char)| {
        return *char == '\n';
    }) else {
        return src.len();
    };
    return cursor + idx;
}

fn word(src : &str, cursor : usize) -> (usize,usize) {
    if let Some(char) = &src[cursor..].chars().next() {
        if *char == '\n' {
            return (cursor,cursor+1)
        }
    }
    let (idx,_) = src[cursor..].char_indices().find(|(idx,char)| {
        return char.is_ascii_whitespace() || *char == '\n';
    }).unwrap_or((src.len() - cursor,char::default()));
    return (cursor,cursor + idx);  
}

pub fn tokenize(src : &str) -> Result<Vec<Token>,LexerError> {
    let mut cursor : usize = 0;
    let src_size = src.len();
    let mut res : Vec<Token> = vec![];

    let mut line =1;
    let mut column = 1;
    let mut start_column_cursor = 1;

    while cursor < src_size{
        cursor = eat_whitespace(src, cursor);
        if let Some(char) = &src[cursor..].chars().next() {
            if *char == ';' {
                cursor = until_new_line(src, cursor);
            }
        }
        if cursor == src_size {
            break;
        }
        let (start_word,end_word) = word(src, cursor);
        let mut end_word = end_word;
        if &src[start_word..end_word] == "\n"{
            line += 1;
            column = 0;
            cursor = end_word;
            start_column_cursor = cursor;
            continue;
        }
        if let Some(sep_idx) = src[start_word..end_word].find("[") {
            if sep_idx == 0 {
                end_word = start_word + sep_idx + 1;
            }
            else if end_word - start_word > 1{
                end_word = start_word + sep_idx;
            }
        }
        else if let Some(sep_idx) = src[start_word..end_word].find("|") {
            if sep_idx == 0 {
                end_word = start_word + sep_idx + 1;
            }
            else if end_word - start_word > 1{
                end_word = start_word + sep_idx;
            }
        }
        else if let Some(sep_idx) = src[start_word..end_word].find("]") {
            if sep_idx == 0 {
                end_word = start_word + sep_idx + 1;
            }
            else if end_word - start_word > 1{
                end_word = start_word + sep_idx;
            }
        }

        let some_tt = match_token_type(&src[start_word..end_word]);
        let some_tt = match some_tt {
            Some(tt) => Some(tt),
            None => {
                if src[start_word..].starts_with("[") {
                    end_word = start_word + 1;
                    Some(TokenType::O_SBR)
                }else if src[start_word..].starts_with("|") {
                    end_word = start_word + 1;
                    Some(TokenType::BAR)
                }else {
                    None
                }
            }
        };
        let Some(tt) = some_tt else {
            return Err(LexerError::UnrecognizedToken(line, column,src[start_word..end_word].to_string()))
        };
        let token = Token{
            token : tt,
            line,
            column
        };
        column = end_word - start_column_cursor;
        res.push(token);
        cursor = end_word;
    }


    res.push(Token { 
        token: TokenType::EOF,
        line: line+1,
        column: 0, 
    });
    return Ok(res);
}