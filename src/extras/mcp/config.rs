use std::collections::HashMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};

pub const DEFAULT_CONNECT_TIMEOUT_SECS: u64 = 30;
pub const DEFAULT_DISCOVERY_TIMEOUT_SECS: u64 = 30;
pub const DEFAULT_TOOL_TIMEOUT_SECS: u64 = 300;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum McpServerConfig {
    Command {
        command: String,
        #[serde(default)]
        args: Vec<String>,
        #[serde(default)]
        env: HashMap<String, String>,
        #[serde(default = "default_connect_timeout_secs")]
        connect_timeout_secs: u64,
        #[serde(default = "default_discovery_timeout_secs")]
        discovery_timeout_secs: u64,
        #[serde(default = "default_tool_timeout_secs")]
        tool_timeout_secs: u64,
    },
    Url {
        url: String,
        #[serde(default)]
        headers: HashMap<String, String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        oauth: Option<OAuthConfig>,
        #[serde(default = "default_connect_timeout_secs")]
        connect_timeout_secs: u64,
        #[serde(default = "default_discovery_timeout_secs")]
        discovery_timeout_secs: u64,
        #[serde(default = "default_tool_timeout_secs")]
        tool_timeout_secs: u64,
    },
}

/// OAuth settings for a URL-based MCP server.
///
/// Accepts either a bare `true` (enable with all defaults: dynamic client
/// registration, no extra scopes) or an object with explicit fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OAuthConfig {
    Enabled(bool),
    Settings(OAuthSettings),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OAuthSettings {
    /// OAuth scopes to request. Empty means none are requested explicitly.
    #[serde(default)]
    pub scopes: Vec<String>,
    /// Pre-registered client id. When absent, dynamic client registration is used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Loopback port for the redirect URI. Defaults to [`DEFAULT_REDIRECT_PORT`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect_port: Option<u16>,
}

pub const DEFAULT_REDIRECT_PORT: u16 = 8970;

const fn default_connect_timeout_secs() -> u64 {
    DEFAULT_CONNECT_TIMEOUT_SECS
}

const fn default_discovery_timeout_secs() -> u64 {
    DEFAULT_DISCOVERY_TIMEOUT_SECS
}

const fn default_tool_timeout_secs() -> u64 {
    DEFAULT_TOOL_TIMEOUT_SECS
}

impl McpServerConfig {
    pub fn connect_timeout(&self) -> Option<Duration> {
        timeout(self.timeout_secs().0)
    }

    pub fn discovery_timeout(&self) -> Option<Duration> {
        timeout(self.timeout_secs().1)
    }

    pub fn tool_timeout(&self) -> Option<Duration> {
        timeout(self.timeout_secs().2)
    }

    fn timeout_secs(&self) -> (u64, u64, u64) {
        match self {
            Self::Command {
                connect_timeout_secs,
                discovery_timeout_secs,
                tool_timeout_secs,
                ..
            }
            | Self::Url {
                connect_timeout_secs,
                discovery_timeout_secs,
                tool_timeout_secs,
                ..
            } => (
                *connect_timeout_secs,
                *discovery_timeout_secs,
                *tool_timeout_secs,
            ),
        }
    }
}

fn timeout(seconds: u64) -> Option<Duration> {
    (seconds > 0).then(|| Duration::from_secs(seconds))
}

impl OAuthConfig {
    /// Returns the resolved settings if OAuth is enabled, or `None` if disabled.
    pub fn settings(&self) -> Option<OAuthSettings> {
        match self {
            OAuthConfig::Enabled(false) => None,
            OAuthConfig::Enabled(true) => Some(OAuthSettings::default()),
            OAuthConfig::Settings(s) => Some(s.clone()),
        }
    }
}

impl OAuthSettings {
    pub fn redirect_port(&self) -> u16 {
        self.redirect_port.unwrap_or(DEFAULT_REDIRECT_PORT)
    }

    pub fn redirect_uri(&self) -> String {
        format!("http://127.0.0.1:{}/callback", self.redirect_port())
    }
}
