use serde::{Deserialize, Serialize};

/// Represent Application Resources
///
/// Applications (or "apps") are containers for developer platform features, and can be installed to Discord servers and/or user accounts.
///
/// More: https://discord.com/developers/docs/resources/application#application-object
///
/// - TODO: add full
#[derive(Serialize, Deserialize, Debug)]
pub struct Application {
    /// ID of the app
    pub id: String,

    /// Name of the app
    pub name: String,

    /// icon hash of the app
    pub icon: Option<String>,

    /// description on the app
    pub description: String,
}
