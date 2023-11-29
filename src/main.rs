use std::env::Args;
use std::process::ExitCode;

struct Command {
    name: &'static str,
    description: &'static str,
    function: fn(Args) -> ExitCode,
}

const COMMANDS: &[Command] = &[
    Command {
        name: "hello",
        description: "Prints 'Hello, world!'",
        function: hello_command,
    },
    Command {
        name: "reverse",
        description: "Reverses the given text",
        function: reverse_command,
    },
    Command {
        name: "uppercase",
        description: "Converts the given text to uppercase",
        function: uppercase_command,
    },
    Command {
        name: "lowercase",
        description: "Converts the given text to lowercase",
        function: lowercase_command,
    },
    Command {
        name: "help",
        description: "Prints this help message",
        function: help_command,
    },
];

fn usage() {
    eprintln!("Usage: cargo run <command>");
    eprintln!("Commands:");
    for command in COMMANDS {
        eprintln!(
            "  {name:12} {description}",
            name = command.name,
            description = command.description
        );
    }
}

fn check_given_text(flag: bool) -> ExitCode {
    if !flag {
        eprintln!("\nError: no text given\n");
        usage();
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn hello_command(_args: Args) -> ExitCode {
    println!("Hello, world!");
    ExitCode::SUCCESS
}

fn reverse_command(args: Args) -> ExitCode {
    let mut flag: bool = false;
    for arg in args {
        flag = true;
        let result: String = arg.chars().rev().collect();
        println!("{result}");
    }
    check_given_text(flag)
}

fn uppercase_command(args: Args) -> ExitCode {
    let mut flag: bool = false;
    for arg in args {
        flag = true;
        println!("{} ", arg.to_uppercase());
    }
    check_given_text(flag)
}

fn lowercase_command(args: Args) -> ExitCode {
    let mut flag: bool = false;
    for arg in args {
        flag = true;
        println!("{} ", arg.to_lowercase());
    }
    check_given_text(flag)
}

fn help_command(_args: Args) -> ExitCode {
    usage();
    ExitCode::SUCCESS
}

fn main() -> ExitCode {
    let mut args: Args = std::env::args();
    let _program: String = args.next().expect("Program name not found");

    if let Some(command_name) = args.next() {
        if let Some(command) = COMMANDS.iter().find(|command| command.name == command_name) {
            (command.function)(args);
        } else {
            eprintln!("\nError: unknown command '{command_name}'\n");
            usage();
            return ExitCode::FAILURE;
        }
    } else {
        eprintln!("\nError: no command given\n");
        usage();
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
