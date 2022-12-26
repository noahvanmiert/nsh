/*
    Made by Noah Van Miert
    26/12/2022

    nsh
*/


use std::io;
use std::io::Write;
use std::process::Command;
use std::env;
use std::path::PathBuf;
use std::path::Path;

/*
    Returns the current directory.
*/
fn get_current_dir() -> PathBuf {
    env::current_dir().unwrap()
}


fn main() {
    loop {
        let mut inp: String = String::new();

        /*
            This will print the current directory
            in a format like this
            [<directory>] ->
            int the color green.
        */
        print!("[\x1b[1;32m{}\x1b[0m] -> \x1b[34m", get_current_dir().display());
        print!("\x1b[0m");
        io::stdout().flush().unwrap();
        
        io::stdin().read_line(&mut inp).expect("failed to read line");

        let mut parts = inp.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                let new = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new);
                
                if let Err(e) = env::set_current_dir(&root) {

                    /*
                        Print the error message in bold
                        and red letters.
                    */
                    eprintln!("\x1b[1;31m{}\x1b[0m", e);
                }
            },

            "help" => {
                println!("Noah Shell (nsh) - version 0.1.0 (linux)");
                println!("For info about a command use 'man [command]' on linux");
                println!("\nnsh commands:");
                println!("\thelp - Prints this help message.");
                println!("\texit - Will terminate nsh process.");
            }
            
            /*
                If you wan't to exit nsh you just
                say exit and nsh will be terminated.
            */
            "exit" => {
                return
            }

            command => {
                let child = Command::new(command)
                    .args(args)
                    .spawn();

                match child {

                    /* If the command is Ok */
                    Ok(mut child) => { 

                        /*
                            Print an error message if the a error
                            occured while executing the current
                            command.
                        */
                        child.wait().expect(
                            "\x1b[1;31mFailed while executing a command\x1b[0m"
                        );
                    }

                    /*
                        If the command is not recognized.
                    */
                    Err(_e) => {
                        eprintln!("\x1b[1;31mCommand '{}' not found\x1b[0m", command)
                    }
                };
            }
        }  
    }
}
