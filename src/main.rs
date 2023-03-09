mod todo;
use std::{env, thread, time};
use std::fs::OpenOptions;
use std::io::{self, Seek, SeekFrom};
use std::process::Command;
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
            "add" | "new" | "post" => {
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
            "remove" | "rm" | "delete" | "del" => {
                if arguments.len() == 0 {
                    match todo_list.remove(None) {
                        Ok(number) => println!("Task {number} removed successfully"),
                        Err(e) => println!("{e}"),
                    };
                } else {
                    match arguments[0].as_str() {
                        "all" | "." | "*" | "--all" => {
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
                        Ok(number) => println!("Task {number} marked as 'done' successfully"),
                        Err(e) => println!("{e}"),
                    };
                } else {
                    match arguments[0].as_str() {
                        "all" | "." | "*" | "--all" => {
                            todo_list.done_all();
                        }
                        "--last" => {
                            match todo_list.done(Some(todo_list.list.len() - 1)) {
                                Ok(number) => {
                                    println!("Task {number} marked as 'done' successfully")
                                }
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
                        Ok(number) => println!("Task {number} marked as 'undone' successfully"),
                        Err(e) => println!("{e}"),
                    };
                } else {
                    match arguments[0].as_str() {
                        "all" | "." | "*" | "--all" => {
                            todo_list.undone_all();
                        }
                        "--last" => {
                            match todo_list.undone(Some(todo_list.list.len() - 1)) {
                                Ok(number) => {
                                    println!("Task {number} marked as 'undone' successfully")
                                }
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
                                            println!("Task/s marked as 'undone' successfully:")
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
                        Ok(number) => println!("Task {number} marked as 'pending' successfully"),
                        Err(e) => println!("{e}"),
                    };
                } else {
                    match arguments[0].as_str() {
                        "all" | "." | "*" | "--all" => {
                            todo_list.pending_all();
                        }
                        "--last" => {
                            match todo_list.pending(Some(todo_list.list.len() - 1)) {
                                Ok(number) => {
                                    println!("Task {number} marked as 'pending' successfully")
                                }
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
                                            println!("Task/s marked as 'pending' successfully:")
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
            "list" | "ls" => {
                if arguments.len() == 0 {
                    todo_list.print();
                } else {
                    match arguments[0].as_str() {
                        "all" | "." | "*" | "--all" => {
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
                        "categorized" | "sorted" => {
                            todo_list.print_categorized();
                        }
                        "sort" | "order" => {
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
            "task" | "get" | "print" => {
                if arguments.len() == 0 {
                    match todo_list.get(None) {
                        Ok(task) => {
                            println!("Task found successfully:");
                            print!("{task}");
                        }
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
                            Ok(task) => {
                                if i == 0 {
                                    println!("Task/s found successfully:")
                                }
                                println!("{task}");
                            }
                            Err(e) => println!("{e}"),
                        };
                    }
                }
            }
            "edit" | "update" => {
                if arguments.len() == 0 {
                    match todo_list.edit(None, None) {
                        Ok(item) => {
                            println!("Task edited successfully:");
                            println!("{item}")
                        }
                        Err(e) => println!("{e}"),
                    };
                } else {
                    match arguments.len() {
                        1 => {
                            match todo_list.edit(Some(arguments[0].to_string()), None) {
                                Ok(item) => {
                                    println!("Task edited successfully:");
                                    println!("{item}")
                                }
                                Err(e) => println!("{e}"),
                            };
                        }
                        2 => {
                            match todo_list.edit(
                                Some(arguments[0].to_string()),
                                Some(arguments[1].to_string()),
                            ) {
                                Ok(item) => {
                                    println!("Task edited successfully:");
                                    println!("{item}")
                                }
                                Err(e) => println!("{e}"),
                            };
                        }
                        _ => {
                            println!("Invalid number of arguments, you must provide a single text between quotes");
                            println!("Examples:\n > todo edit SingleWord\n > todo edit 'Multiple words between quotes'")
                        }
                    }
                }
                file.set_len(0)?;
            }
            "help" | "-h" | "--help" => help(),
            "version" | "-v" | "--version" => version(),
            "url" | "web" | "documentation" | "doc" | "open_url" => {
                if !open_url("https://github.com/USpiri/todo") {
                    println!("Unable to open browser.\nClick or copy this link: https://github.com/USpiri/todo")
                } else {
                    println!("Open default browser")
                }
            }
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
    println!("\nList of available commands:\n");
    println!("     add <'task description'>:           adds a new task/s");
    println!("                                         Example: todo add Multiple 'tasks example'");
    println!("                                         Alternatives: add, new, push");
    println!("     remove <task number>:               remove task/s number n");
    println!(
        "     remove --all:                       delete all tasks. Alt: 'all', '.', '*', '--all'"
    );
    println!("     remove --last:                      delete last tasks");
    println!("                                         Alternatives: remove, rm, delete, del");
    println!("                                         Example: todo remove 2 3");
    println!("     done <task number>:                 mark task number n as done");
    println!("     undone <task number>:               mark task number n as undone");
    println!("     pending <task number>:              mark task number n as pending");
    println!(
        "      - done/pending/undone --all:       mark all tasks. Alt: 'all', '.', '*', '--all'"
    );
    println!("      - done/pending/undone --last:      mark last tasks");
    println!("                                         Example: todo pending 2 3");
    println!("                                                  todo done --last");
    println!("     get <task number>:                  print specific task/s");
    println!("                                         Example: todo get 3");
    println!("                                         Alternatives: get, task, print");
    println!("     list:                               list all tasks in numeric order");
    println!("     list all:                           list all tasks in numeric order");
    println!("                                         Alternatives: 'all', '.', '*', '--all'");
    println!("     list done:                          list all done tasks");
    println!("     list undone:                        list all undone tasks");
    println!("     list pending:                       list all pending tasks");
    println!(
        "     list categorized:                   list all tasks bu status. Alt: status, sorted"
    );
    println!("     list sort:                          Sort list by category. Alt: order");
    println!("                                         Alternatives: list, ls");
    println!("                                         Example: todo list done");
    println!("                                                  todo list --all");
    println!("     edit <task number> <'description'>: Edit specific task");
    println!("                                         Alternatives: edit, update");
    println!("                                         Example: todo edit 2");
    println!("                                                  todo edit 0 'Edited task'");
    println!("     help:                               Print help");
    println!("                                         Alternatives: help, -h, --help");
    println!("     version:                            Print version");
    println!("                                         Alternatives: version, -v, --version");
    println!("     url:                                Open 'todo' documentation in default webbrowser");
    println!("                                         Alternatives: url, web, documentation, doc");
    println!("\nUSAGE:");
    println!("     todo [command] <argument/s>\n");
    println!("The text inside '< >' marks is optional");
    println!("The task description must be enclosed in quotes if it has more than");
    println!("one word, it is not necessary for 'task number'");
    println!("\nFor more detailed information, visit: https://github.com/USpiri/todo\n");
}

fn version() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("todo cli version: {VERSION}");
}

fn open_url(url: &str) -> bool {
    if let Ok(mut child) = Command::new("cmd.exe")
            .arg("/C").arg("start").arg("").arg(&url).spawn() {
        thread::sleep(time::Duration::new(3, 0));
        if let Ok(status) = child.wait() {
            return status.success();
        }
    }
    false
}