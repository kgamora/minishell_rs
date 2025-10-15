use super::command::{parse_command, CommandBuilder};

use super::token::{Token, parse_tok, is_pipe};

pub fn parse_commands(tokens: &[String]) -> Vec<CommandBuilder> {
    let mut result: Vec<CommandBuilder> = Default::default();

    let mapped: Vec<Token> = tokens.iter().map(parse_tok).collect();

    for command in mapped.split(is_pipe) {
        result.push(parse_command(command).unwrap());
    }

    result
}
