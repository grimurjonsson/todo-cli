use crate::todo::{TodoItem, TodoList, TodoState};
use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use std::path::PathBuf;

pub fn serialize_todo_list(list: &TodoList) -> String {
    let mut output = String::new();

    // Add header with human-readable date
    output.push_str(&format!(
        "# Todo List - {}\n\n",
        list.date.format("%B %d, %Y")
    ));

    // Serialize each item
    for item in &list.items {
        let indent = "  ".repeat(item.indent_level);
        output.push_str(&format!(
            "{}- [{}] {}\n",
            indent,
            item.state.to_char(),
            item.content
        ));
    }

    output
}

pub fn parse_todo_list(content: &str, date: NaiveDate, file_path: PathBuf) -> Result<TodoList> {
    let mut items = Vec::new();

    for line in content.lines() {
        // Skip empty lines and headers
        if line.trim().is_empty() || line.trim().starts_with('#') {
            continue;
        }

        // Check if line is a checkbox item
        if let Some(item) = parse_todo_line(line)? {
            items.push(item);
        }
    }

    Ok(TodoList::with_items(date, file_path, items))
}

fn parse_todo_line(line: &str) -> Result<Option<TodoItem>> {
    // Count leading spaces for indent level
    let indent_level = line.len() - line.trim_start().len();
    let indent_level = indent_level / 2; // 2 spaces per indent

    let trimmed = line.trim_start();

    // Check if it's a checkbox line: starts with "- ["
    if !trimmed.starts_with("- [") {
        return Ok(None);
    }

    // Extract the state character
    if trimmed.len() < 5 {
        return Err(anyhow!("Invalid checkbox format"));
    }

    let state_char = trimmed.chars().nth(3).ok_or_else(|| anyhow!("Missing state character"))?;
    let state = TodoState::from_char(state_char)
        .ok_or_else(|| anyhow!("Invalid state character: {}", state_char))?;

    // Extract content after "] "
    let content = if trimmed.len() > 5 {
        trimmed[5..].trim().to_string()
    } else {
        String::new()
    };

    Ok(Some(TodoItem::with_state(content, state, indent_level)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn create_test_date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()
    }

    fn create_test_path() -> PathBuf {
        PathBuf::from("/tmp/2025-12-31.md")
    }

    #[test]
    fn test_serialize_empty_list() {
        let date = create_test_date();
        let path = create_test_path();
        let list = TodoList::new(date, path);

        let markdown = serialize_todo_list(&list);
        assert!(markdown.contains("# Todo List - December 31, 2025"));
    }

    #[test]
    fn test_serialize_simple_list() {
        let date = create_test_date();
        let path = create_test_path();
        let mut list = TodoList::new(date, path);

        list.add_item("Task 1".to_string());
        list.add_item("Task 2".to_string());

        let markdown = serialize_todo_list(&list);
        assert!(markdown.contains("- [ ] Task 1"));
        assert!(markdown.contains("- [ ] Task 2"));
    }

    #[test]
    fn test_serialize_with_states() {
        let date = create_test_date();
        let path = create_test_path();
        let mut list = TodoList::new(date, path);

        list.add_item("Empty task".to_string());
        list.add_item("Checked task".to_string());
        list.add_item("Question task".to_string());
        list.add_item("Important task".to_string());

        list.items[1].state = TodoState::Checked;
        list.items[2].state = TodoState::Question;
        list.items[3].state = TodoState::Exclamation;

        let markdown = serialize_todo_list(&list);
        assert!(markdown.contains("- [ ] Empty task"));
        assert!(markdown.contains("- [x] Checked task"));
        assert!(markdown.contains("- [?] Question task"));
        assert!(markdown.contains("- [!] Important task"));
    }

    #[test]
    fn test_serialize_with_indentation() {
        let date = create_test_date();
        let path = create_test_path();
        let mut list = TodoList::new(date, path);

        list.add_item_with_indent("Parent".to_string(), 0);
        list.add_item_with_indent("Child 1".to_string(), 1);
        list.add_item_with_indent("Grandchild".to_string(), 2);
        list.add_item_with_indent("Child 2".to_string(), 1);

        let markdown = serialize_todo_list(&list);
        assert!(markdown.contains("- [ ] Parent\n"));
        assert!(markdown.contains("  - [ ] Child 1\n"));
        assert!(markdown.contains("    - [ ] Grandchild\n"));
        assert!(markdown.contains("  - [ ] Child 2\n"));
    }

    #[test]
    fn test_parse_simple_list() {
        let content = r#"# Todo List - December 31, 2025

- [ ] Task 1
- [x] Task 2
- [?] Task 3
- [!] Task 4
"#;

        let date = create_test_date();
        let path = create_test_path();
        let list = parse_todo_list(content, date, path).unwrap();

        assert_eq!(list.items.len(), 4);
        assert_eq!(list.items[0].content, "Task 1");
        assert_eq!(list.items[0].state, TodoState::Empty);
        assert_eq!(list.items[1].content, "Task 2");
        assert_eq!(list.items[1].state, TodoState::Checked);
        assert_eq!(list.items[2].content, "Task 3");
        assert_eq!(list.items[2].state, TodoState::Question);
        assert_eq!(list.items[3].content, "Task 4");
        assert_eq!(list.items[3].state, TodoState::Exclamation);
    }

    #[test]
    fn test_parse_with_indentation() {
        let content = r#"# Todo List - December 31, 2025

- [ ] Parent
  - [ ] Child 1
    - [ ] Grandchild
  - [ ] Child 2
"#;

        let date = create_test_date();
        let path = create_test_path();
        let list = parse_todo_list(content, date, path).unwrap();

        assert_eq!(list.items.len(), 4);
        assert_eq!(list.items[0].indent_level, 0);
        assert_eq!(list.items[1].indent_level, 1);
        assert_eq!(list.items[2].indent_level, 2);
        assert_eq!(list.items[3].indent_level, 1);
    }

    #[test]
    fn test_round_trip() {
        let date = create_test_date();
        let path = create_test_path();
        let mut list = TodoList::new(date.clone(), path.clone());

        list.add_item_with_indent("Parent".to_string(), 0);
        list.add_item_with_indent("Child".to_string(), 1);
        list.items[1].state = TodoState::Checked;

        let markdown = serialize_todo_list(&list);
        let parsed = parse_todo_list(&markdown, date, path).unwrap();

        assert_eq!(parsed.items.len(), 2);
        assert_eq!(parsed.items[0].content, "Parent");
        assert_eq!(parsed.items[0].indent_level, 0);
        assert_eq!(parsed.items[0].state, TodoState::Empty);
        assert_eq!(parsed.items[1].content, "Child");
        assert_eq!(parsed.items[1].indent_level, 1);
        assert_eq!(parsed.items[1].state, TodoState::Checked);
    }

    #[test]
    fn test_parse_skips_non_checkbox_lines() {
        let content = r#"# Todo List - December 31, 2025

Some random text
- [ ] Task 1
Another line
- [x] Task 2

Empty line above
"#;

        let date = create_test_date();
        let path = create_test_path();
        let list = parse_todo_list(content, date, path).unwrap();

        assert_eq!(list.items.len(), 2);
        assert_eq!(list.items[0].content, "Task 1");
        assert_eq!(list.items[1].content, "Task 2");
    }
}
