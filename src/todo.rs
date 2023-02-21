use std::{fmt};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn mark_undone (&mut self, index:usize){
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
