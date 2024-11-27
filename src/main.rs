use std::env;
use std::io;
use std::io::Write;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;
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
        let mut programs = input.trim().split('|').peekable();
        let mut flag = None;
        while let Some(program) = programs.next() {
            let mut args: Vec<_> = program.trim().split_whitespace().collect();
            let command = args[0];
            args.remove(0);
            match command {
                "cd" => {
                    // pipe = Stdio::null();
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
                    flag = None;
                }
                "exit" => {
                    return Ok(());
                }
                command => {
                    let cfgin = flag.map_or(Stdio::inherit(), |x: Child| Stdio::from(x.stdout.unwrap()));
                    let child = Command::new(command)
                        .args(args)
                        .stdin(cfgin)
                        .stdout(match programs.peek() {
                            None=> {Stdio::inherit()}
                            Some(_)=> {Stdio::piped()}
                        })
                        .spawn();
                    match child {
                        Ok(mut t) => {
                            t.wait()?;
                            flag = Some(t);
                        }
                        Err(err) => {
                            println!("{err}");
                            flag = None;
                        }
                    }
                }
            }
        }
    }
}
