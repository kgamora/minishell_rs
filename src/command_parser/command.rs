use super::token::{Token, get_string, is_stream, is_string};

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct FileOpener {
    token: Token,
    file_path: String,
}

#[derive(Debug)]
pub enum FileOpenerError {
    InvalidToken,
    EmptyFilePath,
}

impl FileOpener {
    pub fn new(
        token: Token,
        file_path: String,
    ) -> Result<Self, FileOpenerError> {
        // Validate token
        if !is_stream(&token) {
            return Err(FileOpenerError::InvalidToken);
        }

        // Validate file path
        if file_path.trim().is_empty() {
            return Err(FileOpenerError::EmptyFilePath);
        }

        Ok(FileOpener { token, file_path })
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct CommandBuilder {
    argv: Vec<String>,
    stdin: Option<FileOpener>,
    stdout: Option<FileOpener>,
    stderr: Option<FileOpener>,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum SyntaxError {
    EmptyCommand,
    UnexpectedToken,
}

pub fn parse_command(tokens: &[Token]) -> Result<CommandBuilder, SyntaxError> {
    let mut argv: Vec<String> = Vec::with_capacity(tokens.len());
    let mut stdin: Option<FileOpener> = None;
    let mut stdout: Option<FileOpener> = None;
    let stderr: Option<FileOpener> = None;

    let mut it = tokens.iter().peekable();
    while let Some(token) = it.next() {
        if let Some(val) = get_string(token) {
            argv.push(val);
            continue;
        }

        // when not a string after < or >
        let next_tok = it.peek();
        if next_tok.is_none_or(|tok| !is_string(tok)) {
            return Err(SyntaxError::UnexpectedToken);
        }

        let file_path = get_string(next_tok.unwrap()).unwrap();
        match token {
            Token::InputStream => {
                stdin =
                    Some(FileOpener::new(token.clone(), file_path).unwrap());
            },
            Token::OutputStream => {
                stdout =
                    Some(FileOpener::new(token.clone(), file_path).unwrap());
            },
            _ => {
                return Err(SyntaxError::UnexpectedToken);
            },
        }

        it.next();
    }

    if argv.is_empty() {
        return Err(SyntaxError::EmptyCommand);
    }

    Ok(CommandBuilder {
        argv,
        stdin,
        stdout,
        stderr,
    })
}
