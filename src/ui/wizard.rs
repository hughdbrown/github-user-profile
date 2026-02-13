/// The 12 steps of the wizard, in order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WizardStep {
    Welcome,
    Identity,
    About,
    Social,
    Skills,
    Stats,
    Projects,
    Blog,
    Dynamic,
    Extras,
    Layout,
    PreviewGenerate,
}

impl WizardStep {
    pub const ALL: &[WizardStep] = &[
        WizardStep::Welcome,
        WizardStep::Identity,
        WizardStep::About,
        WizardStep::Social,
        WizardStep::Skills,
        WizardStep::Stats,
        WizardStep::Projects,
        WizardStep::Blog,
        WizardStep::Dynamic,
        WizardStep::Extras,
        WizardStep::Layout,
        WizardStep::PreviewGenerate,
    ];

    pub fn index(&self) -> usize {
        WizardStep::ALL
            .iter()
            .position(|s| s == self)
            .unwrap_or(0)
    }

    pub fn next(&self) -> Self {
        let idx = self.index();
        if idx + 1 < WizardStep::ALL.len() {
            WizardStep::ALL[idx + 1]
        } else {
            *self
        }
    }

    pub fn prev(&self) -> Self {
        let idx = self.index();
        if idx > 0 {
            WizardStep::ALL[idx - 1]
        } else {
            *self
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            WizardStep::Welcome => "Welcome",
            WizardStep::Identity => "Identity",
            WizardStep::About => "About",
            WizardStep::Social => "Social Links",
            WizardStep::Skills => "Skills",
            WizardStep::Stats => "Stats",
            WizardStep::Projects => "Projects",
            WizardStep::Blog => "Blog & Content",
            WizardStep::Dynamic => "Dynamic",
            WizardStep::Extras => "Extras",
            WizardStep::Layout => "Layout",
            WizardStep::PreviewGenerate => "Preview & Generate",
        }
    }
}

/// Action returned by step handlers to control the wizard flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepAction {
    /// Continue handling input in the current step.
    Continue,
    /// Advance to the next step.
    NextStep,
    /// Go back to the previous step.
    PrevStep,
    /// Generate output immediately (Ctrl-G).
    Generate,
    /// Quit the wizard.
    Quit,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wizard_step_order() {
        assert_eq!(WizardStep::Welcome.next(), WizardStep::Identity);
        assert_eq!(WizardStep::Identity.next(), WizardStep::About);
        assert_eq!(WizardStep::About.next(), WizardStep::Social);
        assert_eq!(WizardStep::Social.next(), WizardStep::Skills);
        assert_eq!(WizardStep::Skills.next(), WizardStep::Stats);
        assert_eq!(WizardStep::Stats.next(), WizardStep::Projects);
        assert_eq!(WizardStep::Projects.next(), WizardStep::Blog);
        assert_eq!(WizardStep::Blog.next(), WizardStep::Dynamic);
        assert_eq!(WizardStep::Dynamic.next(), WizardStep::Extras);
        assert_eq!(WizardStep::Extras.next(), WizardStep::Layout);
        assert_eq!(WizardStep::Layout.next(), WizardStep::PreviewGenerate);
        assert_eq!(WizardStep::PreviewGenerate.next(), WizardStep::PreviewGenerate);
    }

    #[test]
    fn test_wizard_step_prev() {
        assert_eq!(WizardStep::Welcome.prev(), WizardStep::Welcome);
        assert_eq!(WizardStep::Identity.prev(), WizardStep::Welcome);
        assert_eq!(WizardStep::PreviewGenerate.prev(), WizardStep::Layout);
    }

    #[test]
    fn test_wizard_step_count() {
        assert_eq!(WizardStep::ALL.len(), 12);
    }

    #[test]
    fn test_wizard_step_label() {
        assert_eq!(WizardStep::Welcome.label(), "Welcome");
        assert_eq!(WizardStep::Identity.label(), "Identity");
        assert_eq!(WizardStep::PreviewGenerate.label(), "Preview & Generate");
    }

    #[test]
    fn test_wizard_step_index() {
        assert_eq!(WizardStep::Welcome.index(), 0);
        assert_eq!(WizardStep::Identity.index(), 1);
        assert_eq!(WizardStep::PreviewGenerate.index(), 11);
    }
}
