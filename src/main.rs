use std::{io::{self, Write}, str::SplitWhitespace};
use std::env;

fn main() {

    loop {

        print!("username@hostname> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut cmd_parts = input.trim().split_whitespace(); 
        let command = cmd_parts.next().unwrap_or("");

        match command {        
            "" => continue,
            "exit" | "exit()" => break,
            "ls" => handle_ls(cmd_parts),
            "pwd" => handle_pwd(),
            "cd" => handle_cd(cmd_parts),
            _ => {
                println!("Welp!!")
            }
        }
    }
}


fn handle_ls(mut args: SplitWhitespace<'_>) {
    
    let first_args = args.next().unwrap_or("");

    match first_args {
        "" => println!("This is ls"),
        "-l" => println!("This is ls -l"),
        "-a" => println!("This is ls -a"),
        "-la" => println!("This is ls -la"),
        _ => println!("You typed {}", first_args)
    }
}


fn handle_pwd() {
    
    let path = env::current_dir().unwrap();

    println!("{}", path.display());
}


fn handle_cd(mut _args: SplitWhitespace<'_>) {
// TODO: Implement cd to switch back to home folder
}