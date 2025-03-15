use std::io::{self, Write};

fn main() {
    let mut tasks: Vec<String> = Vec::new();

    loop {
        println!("\n📝 Simple To-Do List");
        println!("1️⃣ Add a Task");
        println!("2️⃣ View Tasks");
        println!("3️⃣ Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap(); // Flush output buffer

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => {
                print!("Enter a new task: ");
                io::stdout().flush().unwrap();

                let mut task = String::new();
                io::stdin().read_line(&mut task).expect("Failed to read input");
                tasks.push(task.trim().to_string());

                println!("✅ Task added!");
            }
            "2" => {
                println!("\n📌 Your To-Do List:");
                if tasks.is_empty() {
                    println!("No tasks added yet.");
                } else {
                    for (index, task) in tasks.iter().enumerate() {
                        println!("{}: {}", index + 1, task);
                    }
                }
            }
            "3" => {
                println!("👋 Exiting... Goodbye!");
                break;
            }
            _ => {
                println!("❌ Invalid choice! Please enter 1, 2, or 3.");
            }
        }
    }
}
