use std::io::{self, Write};

struct Goal {
    title: String,
    summary: String,
    tasks: Vec::<String>,
}

fn main() {
    let created_goal = create_goal();
    println!("Title: {}\nSummary: {}\n", created_goal.title, created_goal.summary);
    
    let mut i = 0;
    for item in created_goal.tasks {
        i +=1;
        println!("Task {}: {}", i, item);
    }
}

fn create_goal() -> Goal {
    let mut title_buffer = String::new();
    let mut summary_buffer = String::new();
    let mut task_buffer = String::new();

    println!("Title: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut title_buffer).unwrap();

    println!("Summary: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut summary_buffer).unwrap();

    let mut task_list = Vec::<String>::new();

    loop {
        println!("Task: ");
        io::stdout().flush().unwrap();                                  
        io::stdin().read_line(&mut task_buffer).unwrap();
        let trimmed_task = task_buffer.trim();

        if trimmed_task.is_empty() || trimmed_task.to_uppercase() == "q".to_uppercase() {
            break;
        }
        
        task_list.push(trimmed_task.to_string());
        task_buffer.clear();
    }

    let new_goal = Goal {
        title: title_buffer.trim().to_string(),
        summary: summary_buffer.trim().to_string(),
        tasks: task_list,
    };

    new_goal
}
