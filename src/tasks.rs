use crate::database;
use chrono::NaiveDateTime;
use rusqlite::params;
use std::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_task() {
        assert_eq!(
            new_task("New Task".to_string(), "2022-02-01 20:00:00".to_string()).unwrap(),
            "New Task".to_string()
        )
    }

    #[test]
    #[should_panic]
    fn test_new_task_fail() {
        assert!(new_task("New Task".to_string(), "invalid date".to_string()).is_ok())
    }
}

fn parse_date(date: &String) -> Result<(), String> {
    match NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M:%S") {
        Ok(_) => Ok(()),
        Err(_) => return Err("not a valid datetime.".to_string()),
    }
}

struct Task {
    name: String,
    date: String,
    done: bool,
}

impl Task {
    fn new(name: String, date: String) -> Task {
        Task {
            name: name,
            date: date,
            done: false,
        }
    }
}

pub fn new_task(name: String, date: String) -> Result<String, Box<dyn Error>> {
    if date != "" {
        parse_date(&date)?
    }
    let conn = database::get_db();
    let task = Task::new(name, date);
    conn.execute(
        "INSERT INTO tasks (task_name, task_date,task_done) VALUES (?1, ?2, ?3)",
        params![task.name, task.date, task.done],
    )?;
    Ok(task.name)
}