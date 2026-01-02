# Todo-CLI Complete Implementation Plan

## Project Overview
A Rust-based terminal UI todo application with daily rolling lists stored as Markdown files, compatible with Obsidian.

**Repository**: `/Users/gimmi/Documents/Sources/rust/todo-cli`

---

## ✅ COMPLETED WORK (Phase 1-5)

### Phase 1: Project Initialization ✓

**Files Created:**
- `Cargo.toml` - Project manifest with all dependencies
- Directory structure: `src/{app,todo,storage,ui,utils}`

**Dependencies Added:**
```toml
[dependencies]
ratatui = "0.29"
crossterm = "0.28"
clap = { version = "4.5", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
pulldown-cmark = "0.12"
chrono = "0.4"
uuid = { version = "1.11", features = ["v4"] }
anyhow = "1.0"
dirs = "5.0"
```

### Phase 2: Core Data Models ✓

**src/todo/state.rs** - TodoState enum
- Four states: `Empty [ ]`, `Checked [x]`, `Question [?]`, `Exclamation [!]`
- Methods: `to_char()`, `from_char()`, `cycle()`, `is_complete()`
- Full unit test coverage

**src/todo/item.rs** - TodoItem struct
```rust
pub struct TodoItem {
    pub id: Uuid,
    pub content: String,
    pub state: TodoState,
    pub indent_level: usize,
}
```
- Methods: `new()`, `with_state()`, `toggle_state()`, `can_indent()`, `can_outdent()`, `indent()`, `outdent()`
- Hierarchical support via `indent_level` (flat list structure, not tree)

**src/todo/list.rs** - TodoList struct
```rust
pub struct TodoList {
    pub date: NaiveDate,
    pub items: Vec<TodoItem>,
    pub file_path: PathBuf,
}
```
- Methods: `add_item()`, `add_item_with_indent()`, `get_incomplete_items()`, `toggle_item_state()`, `delete_item()`, `insert_item()`

### Phase 3: Storage & Serialization ✓

**src/storage/markdown.rs** - Markdown serialization
- **Format**: Obsidian-compatible markdown
- **File naming**: `~/.todo-cli/dailies/YYYY-MM-DD.md`
- **Structure**:
  ```markdown
  # Todo List - December 31, 2025

  - [ ] Top level task
  - [x] Completed task
    - [ ] Nested task (2 spaces per indent)
    - [?] Question task
      - [!] Important nested task
  ```
- **Key decisions**:
  - Two spaces per indent level (Obsidian standard)
  - H1 header with human-readable date
  - Custom states `[?]` and `[!]` work in Obsidian
- Functions: `serialize_todo_list()`, `parse_todo_list()`, `parse_todo_line()`
- Comprehensive round-trip testing

**src/storage/file.rs** - File I/O operations
- `load_todo_list(date)` - Loads from ~/.todo-cli/dailies/YYYY-MM-DD.md
- `save_todo_list(list)` - Atomic writes (write to .tmp, then rename)
- `file_exists(date)` - Check if daily file exists
- Auto-creates directories on first run

**src/utils/paths.rs** - Path resolution
- `get_todo_cli_dir()` - Returns ~/.todo-cli
- `get_dailies_dir()` - Returns ~/.todo-cli/dailies
- `get_config_path()` - Returns ~/.todo-cli/config.toml
- `get_daily_file_path(date)` - Returns full path for date
- `ensure_directories_exist()` - Creates dirs on first run

### Phase 4: Rollover Logic ✓

**src/storage/rollover.rs** - Daily rollover
- `check_and_prompt_rollover()` - Main rollover orchestrator
  - Checks if today's file exists
  - If not, looks for yesterday's incomplete items
  - Prompts user: "Roll over incomplete items to today? (Y/n)"
- `create_rolled_over_list(date, items)` - Creates new list with carried-over items
- `prompt_user_for_rollover(incomplete)` - Interactive CLI prompt

**Rollover Behavior:**
1. On first use each day, checks for yesterday's file
2. Filters incomplete items (not `Checked` state)
3. Shows preview of items to roll over (max 5, then "... and N more")
4. Default is "Yes" (press Enter to accept)
5. Creates new daily file with incomplete items

### Phase 5: CLI Commands ✓

**src/cli.rs** - Clap command definitions
```rust
pub enum Commands {
    Add { task: String },  // Add new todo
    Show,                  // Show today's list
}
```

**src/config.rs** - Configuration management
```rust
pub struct Config {
    pub theme: String,  // Default: "default"
}
```
- `Config::load()` - Loads from ~/.todo-cli/config.toml
- `Config::save()` - Saves config
- Uses TOML format

