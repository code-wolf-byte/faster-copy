use std::vec::Vec;


pub struct Command {
    path: String,
    name: String,
    flags: Vec<String>,
}

impl Command {
    pub fn new(input: Vec<String>) -> Self {
        let mut command = Command {
            path: String::new(),
            name: String::new(),
            flags: Vec::new(),
        };
        
        for (index, data) in input.iter().enumerate() {
            match index {
                0 => command.path = data.clone(),
                1 => command.name = data.clone(),
                _ => command.flags.push(data.clone()),
            }
        }
        
        command
    }

    pub fn print(&self) {
        println!("Path: {}", self.path);
        println!("Name: {}", self.name);
        for el in &self.flags {
            print!("{} ", el);
        }
        println!(); // Add a newline at the end
    }
}
