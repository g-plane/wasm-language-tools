use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
/// Language service configuration. This can be different for each document.
pub struct ServiceConfig {
    /// Configuration about formatting.
    pub format: wat_formatter::config::LanguageOptions,
    /// Configuration about linting.
    pub lint: Lints,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
/// Configuration about linting.
pub struct Lints {
    /// Lint for detecting unused items.
    pub unused: LintLevel,

    /// Lint for detecting shadowing.
    pub shadow: LintLevel,

    #[serde(alias = "implicitModule")]
    /// Lint for top-level module fields without declaring a module.
    pub implicit_module: LintLevel,
}

impl Default for Lints {
    fn default() -> Self {
        Self {
            unused: LintLevel::Warn,
            shadow: LintLevel::Warn,
            implicit_module: LintLevel::Allow,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
/// Severity level when lint reports.
pub enum LintLevel {
    #[serde(alias = "allow")]
    Allow,
    #[serde(alias = "warn")]
    Warn,
    #[serde(alias = "deny")]
    Deny,
}
