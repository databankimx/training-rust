use std::collections::HashMap;
use std::io::{self, BufRead};

// Each department name maps to the list of people in it.
type Directory = HashMap<String, Vec<String>>;

// Named fields make it clear which string is the department and which is the name, so they can't be mixed up.
enum Command {
    Add { dept: String, name: String },
    List(String),
    All,
    Quit,
}

impl Command {
    // A slice pattern matches the words of the line, so each command is recognized by its shape.
    fn from_input(s: &str) -> Option<Self> {
        let words: Vec<&str> = s.trim().split_whitespace().collect();
        match words.as_slice() {
            ["All"] => Some(Command::All),
            ["Quit"] => Some(Command::Quit),
            ["List", dept] => Some(Command::List(dept.to_string())),
            ["Add", name, "to", dept] => Some(Command::Add {
                dept: dept.to_string(),
                name: name.to_string(),
            }),
            _ => None,
        }
    }
}

fn main() {
    let mut dir: Directory = HashMap::new();
    println!("Type 'Add <name> to <department>' to add an employee");
    println!("Type 'List <department>' to list the employees of a department");
    println!("Type 'All' to list all employees by department");
    println!("Type 'Quit' to quit");

    for line in io::stdin().lock().lines() {
        let input = line.expect("error: unable to read user input");
        match Command::from_input(&input) {
            // or_default adds an empty vector for a new department, then the name is pushed onto it.
            Some(Command::Add { dept, name }) => dir.entry(dept).or_default().push(name),

            Some(Command::List(dept)) => match dir.get(&dept) {
                Some(names) => {
                    let mut names = names.clone();
                    names.sort();
                    for name in names {
                        println!("{dept}: {name}");
                    }
                }
                None => println!("I don't recognize that department!"),
            },

            Some(Command::All) => {
                // Sort the department names first, so the whole company is listed in a fixed order.
                let mut depts: Vec<&String> = dir.keys().collect();
                depts.sort();
                for dept in depts {
                    let mut names = dir[dept].clone();
                    names.sort();
                    for name in names {
                        println!("{dept}: {name}");
                    }
                }
            }

            Some(Command::Quit) => break,
            None => println!("Input error!"),
        }
    }
    println!("Have a nice day!");
}