**src/main.rs** - Entry point
- `handle_add(task)` - Adds todo, triggers rollover check
- `handle_show()` - Displays today's list, triggers rollover check
- No subcommand = placeholder message (TUI coming soon)

**CLI Usage:**
```bash
todo add "Your task here"   # ✓ Todo added successfully!
todo show                   # Lists all items with state icons
```

**Testing Completed:**
- ✅ `cargo build` - Compiles successfully
- ✅ `todo add` - Creates ~/.todo-cli/dailies/2025-12-31.md
- ✅ `todo show` - Displays items correctly
- ✅ Markdown file - Obsidian-compatible format verified

---

## 🎯 REMAINING WORK (Phase 6-12)

### Phase 6: Theme System (NOT STARTED)

**Files to Create:**
- `src/ui/theme.rs` - Color scheme definitions

**Implementation:**
```rust
use ratatui::style::Color;

pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub cursor: Color,
    pub checked: Color,      // Green
    pub unchecked: Color,    // Gray
    pub question: Color,     // Yellow
    pub exclamation: Color,  // Red
    pub status_bar_bg: Color,
    pub status_bar_fg: Color,
}

impl Theme {
    pub fn default() -> Self { /* ... */ }
    pub fn dark() -> Self { /* ... */ }
    pub fn light() -> Self { /* ... */ }

    pub fn from_config(config: &Config) -> Self {
        match config.theme.as_str() {
            "dark" => Self::dark(),
            "light" => Self::light(),
            _ => Self::default(),
        }
    }
}
```

**Color Recommendations:**
- Checked: `Color::Green` (completed items)
- Empty: `Color::Gray` or `Color::White` (pending)
- Question: `Color::Yellow` (waiting/uncertain)
- Exclamation: `Color::Red` (important/urgent)
- Cursor: `Color::Cyan` (selected item)

### Phase 7: Application State (NOT STARTED)

**Files to Create:**
- `src/app/mod.rs` - Module exports
- `src/app/mode.rs` - Navigate/Edit mode enum
- `src/app/state.rs` - Main application state

**src/app/mode.rs:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Navigate,  // Default: browse, mark, move, delete
    Edit,      // Text input for new/editing items
}
```

**src/app/state.rs:**
```rust
use crate::todo::TodoList;
use crate::ui::theme::Theme;
use super::mode::Mode;

pub struct AppState {
    pub todo_list: TodoList,
    pub cursor_position: usize,
    pub mode: Mode,
    pub scroll_offset: usize,
    pub edit_buffer: String,         // For Edit mode
    pub edit_cursor_pos: usize,      // Cursor in edit buffer
    pub should_quit: bool,
    pub show_help: bool,
    pub theme: Theme,
    pub unsaved_changes: bool,       // For auto-save indicator
    pub last_save_time: Option<Instant>,
}

impl AppState {
    pub fn new(todo_list: TodoList, theme: Theme) -> Self {
        Self {
            todo_list,
            cursor_position: 0,
            mode: Mode::Navigate,
            scroll_offset: 0,
            edit_buffer: String::new(),
            edit_cursor_pos: 0,
            should_quit: false,
            show_help: false,
            theme,
            unsaved_changes: false,
            last_save_time: None,
        }
    }

    // Navigation helpers
    pub fn move_cursor_up(&mut self) { /* ... */ }
    pub fn move_cursor_down(&mut self) { /* ... */ }
    pub fn selected_item(&self) -> Option<&TodoItem> { /* ... */ }
    pub fn selected_item_mut(&mut self) -> Option<&mut TodoItem> { /* ... */ }
}
```

### Phase 8: Event Handling (NOT STARTED)

**Files to Create:**
- `src/app/event.rs` - Keyboard event handlers

**src/app/event.rs:**
```rust
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use anyhow::Result;
use super::state::AppState;
use super::mode::Mode;
use crate::storage::save_todo_list;

pub fn handle_key_event(key: KeyEvent, state: &mut AppState) -> Result<()> {
    match state.mode {
        Mode::Navigate => handle_navigate_mode(key, state)?,
        Mode::Edit => handle_edit_mode(key, state)?,
    }
    Ok(())
}

