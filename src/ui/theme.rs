use ratatui::style::{Color, Modifier, Style};

/// Color theme for the TUI wizard.
#[derive(Debug, Clone)]
pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub bg: Color,
    pub fg: Color,
    pub accent: Color,
    pub error: Color,
    pub border: Color,
    pub highlight: Color,
}

impl Default for Theme {
    fn default() -> Self {
        // Tokyonight-inspired palette
        Self {
            primary: Color::Rgb(122, 162, 247),   // Blue
            secondary: Color::Rgb(187, 154, 247), // Purple
            bg: Color::Rgb(26, 27, 38),           // Dark background
            fg: Color::Rgb(192, 202, 245),        // Light foreground
            accent: Color::Rgb(158, 206, 106),    // Green
            error: Color::Rgb(247, 118, 142),     // Red/pink
            border: Color::Rgb(59, 66, 97),       // Muted border
            highlight: Color::Rgb(69, 78, 115),   // Selection highlight
        }
    }
}

impl Theme {
    pub fn title_style(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    pub fn input_style(&self) -> Style {
        Style::default().fg(self.fg)
    }

    pub fn selected_style(&self) -> Style {
        Style::default()
            .fg(self.accent)
            .add_modifier(Modifier::BOLD)
    }

    pub fn help_style(&self) -> Style {
        Style::default().fg(self.border)
    }

    pub fn error_style(&self) -> Style {
        Style::default().fg(self.error)
    }

    pub fn focused_border_style(&self) -> Style {
        Style::default().fg(self.primary)
    }

    pub fn unfocused_border_style(&self) -> Style {
        Style::default().fg(self.border)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_colors_exist() {
        let theme = Theme::default();
        // Verify all colors are accessible without panic
        let _primary = theme.primary;
        let _secondary = theme.secondary;
        let _bg = theme.bg;
        let _fg = theme.fg;
        let _accent = theme.accent;
        let _error = theme.error;
        let _border = theme.border;
        let _highlight = theme.highlight;
    }

    #[test]
    fn test_theme_styles() {
        let theme = Theme::default();
        let _title = theme.title_style();
        let _input = theme.input_style();
        let _selected = theme.selected_style();
        let _help = theme.help_style();
        let _error = theme.error_style();
        let _focused = theme.focused_border_style();
        let _unfocused = theme.unfocused_border_style();
    }
}
