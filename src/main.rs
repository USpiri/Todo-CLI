mod todo;
use std::env;
use std::fs::OpenOptions;
use std::io::{self, Seek, SeekFrom};
use todo::{TodoList, TodoItemStatus};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = home::home_dir().map(|mut path| {
        path.push(".rusty-todo.json");
        path
    });
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path.as_ref().expect("Something wen wrong reading the file"))?;
    let mut todo_list: TodoList = match serde_json::from_reader(&file) {
        Ok(todo_list) => todo_list,
        Err(e) if e.is_eof() => TodoList::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?;

    match args.len() {
        // No arguments
        1 => {
            welcome();
            help();
        }
        // One argument
        2 => match args[1].as_str() {
            "add" => {
                match todo_list.ask_add_to_list() {
                    Ok(string) => println!("{string}"),
                    Err(e) => println!("{e}"),
                };
            }
            "remove" => {
                match todo_list.ask_remove_task() {
                    Ok(string) => println!("{string}"),
                    Err(e) => println!("{e}"),
                };
                file.set_len(0)?;
            }
            "done" => {
                match todo_list.ask_mark_done() {
                    Ok(string) => println!("{string}"),
                    Err(e) => println!("{e}"),
                };
                file.set_len(0)?;
            }
            "undone" => {
                match todo_list.ask_mark_undone() {
                    Ok(string) => println!("{string}"),
                    Err(e) => println!("{e}"),
                };
                file.set_len(0)?;
            }
            "pending" => {
                match todo_list.ask_mark_pending() {
                    Ok(string) => println!("{string}"),
                    Err(e) => println!("{e}"),
                };
                file.set_len(0)?;
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
            "all-done" => {
                todo_list.mark_all_as(TodoItemStatus::Done);
                file.set_len(0)?;
            }
            "all-undone" => {
                todo_list.mark_all_as(TodoItemStatus::Undone);
                file.set_len(0)?;
            }
            "all-pending" => {
                todo_list.mark_all_as(TodoItemStatus::Pending);
                file.set_len(0)?;
            }
            "remove-all" => {
                todo_list = TodoList::new();
                file.set_len(0)?;
            }
            "help" => help(),
            _ => {
                help();
                panic!("You must provide an accepted command");
            }
        },
        // One command and one argument
        3 => match args[1].as_str() {
            "add" => {
                todo_list.add_to_list(args[2].to_string());
            }
            "remove" => {
                let number:usize = args[2].parse().expect("Error converting integer");
                if number >= todo_list.list.len() {
                    panic!("{} is not a valid task", number);
                } else {
                    todo_list.remove_task(number);
                }
                file.set_len(0)?;
            }
            "done" => {
                let number:usize = args[2].parse().expect("Error converting integer");
                if number >= todo_list.list.len() {
                    panic!("{} is not a valid task", number);
                } else {
                    todo_list.mark_done(args[2].parse().expect("Error converting integer"));
                }
                file.set_len(0)?;
            }
            "undone" => {
                let number:usize = args[2].parse().expect("Error converting integer");
                if number >= todo_list.list.len() {
                    panic!("{} is not a valid task", number);
                } else {
                    todo_list.mark_undone(args[2].parse().expect("Error converting integer"));
                }
                file.set_len(0)?;
            }
            "pending" => {
                let number:usize = args[2].parse().expect("Error converting integer");
                if number >= todo_list.list.len() {
                    panic!("{} is not a valid task", number);
                } else {
                    todo_list.mark_pending(args[2].parse().expect("Error converting integer"));
                }
                file.set_len(0)?;
            }
            "task" => {
                let number:usize = args[2].parse().expect("Error converting integer");
                if number >= todo_list.list.len() {
                    panic!("{} is not a valid task", number);
                } else {
                    todo_list.print_task(number);
                }
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

fn welcome() {
    println!("\nWelcome to todo CLI app!");
    println!("'todo-cli' was developed in the course of learning rust by USpiri");
}

fn help() {
    println!("\nList of available commands:");
    println!("     add <'task description'>:     add a new task");
    println!("     remove <task number>:         remove task number n");
    println!("     remove-all:                   delete all tasks");
    println!("     done <task number>:           mark task number n as done");
    println!("     undone <task number>:         mark task number n as undone");
    println!("     pending <task number>:        mark task number n as pending");
    println!("     all-done:                     mark all tasks as done");
    println!("     all-undone:                   mark all tasks as undone");
    println!("     all-pending:                  mark all tasks as pending");
    println!("     task <task number>:           print specific task");
    println!("     list:                         list all tasks in numeric order");
    println!("     list-done:                    list all tasks marked as done");
    println!("     list-undone:                  list all tasks marked as undone");
    println!("     list-pending:                 list all tasks marked as pending");
    println!("     list-all:                     list all tasks by category");
    println!("\nUSAGE: \n     todo [command] <argument>");
    println!("\nThe text inside '< >' marks is optional");
    println!("Task description must have double quotation marks, it is not necessary for 'task number'\n");
    println!("For more information please visit: https://github.com/USpiri/Todo-CLI\n")
}