fn handle_navigate_mode(key: KeyEvent, state: &mut AppState) -> Result<()> {
    match (key.code, key.modifiers) {
        // Navigation
        (KeyCode::Up, KeyModifiers::NONE) | (KeyCode::Char('k'), KeyModifiers::NONE) => {
            state.move_cursor_up();
        }
        (KeyCode::Down, KeyModifiers::NONE) | (KeyCode::Char('j'), KeyModifiers::NONE) => {
            state.move_cursor_down();
        }

        // Toggle state (cycle through all 4 states)
        (KeyCode::Char(' '), KeyModifiers::NONE) => {
            if let Some(item) = state.selected_item_mut() {
                item.toggle_state();
                state.unsaved_changes = true;
            }
        }

        // Move item up/down with children
        (KeyCode::Up, KeyModifiers::SHIFT | KeyModifiers::SUPER) |
        (KeyCode::Up, KeyModifiers::SHIFT | KeyModifiers::CONTROL) => {
            move_item_with_children_up(state)?;
        }
        (KeyCode::Down, KeyModifiers::SHIFT | KeyModifiers::SUPER) |
        (KeyCode::Down, KeyModifiers::SHIFT | KeyModifiers::CONTROL) => {
            move_item_with_children_down(state)?;
        }

        // Indent/outdent
        (KeyCode::Right, KeyModifiers::SHIFT | KeyModifiers::SUPER) |
        (KeyCode::Right, KeyModifiers::SHIFT | KeyModifiers::CONTROL) => {
            indent_item(state)?;
        }
        (KeyCode::Left, KeyModifiers::SHIFT | KeyModifiers::SUPER) |
        (KeyCode::Left, KeyModifiers::SHIFT | KeyModifiers::CONTROL) => {
            outdent_item(state)?;
        }

        // Enter edit mode
        (KeyCode::Char('i'), KeyModifiers::NONE) | (KeyCode::Enter, KeyModifiers::NONE) => {
            enter_edit_mode(state);
        }

        // New item
        (KeyCode::Char('n'), KeyModifiers::NONE) => {
            new_item_below(state);
        }

        // Delete item
        (KeyCode::Char('d'), KeyModifiers::NONE) => {
            delete_current_item(state)?;
        }

        // Help toggle
        (KeyCode::Char('?'), KeyModifiers::NONE) => {
            state.show_help = !state.show_help;
        }

        // Quit
        (KeyCode::Char('q'), KeyModifiers::NONE) => {
            state.should_quit = true;
        }

        _ => {}
    }

    // Auto-save on changes (debounced in main loop)
    if state.unsaved_changes {
        save_todo_list(&state.todo_list)?;
        state.unsaved_changes = false;
        state.last_save_time = Some(Instant::now());
    }

    Ok(())
}

fn handle_edit_mode(key: KeyEvent, state: &mut AppState) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            // Cancel edit
            state.mode = Mode::Navigate;
            state.edit_buffer.clear();
        }
        KeyCode::Enter => {
            // Save edit
            save_edit_buffer(state)?;
            state.mode = Mode::Navigate;
        }
        KeyCode::Backspace => {
            if state.edit_cursor_pos > 0 {
                state.edit_buffer.remove(state.edit_cursor_pos - 1);
                state.edit_cursor_pos -= 1;
            }
        }
        KeyCode::Left => {
            if state.edit_cursor_pos > 0 {
                state.edit_cursor_pos -= 1;
            }
        }
        KeyCode::Right => {
            if state.edit_cursor_pos < state.edit_buffer.len() {
                state.edit_cursor_pos += 1;
            }
        }
        KeyCode::Char(c) => {
            state.edit_buffer.insert(state.edit_cursor_pos, c);
            state.edit_cursor_pos += 1;
        }
        _ => {}
    }
    Ok(())
}

// Helper functions (implement these)
fn move_item_with_children_up(state: &mut AppState) -> Result<()> { /* ... */ }
fn move_item_with_children_down(state: &mut AppState) -> Result<()> { /* ... */ }
fn indent_item(state: &mut AppState) -> Result<()> { /* ... */ }
fn outdent_item(state: &mut AppState) -> Result<()> { /* ... */ }
fn enter_edit_mode(state: &mut AppState) { /* ... */ }
fn new_item_below(state: &mut AppState) { /* ... */ }
fn delete_current_item(state: &mut AppState) -> Result<()> { /* ... */ }
fn save_edit_buffer(state: &mut AppState) -> Result<()> { /* ... */ }
```

**Key Decisions Made:**
- ✅ Support both Cmd (macOS) and Ctrl (Linux/Windows) for move/indent operations
- ✅ Simple single-line text input for Edit mode (no complex text editor widget)
- ✅ Move items WITH all children (recursive logic)
- ✅ Auto-save on every change (atomic file writes prevent data loss)

### Phase 9: Hierarchy Operations (NOT STARTED)

**Files to Create:**
- `src/todo/hierarchy.rs` - Move with children logic

**src/todo/hierarchy.rs:**
```rust
use super::{TodoItem, TodoList};
use anyhow::{Result, anyhow};

