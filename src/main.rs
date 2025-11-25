use std::{
    collections::{HashMap, HashSet},
    io::{self, Write},
};

enum Command<'a> {
    Add { item: &'a str, collection: &'a str },
    Show,
    Exit,
    Empty,
    Unknown,
}

fn parse_command(line: &str) -> Command<'_> {
    let mut parts = line.trim().split_whitespace();
    let Some(cmd) = parts.next() else {
        return Command::Empty;
    };

    match cmd {
        "add" => {
            let Some(item) = parts.next() else {
                return Command::Unknown;
            };

            // expect the word "to"
            match parts.next() {
                Some("to") => {}
                _ => return Command::Unknown,
            }

            let Some(collection) = parts.next() else {
                return Command::Unknown;
            };

            Command::Add { item, collection }
        }
        "show" => Command::Show,
        "exit" => Command::Exit,
        _ => Command::Unknown,
    }
}

fn main() {
    println!("Use: 'exit', 'show' or 'add <item> to <collection>'");

    let mut collections: HashMap<String, HashSet<String>> = HashMap::new();
    let mut input = String::new();

    loop {
        input.clear();
        print!("> ");
        io::stdout().flush().ok();

        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read line");
            continue;
        }

        let line = input.to_lowercase(); // you can remove this if you want case-sensitive items
        match parse_command(&line) {
            Command::Empty => {
                println!("Enter a command");
            }
            Command::Add { item, collection } => {
                let set = collections.entry(collection.to_owned()).or_default();
                set.insert(item.to_owned());
            }
            Command::Show => {
                for (name, items) in &collections {
                    println!("{name}: {items:?}");
                }
            }
            Command::Exit => break,
            Command::Unknown => {
                println!("Unknown command. Use: 'exit', 'show' or 'add <item> to <collection>'");
            }
        }
    }
}
