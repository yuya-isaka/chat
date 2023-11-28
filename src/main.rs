use std::process::ExitCode;

fn usage(program: &str) {
    eprintln!("Usage: ./{program} <command>");
    eprintln!("Commands:");
    eprintln!("  hello");
    eprintln!("  reverse <text>");
}

fn main() -> std::process::ExitCode {
    let mut args = std::env::args();
    let program: String = args.next().expect("Program name not found");

    if let Some(command) = args.next() {
        if command == "hello" {
            println!("Hello, world!");
        } else if command == "reverse" {
            let mut flag: bool = false;
            for arg in args {
                flag = true;
                print!("{} ", arg.chars().rev().collect::<String>());
            }
            if flag {
                println!();
            } else {
                eprintln!("Error: no text given");
                usage(&program);
                return ExitCode::FAILURE;
            }
        } else {
            eprintln!("Error: unknown command '{command}'");
            usage(&program);
            return ExitCode::FAILURE;
        }
    } else {
        eprintln!("Error: no command given");
        usage(&program);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
