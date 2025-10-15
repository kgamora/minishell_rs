extern crate nix;

use nix::unistd::{read, write};

use std::os::fd::AsFd;

use shell_words::split;

mod command_parser;

use crate::command_parser::{CommandBuilder, parse_commands};

fn prepare_and_run() {
    // first -- change all stdins to for commands more than the second
    // 
}

fn main() {
    // TODO: Better read
    {
        let stdout = std::io::stdout();
        write(stdout.as_fd(), b"$ ").unwrap();
    };
    let mut buf = [u8::MIN; 1024];
    let len = {
        let stdin = std::io::stdin();
        read(stdin.as_fd(), &mut buf).unwrap()
    };

    let prompt = str::from_utf8(&buf[0..len]).unwrap();

    // TODO: Move split and other to parse
    let tokens = split(prompt).unwrap();

    // command = cmd [args]... [< in] [> out]
    // command [| command]...

    let commands: Vec<CommandBuilder> = parse_commands(&tokens);

    for x in &commands {
        println!("{:#?}", x);
    }

    // TODO: prepare_and_run(commands);
}
