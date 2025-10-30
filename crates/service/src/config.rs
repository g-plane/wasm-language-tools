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
    #[serde(alias = "inlayHint")]
    /// Configuration about inlay hints.
    pub inlay_hint: InlayHintOptions,
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

    #[serde(alias = "uselessCatch")]
    /// Lint for detecting useless catch clauses.
    pub useless_catch: LintLevel,
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
            useless_catch: LintLevel::Warn,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
/// Configuration about inlay hints.
pub struct InlayHintOptions {
    /// Inlay hints for indicating types.
    pub types: bool,

    /// Inlay hints that show at the end of blocks and functions.
    pub ending: bool,

    /// Inlay hints for showing idx.
    pub index: bool,
}

impl Default for InlayHintOptions {
    fn default() -> Self {
        Self {
            types: true,
            ending: true,
            index: true,
        }
    }
}

#[derive(Debug)]
pub(crate) enum ConfigState {
    Uninit,
    Inherit,
    Override(ServiceConfig),
}

impl LanguageService {
    #[inline]
    // This should be used internally.
    pub(crate) fn get_config_or_global(&self, document: Document) -> &ServiceConfig {
        match document.config(self) {
            ConfigState::Override(config) => config,
            _ => &self.global_config,
        }
    }

    #[inline]
    /// Get configurations of all opened documents.
    pub fn get_configs(&self) -> impl Iterator<Item = (String, &ServiceConfig)> {
        self.documents
            .iter()
            .filter_map(|pair| match pair.value().config(self) {
                ConfigState::Override(config) => Some((pair.key().raw(self), config)),
                _ => None,
            })
    }

    #[inline]
    /// Update or insert configuration of a specific document.
    ///
    /// Set `config` to `None` to inherit global configuration.
    pub fn set_config(&mut self, uri: String, config: Option<ServiceConfig>) {
        if let Some(document) = self.get_document(uri) {
            document
                .set_config(self)
                .to(config.map_or(ConfigState::Inherit, ConfigState::Override));
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
        service.set_config(uri.clone(), Some(ServiceConfig::default()));
        assert_eq!(service.get_configs().next().unwrap().0, uri);
    }
}
