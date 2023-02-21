mod todo;
use std::env;
use std::io;
use todo::TodoList;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let mut todo_list = TodoList::new();

    todo_list.add_to_list("Some string".to_string());
    todo_list.add_to_list("Some other string".to_string());
    todo_list.add_to_list("Some string".to_string());
    todo_list.add_to_list("Some other string".to_string());
    todo_list.mark_done(2);
    todo_list.mark_pending(1);

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
                if number > todo_list.list.len() {
                    panic!("{} is not a valid task", number);
                } else {
                    todo_list.remove_task(number);
                }
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
                todo_list.remove_task(args[2].parse().expect("Error converting integer"));
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
}
