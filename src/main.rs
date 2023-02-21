mod todo;
use std::env;
use std::fs::OpenOptions;
use std::io::{self, SeekFrom, Seek};
use todo::TodoList;

fn main() -> io::Result<()> {
    
    let args: Vec<String> = env::args().collect();
    // let mut todo_list = TodoList::new();
    let path = home::home_dir().map(|mut path| {
        path.push(".rusty-todo.json");
        path
    });
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path.as_ref().expect("Error"))?; 
    let mut todo_list:TodoList = match serde_json::from_reader(&file) {
        Ok(todo_list) => todo_list,
        Err(e) if e.is_eof() => TodoList::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?;

    match args.len() {
        // No arguments
        1 => {
            println!("Hello No args");
        }
        // One argument
        2 => match args[1].as_str() {
            "add" => {
                println!("Enter your new task:");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Please enter a valid text.");
                if input.trim().chars().count() == 0 {
                    panic!("You must provide an accepted text");
                } else {
                    todo_list.add_to_list(input);
                }
            }
            "remove" => {
                println!("Enter the number of task to remove:");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Please enter a valid text.");
                let number:usize = input.trim().parse().expect("Input not a number");
                if number >= todo_list.list.len() {
                    panic!("{} is not a valid task", number);
                } else {
                    todo_list.remove_task(number);
                }
                file.set_len(0)?;
            }
            "done" => {
                println!("Enter the number of task to mark as done:");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Please enter a valid text.");
                let number:usize = input.trim().parse().expect("Input not a number");
                if number > todo_list.list.len() {
                    panic!("{} is not a valid task", number);
                } else {
                    todo_list.mark_done(number);
                }
            }
            "undone" =>  {
                println!("Enter the number of task to mark as undone:");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Please enter a valid text.");
                let number:usize = input.trim().parse().expect("Input not a number");
                if number > todo_list.list.len() {
                    panic!("{} is not a valid task", number);
                } else {
                    todo_list.mark_undone(number);
                }
            }
            "pending" => {
                println!("Enter the number of task to mark as pending:");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Please enter a valid text.");
                let number:usize = input.trim().parse().expect("Input not a number");
                if number > todo_list.list.len() {
                    panic!("{} is not a valid task", number);
                } else {
                    todo_list.mark_pending(number);
                }
            }
            "list" => {
                todo_list.print();
            }
            "list-done" => {
                todo_list.print_list_done();
            }
            "list-undone" => {
                todo_list.print_list_undone();
            }
            "list-pending" => {
                todo_list.print_list_pending();
            }
            "list-all" => {
                todo_list.print_all();
            }
            "order-list" => {
                todo_list.order_list();
                println!("New ordered list:");
                todo_list.print();
            }
            _ => {
                panic!("You must provide an accepted command")
            }
        },
        // One command and one argument
        3 => match args[1].as_str() {
            "add" => {
                todo_list.add_to_list(args[2].to_string());
            }
            "remove" => {
                let number = args[2].parse().expect("Error converting integer");
                if number >= todo_list.list.len() {
                    panic!("{} is not a valid task", number);
                } else {
                    todo_list.remove_task(number);
                }
                file.set_len(0)?;
            }
            "done" => {
                todo_list.mark_done(args[2].parse().expect("Error converting integer"));
            }
            "undone" => {
                todo_list.mark_undone(args[2].parse().expect("Error converting integer"));
            }
            "pending" => {
                todo_list.mark_pending(args[2].parse().expect("Error converting integer"));
            }
            _ => {
                panic!("You must provide an accepted command")
            }
        },
        // All other cases
        _ => {
            println!("Invalid number of arguments");
        }
    }
    serde_json::to_writer(file, &todo_list)?;
    Ok(())
}
