use std::{io, env};
use std::process;
use std::ffi::{CString};
use nix::unistd::{fork, ForkResult, execvp};
use nix::sys::wait::waitpid;

fn main() -> io::Result<()> {
    loop {
        let path = env::current_dir()?;
        println!("{}", path.display());
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("not valid input.");
        if input.contains("cd") {
            let path :String = input.split("cd ").collect();
            assert!(env::set_current_dir(path.trim()).is_ok());
        }
        if input.contains("exit") {
            process::exit(0x0100);
        }
        if input.trim() == ""{
            //handle error message somehow
            }
        else {
            match unsafe { fork() }.unwrap() {
                ForkResult::Parent { child } => {
                    waitpid(child, None).expect("incorrect input");
                }
                ForkResult::Child => {
                    let input = input.trim();
                    let c_input = CString::new(input).unwrap();
                    let externalized =  externalize(input);
                    execvp(c_input.as_c_str(), &externalized).unwrap();
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
