use std::{io, env};
use std::process;
use std::ffi::CString;
use nix::unistd::{fork, ForkResult, execvp, chdir};
use nix::sys::wait::waitpid;

fn main() -> io::Result<()> {
    loop {
        let path = env::current_dir()?;
        println!("{}", path.display());
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("not valid input.");
        if input == "exit" {
            process::exit(0x0100);
        }
        if input == "cd" {
            let args : String = env::args().collect();
            let path = args.as_str();
            chdir(path).expect("incorrect input");
        }
        if input == " " {
            let path = env::current_dir()?;
            println!("{}", path.display());
        } else {
            let input = input.trim();
            let c_input = externalize(input);
            match unsafe { fork() }.unwrap() {
                ForkResult::Parent { child } => {
                    waitpid(child, None).expect("incorrect input");
                }
                ForkResult::Child => {
                    execvp(&c_input[0], &c_input).unwrap();
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
