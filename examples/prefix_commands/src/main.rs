use async_trait::async_trait;
use rustycord::{
    bot::BotBase,
    client::Client,
    gateway::intents,
    handlers::message_handler::MessageHandler,
    logger,
    message::ChannelMessage,
    prefix::{EchoPrefixCommand, HelpCommand, PingCommand, PrefixCommand, PrefixListener},
};
use std::sync::Arc;

/// Custom command that shows server info
struct InfoCommand;

#[async_trait]
impl PrefixCommand for InfoCommand {
    async fn execute(
        &self,
        message: &ChannelMessage,
        _args: Vec<&str>,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        let response = format!(
            "**Server Information**\n\
             Channel ID: {}\n\
             Your ID: {}",
            message.channel_id, message.author.id
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

/// Custom command that calculates math expressions (simple)
struct MathCommand;

#[async_trait]
impl PrefixCommand for MathCommand {
    async fn execute(
        &self,
        _message: &ChannelMessage,
        args: Vec<&str>,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        if args.len() < 3 {
            return Ok(Some(
                "Usage: `!math <number> <operator> <number>`\nExample: `!math 5 + 3`".to_string(),
            ));
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
            _ => {
                return Ok(Some(
                    "Unknown operator. Supported: +, -, *, /, %, ^".to_string(),
                ))
            }
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

/// Custom command that shows user info
struct UserCommand;

#[async_trait]
impl PrefixCommand for UserCommand {
    async fn execute(
        &self,
        message: &ChannelMessage,
        args: Vec<&str>,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        let user = if args.is_empty() {
            &message.author
        } else {
            // In a real implementation, you'd parse the mention or username
            // For this example, we'll just show the message author
            &message.author
        };

        let response = format!(
            "**User Information**\n\
             Username: {}#{}\n\
             ID: {}\n\
             Bot: {}\n\
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

/// Message handler that uses the prefix listener
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
    async fn on_message_create(
        &self,
        message: &ChannelMessage,
        client: &Client,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Skip messages from bots
        if message.author.bot.unwrap_or(false) {
            return Ok(());
        }

        // Try to handle the message with the prefix listener
        if let Some(response) = self.listener.handle_message(message).await? {
            client
                .send_text_message(&message.channel_id, &response)
                .await?;
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize logging
    logger::setup_logger("info".to_string())?;

    // Get bot token
    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN environment variable not set");

    println!("🚀 Starting rustycord Prefix Commands Example...");

    // Create bot instance with proper intents for message handling
    let intents = intents::ALL_INTENTS;
    let mut bot = BotBase::new(Some(intents)).await;

    // Login to Discord
    let user_info = bot.login(token.clone()).await;
    println!("🔑 Logged in as: {}", user_info.username);

    // Create prefix listener with "!" prefix
    let listener = Arc::new(PrefixListener::new("!"));

    // Register built-in commands
    println!("📝 Registering built-in commands...");
    listener
        .register_command("help", Box::new(HelpCommand::new(listener.clone())))
        .await;
    listener
        .register_command("ping", Box::new(PingCommand))
        .await;
    listener
        .register_command("echo", Box::new(EchoPrefixCommand))
        .await;

    // Register custom commands
    println!("🔧 Registering custom commands...");
    listener
        .register_command("info", Box::new(InfoCommand))
        .await;
    listener
        .register_command("math", Box::new(MathCommand))
        .await;
    listener
        .register_command("user", Box::new(UserCommand))
        .await;

    // Register the prefix message handler with the event dispatcher
    if let Some(client) = &bot.client {
        let event_dispatcher = client.get_event_dispatcher();
        let message_handlers = event_dispatcher.get_message_handlers();

        message_handlers
            .add_handler(PrefixMessageHandler::new(listener.clone()))
            .await;

        println!("✅ Prefix message handler registered!");
    }

    println!("✅ Bot setup complete! Available commands:");
    let commands = listener.list_commands().await;
    for command in commands {
        if let Some(help) = listener.get_command_help(&command).await {
            println!("  • !{}: {}", command, help);
        }
    }

    println!("\n🤖 Bot is now running! Try these commands in Discord:");
    println!("  !help - Show all available commands");
    println!("  !ping - Test bot responsiveness");
    println!("  !echo <text> - Echo back your text");
    println!("  !info - Show server information");
    println!("  !math 5 + 3 - Perform calculations");
    println!("  !user - Show your user information");

    // Connect to Discord Gateway
    bot.connect(bot.intents, Some(true)).await;

    // Keep the bot running
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
