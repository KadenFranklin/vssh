use std::{io, env};
use std::process;
use std::path::Path;
use std::ffi::CString;
use nix::unistd::{fork, ForkResult, execvp};
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
            let root = Path::new("/");
            assert!(env::set_current_dir(&root).is_ok());
        }
        if input == " " {
            let path = env::current_dir()?;
            println!("{}", path.display());
        } else {
            let c_input = externalize(input.as_str());
            match unsafe { fork() }.unwrap() {
                ForkResult::Parent { child } => {
                    waitpid(child, None);
                }
                ForkResult::Child => {
                    let exe = CString::new(c_input).unwrap();
                    execvp(exe.as_c_str(), &[exe.as_c_str()]).unwrap();
                }
            }
        }
        Ok(());
    }
}

fn externalize(command: &str) -> Box<[CString]> {
    let converted = command.split_whitespace()
        .map(|s| CString::new(s).unwrap())
        .collect::<Vec<_>>();
    converted.into_boxed_slice()
}
