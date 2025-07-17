# Prefix Commands

rustycord provides a powerful prefix-based command system that allows you to easily create and manage bot commands with specific prefixes (like `!`, `?`, or custom prefixes).

## Overview

The prefix command system consists of:

- **`PrefixListener`** - Manages commands for a specific prefix
- **`PrefixCommand`** trait - Define your own commands
- Built-in commands (`HelpCommand`, `PingCommand`, `EchoPrefixCommand`)
- Integration with the existing message handler system

## Basic Usage

### 1. Create a Prefix Listener

```rust
use rustycord::prefix::PrefixListener;
use std::sync::Arc;

// Create a listener for "!" prefix
let listener = Arc::new(PrefixListener::new("!"));

// For case-sensitive commands
let case_listener = Arc::new(PrefixListener::new_case_sensitive("!"));
```

### 2. Register Built-in Commands

```rust
use rustycord::prefix::{HelpCommand, PingCommand, EchoPrefixCommand};

// Register built-in commands
listener.register_command("help", Box::new(HelpCommand::new(listener.clone()))).await;
listener.register_command("ping", Box::new(PingCommand)).await;
listener.register_command("echo", Box::new(EchoPrefixCommand)).await;
```

### 3. Create Custom Commands

```rust
use async_trait::async_trait;
use rustycord::prefix::PrefixCommand;
use rustycord::message::ChannelMessage;

struct InfoCommand;

#[async_trait]
impl PrefixCommand for InfoCommand {
    async fn execute(&self, message: &ChannelMessage, _args: Vec<&str>) 
        -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> 
    {
        let response = format!(
            "**Server Information**\\n\\
             Channel ID: {}\\n\\
             Your ID: {}",
            message.channel_id,
            message.author.id
        );
        Ok(Some(response))
    }
    
    fn description(&self) -> &str {
        "Show information about the current server and channel"
    }
    
    fn aliases(&self) -> Vec<&str> {
        vec!["server", "guild"]
    }
}

// Register the custom command
listener.register_command("info", Box::new(InfoCommand)).await;
```

### 4. Integrate with Message Handler

```rust
use rustycord::handlers::message_handler::MessageHandler;

struct PrefixMessageHandler {
    listener: Arc<PrefixListener>,
}

impl PrefixMessageHandler {
    fn new(listener: Arc<PrefixListener>) -> Self {
        Self { listener }
    }
}

#[async_trait]
impl MessageHandler for PrefixMessageHandler {
    async fn on_message_create(&self, message: &ChannelMessage, client: &Client) 
        -> Result<(), Box<dyn std::error::Error + Send + Sync>> 
    {
        // Skip messages from bots
        if message.author.bot.unwrap_or(false) {
            return Ok(());
        }
        
        // Try to handle the message with the prefix listener
        if let Some(response) = self.listener.handle_message(message).await? {
            client.send_text_message(&message.channel_id, &response).await?;
        }
        
        Ok(())
    }
}
```

### 5. Register with Event Dispatcher

```rust
// Register with the bot's event dispatcher
if let Some(client) = &bot.client {
    let event_dispatcher = client.get_event_dispatcher();
    let message_handlers = event_dispatcher.get_message_handlers();
    
    message_handlers.add_handler(PrefixMessageHandler::new(listener.clone())).await;
}
```

## Built-in Commands

### Help Command

The `HelpCommand` automatically lists all registered commands and provides detailed help.

```rust
// Usage in Discord:
// !help          - Lists all commands
// !help <command> - Shows help for specific command
```

### Ping Command

A simple responsiveness test command.

```rust
// Usage: !ping
// Response: "Pong! 🏓"
```

### Echo Command

Echoes back the provided text.

```rust
// Usage: !echo Hello World
// Response: "Hello World"
```

## Advanced Examples

### Math Calculator Command

