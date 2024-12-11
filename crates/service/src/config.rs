use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
/// Language service configuration. This can be different for each document.
pub struct ServiceConfig {
    /// Configuration about formatting.
    pub format: wat_formatter::config::LanguageOptions,
}