impl TodoList {
    /// Move item and all its children up by one position
    pub fn move_item_with_children_up(&mut self, index: usize) -> Result<()> {
        if index == 0 {
            return Err(anyhow!("Cannot move first item up"));
        }

        let (item_start, item_end) = self.get_item_range(index)?;

        // Find the previous item's range
        let prev_end = item_start - 1;
        let (prev_start, _) = self.get_item_range_reverse(prev_end)?;

        // Extract both ranges
        let mut prev_items: Vec<_> = self.items.drain(prev_start..item_start).collect();
        let mut current_items: Vec<_> = self.items.drain(prev_start..prev_start + (item_end - item_start)).collect();

        // Swap them
        self.items.splice(prev_start..prev_start, current_items);
        self.items.splice(prev_start + (item_end - item_start)..prev_start + (item_end - item_start), prev_items);

        Ok(())
    }

    /// Move item and all its children down by one position
    pub fn move_item_with_children_down(&mut self, index: usize) -> Result<()> {
        let (item_start, item_end) = self.get_item_range(index)?;

        if item_end >= self.items.len() {
            return Err(anyhow!("Cannot move last item down"));
        }

        // Find the next item's range
        let (next_start, next_end) = self.get_item_range(item_end)?;

        // Extract both ranges
        let mut current_items: Vec<_> = self.items.drain(item_start..item_end).collect();
        let mut next_items: Vec<_> = self.items.drain(item_start..item_start + (next_end - next_start)).collect();

        // Swap them
        self.items.splice(item_start..item_start, next_items);
        self.items.splice(item_start + (next_end - next_start)..item_start + (next_end - next_start), current_items);

        Ok(())
    }

    /// Get the range (start, end) of an item and all its children
    fn get_item_range(&self, index: usize) -> Result<(usize, usize)> {
        if index >= self.items.len() {
            return Err(anyhow!("Index out of bounds"));
        }

        let base_indent = self.items[index].indent_level;
        let mut end = index + 1;

        // Find all children (items with higher indent that come immediately after)
        while end < self.items.len() && self.items[end].indent_level > base_indent {
            end += 1;
        }

        Ok((index, end))
    }

    /// Get the range searching backwards
    fn get_item_range_reverse(&self, index: usize) -> Result<(usize, usize)> {
        if index >= self.items.len() {
            return Err(anyhow!("Index out of bounds"));
        }

        // Find the parent or sibling
        let base_indent = self.items[index].indent_level;
        let mut start = index;

        // Search backwards for the start of this item's group
        while start > 0 && self.items[start - 1].indent_level >= base_indent {
            start -= 1;
        }

        Ok((start, index + 1))
    }

    /// Indent item (increase indent_level by 1)
    pub fn indent_item(&mut self, index: usize) -> Result<()> {
        if index >= self.items.len() {
            return Err(anyhow!("Index out of bounds"));
        }

        if index == 0 {
            return Err(anyhow!("Cannot indent first item"));
        }

        let prev_indent = self.items[index - 1].indent_level;
        let current_indent = self.items[index].indent_level;

        // Can only indent to at most one level beyond previous item
        if current_indent > prev_indent {
            return Err(anyhow!("Cannot indent beyond parent level"));
        }

        self.items[index].indent_level += 1;
        Ok(())
    }

