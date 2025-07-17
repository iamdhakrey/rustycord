# Prefix Commands Example

This example demonstrates how to use RustCord's prefix command system to create a bot with multiple commands.

## Features

- **Prefix-based commands** with "!" prefix
- **Built-in commands**: help, ping, echo
- **Custom commands**: info, math, user
- **Automatic help generation**
- **Error handling and logging**

## Setup

1. **Copy environment file**:
   ```bash
   cp .env.example .env
   ```

2. **Add your Discord bot token** to `.env`:
   ```env
   DISCORD_TOKEN=your_bot_token_here
   ```

3. **Ensure your bot has the Message Content Intent enabled** in the Discord Developer Portal:
   - Go to your application in the Discord Developer Portal
   - Navigate to the "Bot" section
   - Enable "Message Content Intent" under "Privileged Gateway Intents"

4. **Run the bot**:
   ```bash
   cargo run
   ```

## Available Commands

| Command | Description | Example |
|---------|-------------|---------|
| `!help` | Show all available commands | `!help` |
| `!help <command>` | Get help for a specific command | `!help math` |
| `!ping` | Test bot responsiveness | `!ping` |
| `!echo <text>` | Echo back the provided text | `!echo Hello World` |
| `!info` | Show server and channel information | `!info` |
| `!math <num> <op> <num>` | Perform basic calculations | `!math 5 + 3` |
| `!user` | Show your user information | `!user` |

## Math Command Examples

The math command supports these operators:
- `!math 10 + 5` → `10 + 5 = 15`
- `!math 20 - 7` → `20 - 7 = 13`
- `!math 6 * 4` → `6 * 4 = 24`
- `!math 15 / 3` → `15 / 3 = 5`
- `!math 17 % 5` → `17 % 5 = 2`
- `!math 2 ^ 3` → `2 ^ 3 = 8`

## Code Structure

- **`InfoCommand`** - Shows server/channel information
- **`MathCommand`** - Performs basic arithmetic calculations
- **`UserCommand`** - Displays user information
- **`PrefixMessageHandler`** - Integrates prefix commands with the message handling system

## Troubleshooting

### Commands not responding
1. **Check Message Content Intent**: Ensure it's enabled in Discord Developer Portal
2. **Verify bot permissions**: Bot needs "Send Messages" and "Read Message History" permissions
3. **Check logs**: Run with `RUST_LOG=debug cargo run` for detailed logging

### "Invalid Token" error
1. **Verify token**: Make sure your `.env` file contains the correct bot token
2. **No extra spaces**: Ensure there are no trailing spaces in the `.env` file

### Bot connects but doesn't see messages
1. **Message Content Intent**: This is the most common issue - enable it in Discord Developer Portal
2. **Channel permissions**: Ensure the bot can read messages in the channel you're testing

## Example Output

When running successfully:
```
🚀 Starting RustCord Prefix Commands Example...
🔑 Logged in as: YourBot#1234
📝 Registering built-in commands...
🔧 Registering custom commands...
✅ Prefix message handler registered!
✅ Bot setup complete! Available commands:
  • !help: Show available commands or get help for a specific command
  • !ping: Test if the bot is responding
  • !echo: Echo back the provided text
  • !info: Show information about the current server and channel
  • !math: Perform basic math calculations
  • !user: Show information about a user (defaults to yourself)

🤖 Bot is now running! Try these commands in Discord:
  !help - Show all available commands
  !ping - Test bot responsiveness
  !echo <text> - Echo back your text
  !info - Show server information
  !math 5 + 3 - Perform calculations
  !user - Show your user information
```

## Extending the Example

To add your own commands:

1. **Create a command struct**:
   ```rust
   struct MyCommand;
   
   #[async_trait]
   impl PrefixCommand for MyCommand {
       async fn execute(&self, message: &ChannelMessage, args: Vec<&str>) 
           -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> 
       {
           Ok(Some("My custom response!".to_string()))
       }
       
       fn description(&self) -> &str {
           "My custom command description"
       }
   }
   ```

2. **Register the command**:
   ```rust
   listener.register_command("mycommand", Box::new(MyCommand)).await;
   ```

For more advanced usage, see the [prefix commands documentation](../../docs/user-guide/prefix-commands.md).
