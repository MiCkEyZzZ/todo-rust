use chrono::NaiveDate;
use colored::*;
use std::io::{self, Write};

// определение типажа для задачи
pub trait TaskTrait {
    fn get_id(&self) -> u32;
    fn get_title(&self) -> &str;
    fn get_description(&self) -> &str;
    fn get_deadline(&self) -> Option<&str>;
    fn set_title(&mut self, title: String);
    fn set_description(&mut self, description: String);
    fn set_deadline(&mut self, deadline: Option<String>);
    fn toggle_completed(&mut self);
    fn print(&self);
}

// определение структуры для задачи
#[derive(Debug)]
pub struct Task {
    id: u32,
    title: String,
    description: String,
    completed: bool,
    deadline: Option<String>,
}

// реализация типажа для структуры задачи
impl Task {
    fn new(id: u32, title: String, description: String) -> Self {
        Self {
            id,
            title,
            description,
            completed: false,
            deadline: None,
        }
    }
}

// реализация типаж для структуры задачи
impl TaskTrait for Task {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_title(&self) -> &str {
        &self.title
    }

    fn get_description(&self) -> &str {
        &self.description
    }

    fn get_deadline(&self) -> Option<&str> {
        self.deadline.as_deref()
    }

    fn set_title(&mut self, title: String) {
        self.title = title;
    }

    fn set_description(&mut self, description: String) {
        self.description = description;
    }

    fn set_deadline(&mut self, deadline: Option<String>) {
        self.deadline = deadline;
    }

    fn toggle_completed(&mut self) {
        self.completed = !self.completed;
    }

    fn print(&self) {
        println!("идентификатор: {}", self.id);
        println!("название: {}", self.title);
        println!("описание: {}", self.description);

        if let Some(deadline) = &self.deadline {
            println!("срок выполнения: {}", deadline)
        }

        let status = if self.completed {
            "Выполнен".green().bold()
        } else {
            "Не выполнен".red().bold()
        };

        println!("Статус задачи: {}", status);
    }
}

// определение структуры для списка задач
pub struct TasksList {
    pub tasks: Vec<Box<dyn TaskTrait>>,
}

// реализация типажа для структуры списка задач
impl TasksList {
    fn new() -> TasksList {
        TasksList { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: Box<dyn TaskTrait>) {
        self.tasks.push(task);
    }

    fn remove_task(&mut self, id: u32) {
        if let Some(i) = self.tasks.iter().position(|task| task.get_id() == id) {
            self.tasks.remove(i);
        }
    }

    fn find_task_by_id(&mut self, id: u32) -> Option<&Box<dyn TaskTrait>> {
        self.tasks.iter().find(|task| task.get_id() == id)
    }

    fn find_task_by_title(&mut self, title: &str) -> Vec<&Box<dyn TaskTrait>> {
        self.tasks
            .iter()
            .filter(|task| task.get_title() == title)
            .collect()
    }

    fn sort_by_title(&mut self) {
        self.tasks.sort_by(|a, b| a.get_title().cmp(b.get_title()));
    }

    fn sort_by_deadline(&mut self) {
        self.tasks.sort_by(|a, b| {
            let a_deadline = a.get_deadline();
            let b_deadline = b.get_deadline();

            match (a_deadline, b_deadline) {
                (Some(a_date), Some(b_date)) => {
                    let a_date = NaiveDate::parse_from_str(a_date, "%Y-%m-%d").unwrap();
                    let b_date = NaiveDate::parse_from_str(b_date, "%Y-%m-%d").unwrap();
                    a_date.cmp(&b_date)
                }
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Equal,
                (None, None) => std::cmp::Ordering::Greater,
            }
        });
    }

    fn print_all_tasks(&self) {
        for task in &self.tasks {
            task.print();
            println!("---------------------------------------------------------------------------");
        }
    }
}

fn main() {
    let mut tasks_list = TasksList::new();

    loop {
        println!("🦀 🚀 Приложение список задач");
        println!("Привет 👋, выбирай команды из списка 👇👇👇 и следуй инструкции.");
        println!("-------------------------------------------------------------------------------");
        println!("1. Добавить задачу");
        println!("2. Показать задачи");
        println!("3. Переключить задачу, как выполненная");
        println!("4. Удалить задачу");
        println!("5. Найти задачу по идентификатору");
        println!("6. Найти задачу по названию");
        println!("7. Отсортировать задачу по названию");
        println!("8. Отсортировать задач по сроку");
        println!("9. Выход из приложения");
        print!("Выберете команду из списка выше: ");

        let choice: u32 = read_user_choice("Выберите команду из списка: ");

        match choice {
            1 => {
                let id: u32 = read_user_choice("Введите идентификатор задачи: ");
                let title = read_input("Введите название задачи: ");
                let description = read_input("Введите описание задачи: ");
                let deadline = read_input("Введите сроки выполнения задачи (Г-м-д): ");
                let mut task = Box::new(Task::new(id, title, description));
                task.set_deadline(Some(deadline));
                tasks_list.add_task(task);
            }
            2 => tasks_list.print_all_tasks(),
            3 => {
                let id: u32 =
                    read_user_choice("Введите идентификатор задачи, чтобы переключить завершение");

                if let Some(i) = tasks_list.tasks.iter().position(|task| task.get_id() == id) {
                    tasks_list.tasks[i].toggle_completed();
                    println!("Задача с идентификатором {} выполнена.", id);
                } else {
                    println!("Задача с идентификатором {} не выполнена.", id);
                }
            }
            4 => {
                let id: u32 = read_user_choice("Введите идентификатор задачи для удаления");
                tasks_list.remove_task(id);
                println!("Задача с идентификатором {} удалена", id);
            }
            5 => {
                let id: u32 = read_user_choice("Введите идентификатор для поиска задачи");

                if let Some(task) = tasks_list.find_task_by_id(id) {
                    println!("Задача найдена");
                    task.print();
                } else {
                    println!("Задача с идентификатором {} не найдена", id);
                }
            }
            6 => {
                let title = read_input("Введите имя для поиска задачи");
                let task_with_title = tasks_list.find_task_by_title(&title);
                println!("Задача с названием {}", title);

                for t in task_with_title {
                    t.print();
                    println!("-------------------------------------------------------------------");
                }
            }
            7 => {
                tasks_list.sort_by_title();
                println!("-----------------------------------------------------------------------");
            }
            8 => {
                tasks_list.sort_by_deadline();
                println!("Задачи отсортированы по срокам исполнения.");
            }
            9 => break,
            _ => println!("Неверный выбор. Пожалуйста, попробуйте еще раз."),
        }
    }
}

fn read_user_choice(prompt: &str) -> u32 {
    loop {
        println!("{}", prompt);
        println!("-------------------------------------------------------------------------------");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse() {
            Ok(n) => return n,
            Err(_) => println!("Пожалуйста, введите корректный номер команды."),
        }
    }
}

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    println!("-----------------------------------------------------------------------------------");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}
