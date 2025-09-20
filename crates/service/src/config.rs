use crate::{LanguageService, document::Document};
use salsa::Setter;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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

    #[serde(alias = "multiModules")]
    /// Lint for detecting multiple modules in a single file.
    pub multi_modules: LintLevel,

    /// Lint for detecting unreachable code.
    pub unreachable: LintLevel,

    #[serde(alias = "needlessMut")]
    /// Lint for detecting mutable globals that are never mutated.
    pub needless_mut: LintLevel,

    #[serde(alias = "multiMemories")]
    /// Lint for detecting multiple memories in one module.
    pub multi_memories: LintLevel,
}

impl Default for Lints {
    fn default() -> Self {
        Self {
            unused: LintLevel::Warn,
            shadow: LintLevel::Warn,
            implicit_module: LintLevel::Allow,
            multi_modules: LintLevel::Deny,
            unreachable: LintLevel::Hint,
            needless_mut: LintLevel::Warn,
            multi_memories: LintLevel::Allow,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
/// Severity level when lint reports.
pub enum LintLevel {
    #[serde(alias = "allow")]
    Allow,
    #[serde(alias = "hint")]
    Hint,
    #[serde(alias = "warn")]
    Warn,
    #[serde(alias = "deny")]
    Deny,
}

impl LanguageService {
    #[inline]
    // This should be used internally.
    pub(crate) fn get_config(&self, document: Document) -> &ServiceConfig {
        document.config(self).unwrap_or(&self.global_config)
    }

    #[inline]
    /// Get configurations of all opened documents.
    pub fn get_configs(&self) -> impl Iterator<Item = (String, &ServiceConfig)> {
        self.documents.iter().filter_map(|pair| {
            pair.value()
                .config(self)
                .map(|config| (pair.key().raw(self), config))
        })
    }

    #[inline]
    /// Update or insert configuration of a specific document.
    pub fn set_config(&mut self, uri: String, config: ServiceConfig) {
        if let Some(document) = self.get_document(uri) {
            document.set_config(self).to(Some(config));
        }
    }

    #[inline]
    /// Update global configuration.
    pub fn set_global_config(&mut self, config: ServiceConfig) {
        self.global_config = Arc::new(config);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_and_set_configs() {
        let mut service = LanguageService::default();
        assert_eq!(service.get_configs().count(), 0);

        let uri = "untitled://test".to_string();
        service.commit(uri.clone(), "".into());
        service.set_config(uri.clone(), ServiceConfig::default());
        assert_eq!(service.get_configs().next().unwrap().0, uri);
    }
}
