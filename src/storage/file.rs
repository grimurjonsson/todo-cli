use super::markdown::{parse_todo_list, serialize_todo_list};
use crate::todo::TodoList;
use crate::utils::paths::{ensure_directories_exist, get_daily_file_path};
use anyhow::{Context, Result};
use chrono::NaiveDate;
use std::fs;
use std::path::Path;

pub fn load_todo_list(date: NaiveDate) -> Result<TodoList> {
    ensure_directories_exist()?;

    let file_path = get_daily_file_path(date)?;

    if !file_path.exists() {
        return Ok(TodoList::new(date, file_path));
    }

    let content = fs::read_to_string(&file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

    parse_todo_list(&content, date, file_path)
        .with_context(|| "Failed to parse todo list")
}

pub fn save_todo_list(list: &TodoList) -> Result<()> {
    ensure_directories_exist()?;

    let content = serialize_todo_list(list);

    // Atomic write: write to temp file then rename
    let temp_path = list.file_path.with_extension("tmp");

    fs::write(&temp_path, content)
        .with_context(|| format!("Failed to write to temp file: {}", temp_path.display()))?;

    fs::rename(&temp_path, &list.file_path)
        .with_context(|| format!("Failed to rename temp file to: {}", list.file_path.display()))?;

    Ok(())
}

pub fn file_exists(date: NaiveDate) -> Result<bool> {
    let file_path = get_daily_file_path(date)?;
    Ok(file_path.exists())
}

#[allow(dead_code)]
pub fn delete_file(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_file(path)
            .with_context(|| format!("Failed to delete file: {}", path.display()))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use chrono::NaiveDate;

    // Helper to setup temporary test directory
    fn setup_test_dir() -> TempDir {
        TempDir::new().unwrap()
    }

    #[test]
    fn test_save_and_load_todo_list() {
        let date = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
        let temp_dir = setup_test_dir();
        let file_path = temp_dir.path().join("2025-12-31.md");

        // Create a list
        let mut list = TodoList::new(date, file_path.clone());
        list.add_item("Test task 1".to_string());
        list.add_item("Test task 2".to_string());

        // Save it
        let content = serialize_todo_list(&list);
        fs::write(&file_path, content).unwrap();

        // Load it back
        let loaded_content = fs::read_to_string(&file_path).unwrap();
        let loaded_list = parse_todo_list(&loaded_content, date, file_path).unwrap();

        assert_eq!(loaded_list.items.len(), 2);
        assert_eq!(loaded_list.items[0].content, "Test task 1");
        assert_eq!(loaded_list.items[1].content, "Test task 2");
    }

    #[test]
    fn test_serialize_and_deserialize_preserves_data() {
        let date = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
        let temp_dir = setup_test_dir();
        let file_path = temp_dir.path().join("test.md");

        let mut list = TodoList::new(date, file_path.clone());
        list.add_item_with_indent("Parent".to_string(), 0);
        list.add_item_with_indent("Child".to_string(), 1);
        list.items[1].state = crate::todo::TodoState::Checked;

        let markdown = serialize_todo_list(&list);
        let parsed = parse_todo_list(&markdown, date, file_path).unwrap();

        assert_eq!(parsed.items.len(), 2);
        assert_eq!(parsed.items[0].content, "Parent");
        assert_eq!(parsed.items[1].content, "Child");
        assert_eq!(parsed.items[1].state, crate::todo::TodoState::Checked);
    }
}
