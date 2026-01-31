use std::{io::{self, Write}, str::SplitWhitespace};
use std::{env, path};

fn main() {

    set_home_dir(true);
    
    // whoami::username() returns: Result<String, whoami::Error>
    // .unwrap_or_else(...) returns: String (The actual name or "guest")
    let username = whoami::username().unwrap_or_else(|_| "guest".to_string());

    // whoami::hostname() returns: Result<String, whoami::Error>
    // .unwrap_or_else(...) returns: String
    let hostname = whoami::hostname().unwrap_or_else(|_| "unknown".to_string());

    loop {

        print!("{}@{} ~ $ ", username, hostname);
        
        // Returns: Result<(), std::io::Error>
        // (Returns Ok(()) if successful, or an Error if it fails)
        io::stdout().flush().unwrap();

        let mut input = String::new();
        
        // Returns: Result<usize, std::io::Error>
        // (The 'usize' is the number of bytes read, e.g., 5 bytes)
        io::stdin().read_line(&mut input).unwrap();

        let mut cmd_parts = input.trim().split_whitespace(); 
        
        // Returns: Option<&str>
        // (Returns Some("ls") if text exists, or None if empty)
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
    
    // Returns: Option<&str>
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
    
    // Returns: Result<PathBuf, std::io::Error>
    // (We unwrap it to get the actual PathBuf)
    let path = env::current_dir().unwrap();

    // path.display() returns: std::path::Display
    // (A helper struct that makes paths safe to print)
    println!("{}", path.display());
}


fn handle_cd(mut args: SplitWhitespace<'_>) {

    // Returns: Option<&str>
    let first_args = args.next().unwrap_or("");

    match first_args {
        "" => set_home_dir(false),
        path => set_current_user_dir(path),
    }

    fn set_current_user_dir(path: &str) {

        // Returns: &Path (A slice/view of a path)
        let converted_path = path::Path::new(path);

        // Returns: Result<(), std::io::Error>
        // (We check if it is 'Err' to catch failures)
        if let Err(e) = env::set_current_dir(&converted_path) {
            
            eprint!("Error: {}", e);
        } else {

            println!("{}", converted_path.display());
        }     
    }
}

fn set_home_dir(silent: bool) {
    
    // Returns: Option<PathBuf>
    // (Returns Some(PathBuf) if found, None if the OS has no home)
    let home_path = env::home_dir().expect("No home dir found. Sus!");

    // Returns: Result<(), std::io::Error>
    if let Err(e) = env::set_current_dir(&home_path) {

        eprint!("Error: {}", e);
    } else {

        if !silent {
            println!("{}", home_path.display());
        }
    }
}