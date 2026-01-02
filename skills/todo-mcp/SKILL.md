---
name: todo-mcp
description: Interact with the todo-mcp server to list, create, update, complete, and delete todos with hierarchical nesting support. Use when user asks about their todos, task management, daily planning, or mentions "my todos".
---

<objective>
Manage todos via the todo-mcp server. Supports hierarchical todos (parent/child relationships), due dates, descriptions, and multiple states.
</objective>

<quick_start>
<list_todos>
**Tool**: `todo-mcp_list_todos`

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `date` | string | No | Date in YYYY-MM-DD format. Defaults to today. |

**CRITICAL**: The response contains a `formatted` field with pre-formatted markdown.

**MANDATORY**: Display the COMPLETE `formatted` field. NEVER truncate, summarize, or hide items.

Wrap in a code block to preserve formatting:

~~~
```
## Todos for 2026-01-02 (5/10)

⬜ Task one
✅ Completed task
  ⬜ Subtask
```
~~~

**FORBIDDEN**: 
- "(+ N more completed)" 
- "X completed, Y remaining"
- Hiding/collapsing completed items
- Any form of summarization

Automatically rolls over incomplete todos from previous days if today's list is empty.
</list_todos>

<create_todo>
**Tool**: `todo-mcp_create_todo`

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `content` | string | Yes | The todo text. Cannot be empty. |
| `description` | string | No | Additional notes or details. |
| `due_date` | string | No | Due date in YYYY-MM-DD format. |
| `parent_id` | string | No | UUID of parent todo to nest under. Get IDs from `list_todos`. |
| `date` | string | No | Which day's list to add to. Defaults to today. |

Example - create nested todo with due date:
```
content: "Review PR #123"
description: "Check for security issues and test coverage"
due_date: "2026-01-05"
parent_id: "4497a476-61d0-4f13-9603-65b1eae5e37f"
```
</create_todo>

<update_todo>
**Tool**: `todo-mcp_update_todo`

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | string | Yes | UUID of the todo to update. |
| `content` | string | No | New content text. |
| `description` | string | No | New description. Empty string clears it. |
| `due_date` | string | No | New due date in YYYY-MM-DD format. |
| `state` | string | No | New state (see states below). |
| `date` | string | No | Date in YYYY-MM-DD format. Defaults to today. |

**States**:
- `' '` (space) - pending
- `'x'` - done
- `'?'` - question
- `'!'` - important
</update_todo>

<mark_complete>
**Tool**: `todo-mcp_mark_complete`

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | string | Yes | UUID of the todo to toggle. |
| `date` | string | No | Date in YYYY-MM-DD format. Defaults to today. |

Toggles completion: marks pending as done `[x]`, or done as pending `[ ]`.
</mark_complete>

<delete_todo>
**Tool**: `todo-mcp_delete_todo`

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | string | Yes | UUID of the todo to delete. |
| `date` | string | No | Date in YYYY-MM-DD format. Defaults to today. |

**Warning**: Deletes the todo AND all its children. Irreversible.
</delete_todo>
</quick_start>

<workflow>
**Common patterns**:

1. **Show todos**: Call `list_todos`, wrap `response.formatted` in a code block to preserve checkbox formatting
2. **Add subtask**: First `list_todos` to get parent UUID, then `create_todo` with `parent_id`
3. **Complete task**: Use `mark_complete` with the todo's `id`
4. **Bulk operations**: Chain multiple tool calls for efficiency
</workflow>

<anti_patterns>
**DO NOT**:
- Reformat the `formatted` field output (it's already properly formatted)
- Strip the `[ ]` or `[x]` checkbox markers
- Change indentation of nested items
- Convert to a different list format
- Display as raw markdown (brackets get stripped) - always use code block
- Truncate or summarize the list (e.g., "+ N more completed")
- Hide completed items
- Add summaries like "X completed, Y remaining"

**ALWAYS show the COMPLETE list exactly as returned. No exceptions.**
</anti_patterns>

<success_criteria>
- Todo operations complete without errors
- User sees formatted todo list when requested
- Hierarchical structure preserved (parent/child relationships)
- Due dates and descriptions displayed when present
</success_criteria>
