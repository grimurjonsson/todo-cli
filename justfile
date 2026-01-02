default:
    @just --list

build:
    cargo build --release

test:
    cargo test

start-mcp-server:
    cargo run --release --bin todo-mcp

start-mcp-server-debug:
    RUST_LOG=debug cargo run --bin todo-mcp

start-api-server port="3000":
    cargo run --release -- serve start --port {{port}} --daemon

stop-api-server:
    cargo run -- serve stop

api-status:
    cargo run -- serve status

inspect-mcp:
    npx @modelcontextprotocol/inspector cargo run --release --bin todo-mcp

tui:
    cargo run --release

configure-mcp-opencode:
    #!/usr/bin/env bash
    set -euo pipefail
    
    # Build release binary first
    cargo build --release --bin todo-mcp
    
    BINARY_PATH="$(pwd)/target/release/todo-mcp"
    CONFIG_DIR="$HOME/.config/opencode"
    CONFIG_FILE="$CONFIG_DIR/opencode.json"
    
    # Ensure config directory exists
    mkdir -p "$CONFIG_DIR"
    
    # MCP server config to add
    MCP_CONFIG=$(cat <<EOF
    {
      "type": "local",
      "command": ["$BINARY_PATH"],
      "enabled": true
    }
    EOF
    )
    
    if [ -f "$CONFIG_FILE" ]; then
        # File exists - merge with existing config
        if jq -e '.mcp' "$CONFIG_FILE" > /dev/null 2>&1; then
            # mcp section exists - add/update todo-mcp entry
            jq --argjson mcp "$MCP_CONFIG" '.mcp["todo-mcp"] = $mcp' "$CONFIG_FILE" > "$CONFIG_FILE.tmp"
        else
            # no mcp section - add it
            jq --argjson mcp "$MCP_CONFIG" '. + {mcp: {"todo-mcp": $mcp}}' "$CONFIG_FILE" > "$CONFIG_FILE.tmp"
        fi
        mv "$CONFIG_FILE.tmp" "$CONFIG_FILE"
        echo "✓ Updated $CONFIG_FILE with todo-mcp server"
    else
        # Create new config file
        cat > "$CONFIG_FILE" <<EOF
    {
      "\$schema": "https://opencode.ai/config.json",
      "mcp": {
        "todo-mcp": $MCP_CONFIG
      }
    }
    EOF
        echo "✓ Created $CONFIG_FILE with todo-mcp server"
    fi
    
    echo ""
    echo "MCP server configured:"
    echo "  Binary: $BINARY_PATH"
    echo ""
    echo "Restart OpenCode to load the new MCP server."

remove-mcp-opencode:
    #!/usr/bin/env bash
    set -euo pipefail
    
    CONFIG_FILE="$HOME/.config/opencode/opencode.json"
    
    if [ -f "$CONFIG_FILE" ] && jq -e '.mcp["todo-mcp"]' "$CONFIG_FILE" > /dev/null 2>&1; then
        jq 'del(.mcp["todo-mcp"])' "$CONFIG_FILE" > "$CONFIG_FILE.tmp"
        mv "$CONFIG_FILE.tmp" "$CONFIG_FILE"
        echo "✓ Removed todo-mcp from $CONFIG_FILE"
    else
        echo "todo-mcp not found in OpenCode config"
    fi

install-claude-skill:
    #!/usr/bin/env bash
    set -euo pipefail
    
    SKILL_NAME="todo-mcp"
    SOURCE_DIR="$(pwd)/skills/$SKILL_NAME"
    TARGET_DIR="$HOME/.claude/skills/$SKILL_NAME"
    
    if [ ! -d "$SOURCE_DIR" ]; then
        echo "Error: Source skill directory not found: $SOURCE_DIR"
        exit 1
    fi
    
    mkdir -p "$TARGET_DIR"
    cp -r "$SOURCE_DIR"/* "$TARGET_DIR/"
    
    echo "✓ Installed $SKILL_NAME skill to $TARGET_DIR"

install-opencode-skill:
    #!/usr/bin/env bash
    set -euo pipefail
    
    SKILL_NAME="todo-mcp"
    SOURCE_DIR="$(pwd)/skills/$SKILL_NAME"
    TARGET_DIR="$HOME/.config/opencode/skill/$SKILL_NAME"
    
    if [ ! -d "$SOURCE_DIR" ]; then
        echo "Error: Source skill directory not found: $SOURCE_DIR"
        exit 1
    fi
    
    mkdir -p "$TARGET_DIR"
    cp -r "$SOURCE_DIR"/* "$TARGET_DIR/"
    
    echo "✓ Installed $SKILL_NAME skill to $TARGET_DIR"
