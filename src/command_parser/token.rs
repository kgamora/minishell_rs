#[derive(Debug, Clone)]
pub enum Token {
    // TODO: add other types of streams
    InputStream,
    OutputStream,
    Pipe,
    String(String),
}

pub fn parse_tok(s: &String) -> Token {
    match s.as_str() {
        "<" => Token::InputStream,
        ">" => Token::OutputStream,
        "|" => Token::Pipe,
        _ => Token::String(s.clone()),
    }
}

pub fn get_string(tok: &Token) -> Option<String> {
    match tok {
        Token::String(s) => Some(s.clone()),
        _ => None,
    }
}

pub fn is_string(tok: &Token) -> bool {
    matches!(tok, Token::String(_))
}

pub fn is_stream(tok: &Token) -> bool {
    matches!(tok, Token::InputStream | Token::OutputStream)
}

pub fn is_pipe(tok: &Token) -> bool {
    matches!(tok, Token::Pipe)
}