```rust
struct MathCommand;

#[async_trait]
impl PrefixCommand for MathCommand {
    async fn execute(&self, _message: &ChannelMessage, args: Vec<&str>) 
        -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> 
    {
        if args.len() < 3 {
            return Ok(Some("Usage: `!math <number> <operator> <number>`\\nExample: `!math 5 + 3`".to_string()));
        }
        
        let num1: f64 = match args[0].parse() {
            Ok(n) => n,
            Err(_) => return Ok(Some("Invalid first number".to_string())),
        };
        
        let operator = args[1];
        
        let num2: f64 = match args[2].parse() {
            Ok(n) => n,
            Err(_) => return Ok(Some("Invalid second number".to_string())),
        };
        
        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" | "x" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    return Ok(Some("Cannot divide by zero!".to_string()));
                }
                num1 / num2
            }
            "%" => num1 % num2,
            "^" | "**" => num1.powf(num2),
            _ => return Ok(Some("Unknown operator. Supported: +, -, *, /, %, ^".to_string())),
        };
        
        Ok(Some(format!("{} {} {} = {}", num1, operator, num2, result)))
    }
    
    fn description(&self) -> &str {
        "Perform basic math calculations"
    }
    
    fn aliases(&self) -> Vec<&str> {
        vec!["calc", "calculate"]
    }
}
```

### User Information Command

```rust
struct UserCommand;

#[async_trait]
impl PrefixCommand for UserCommand {
    async fn execute(&self, message: &ChannelMessage, _args: Vec<&str>) 
        -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> 
    {
        let user = &message.author;
        
        let response = format!(
            "**User Information**\\n\\
             Username: {}#{}\\n\\
             ID: {}\\n\\
             Bot: {}\\n\\
             MFA Enabled: {}",
            user.name,
            user.discriminator,
            user.id,
            user.bot.unwrap_or(false),
            user.mfa_enabled
        );
        
        Ok(Some(response))
    }
    
    fn description(&self) -> &str {
        "Show information about a user (defaults to yourself)"
    }
    
    fn aliases(&self) -> Vec<&str> {
        vec!["whois", "profile"]
    }
}
```

## Complete Example

See the [prefix commands example](../examples/prefix-commands.md) for a complete working bot that demonstrates all these features.

## API Reference

### PrefixListener

- `new(prefix: &str)` - Create a new case-insensitive prefix listener
- `new_case_sensitive(prefix: &str)` - Create a case-sensitive prefix listener
- `register_command(name: &str, command: Box<dyn PrefixCommand>)` - Register a command
- `unregister_command(name: &str)` - Remove a command
- `handle_message(message: &ChannelMessage)` - Process a message for commands
- `list_commands()` - Get all registered command names
- `get_command_help(command_name: &str)` - Get help for a specific command
- `prefix()` - Get the prefix being used

### PrefixCommand Trait

- `execute(message: &ChannelMessage, args: Vec<&str>)` - Execute the command
- `description()` - Get command description for help
- `aliases()` - Get command aliases (optional)

## Best Practices

1. **Use descriptive command names** - Make commands easy to discover
2. **Provide good help text** - Users should understand what commands do
3. **Handle errors gracefully** - Return helpful error messages
4. **Use aliases for common commands** - Make frequently used commands easy to type
5. **Skip bot messages** - Always check `message.author.bot` to avoid infinite loops
6. **Validate arguments** - Check argument count and types before processing
7. **Use case-insensitive commands** - Unless you specifically need case sensitivity

## Error Handling

Commands should return `Result<Option<String>, Box<dyn std::error::Error + Send + Sync>>`:

- `Ok(Some(response))` - Command executed successfully with a response
- `Ok(None)` - Command executed successfully but no response needed
- `Err(error)` - Command failed with an error

The prefix system will log errors and continue processing other messages.

## Limitations

- Commands are processed sequentially, not in parallel
- Aliases are logged but not currently stored (feature limitation)
- No built-in cooldown or rate limiting (implement in your command)
- No built-in permission system (implement in your command)

## Migration from Simple Message Handlers

If you're currently using simple message handlers with manual prefix checking:

```rust
// Old approach
if message.content.starts_with("!ping") {
    client.send_text_message(&message.channel_id, "Pong!").await?;
}

// New approach with prefix system
struct PingCommand;

#[async_trait]
impl PrefixCommand for PingCommand {
    async fn execute(&self, _message: &ChannelMessage, _args: Vec<&str>) 
        -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> 
    {
        Ok(Some("Pong!".to_string()))
    }
    
    fn description(&self) -> &str {
        "Test if the bot is responding"
    }
}
```

The prefix system provides better organization, automatic help generation, and easier command management.
