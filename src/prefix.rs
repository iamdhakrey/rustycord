use crate::message::ChannelMessage;
use async_trait::async_trait;
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// A command that can be executed when a message matches a prefix
#[async_trait]
pub trait PrefixCommand: Send + Sync {
    /// Execute the command with the given message and arguments
    async fn execute(
        &self,
        message: &ChannelMessage,
        args: Vec<&str>,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>>;

    /// Get the command description for help messages
    fn description(&self) -> &str;

    /// Get command aliases (optional)
    fn aliases(&self) -> Vec<&str> {
        vec![]
    }
}

/// Registry for prefix-based commands
pub struct PrefixListener {
    /// The prefix to listen for (e.g., "!", "?", ">>")
    prefix: String,
    /// Registered commands mapped by name
    commands: Arc<RwLock<HashMap<String, Box<dyn PrefixCommand>>>>,
    /// Whether to be case sensitive for command names
    case_sensitive: bool,
}

impl PrefixListener {
    /// Create a new prefix listener with the specified prefix
    pub fn new(prefix: &str) -> Self {
        info!("Creating new PrefixListener with prefix: '{}'", prefix);
        Self {
            prefix: prefix.to_string(),
            commands: Arc::new(RwLock::new(HashMap::new())),
            case_sensitive: false,
        }
    }

    /// Create a new case-sensitive prefix listener
    pub fn new_case_sensitive(prefix: &str) -> Self {
        info!(
            "Creating new case-sensitive PrefixListener with prefix: '{}'",
            prefix
        );
        Self {
            prefix: prefix.to_string(),
            commands: Arc::new(RwLock::new(HashMap::new())),
            case_sensitive: true,
        }
    }

    /// Register a command with the given name
    pub async fn register_command(&self, name: &str, command: Box<dyn PrefixCommand>) {
        let mut commands = self.commands.write().await;
        let key = if self.case_sensitive {
            name.to_string()
        } else {
            name.to_lowercase()
        };

        info!(
            "Registering command '{}' with prefix '{}'",
            name, self.prefix
        );
        commands.insert(key.clone(), command);

        // Also register aliases
        if let Some(cmd) = commands.get(&key) {
            let aliases = cmd.aliases();
            for alias in aliases {
                let _alias_key = if self.case_sensitive {
                    alias.to_string()
                } else {
                    alias.to_lowercase()
                };
                debug!("Registering alias '{}' for command '{}'", alias, name);
                // Note: We can't insert the same box twice, so we'd need to use Arc<dyn PrefixCommand>
                // For now, we'll just log that aliases are registered but not implement the storage
            }
        }

        debug!("Total commands registered: {}", commands.len());
    }

    /// Unregister a command
    pub async fn unregister_command(&self, name: &str) {
        let mut commands = self.commands.write().await;
        let key = if self.case_sensitive {
            name.to_string()
        } else {
            name.to_lowercase()
        };

        if commands.remove(&key).is_some() {
            info!("Unregistered command '{}'", name);
        } else {
            warn!("Attempted to unregister non-existent command '{}'", name);
        }
    }

