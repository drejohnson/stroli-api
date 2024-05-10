use serde::{Deserialize, Serialize};

// Define an enum to represent the built-in OAuth provider names
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinOAuthProviderNames {
    Apple,
    Azure,
    Discord,
    Github,
    Google,
    Slack,
}

impl BuiltinOAuthProviderNames {
    // Function to convert enum variants to string literals
    pub fn as_str(&self) -> &'static str {
        match self {
            BuiltinOAuthProviderNames::Apple => "builtin::oauth_apple",
            BuiltinOAuthProviderNames::Azure => "builtin::oauth_azure",
            BuiltinOAuthProviderNames::Discord => "builtin::oauth_discord",
            BuiltinOAuthProviderNames::Github => "builtin::oauth_github",
            BuiltinOAuthProviderNames::Google => "builtin::oauth_google",
            BuiltinOAuthProviderNames::Slack => "builtin::oauth_slack",
        }
    }
}

// define and enum to represent the possible authentication providers
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinAuthProviderNames {
    EmailPassword,
    WebAuthn,
    MagicLink,
}

impl BuiltinAuthProviderNames {
    // Function to convert enum variants to string literals
    pub fn as_str(&self) -> &'static str {
        match self {
            BuiltinAuthProviderNames::EmailPassword => "builtin::local_emailpassword",
            BuiltinAuthProviderNames::WebAuthn => "builtin::local_webauthn",
            BuiltinAuthProviderNames::MagicLink => "builtin::local_magic_link",
        }
    }
}
