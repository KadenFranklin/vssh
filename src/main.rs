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

//vssh, the Very Simple SHell:
// Displays the current working directory while awaiting user input.
// If the user types exit, the program ends.
// If the user types cd [dir], change current working directory accordingly.
// If the user types a blank line, ignore it and display the prompt once again.
// Execute any other command the user types by spawning a new process:
// Be sure to include the nix crate in Cargo.toml.
// Use fork to create the child process.
// Within the child process, use execvp to execute the command.
// Within the parent process, use waitpid to wait for the child process to complete.

// https://hendrix-cs.github.io/csci320/projects/rust2.html

// there are lots of links to the rust documentation on the corresponding website.