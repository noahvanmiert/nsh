/*
    Made by Noah Van Miert
    26/12/2022
    nsh
*/

use std::env;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::exit;
use std::str::SplitWhitespace;

/*
    Returns the current path
*/
fn get_current_abs_path() -> PathBuf {
    env::current_dir().unwrap()
}

fn get_current_path() -> String {
    let cwd: PathBuf = get_current_abs_path();
    let path: String = String::from(cwd.to_string_lossy());

    let home_dir: PathBuf = env::home_dir().unwrap();
    let home: String = String::from(home_dir.to_string_lossy());

    if path == home {
        return "~".to_string();
    } else if path.starts_with(home.as_str()) {
        let cutted_string: String = path[home.len()..].to_string();

        return "~".to_string() + &cutted_string;
    } else {
        return path;
    }
}

struct Alias {
    name: String,
    command: String,
}

fn exec_buildin_command(command: &str) {
    match command {
        "@help" => {
            println!("Noah Shell (nsh) - version 0.1.0 (linux)");
            println!("For info about a command use 'man [command]' on linux\n");

            println!("nsh commands start with an '@' for example '@help'");
            println!("nsh commands:");
            println!("\thelp - Prints this help message.");
            println!("\texit - Will terminate nsh process.");
        }

        "@exit" => {
            exit(0);
        }

        &_ => {
            eprintln!("\x1b[1;31mUnkown buildin command '{}'\x1b[0m", command);
        }
    }
}

fn main() {
    let greeting: &str = "Welcome to Noah Shell (nsh)";
    let mut aliases: Vec<Alias> = Vec::new();

    // ==============================
    // Here you can add aliases

    aliases.push(Alias {
        name: "ll".to_string(),
        command: "exa -l --icons".to_string(),
    });

    // ==============================

    /*
        Print the greeting message
    */
    println!("{}", greeting);

    loop {
        let mut inp: String = String::new();

        /*
            This will print the current directory
            in a format like this
            [<directory>] ->
            int the color green.
        */
        print!("[\x1b[1;32m{}\x1b[0m] -> \x1b[34m", get_current_path());
        print!("\x1b[0m");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut inp)
            .expect("failed to read line");

        let mut parts: SplitWhitespace = inp.trim().split_whitespace();
        let o_command_: Option<&str> = parts.next();

        /*
            If no command is given just an empty string.
        */
        if o_command_.is_none() {
            continue;
        }

        let mut command: &str = o_command_.unwrap();
        let mut args: SplitWhitespace = parts;

        /*
            Check if the current command is an
            alias.
        */
        for alias in &aliases {
            if alias.name == command {
                let mut alias_parts: SplitWhitespace = alias.command.trim().split_whitespace();

                command = alias_parts.next().unwrap();
                args = alias_parts;
            }
        }


        /*
            Check if the command is a nsh command.
        */
        if command.chars().nth(0).unwrap() == '@' {
            exec_buildin_command(command);
            continue;
        }


        match command {
            "cd" => {
                let new: &str = args.peekable().peek().map_or("/", |x| *x);
                let root: &Path = Path::new(new);

                if let Err(e) = env::set_current_dir(&root) {
                    /*
                        Print the error message in bold
                        and red letters.
                    */
                    eprintln!("\x1b[1;31m{}\x1b[0m", e);
                }
            }

            command => {
                let child = Command::new(command).args(args).spawn();

                match child {
                    /* If the command is Ok */
                    Ok(mut child) => {
                        /*
                            Print an error message if the a error
                            occured while executing the current
                            command.
                        */
                        child
                            .wait()
                            .expect("\x1b[1;31mFailed while executing a command\x1b[0m");
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
