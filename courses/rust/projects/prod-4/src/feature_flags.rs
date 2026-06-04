use crate::config::FeatureFlags;

/// A simple in-memory feature flag service backed by configuration values.
#[derive(Debug, Clone)]
pub struct FeatureFlagService {
    flags: FeatureFlags,
}

impl FeatureFlagService {
    /// Create a new service from the loaded feature flag configuration.
    pub fn new(flags: FeatureFlags) -> Self {
        Self { flags }
    }

    /// Check whether a named feature is enabled.
    ///
    /// Known flag names:
    /// - `"enable_search"`
    /// - `"enable_websockets"`
    ///
    /// Unknown flag names return `false`.
    pub fn is_enabled(&self, flag: &str) -> bool {
        match flag {
            "enable_search" => self.flags.enable_search,
            "enable_websockets" => self.flags.enable_websockets,
            _ => false,
        }
    }

    /// Return the maximum number of tasks allowed per project.
    pub fn max_tasks_per_project(&self) -> usize {
        self.flags.max_tasks_per_project
    }

    /// Return a reference to the underlying flag values.
    pub fn flags(&self) -> &FeatureFlags {
        &self.flags
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_flags(search: bool, ws: bool, max: usize) -> FeatureFlags {
        FeatureFlags {
            enable_search: search,
            enable_websockets: ws,
            max_tasks_per_project: max,
        }
    }

    #[test]
    fn enabled_flag_returns_true() {
        let svc = FeatureFlagService::new(make_flags(true, false, 10));
        assert!(svc.is_enabled("enable_search"));
        assert!(!svc.is_enabled("enable_websockets"));
    }

    #[test]
    fn disabled_flag_returns_false() {
        let svc = FeatureFlagService::new(make_flags(false, false, 5));
        assert!(!svc.is_enabled("enable_search"));
    }

    #[test]
    fn unknown_flag_returns_false() {
        let svc = FeatureFlagService::new(make_flags(true, true, 10));
        assert!(!svc.is_enabled("nonexistent_flag"));
    }

    #[test]
    fn max_tasks_per_project_value() {
        let svc = FeatureFlagService::new(make_flags(true, true, 42));
        assert_eq!(svc.max_tasks_per_project(), 42);
    }
}
