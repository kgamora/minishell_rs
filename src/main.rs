extern crate nix;

use nix::{
    fcntl::{OFlag, open},
    sys::{stat::Mode, wait::waitpid},
    unistd::{
        ForkResult, dup2_stdin, dup2_stdout, execvp, fork, pipe, read, write,
    },
};

use std::{
    ffi::{CStr, CString},
    os::fd::{AsFd, OwnedFd},
};

use shell_words::split;

mod command_parser;

use crate::command_parser::{
    CommandBuilder, command::FileOpener, parse_commands, token::Token,
};

fn prepare_args(argv: &[String]) -> (CString, Vec<CString>) {
    let c_str_argv: Vec<CString> = argv
        .iter()
        .map(|s| CString::new(s.as_bytes()).unwrap())
        .collect();

    let filename = c_str_argv.first().unwrap().clone();

    (filename, c_str_argv)
}

fn open_file(file_opener: &FileOpener) -> OwnedFd {
    let FileOpener { token, file_path } = file_opener;

    let (oflag, mode) = match token {
        Token::InputStream => (OFlag::O_RDONLY, Mode::empty()),
        Token::OutputStream => (
            OFlag::O_CREAT | OFlag::O_TRUNC | OFlag::O_WRONLY,
            Mode::S_IRWXU,
        ),
        _ => panic!("Unexpected token where stream was expected"),
    };

    open(file_path.as_str(), oflag, mode).unwrap()
}

fn prepare_and_run(commands: &[CommandBuilder]) {
    let mut it = commands.iter().peekable();

    let mut prev_read_fd: Option<OwnedFd> = None;
    while let Some(command) = it.next() {
        let mut stdin_fd: Option<OwnedFd> = None;
        let mut stdout_fd: Option<OwnedFd> = None;
        // TODO: return the field
        let _stderr_fd: Option<OwnedFd> = None;

        // If previous command left the pipe, connect it to stdin
        if let Some(fd) = prev_read_fd {
            stdin_fd = Some(fd);
            prev_read_fd = None;
        }

        // If there's a next command, create a pipe and connect stdout
        if it.peek().is_some() {
            let (read_fd, write_fd) = pipe().unwrap();
            prev_read_fd = Some(read_fd);
            stdout_fd = Some(write_fd);
        }

        let CommandBuilder {
            argv,
            stdin: stdin_builder,
            stdout: stdout_builder,
            _stderr_builder: _,
        } = command;

        // If input stream (`<`) is present
        if let Some(file_opener) = stdin_builder {
            drop(stdin_fd.take());
            stdin_fd = Some(open_file(file_opener));
        }

        // If output stream (`>`) is present
        if let Some(file_opener) = stdout_builder {
            drop(stdout_fd.take());
            stdout_fd = Some(open_file(file_opener));
        }

        match unsafe { fork() } {
            Ok(ForkResult::Parent { child }) => {
                drop(stdin_fd);
                drop(stdout_fd);

                waitpid(child, None).unwrap();
            },
            Ok(ForkResult::Child) => {
                // Prepare args
                let (filename, c_str_argv) = prepare_args(argv);
                let args_refs: Vec<&CStr> =
                    c_str_argv.iter().map(CString::as_c_str).collect();
                // Prepare file descriptors
                if let Some(fd) = stdin_fd {
                    dup2_stdin(&fd).unwrap();
                    drop(fd);
                }
                if let Some(fd) = stdout_fd {
                    dup2_stdout(&fd).unwrap();
                    drop(fd);
                }
                drop(prev_read_fd.take());

                // exec
                let _ = execvp(filename.as_c_str(), &args_refs);
            },
            Err(_) => panic!("Fork failed"),
        }
    }
}

fn main() {
    loop {
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

        // for x in &commands {
        //     println!("{:#?}", x);
        // }

        prepare_and_run(&commands);
    }
}
