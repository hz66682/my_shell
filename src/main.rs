use std::env;
use std::io;
use std::process::Command;
use std::io::Write;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        print!("{}>", env::current_dir()?.display());
        io::stdout().flush()?;
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("something err");
            }
        };
        if input.trim() == "" {
            continue;
        }
        let mut args: Vec<_> = input.split_whitespace().collect();
        let command = args[0];
        args.remove(0);
        match command {
            "cd" => {
                if args.len() > 1 {
                    eprintln!("the number of arguments is too much");
                    continue;
                }
                let default_path = env::var("HOME")?;
                if args.len() == 0 {
                    args.push("~"); 
                }
                let path = args[0].replace("~", &default_path);
                if let Err(err) = env::set_current_dir(path) {
                    eprintln!("{err}");
                }
            }
            "exit" => {
                return Ok(());
            }
            command => {
                if let Ok(mut child) = Command::new(command).args(args).spawn() {
                    child.wait()?;
                } else {
                    eprintln!("something err");
                }
            }
        }
    }
}
