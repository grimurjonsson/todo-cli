use super::state::AppState;
use super::mode::Mode;
use crate::storage::save_todo_list;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_event(key: KeyEvent, state: &mut AppState) -> Result<()> {
    match state.mode {
        Mode::Navigate => handle_navigate_mode(key, state)?,
        Mode::Edit => handle_edit_mode(key, state)?,
    }
    Ok(())
}

fn handle_navigate_mode(key: KeyEvent, state: &mut AppState) -> Result<()> {
    match (key.code, key.modifiers) {
        // Move item up/down with children (Alt/Option+Shift+Arrows) - MUST come before plain navigation
        (KeyCode::Up, mods) if mods.intersects(KeyModifiers::SHIFT) &&
                                mods.intersects(KeyModifiers::ALT) => {
            if let Err(_) = state.todo_list.move_item_with_children_up(state.cursor_position) {
                // Silently ignore errors (item at top, etc.)
            } else {
                state.cursor_position = state.cursor_position.saturating_sub(1);
                state.unsaved_changes = true;
            }
        }
        (KeyCode::Down, mods) if mods.intersects(KeyModifiers::SHIFT) &&
                                  mods.intersects(KeyModifiers::ALT) => {
            let old_pos = state.cursor_position;
            if let Err(_) = state.todo_list.move_item_with_children_down(state.cursor_position) {
                // Silently ignore errors
            } else {
                state.cursor_position = (old_pos + 1).min(state.todo_list.items.len().saturating_sub(1));
                state.unsaved_changes = true;
            }
        }

        // Navigation (plain arrows)
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

        // Indent/outdent WITH children using Alt/Option+Shift+Left/Right
        (KeyCode::Right, mods) if mods.intersects(KeyModifiers::SHIFT) &&
                                   mods.intersects(KeyModifiers::ALT) => {
            if let Err(_) = state.todo_list.indent_item_with_children(state.cursor_position) {
                // Silently ignore errors
            } else {
                state.unsaved_changes = true;
            }
        }
        (KeyCode::Left, mods) if mods.intersects(KeyModifiers::SHIFT) &&
                                  mods.intersects(KeyModifiers::ALT) => {
            if let Err(_) = state.todo_list.outdent_item_with_children(state.cursor_position) {
                // Silently ignore errors
            } else {
                state.unsaved_changes = true;
            }
        }

        // Indent/outdent single item (WITHOUT children) with Tab/Shift+Tab
        (KeyCode::Tab, KeyModifiers::NONE) => {
            if let Err(_) = state.todo_list.indent_item(state.cursor_position) {
                // Silently ignore errors
            } else {
                state.unsaved_changes = true;
            }
        }
        (KeyCode::BackTab, _) => {
            // BackTab is sent when Shift+Tab is pressed
            if let Err(_) = state.todo_list.outdent_item(state.cursor_position) {
                // Silently ignore errors
            } else {
                state.unsaved_changes = true;
            }
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

    // Auto-save on changes
    if state.unsaved_changes {
        save_todo_list(&state.todo_list)?;
        state.unsaved_changes = false;
        state.last_save_time = Some(std::time::Instant::now());
    }

    Ok(())
}

fn handle_edit_mode(key: KeyEvent, state: &mut AppState) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            // Cancel edit
            state.mode = Mode::Navigate;
            state.edit_buffer.clear();
            state.edit_cursor_pos = 0;
            state.is_creating_new_item = false;
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
        KeyCode::Home => {
            state.edit_cursor_pos = 0;
        }
        KeyCode::End => {
            state.edit_cursor_pos = state.edit_buffer.len();
        }
        KeyCode::Char(c) => {
            state.edit_buffer.insert(state.edit_cursor_pos, c);
            state.edit_cursor_pos += 1;
        }
        _ => {}
    }
    Ok(())
}

fn enter_edit_mode(state: &mut AppState) {
    if let Some(item) = state.selected_item() {
        state.edit_buffer = item.content.clone();
        state.edit_cursor_pos = state.edit_buffer.len();
        state.mode = Mode::Edit;
        state.is_creating_new_item = false;
    }
}

fn new_item_below(state: &mut AppState) {
    state.edit_buffer.clear();
    state.edit_cursor_pos = 0;
    state.mode = Mode::Edit;
    state.is_creating_new_item = true;
}

fn delete_current_item(state: &mut AppState) -> Result<()> {
    if state.todo_list.items.is_empty() {
        return Ok(());
    }

    state.todo_list.delete_item(state.cursor_position)?;
    state.clamp_cursor();
    state.unsaved_changes = true;
    Ok(())
}

fn save_edit_buffer(state: &mut AppState) -> Result<()> {
    if state.edit_buffer.trim().is_empty() {
        state.edit_buffer.clear();
        state.edit_cursor_pos = 0;
        state.is_creating_new_item = false;
        return Ok(());
    }

    if state.is_creating_new_item {
        // Creating new item below current position
        if state.todo_list.items.is_empty() {
            // List is empty, add first item
            state.todo_list.add_item(state.edit_buffer.clone());
            state.cursor_position = 0;
        } else {
            // Insert below current item with same indent level
            let indent_level = state.todo_list.items[state.cursor_position].indent_level;
            let insert_position = state.cursor_position + 1;
            state.todo_list.insert_item(insert_position, state.edit_buffer.clone(), indent_level)?;
            state.cursor_position = insert_position;
        }
        state.is_creating_new_item = false;
    } else {
        // Editing existing item
        if state.cursor_position < state.todo_list.items.len() {
            state.todo_list.items[state.cursor_position].content = state.edit_buffer.clone();
        } else {
            // Fallback: adding new item at end
            state.todo_list.add_item_with_indent(state.edit_buffer.clone(), 0);
            state.cursor_position = state.todo_list.items.len() - 1;
        }
    }

    state.edit_buffer.clear();
    state.edit_cursor_pos = 0;
    state.unsaved_changes = true;

    Ok(())
}