    /// Outdent item (decrease indent_level by 1)
    pub fn outdent_item(&mut self, index: usize) -> Result<()> {
        if index >= self.items.len() {
            return Err(anyhow!("Index out of bounds"));
        }

        if self.items[index].indent_level == 0 {
            return Err(anyhow!("Cannot outdent top-level item"));
        }

        self.items[index].indent_level -= 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;
    use crate::utils::paths::get_daily_file_path;

    #[test]
    fn test_get_item_range() {
        let mut list = create_test_list();
        list.add_item_with_indent("Parent".to_string(), 0);
        list.add_item_with_indent("Child 1".to_string(), 1);
        list.add_item_with_indent("Grandchild".to_string(), 2);
        list.add_item_with_indent("Child 2".to_string(), 1);
        list.add_item_with_indent("Another parent".to_string(), 0);

        let (start, end) = list.get_item_range(0).unwrap();
        assert_eq!(start, 0);
        assert_eq!(end, 4); // Parent + 3 descendants

        let (start, end) = list.get_item_range(4).unwrap();
        assert_eq!(start, 4);
        assert_eq!(end, 5); // No children
    }

    #[test]
    fn test_move_item_with_children() {
        // Add comprehensive tests for move operations
    }

    #[test]
    fn test_indent_outdent() {
        // Add tests for indent/outdent validation
    }
}
```

**Update src/todo/mod.rs:**
```rust
pub mod hierarchy;
pub mod item;
pub mod list;
pub mod state;

pub use item::TodoItem;
pub use list::TodoList;
pub use state::TodoState;
```

### Phase 10: UI Components (NOT STARTED)

**Files to Create:**
- `src/ui/mod.rs` - Main TUI loop
- `src/ui/components/mod.rs` - Component exports
- `src/ui/components/todo_item.rs` - Single item widget
- `src/ui/components/todo_list.rs` - Scrollable list
- `src/ui/components/status_bar.rs` - Bottom status bar

**src/ui/mod.rs:**
```rust
pub mod components;
pub mod theme;

use crate::app::state::AppState;
use crate::app::event::handle_key_event;
use anyhow::Result;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use std::time::Duration;

pub fn run_tui(mut state: AppState) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Main loop
    loop {
        terminal.draw(|f| {
            components::render(f, &state);
        })?;

        // Poll for events with timeout (for auto-save debouncing)
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                handle_key_event(key, &mut state)?;
            }
        }

        if state.should_quit {
            break;
        }
    }

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
```

**src/ui/components/mod.rs:**
```rust
pub mod status_bar;
pub mod todo_item;
pub mod todo_list;

use crate::app::state::AppState;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),      // Todo list
            Constraint::Length(1),   // Status bar
        ])
        .split(f.area());

    // Render todo list
    todo_list::render(f, state, chunks[0]);

    // Render status bar
    status_bar::render(f, state, chunks[1]);

    // Render help overlay if active
    if state.show_help {
        render_help_overlay(f, state);
    }
}

fn render_help_overlay(f: &mut Frame, state: &AppState) {
    // TODO: Implement centered help popup
}
```

**src/ui/components/todo_list.rs:**
```rust
use crate::app::state::AppState;
use crate::app::mode::Mode;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let items: Vec<ListItem> = state
        .todo_list
        .items
        .iter()
        .enumerate()
        .map(|(idx, item)| {
            let indent = "  ".repeat(item.indent_level);
            let checkbox = format!("{}", item.state);
            let content = format!("{}{} {}", indent, checkbox, item.content);

            let style = if idx == state.cursor_position {
                Style::default()
                    .fg(state.theme.cursor)
                    .add_modifier(Modifier::REVERSED)
            } else {
                let color = match item.state {
                    crate::todo::TodoState::Checked => state.theme.checked,
                    crate::todo::TodoState::Question => state.theme.question,
                    crate::todo::TodoState::Exclamation => state.theme.exclamation,
                    _ => state.theme.foreground,
                };
                Style::default().fg(color)
            };

            ListItem::new(Line::from(Span::styled(content, style)))
        })
        .collect();

    let title = format!(
        " Todo List - {} ",
        state.todo_list.date.format("%B %d, %Y")
    );

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(Style::default().fg(state.theme.foreground));

    f.render_widget(list, area);

    // Render edit mode cursor if active
    if state.mode == Mode::Edit {
        render_edit_cursor(f, state, area);
    }
}

fn render_edit_cursor(f: &mut Frame, state: &AppState, area: Rect) {
    // TODO: Show blinking cursor in edit buffer
}
```

**src/ui/components/status_bar.rs:**
```rust
use crate::app::state::AppState;
use crate::app::mode::Mode;
use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let mode_text = match state.mode {
        Mode::Navigate => "NAVIGATE",
        Mode::Edit => "EDIT",
    };

    let save_indicator = if state.unsaved_changes {
        " [unsaved]"
    } else {
        ""
    };

    let help_text = " Press ? for help, q to quit";

    let status_line = format!(
        " {} | {} items{} {}",
        mode_text,
        state.todo_list.items.len(),
        save_indicator,
        help_text
    );

    let status = Paragraph::new(Line::from(vec![
        Span::styled(status_line, Style::default()
            .fg(state.theme.status_bar_fg)
            .bg(state.theme.status_bar_bg))
    ]));

    f.render_widget(status, area);
}
```

**src/ui/components/todo_item.rs:**
```rust
// Individual item rendering helper (if needed)
// Can be used by todo_list.rs for complex item rendering
```

### Phase 11: Update Main Entry Point (NOT STARTED)

**Update src/main.rs:**
```rust
mod app;
mod cli;
mod config;
mod storage;
mod todo;
mod ui;
mod utils;

