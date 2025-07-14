use std::io;

struct Task {
    title: String,
    description: String,
    priority: String,
    completed: bool,
}

fn get_priority_from_input(input: &str) -> String {
    match input.trim().to_lowercase().chars().next() {
        Some('h') => String::from("High"),
        Some('m') => String::from("Medium"),
        Some('l') => String::from("Low"),
        _ => String::from("Medium"),  // Default to Medium if input is invalid
    }
}

fn get_priority_emoji(priority: &str) -> &str {
    match priority {
        "High" => "🔴",
        "Medium" => "🟡",
        "Low" => "🟢",
        _ => "⚪",
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    println!("\n📝  Enter task details:");
    
    println!("📌  Title:");
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Failed to read title");
    
    println!("📋  Description:");
    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Failed to read description");
    
    println!("⭐  Priority (H)igh/(M)edium/(L)ow:");
    let mut priority_input = String::new();
    io::stdin().read_line(&mut priority_input).expect("Failed to read priority");
    let priority = get_priority_from_input(&priority_input);
    println!("    Priority set to: {} {}", priority, get_priority_emoji(&priority));
    
    let task = Task {
        title: title.trim().to_string(),
        description: description.trim().to_string(),
        completed: false,
        priority,
    };
    
    tasks.push(task);
    println!("\n✅  Task added successfully!");
}

fn complete_task(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("\n📭  No tasks available!");
        return;
    }
    
    println!("\n📋  Available tasks:");
    for (i, task) in tasks.iter().enumerate() {
        let status = if task.completed { "✅" } else { "⏳" };
        println!("    {}. {} {} {}", i + 1, status, get_priority_emoji(&task.priority), task.title);
    }
    
    println!("\n🔍  Enter task number to mark as complete:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    if let Ok(index) = input.trim().parse::<usize>() {
        if index > 0 && index <= tasks.len() {
            tasks[index - 1].completed = true;
            println!("\n✅  Task marked as complete!");
        } else {
            println!("\n❌  Invalid task number!");
        }
    } else {
        println!("\n❌  Invalid input!");
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("\n📭  No tasks available!");
        return;
    }
    
    println!("\n📋  Available tasks:");
    for (i, task) in tasks.iter().enumerate() {
        let status = if task.completed { "✅" } else { "⏳" };
        println!("    {}. {} {} {}", i + 1, status, get_priority_emoji(&task.priority), task.title);
    }
    
    println!("\n🗑️  Enter task number to delete:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    if let Ok(index) = input.trim().parse::<usize>() {
        if index > 0 && index <= tasks.len() {
            tasks.remove(index - 1);
            println!("\n🗑️  Task deleted successfully!");
        } else {
            println!("\n❌  Invalid task number!");
        }
    } else {
        println!("\n❌  Invalid input!");
    }
}

fn display_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("\n📭  No tasks available!");
        return;
    }
    
    println!("\n📋  All Tasks:");
    for (i, task) in tasks.iter().enumerate() {
        let status = if task.completed { "✅" } else { "⏳" };
        println!("\n    📌  Task {}:", i + 1);
        println!("    📝  Title: {}", task.title);
        println!("    📋  Description: {}", task.description);
        println!("    🔍  Status: {}", status);
        println!("    ⭐  Priority: {} {}", 
            task.priority,
            get_priority_emoji(&task.priority)
        );
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("\n🎯  Task Management System  🎯");
        println!("    ========================");
        println!("    1. 📝  Add Task");
        println!("    2. ✅  Complete Task");
        println!("    3. 🗑️   Delete Task");
        println!("    4. 📋  Display Tasks");
        println!("    5. 👋  Exit");
        println!("\n🔍  Enter your choice:");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse() {
            Ok(choice) => match choice {
                1 => add_task(&mut tasks),
                2 => complete_task(&mut tasks),
                3 => delete_task(&mut tasks),
                4 => display_tasks(&tasks),
                5 => break,
                _ => println!("\n❌  Invalid choice. Please try again."),
            },
            Err(_) => println!("\n❌  Invalid input. Please enter a number."),
        }
    }
    println!("\n👋  Goodbye!");
}
