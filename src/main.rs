use std::{io, env};
use std::process;
use std::ffi::CString;
use nix::unistd::{fork, ForkResult, execvp};
use nix::sys::wait::waitpid;
use std::env::{current_dir};

fn main() -> io::Result<()> {
    loop {
        let path = env::current_dir()?;
        println!("{}", path.display());
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("not valid input.");
        if input.contains("exit") {
            process::exit(0x0100);
        }
        if input.contains("cd") {
            let mut path :String = input.split("cd ").collect();
            path = current_dir().unwrap().to_str().unwrap().to_owned() + "/" + path.as_str();
            println!("{}", path);
            std::env::set_current_dir(path).unwrap_err();
        }
        else {
            let input = input.trim();
            let c_input = externalize(input);
            match unsafe { fork() }.unwrap() {
                ForkResult::Parent { child } => {
                    waitpid(child, None).expect("incorrect input");
                }
                ForkResult::Child => {
                    execvp(&c_input[0], &c_input).unwrap_err();
                }
            }
        }
    }
}

fn externalize(command: &str) -> Box<[CString]> {
    let converted = command.split_whitespace()
        .map(|s| CString::new(s).unwrap())
        .collect::<Vec<_>>();
    converted.into_boxed_slice()
}
