use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

fn main() {
    println!("Use: 'exit', 'show' or 'add <item> to <collection>");

    let mut collections: HashMap<String, HashSet<String>> = HashMap::new();

    loop {
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let command = command.to_lowercase();
        let mut command_parts = command.trim().split_whitespace();
        if command_parts.clone().count() < 1 {
            println!("Enter a command")
        }

        match command_parts.next() {
            Some("add") => {
                let value = command_parts
                    .next()
                    .expect("Add command must have such format: 'add <item> to <collection>'");
                command_parts.next();
                let collection_name = command_parts
                    .next()
                    .expect("Add command must have such format: 'add <item> to <collection>'");
                let collection = collections
                    .entry(collection_name.to_string())
                    .or_insert(HashSet::new());
                collection.insert(value.to_string());
            }
            Some("show") => {
                println!("{collections:#?}");
                println!("What's next?");
            }
            Some("exit") => {
                break;
            }
            _ => println!("Unknown command. Use: 'exit', 'show' or 'add <item> to <collection>'"),
        }
    }
}
