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

    pub fn print(&self) {
        for (index, item) in self.list.iter().enumerate() {
            println!("{}. [ {:?} ] - {}", index, item.status, item.name)
        }
    }

    fn mark_all(&mut self, status: TodoItemStatus) {
        for task in self.list.iter_mut() {
            task.status = status;
        }
    }

    pub fn get(&mut self, index: Option<usize>) -> Result<String, String> {
        match index.and_then(|i| self.list.get(i)
    .map(|task| format!("{}. {}", i, task))
    .ok_or_else(|| format!("{} is not a valid task", i).into()))
            None => {
                println!("Enter the number of task you want:");
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => (),
                    Err(_) => return Err("Error getting arguments".to_string()),
                }
                if input.trim().chars().count() == 0 {
                    return Err("You must provide some number".to_string());
                }
                let number: usize = match input.trim().parse() {
                    Ok(number) => number,
                    Err(_) => return Err("Error: Please enter a valid number.".to_string()),
                };
                if number >= self.list.len() {
                    return Err(number.to_string()
                + " is not a valid task, please list tasks to see what numbers are available");
                }
                match self.get(Some(number)) {
                    Ok(string) => return Ok(string),
                    Err(_) => return Err("Error getting task ".to_string() + &number.to_string()),
                }
            }
        }
    }

    pub fn add(&mut self, name: Option<String>) -> Result<String, String> {
        match name {
            Some(name) => {
                let todo_item = TodoItem::new(name.trim().to_string());
                self.list.push(todo_item);
                Ok(name)
            }

            None => {
                println!("Enter your new task:");
                let mut input = String::new();

                match io::stdin().read_line(&mut input) {
                    Ok(_) => (),
                    Err(_) => return Err("Error getting arguments".to_string()),
                }
                if input.trim().chars().count() == 0 {
                    return Err("You must provide some text".to_string());
                }
                match self.add(Some(input.trim().to_string())) {
                    Ok(name) => return Ok(name),
                    Err(_) => return Err("Error adding task".to_string()),
                }
            }
        }
    }

    pub fn remove(&mut self, index: Option<usize>) -> Result<usize, String> {
        match index {
            Some(index) => {
                if index >= self.list.len() {
                    return Err(index.to_string()
                        + " is not a valid task, please list tasks to see what numbers are available");
                }
                self.list.remove(index);
                Ok(index)
            }

            None => {
                println!("Enter the number of task to remove:");
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => (),
                    Err(_) => return Err("Error gettong arguments".to_string()),
                }
                if input.trim().chars().count() == 0 {
                    return Err("You must provide some number".to_string());
                }
                let number: usize = match input.trim().parse() {
                    Ok(number) => number,
                    Err(_) => return Err("Error: Please enter a valid number.".to_string()),
                };
                if number >= self.list.len() {
                    return Err(number.to_string()
                        + " is not a valid task, please list tasks to see what numbers are available");
                }
                match self.remove(Some(number)) {
                    Ok(index) => return Ok(index),
                    Err(_) => return Err("Error removing task ".to_string() + &number.to_string()),
                }
            }
        }
    }

    pub fn remove_all(&mut self) {
        self.list = Vec::new();
        println!("All tasks have been removed")
    }

    pub fn edit(
        &mut self,
        number: Option<String>,
        content: Option<String>,
    ) -> Result<String, String> {
        match number {
            Some(number) => {
                let index: usize = match number.trim().parse() {
                    Ok(index) => index,
                    Err(_) => return Err("Error: Please enter a valid number.".to_string()),
                };
                if index >= self.list.len() {
                    return Err(index.to_string()
                    + " is not a valid task, please list tasks to see what numbers are available");
                }
                match content {
                    Some(content) => {
                        self.list[index].name = content.trim().to_string();
                        return Ok(index.to_string() + ". " + &self.list[index].to_string());
                    }
                    None => {
                        println!("Task found successfully.Enter new task content:");
                        let mut input_content = String::new();
                        match io::stdin().read_line(&mut input_content) {
                            Ok(_) => (),
                            Err(_) => return Err("Error getting arguments".to_string()),
                        }
                        if input_content.trim().chars().count() == 0 {
                            return Err("You must provide something".to_string());
                        }
                        match self.edit(Some(index.to_string()), Some(input_content.trim().to_string())) {
                            Ok(item) => return Ok(item),
                            Err(_) => {
                                return Err("Error editing task ".to_string() + &index.to_string())
                            }
                        }
                    }
                }
            }
            None => {
                println!("Enter the number of task you want to edit:");
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => (),
                    Err(_) => return Err("Error getting arguments".to_string()),
                }
                if input.trim().chars().count() == 0 {
                    return Err("You must provide some number".to_string());
                }
                let number: usize = match input.trim().parse() {
                    Ok(number) => number,
                    Err(_) => return Err("Error: Please enter a valid number.".to_string()),
                };
                if number >= self.list.len() {
                    return Err(number.to_string()
                + " is not a valid task, please list tasks to see what numbers are available");
                }
                println!("Task found successfully.Enter new task content:");
                let mut input_content = String::new();
                match io::stdin().read_line(&mut input_content) {
                    Ok(_) => (),
                    Err(_) => return Err("Error getting arguments".to_string()),
                }
                if input_content.trim().chars().count() == 0 {
                    return Err("You must provide something".to_string());
                }
                match self.edit(Some(number.to_string()), Some(input_content.trim().to_string())) {
                    Ok(item) => return Ok(item),
                    Err(_) => return Err("Error editing task ".to_string() + &number.to_string()),
                }
            }
        }
    }

    pub fn done(&mut self, index: Option<usize>) -> Result<usize, String> {
        match index {
            Some(index) => {
                if index >= self.list.len() {
                    return Err(index.to_string()
                    + " is not a valid task, please list tasks to see what numbers are available");
                }
                if self.list[index].status == TodoItemStatus::Undone
                    || self.list[index].status == TodoItemStatus::Pending
                {
                    self.list[index].status = TodoItemStatus::Done
                } else {
                    self.list[index].status = TodoItemStatus::Undone
                }
                Ok(index)
            }

            None => {
                println!("Enter the number of task to mark as done:");
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => (),
                    Err(_) => return Err("Error getting arguments".to_string()),
                }
                if input.trim().chars().count() == 0 {
                    return Err("You must provide some number".to_string());
                }
                let number: usize = match input.trim().parse() {
                    Ok(number) => number,
                    Err(_) => return Err("Error: Please enter a valid number.".to_string()),
                };
                if number >= self.list.len() {
                    return Err(number.to_string()
                + " is not a valid task, please list tasks to see what numbers are available");
                }
                match self.done(Some(number)) {
                    Ok(index) => return Ok(index),
                    Err(_) => return Err("Error marking task ".to_string() + &number.to_string()),
                }
            }
        }
    }

    pub fn done_all(&mut self) {
        self.mark_all(TodoItemStatus::Done);
        println!("All tasks have been marked as 'Done'")
    }

    pub fn undone(&mut self, index: Option<usize>) -> Result<usize, String> {
        match index {
            Some(index) => {
                if index >= self.list.len() {
                    return Err(index.to_string()
                        + " is not a valid task, please list tasks to see what numbers are available");
                }
                if self.list[index].status == TodoItemStatus::Undone {
                    println!("Task currently undone")
                } else {
                    self.list[index].status = TodoItemStatus::Undone
                }
                Ok(index)
            }

            None => {
                println!("Enter the number of task to mark as undone:");
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => (),
                    Err(_) => return Err("Error getting arguments".to_string()),
                }
                if input.trim().chars().count() == 0 {
                    return Err("You must provide some number".to_string());
                }
                let number: usize = match input.trim().parse() {
                    Ok(number) => number,
                    Err(_) => return Err("Error: Please enter a valid number.".to_string()),
                };
                if number >= self.list.len() {
                    return Err(number.to_string()
                + " is not a valid task, please list tasks to see what numbers are available");
                }
                match self.done(Some(number)) {
                    Ok(index) => return Ok(index),
                    Err(_) => return Err("Error marking task ".to_string() + &number.to_string()),
                }
            }
        }
    }

    pub fn undone_all(&mut self) {
        self.mark_all(TodoItemStatus::Undone);
        println!("All tasks have been marked as 'Undone'")
    }

    pub fn pending(&mut self, index: Option<usize>) -> Result<usize, String> {
        match index {
            Some(index) => {
                if index >= self.list.len() {
                    return Err(index.to_string()
                        + " is not a valid task, please list tasks to see what numbers are available");
                }
                if self.list[index].status == TodoItemStatus::Pending {
                    println!("Task currently pending")
                } else {
                    self.list[index].status = TodoItemStatus::Pending
                }
                Ok(index)
            }

            None => {
                println!("Enter the number of task to mark as pending:");
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => (),
                    Err(_) => return Err("Error getting arguments".to_string()),
                }
                if input.trim().chars().count() == 0 {
                    return Err("You must provide some number".to_string());
                }
                let number: usize = match input.trim().parse() {
                    Ok(number) => number,
                    Err(_) => return Err("Error: Please enter a valid number.".to_string()),
                };
                if number >= self.list.len() {
                    return Err(number.to_string()
                + " is not a valid task, please list tasks to see what numbers are available");
                }
                match self.pending(Some(number)) {
                    Ok(index) => return Ok(index),
                    Err(_) => return Err("Error marking task ".to_string() + &number.to_string()),
                }
            }
        }
    }
 
    pub fn pending_all(&mut self) {
        self.mark_all(TodoItemStatus::Pending);
        println!("All tasks have been marked as 'Pending'")
    }

    fn print_list(&self, status: TodoItemStatus) {
        println!("{:?} TASKS:", status.to_string().to_uppercase());
        for (index, item) in self.list.iter().enumerate() {
            if item.status == status {
                println!("{}. [ {:?} ] - {}", index, item.status, item.name)
            }
        }
    }

    pub fn print_list_done(&self) {
        self.print_list(TodoItemStatus::Done);
    }

    pub fn print_list_undone(&self) {
        self.print_list(TodoItemStatus::Undone);
    }

    pub fn print_list_pending(&self) {
        self.print_list(TodoItemStatus::Pending);
    }

    pub fn print_categorized(&self) {
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

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {:?} ] - {}", self.status, self.name)
    }
}

impl fmt::Display for TodoItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TodoItemStatus::Done => write!(f, "Done"),
            TodoItemStatus::Undone => write!(f, "Undone"),
            TodoItemStatus::Pending => write!(f, "Pending"),
        }
    }
}

impl fmt::Debug for TodoItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Done => write!(f, "{}", "✔️"),
            Self::Undone => write!(f, "{}", "✖️"),
            _ => write!(f, "{}", "➖"),
        }
    }
}
