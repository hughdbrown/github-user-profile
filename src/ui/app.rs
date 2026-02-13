use crate::config::profile::ProfileConfig;
use crate::ui::wizard::WizardStep;

/// Basic or Advanced wizard mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Basic,
    Advanced,
}

/// Result of the wizard generation step.
#[derive(Debug, Clone)]
pub struct GenerateResult {
    pub readme_content: String,
    pub toml_content: Option<String>,
    pub write_toml: bool,
}

/// Top-level wizard state container.
#[derive(Debug, Clone)]
pub struct WizardState {
    current_step: WizardStep,
    mode: Mode,
    config: ProfileConfig,
}

impl WizardState {
    pub fn new(mode: Mode) -> Self {
        Self {
            current_step: WizardStep::Welcome,
            mode,
            config: ProfileConfig::default(),
        }
    }

    pub fn current_step(&self) -> WizardStep {
        self.current_step
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn config(&self) -> &ProfileConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut ProfileConfig {
        &mut self.config
    }

    pub fn next_step(&mut self) {
        self.current_step = self.current_step.next();
    }

    pub fn prev_step(&mut self) {
        self.current_step = self.current_step.prev();
    }

    pub fn build_config(self) -> ProfileConfig {
        self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wizard_state_initial() {
        let state = WizardState::new(Mode::Basic);
        assert_eq!(state.current_step(), WizardStep::Welcome);
        assert!(state.config().meta.username.is_empty());
    }

    #[test]
    fn test_wizard_state_advance() {
        let mut state = WizardState::new(Mode::Basic);
        state.next_step();
        assert_eq!(state.current_step(), WizardStep::Identity);
    }

    #[test]
    fn test_wizard_state_back() {
        let mut state = WizardState::new(Mode::Basic);
        state.next_step(); // Welcome -> Identity
        state.prev_step(); // Identity -> Welcome
        assert_eq!(state.current_step(), WizardStep::Welcome);

        state.prev_step(); // Welcome -> Welcome (can't go back)
        assert_eq!(state.current_step(), WizardStep::Welcome);
    }

    #[test]
    fn test_wizard_state_generate_early() {
        let mut state = WizardState::new(Mode::Basic);
        state.next_step(); // Identity
        state.next_step(); // About
        state.config_mut().meta.username = "alice".to_string();
        let config = state.build_config();
        assert_eq!(config.meta.username, "alice");
    }

    #[test]
    fn test_wizard_state_mode_basic() {
        let state = WizardState::new(Mode::Basic);
        assert_eq!(state.mode(), Mode::Basic);
    }

    #[test]
    fn test_wizard_state_mode_advanced() {
        let state = WizardState::new(Mode::Advanced);
        assert_eq!(state.mode(), Mode::Advanced);
    }

    #[test]
    fn test_wizard_state_set_mode() {
        let mut state = WizardState::new(Mode::Basic);
        state.set_mode(Mode::Advanced);
        assert_eq!(state.mode(), Mode::Advanced);
    }
}
