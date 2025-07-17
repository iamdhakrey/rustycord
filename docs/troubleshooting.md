# Troubleshooting RustCord

Common issues and solutions when using RustCord.

## Bot Not Responding to Commands

### Symptoms
- Bot connects successfully
- Bot appears online in Discord
- Commands like `!ping` or `!help` don't trigger any response
- No message events in logs

### Solution: Enable Message Content Intent

**This is the most common issue.** Discord requires explicit permission to read message content.

1. **Go to Discord Developer Portal**:
   - Visit [https://discord.com/developers/applications](https://discord.com/developers/applications)
   - Select your application

2. **Navigate to Bot section**:
   - Click "Bot" in the left sidebar

3. **Enable Message Content Intent**:
   - Scroll down to "Privileged Gateway Intents"
   - Toggle ON "Message Content Intent"
   - Save changes

4. **Update your code** to use proper intents:
   ```rust
   use rustcord::gateway::intents;
   
   // Include message-related intents
   let intents = intents::GUILDS | intents::GUILD_MESSAGES | intents::MESSAGE_CONTENT;
   let mut bot = BotBase::new(Some(intents)).await;
   ```

### Verify in Logs

Look for these log messages when your bot starts:
```
✅ Message handler registered: YourHandler (Total: 1)
🚀 Bot is ready!
🏰 Joined guild: YourServerName
```

## Authentication Issues

### Invalid Token Error
```
Error: HTTP error: 401 Unauthorized
```

**Solutions**:
1. **Check your `.env` file**:
   ```env
   DISCORD_TOKEN=your_actual_bot_token_here
   ```
2. **No extra spaces or quotes** around the token
3. **Regenerate token** if necessary in Discord Developer Portal

### Token Not Found Error
```
DISCORD_TOKEN environment variable not set
```

**Solutions**:
1. **Create `.env` file** in your project root
2. **Copy from example**: `cp .env.example .env`
3. **Add dotenv dependency** in `Cargo.toml`:
   ```toml
   [dependencies]
   dotenv = "0.15"
   ```
4. **Call dotenv in code**:
   ```rust
   dotenv::dotenv().ok();
   ```

## Permission Issues

### Missing Access Error
```
Error: Missing Access
```

**Solutions**:
1. **Re-invite your bot** with proper permissions
2. **Check bot role position** (should be above roles it needs to manage)
3. **Grant essential permissions**:
   - Send Messages
   - Read Message History
   - Read Messages

### Generate proper invite URL:
1. **Discord Developer Portal** → Your App → OAuth2 → URL Generator
2. **Select Scopes**: `bot`
3. **Select Permissions**:
   - Send Messages
   - Read Message History
   - Use Slash Commands (if needed)
4. **Use generated URL** to invite bot

## Connection Issues

### WebSocket Connection Failed
```
Error: Connection timeout
Error: WebSocket connection failed
```

**Solutions**:
1. **Check internet connection**
2. **Verify Discord status**: [https://discordstatus.com](https://discordstatus.com)
3. **Check firewall settings**
4. **Try different network** (mobile hotspot to test)

### Gateway Connection Issues
```
Failed to connect to Discord gateway
```

**Solutions**:
1. **Check bot token validity**
2. **Verify intents configuration**
3. **Check rate limits** (too many connection attempts)

## Message Handler Issues

### Handler Not Triggered
```
Handler registered but not executing
```

**Check these**:
1. **Message Content Intent** enabled
2. **Bot not responding to its own messages**:
   ```rust
   if message.author.bot.unwrap_or(false) {
       return Ok(());
   }
   ```
3. **Correct message format** (right prefix, etc.)

### Multiple Responses
```
Bot responding multiple times to same command
```

**Solutions**:
1. **Remove duplicate handlers**
2. **Check for multiple bot instances**
3. **Ensure handlers return early** when appropriate

## Logging and Debugging

### Enable Debug Logging

```rust
// In your main function
logger::setup_logger("debug".to_string())?;

// Or via environment variable
RUST_LOG=debug cargo run
```

### Check Log Files

Most examples create `output.log`:
```bash
tail -f output.log
grep -i "error" output.log
grep -i "message" output.log
```

### Common Log Messages

**Success indicators**:
```
✅ Successfully authenticated as: YourBot
🔌 Connected to The Discord
🚀 Bot is ready!
🏰 Joined guild: YourServer
```

**Problem indicators**:
```
❌ Authentication failed
❌ Failed to connect to Discord gateway
Error: 401 Unauthorized
Error: Missing Access
```

## Performance Issues

### High CPU Usage
```
Bot using too much CPU
```

**Solutions**:
1. **Check for infinite loops** in message handlers
2. **Avoid blocking operations** in async handlers
3. **Use appropriate log levels** (avoid "trace" in production)

### Memory Leaks
```
Memory usage keeps growing
```

**Solutions**:
1. **Check for unbounded collections** in handlers
2. **Properly drop resources** in async code
3. **Use weak references** where appropriate

## Development Tips

### Quick Testing
1. **Create a test server** for development
2. **Use debug logging** during development
3. **Test with simple commands first** (`!ping`)

### Code Organization
```rust
// Separate concerns
struct MyBot {
    client: Client,
    commands: PrefixListener,
}

impl MyBot {
    async fn setup(&mut self) -> Result<(), Box<dyn Error>> {
        // Setup logic here
    }
}
```

### Error Handling
```rust
// Always handle errors gracefully
match some_operation().await {
    Ok(result) => {
        log::info!("Operation successful: {:?}", result);
    }
    Err(e) => {
        log::error!("Operation failed: {}", e);
        // Don't panic, return gracefully
        return Err(e);
    }
}
```

## Getting Help

If you're still having issues:

1. **Check the examples** in the repository
2. **Review the documentation** 
3. **Search existing issues** on GitHub
4. **Create a minimal reproduction case**
5. **Include logs and error messages** when asking for help

### Useful Information to Include

When reporting issues:
- Rust version: `rustc --version`
- Operating system
- Complete error messages
- Minimal code that reproduces the issue
- Bot permissions and intents configuration