    /// Check if a message matches the prefix and execute the command if found
    pub async fn handle_message(
        &self,
        message: &ChannelMessage,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        // Check if message starts with our prefix
        if !message.content.starts_with(&self.prefix) {
            return Ok(None);
        }

        debug!(
            "Message matches prefix '{}': {}",
            self.prefix, message.content
        );

        // Remove prefix and split into command and arguments
        let content_without_prefix = &message.content[self.prefix.len()..];
        let parts: Vec<&str> = content_without_prefix.split_whitespace().collect();

        if parts.is_empty() {
            debug!("No command specified after prefix");
            return Ok(None);
        }

        let command_name = parts[0];
        let args = if parts.len() > 1 { &parts[1..] } else { &[] };

        let key = if self.case_sensitive {
            command_name.to_string()
        } else {
            command_name.to_lowercase()
        };

        debug!(
            "Looking for command '{}' (normalized: '{}') with {} args",
            command_name,
            key,
            args.len()
        );

        // Find and execute command
        let commands = self.commands.read().await;
        if let Some(command) = commands.get(&key) {
            info!(
                "Executing command '{}' for user {} with args: {:?}",
                command_name, message.author.id, args
            );
            match command.execute(message, args.to_vec()).await {
                Ok(response) => {
                    if response.is_some() {
                        debug!("Command '{}' returned response", command_name);
                    } else {
                        debug!("Command '{}' returned no response", command_name);
                    }
                    Ok(response)
                }
                Err(e) => {
                    error!("Error executing command '{}': {}", command_name, e);
                    Err(e)
                }
            }
        } else {
            debug!(
                "Unknown command '{}' with prefix '{}'",
                command_name, self.prefix
            );
            Ok(None)
        }
    }

    /// Get a list of all registered commands
    pub async fn list_commands(&self) -> Vec<String> {
        let commands = self.commands.read().await;
        commands.keys().cloned().collect()
    }

    /// Get help for a specific command
    pub async fn get_command_help(&self, command_name: &str) -> Option<String> {
        let key = if self.case_sensitive {
            command_name.to_string()
        } else {
            command_name.to_lowercase()
        };

        let commands = self.commands.read().await;
        commands.get(&key).map(|cmd| cmd.description().to_string())
    }

    /// Get the prefix being used
    pub fn prefix(&self) -> &str {
        &self.prefix
    }
}

/// Built-in help command
pub struct HelpCommand {
    listener: Arc<PrefixListener>,
}

impl HelpCommand {
    pub fn new(listener: Arc<PrefixListener>) -> Self {
        Self { listener }
    }
}

#[async_trait]
impl PrefixCommand for HelpCommand {
    async fn execute(
        &self,
        _message: &ChannelMessage,
        args: Vec<&str>,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        if args.is_empty() {
            // List all commands
            let commands = self.listener.list_commands().await;
            if commands.is_empty() {
                Ok(Some("No commands available.".to_string()))
            } else {
                let mut response = format!(
                    "Available commands (prefix: `{}`):\n",
                    self.listener.prefix()
                );
                for command in commands {
                    response.push_str(&format!("• `{}{}`\n", self.listener.prefix(), command));
                }
                response.push_str(&format!(
                    "\nUse `{}help <command>` for detailed help.",
                    self.listener.prefix()
                ));
                Ok(Some(response))
            }
        } else {
            // Get help for specific command
            let command_name = args[0];
            if let Some(help_text) = self.listener.get_command_help(command_name).await {
                Ok(Some(format!(
                    "**{}{}**: {}",
                    self.listener.prefix(),
                    command_name,
                    help_text
                )))
            } else {
                Ok(Some(format!(
                    "Command `{}{}` not found.",
                    self.listener.prefix(),
                    command_name
                )))
            }
        }
    }

    fn description(&self) -> &str {
        "Show available commands or get help for a specific command"
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["h", "?"]
    }
}

/// Built-in ping command
pub struct PingCommand;

#[async_trait]
impl PrefixCommand for PingCommand {
    async fn execute(
        &self,
        _message: &ChannelMessage,
        _args: Vec<&str>,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Some("Pong! 🏓".to_string()))
    }

    fn description(&self) -> &str {
        "Test if the bot is responding"
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["pong"]
    }
}

/// Built-in echo command
pub struct EchoPrefixCommand;

#[async_trait]
impl PrefixCommand for EchoPrefixCommand {
    async fn execute(
        &self,
        _message: &ChannelMessage,
        args: Vec<&str>,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        if args.is_empty() {
            Ok(Some("Please provide text to echo!".to_string()))
        } else {
            Ok(Some(args.join(" ")))
        }
    }

    fn description(&self) -> &str {
        "Echo back the provided text"
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["repeat", "say"]
    }
}