use anyhow::Result;
use chrono::Local;
use clap::Parser;
use cli::{Cli, Commands};
use storage::{check_and_prompt_rollover, save_todo_list};
use config::Config;
use ui::theme::Theme;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load()?;

    match cli.command {
        Some(Commands::Add { task }) => {
            handle_add(task)?;
        }
        Some(Commands::Show) => {
            handle_show()?;
        }
        None => {
            // No command - launch TUI
            let list = check_and_prompt_rollover()?.unwrap_or_else(|| {
                let today = Local::now().date_naive();
                todo::TodoList::new(today, utils::paths::get_daily_file_path(today).unwrap())
            });

            let theme = Theme::from_config(&config);
            let state = app::state::AppState::new(list, theme);

            ui::run_tui(state)?;
        }
    }

    Ok(())
}

fn handle_add(task: String) -> Result<()> {
    let mut list = check_and_prompt_rollover()?.unwrap_or_else(|| {
        let today = Local::now().date_naive();
        todo::TodoList::new(today, utils::paths::get_daily_file_path(today).unwrap())
    });

    list.add_item(task);
    save_todo_list(&list)?;

    println!("✓ Todo added successfully!");

    Ok(())
}

fn handle_show() -> Result<()> {
    let list = check_and_prompt_rollover()?.unwrap_or_else(|| {
        let today = Local::now().date_naive();
        todo::TodoList::new(today, utils::paths::get_daily_file_path(today).unwrap())
    });

    if list.is_empty() {
        println!("No todos for today!");
        return Ok(());
    }

    println!("\n📋 Todo List - {}\n", list.date.format("%B %d, %Y"));

    for (idx, item) in list.items.iter().enumerate() {
        let indent = "  ".repeat(item.indent_level);
        println!("{}{}. {} {}", indent, idx + 1, item.state, item.content);
    }

    println!();

    Ok(())
}
```

### Phase 12: Polish & Final Features (NOT STARTED)

**Help Overlay** - Add to `src/ui/components/mod.rs`:
```rust
fn render_help_overlay(f: &mut Frame, state: &AppState) {
    use ratatui::widgets::{Clear, Paragraph, Wrap};
    use ratatui::layout::{Alignment, Constraint, Direction, Layout};

    let help_text = r#"
    TODO-CLI Help

    Navigate Mode:
      ↑/↓ or j/k         Move cursor
      Space              Cycle state ([ ] → [x] → [?] → [!])
      Cmd/Ctrl+Shift+↑/↓ Move item with children
      Cmd/Ctrl+Shift+→/← Indent/outdent item
      i or Enter         Edit current item
      n                  New item below
      d                  Delete item
      ?                  Toggle help
      q                  Quit

    Edit Mode:
      Esc                Cancel edit
      Enter              Save and exit
      ←/→                Move cursor
      Backspace          Delete character
    "#;

    // Center the help popup
    let area = centered_rect(60, 60, f.area());

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Help ")
        .style(Style::default().bg(state.theme.background));

    let paragraph = Paragraph::new(help_text)
        .block(block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(Clear, area); // Clear background
    f.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
```

**Delete Confirmation** - Add to event handler:
```rust
fn delete_current_item(state: &mut AppState) -> Result<()> {
    if state.todo_list.items.is_empty() {
        return Ok(());
    }

    // Simple implementation: just delete
    // Advanced: show confirmation dialog
    state.todo_list.delete_item(state.cursor_position)?;

    // Adjust cursor if needed
    if state.cursor_position >= state.todo_list.items.len() && state.cursor_position > 0 {
        state.cursor_position -= 1;
    }

    state.unsaved_changes = true;
    Ok(())
}
```

**Error Handling:**
- Graceful degradation for file permission errors
- Invalid markdown recovery (skip bad lines, continue parsing)
- Terminal resize handling (Ratatui handles this automatically)

**Cross-Platform Testing:**
- Test Cmd vs Ctrl modifiers on macOS and Linux
- Verify keyboard shortcuts work in different terminals
- Test on Windows (may need WSL)

---

## 📊 ARCHITECTURE DECISIONS SUMMARY

### Design Pattern
✅ **Modified Elm Architecture (TEA)**
- Model: `AppState` holds all application state
- Update: Event handlers transform state
- View: Rendering logic draws from state
- Clean separation of concerns, easy to test

### Data Structure
✅ **Flat list with indent levels** (not tree)
- Simpler to implement and serialize
- Easier to navigate with keyboard
- Sufficient for most use cases
- Can refactor to tree later if needed

### File I/O Strategy
✅ **Atomic writes** (write to .tmp → rename)
- Prevents data corruption
- Safe even if app crashes mid-write

### Save Strategy
✅ **Auto-save on every change**
- Never lose work
- Seamless experience
- Good for markdown files (small, fast)

### Text Editing
✅ **Simple single-line input** (phase 1)
- Faster to implement
- Covers 90% of use cases
- Can upgrade to `tui-textarea` later if needed

### Move Behavior
✅ **Move item WITH all children**
- More intuitive for users
- Requires recursive range calculation
- Implementation in `src/todo/hierarchy.rs`

### Keyboard Shortcuts
✅ **Support both Cmd (macOS) and Ctrl (Windows/Linux)**
- `KeyModifiers::SUPER | KeyModifiers::CONTROL`
- Better cross-platform compatibility

### State Management
✅ **Synchronous event loop** (no async needed)
- Simpler code
- Sufficient for file I/O performance
- Event polling with 100ms timeout

---

## 🧪 TESTING STRATEGY

### Unit Tests (Already Implemented)
- ✅ `src/todo/state.rs` - State transitions
- ✅ `src/todo/item.rs` - Item operations
- ✅ `src/todo/list.rs` - List operations
- ✅ `src/storage/markdown.rs` - Serialization round-trips

### Integration Tests (TODO)
- [ ] Rollover scenarios (yesterday → today)
- [ ] Multi-day file handling
- [ ] Concurrent file access (warn if file modified externally)

### Manual Testing Checklist (TODO)
- [ ] macOS: Cmd+Shift+Arrow works
- [ ] Linux: Ctrl+Shift+Arrow works
- [ ] Windows: Ctrl+Shift+Arrow works
- [ ] Terminal resize doesn't crash
- [ ] Help overlay displays correctly
- [ ] Edit mode cursor visible
- [ ] State cycling works (all 4 states)
- [ ] Indent/outdent validation
- [ ] Move with children preserves hierarchy
- [ ] Auto-save indicator updates
- [ ] Obsidian can read generated files
- [ ] Markdown files are human-readable

---

## 🚀 NEXT STEPS

### Immediate (Phase 6-8)
1. **Theme System** - `src/ui/theme.rs`
2. **App State** - `src/app/{mode.rs,state.rs}`
3. **Event Handling** - `src/app/event.rs`

### Short-term (Phase 9-10)
4. **Hierarchy Operations** - `src/todo/hierarchy.rs`
5. **UI Components** - `src/ui/components/`

### Final (Phase 11-12)
6. **TUI Integration** - Update `src/main.rs`
7. **Polish** - Help, errors, testing

### Estimated Completion Time
- Phase 6-8: 2-3 hours
- Phase 9-10: 3-4 hours
- Phase 11-12: 2-3 hours
- **Total: 7-10 hours** of focused work

---

## 📝 DEVELOPMENT NOTES

### Current State
- **Working**: CLI commands (`todo add`, `todo show`)
- **Working**: Markdown serialization (Obsidian-compatible)
- **Working**: Daily rollover with prompt
- **Not Started**: TUI interface

### Files Modified from Template
- `Cargo.toml` - Dependencies added
- `src/main.rs` - CLI routing implemented

### Dependencies Rationale
- `ratatui` - Modern TUI framework (actively maintained fork of tui-rs)
- `crossterm` - Cross-platform terminal backend
- `clap` - Best-in-class CLI parsing
- `chrono` - Date handling for daily files
- `serde + toml` - Config file handling
- `pulldown-cmark` - Markdown parsing (though mostly custom for checkboxes)
- `uuid` - Unique item IDs (for future features)
- `anyhow` - Ergonomic error handling
- `dirs` - Home directory resolution

### Performance Considerations
- Files are small (typical: <100 items)
- No need for database or indexing
- Atomic writes are fast enough
- Virtual scrolling not needed until 1000+ items

### Future Enhancements (Post-v1.0)
- [ ] Search/filter functionality
- [ ] Tags support (#work, #personal)
- [ ] Recurring tasks
- [ ] Time tracking per task
- [ ] Multi-file projects (beyond dailies)
- [ ] Git auto-commit integration
- [ ] Statistics view (completion rates)
- [ ] Undo/redo stack
- [ ] Vi-style command mode (`:w`, `:q`)
- [ ] Export to other formats (PDF, HTML)
- [ ] Archive old completed items
- [ ] Sync to remote server
- [ ] Mobile companion app

---

## 🔗 RESOURCES

### Documentation
- [Ratatui Book](https://ratatui.rs/)
- [Crossterm Docs](https://docs.rs/crossterm)
- [Clap Docs](https://docs.rs/clap)
- [Obsidian Markdown Spec](https://help.obsidian.md/Editing+and+formatting/Basic+formatting+syntax)

### Example Projects
- [Ratatui Examples](https://github.com/ratatui/ratatui/tree/main/examples)
- [Ratatui Async Template](https://github.com/ratatui/templates)

### Key Ratatui Concepts
- **Immediate Mode**: Redraw entire UI every frame
- **Layout System**: Constraints-based (flex-like)
- **Widgets**: Composable UI components
- **Styles**: Colors, modifiers (bold, italic, etc.)
- **Events**: Keyboard, mouse (crossterm)

---

## 📂 FILE TREE (Complete Project)

```
todo-cli/
├── Cargo.toml
├── IMPLEMENTATION_PLAN.md (this file)
├── README.md (TODO: create)
├── src/
│   ├── main.rs ✅ (needs TUI integration)
│   ├── cli.rs ✅
│   ├── config.rs ✅
│   │
│   ├── app/
│   │   ├── mod.rs ❌
│   │   ├── mode.rs ❌
│   │   ├── state.rs ❌
│   │   └── event.rs ❌
│   │
│   ├── todo/
│   │   ├── mod.rs ✅
│   │   ├── state.rs ✅
│   │   ├── item.rs ✅
│   │   ├── list.rs ✅
│   │   └── hierarchy.rs ❌
│   │
│   ├── storage/
│   │   ├── mod.rs ✅
│   │   ├── markdown.rs ✅
│   │   ├── file.rs ✅
│   │   └── rollover.rs ✅
│   │
│   ├── ui/
│   │   ├── mod.rs ❌
│   │   ├── theme.rs ❌
│   │   └── components/
│   │       ├── mod.rs ❌
│   │       ├── todo_item.rs ❌
│   │       ├── todo_list.rs ❌
│   │       └── status_bar.rs ❌
│   │
│   └── utils/
│       ├── mod.rs ✅
│       └── paths.rs ✅
│
└── tests/ (TODO: integration tests)
    └── integration/
        ├── cli_tests.rs
        └── rollover_tests.rs
```

**Legend:**
- ✅ Completed and tested
- ❌ Not started
- 📝 Partially implemented

---

## 🎯 SUCCESS CRITERIA

The project is complete when:

1. ✅ **CLI Works**
   - `todo add "task"` creates markdown file
   - `todo show` displays list
   - Rollover prompt appears on new day

2. ❌ **TUI Works**
   - Launch with `todo` (no args)
   - Navigate with arrow keys
   - Toggle state with Space
   - Move items with Cmd/Ctrl+Shift+Arrow
   - Indent/outdent with Cmd/Ctrl+Shift+Left/Right
   - Edit mode for creating/editing items
   - Help overlay with `?`
   - Quit with `q`

3. ❌ **Data Integrity**
   - Auto-save works
   - No data loss on crash (atomic writes)
   - Markdown files readable in Obsidian
   - Round-trip: File → App → File preserves data

4. ❌ **Cross-Platform**
   - Works on macOS, Linux, Windows
   - Keyboard shortcuts work on all platforms

5. ❌ **Polish**
   - Clean UI with colors
   - Status bar shows mode and stats
   - Error messages are helpful
   - README with screenshots

---

## 💡 TIPS FOR CONTINUATION

### Starting Fresh
```bash
cd /Users/gimmi/Documents/Sources/rust/todo-cli
cargo build
cargo run -- show  # Test CLI
```

### Testing
```bash
# Test CLI
cargo run -- add "Test task"
cargo run -- show
cat ~/.todo-cli/dailies/$(date +%Y-%m-%d).md

# Run unit tests
cargo test

# Run with warnings
cargo build 2>&1 | grep warning
```

### Debugging
```bash
# Enable Rust backtrace
RUST_BACKTRACE=1 cargo run

# Watch for changes (install cargo-watch)
cargo watch -x run
```

### Next File to Create
Start with **`src/ui/theme.rs`** - it's self-contained and needed by all UI components.

---

**Author**: Claude (Sonnet 4.5)
**Date**: December 31, 2025
**Status**: 58% complete (CLI done, TUI remaining)
**Version**: 0.1.0
