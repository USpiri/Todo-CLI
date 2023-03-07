mod todo;
use std::env;
use std::fs::OpenOptions;
use std::io::{self, Seek, SeekFrom};
use todo::TodoList;

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

    if args.len() > 1 {
        let arguments = &args[2..];
        match args[1].as_str() {
            "add" => {
                if arguments.len() == 0 {
                    match todo_list.add(None) {
                        Ok(string) => println!("Task/s added successfully: \n{string}"),
                        Err(e) => println!("{e}"),
                    };
                } else {
                    for (i, task) in arguments.iter().enumerate() {
                        match todo_list.add(Some(task.to_string())) {
                            Ok(name) => {
                                if i == 0 {
                                    println!("Task/s added successfully:")
                                }
                                println!(" > {name}")
                            }
                            Err(e) => println!("{e}"),
                        };
                    }
                }
            }
            "remove" => {
                if arguments.len() == 0 {
                    match todo_list.remove(None) {
                        Ok(string) => println!("{string}"),
                        Err(e) => println!("{e}"),
                    };
                } else {
                    match arguments[0].as_str() {
                        "--all" => {
                            todo_list.remove_all();
                        }
                        "--last" => {
                            match todo_list.remove(Some(todo_list.list.len() - 1)) {
                                Ok(number) => println!("Task {number} have been removed"),
                                Err(e) => println!("{e}"),
                            };
                        }
                        _ => {
                            let mut order: Vec<usize> = Vec::new();
                            for element in arguments.iter() {
                                match element.parse() {
                                    Ok(number) => order.push(number),
                                    Err(_) => println!("{element} is not a number"),
                                }
                            }
                            order.sort();
                            order.reverse();
                            for (i, task) in order.iter().enumerate() {
                                match todo_list.remove(Some(task.to_owned())) {
                                    Ok(name) => {
                                        if i == 0 {
                                            println!("Task/s removed successfully:")
                                        }
                                        println!(" > {name}")
                                    }
                                    Err(e) => println!("{e}"),
                                };
                            }
                        }
                    }
                }
                file.set_len(0)?;
            }
            "done" => {
                if arguments.len() == 0 {
                    match todo_list.done(None) {
                        Ok(string) => println!("{string}"),
                        Err(e) => println!("{e}"),
                    };
                } else {
                    match arguments[0].as_str() {
                        "--all" => {
                            todo_list.done_all();
                        }
                        "--last" => {
                            match todo_list.done(Some(todo_list.list.len() - 1)) {
                                Ok(number) => println!("Task {number} have been removed"),
                                Err(e) => println!("{e}"),
                            };
                        }
                        _ => {
                            let mut order: Vec<usize> = Vec::new();
                            for element in arguments.iter() {
                                match element.parse() {
                                    Ok(number) => order.push(number),
                                    Err(_) => println!("{element} is not a number"),
                                }
                            }
                            for (i, task) in order.iter().enumerate() {
                                match todo_list.done(Some(task.to_owned())) {
                                    Ok(name) => {
                                        if i == 0 {
                                            println!("Task/s marked as 'done' successfully:")
                                        }
                                        println!(" > {name}")
                                    }
                                    Err(e) => println!("{e}"),
                                };
                            }
                        }
                    }
                }
                file.set_len(0)?;
            }
            "undone" => {
                if arguments.len() == 0 {
                    match todo_list.undone(None) {
                        Ok(string) => println!("{string}"),
                        Err(e) => println!("{e}"),
                    };
                } else {
                    match arguments[0].as_str() {
                        "--all" => {
                            todo_list.undone_all();
                        }
                        "--last" => {
                            match todo_list.undone(Some(todo_list.list.len() - 1)) {
                                Ok(number) => println!("Task {number} have been removed"),
                                Err(e) => println!("{e}"),
                            };
                        }
                        _ => {
                            let mut order: Vec<usize> = Vec::new();
                            for element in arguments.iter() {
                                match element.parse() {
                                    Ok(number) => order.push(number),
                                    Err(_) => println!("{element} is not a number"),
                                }
                            }
                            for (i, task) in order.iter().enumerate() {
                                match todo_list.undone(Some(task.to_owned())) {
                                    Ok(name) => {
                                        if i == 0 {
                                            println!("Task/s marked as 'done' successfully:")
                                        }
                                        println!(" > {name}")
                                    }
                                    Err(e) => println!("{e}"),
                                };
                            }
                        }
                    }
                }
                file.set_len(0)?;
            }
            "pending" => {
                if arguments.len() == 0 {
                    match todo_list.pending(None) {
                        Ok(string) => println!("{string}"),
                        Err(e) => println!("{e}"),
                    };
                } else {
                    match arguments[0].as_str() {
                        "--all" => {
                            todo_list.pending_all();
                        }
                        "--last" => {
                            match todo_list.pending(Some(todo_list.list.len() - 1)) {
                                Ok(number) => println!("Task {number} have been removed"),
                                Err(e) => println!("{e}"),
                            };
                        }
                        _ => {
                            let mut order: Vec<usize> = Vec::new();
                            for element in arguments.iter() {
                                match element.parse() {
                                    Ok(number) => order.push(number),
                                    Err(_) => println!("{element} is not a number"),
                                }
                            }
                            for (i, task) in order.iter().enumerate() {
                                match todo_list.pending(Some(task.to_owned())) {
                                    Ok(name) => {
                                        if i == 0 {
                                            println!("Task/s marked as 'done' successfully:")
                                        }
                                        println!(" > {name}")
                                    }
                                    Err(e) => println!("{e}"),
                                };
                            }
                        }
                    }
                }
                file.set_len(0)?;
            }
            "list" => {
                if arguments.len() == 0 {
                    todo_list.print();
                } else {
                    match arguments[0].as_str() {
                        "all" => {
                            todo_list.print();
                        }
                        "done" => {
                            todo_list.print_list_done();
                        }
                        "undone" => {
                            todo_list.print_list_undone();
                        }
                        "pending" => {
                            todo_list.print_list_pending();
                        }
                        "categorized" => {
                            todo_list.print_categorized();
                        }
                        "order" => {
                            todo_list.order_list();
                            println!("New ordered list:");
                            todo_list.print();
                        }
                        _ => {
                            println!("Unknown 'list' command.");
                            help();
                        }
                    }
                }
            }
            "task" => {
                if arguments.len() == 0 {
                    match todo_list.get(None) {
                        Ok(_) => print!(""),
                        Err(e) => println!("{e}"),
                    };
                } else {
                    let mut order: Vec<usize> = Vec::new();
                    for element in arguments.iter() {
                        match element.parse() {
                            Ok(number) => order.push(number),
                            Err(_) => println!("{element} is not a number"),
                        }
                    }
                    for (i, task) in order.iter().enumerate() {
                        match todo_list.get(Some(task.to_owned())) {
                            Ok(index) => {
                                if i == 0 {
                                    println!("Task/s found successfully:")
                                }
                                todo_list.print_task(index);
                            }
                            Err(e) => println!("{e}"),
                        };
                    }
                }
            }
            "edit" => {
                println!("TODO");
                file.set_len(0)?;
            }
            "help" => help(),
            _ => {
                println!("Unkown command. Please see the list of available commands");
                help();
            }
        }
    } else {
        welcome();
        help();
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
