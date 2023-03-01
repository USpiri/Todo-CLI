use serde::{Deserialize, Serialize};
use std::{fmt, io};

#[derive(PartialEq, Eq, Serialize, Deserialize, Copy, Clone)]
pub enum TodoItemStatus {
    Done,
    Undone,
    Pending,
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    pub list: Vec<TodoItem>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { list: Vec::new() }
    }

    pub fn add_to_list(&mut self, name: String) {
        let todo_item = TodoItem::new(name);
        self.list.push(todo_item);
    }

    pub fn print(&self) {
        for (index, item) in self.list.iter().enumerate() {
            println!("{}. [ {:?} ] - {}", index, item.status, item.name)
        }
    }

    pub fn mark_done(&mut self, index: usize) {
        if self.list[index].status == TodoItemStatus::Undone
            || self.list[index].status == TodoItemStatus::Pending
        {
            self.list[index].status = TodoItemStatus::Done
        } else {
            self.list[index].status = TodoItemStatus::Undone
        }
    }

    pub fn mark_undone(&mut self, index: usize) {
        if self.list[index].status == TodoItemStatus::Undone {
            println!("Task currently undone")
        } else {
            self.list[index].status = TodoItemStatus::Undone
        }
    }

    pub fn mark_pending(&mut self, index: usize) {
        if self.list[index].status == TodoItemStatus::Pending {
            println!("Task currently pending")
        } else {
            self.list[index].status = TodoItemStatus::Pending
        }
    }

    pub fn remove_task(&mut self, index: usize) {
        self.list.remove(index);
    }

    pub fn print_list_done(&self) {
        println!("DONE TASKS:");
        for (index, item) in self.list.iter().enumerate() {
            if item.status == TodoItemStatus::Done {
                println!("{}. [ {:?} ] - {}", index, item.status, item.name)
            }
        }
    }

    pub fn print_list_undone(&self) {
        println!("UNDONE TASKS:");
        for (index, item) in self.list.iter().enumerate() {
            if item.status == TodoItemStatus::Undone {
                println!("{}. [ {:?} ] - {}", index, item.status, item.name)
            }
        }
    }

    pub fn print_list_pending(&self) {
        println!("PENDING TASKS:");
        for (index, item) in self.list.iter().enumerate() {
            if item.status == TodoItemStatus::Pending {
                println!("{}. [ {:?} ] - {}", index, item.status, item.name)
            }
        }
    }

    pub fn print_all(&self) {
        self.print_list_done();
        self.print_list_undone();
        self.print_list_pending();
    }

    pub fn order_list(&mut self) {
        let mut ordered_list: TodoList = TodoList::new();
        for item in self.get_filtered_items_by_status(TodoItemStatus::Undone) {
            ordered_list.list.push(TodoItem {
                name: item.name.to_string(),
                status: TodoItemStatus::Undone,
            })
        }
        for item in self.get_filtered_items_by_status(TodoItemStatus::Pending) {
            ordered_list.list.push(TodoItem {
                name: item.name.to_string(),
                status: TodoItemStatus::Pending,
            })
        }
        for item in self.get_filtered_items_by_status(TodoItemStatus::Done) {
            ordered_list.list.push(TodoItem {
                name: item.name.to_string(),
                status: TodoItemStatus::Done,
            })
        }
        self.list = ordered_list.list;
    }

    fn get_filtered_items_by_status(&self, status: TodoItemStatus) -> Vec<&TodoItem> {
        let ordered = self
            .list
            .iter()
            .filter(|&item| item.status == status)
            .collect();
        ordered
    }

    pub fn ask_add_to_list(&mut self) -> Result<String, String> {
        println!("Enter your new task:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Please enter a valid Text");
        if input.trim().chars().count() == 0 {
            return Err("You must provide some text".to_string());
        }
        self.add_to_list(input);
        Ok("Task ".to_string() + &self.list.len().to_string() + " added successfully")
    }

    pub fn ask_remove_task(&mut self) -> Result<String, String> {
        println!("Enter the number of task to remove:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Please enter a valid number.");
        let number: usize = match input.trim().parse() {
            Ok(number) => number,
            Err(_) => return Err("Error: Please enter a valid number.".to_string()),
        };
        if number >= self.list.len() {
            return Err(number.to_string()
                + " is not a valid task, please list tasks to see what numbers are available");
        } else {
            self.remove_task(number);
        }
        Ok("Task removed".to_string())
    }

    pub fn ask_mark_done(&mut self) -> Result<String, String> {
        println!("Enter the number of task to mark as done:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Please enter a valid number.");
        let number: usize = match input.trim().parse() {
            Ok(number) => number,
            Err(_) => return Err("Error: Please enter a valid number.".to_string()),
        };
        if number >= self.list.len() {
            return Err(number.to_string()
                + " is not a valid task, please list tasks to see what numbers are available");
        } else {
            self.mark_done(number);
        }
        Ok("Task marked as done".to_string())
    }

    pub fn ask_mark_undone(&mut self) -> Result<String, String> {
        println!("Enter the number of task to mark as undone:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Please enter a valid number.");
        let number: usize = match input.trim().parse() {
            Ok(number) => number,
            Err(_) => return Err("Error: Please enter a valid number.".to_string()),
        };
        if number >= self.list.len() {
            return Err(number.to_string()
                + " is not a valid task, please list tasks to see what numbers are available");
        } else {
            self.mark_undone(number);
        }
        Ok("Task marked as undone".to_string())
    }

    pub fn ask_mark_pending(&mut self) -> Result<String, String> {
        println!("Enter the number of task to mark as pending:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Please enter a valid number.");
        let number: usize = match input.trim().parse() {
            Ok(number) => number,
            Err(_) => return Err("Error: Please enter a valid number.".to_string()),
        };
        if number >= self.list.len() {
            return Err(number.to_string()
                + " is not a valid task, please list tasks to see what numbers are available");
        } else {
            self.mark_pending(number);
        }
        Ok("Task marked as pending".to_string())
    }

    pub fn mark_all_as(&mut self, status: TodoItemStatus) {
        for task in self.list.iter_mut() {
            task.status = status;
        }
    }

    pub fn print_task(&self, index: usize) {
        println!("{}. [ {:?} ] - {}", index, self.list[index].status, self.list[index].name)
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoItem {
    name: String,
    status: TodoItemStatus,
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        TodoItem {
            name: name,
            status: TodoItemStatus::Undone,
        }
    }
}

impl fmt::Debug for TodoItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Done => write!(f, "{}", "✔️ "),
            Self::Undone => write!(f, "{}", "✖️ "),
            _ => write!(f, "{}", "➖"),
        }
    }
}
