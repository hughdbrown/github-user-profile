use crossterm::event::{KeyCode, KeyEvent};

// ── TextInput ──────────────────────────────────────────────────────────

/// Single-line text input widget with cursor support.
#[derive(Debug, Clone)]
pub struct TextInput {
    pub label: String,
    value: String,
    cursor: usize,
    pub focused: bool,
}

impl TextInput {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            value: String::new(),
            cursor: 0,
            focused: false,
        }
    }

    pub fn with_value(label: &str, value: &str) -> Self {
        let len = value.len();
        Self {
            label: label.to_string(),
            value: value.to_string(),
            cursor: len,
            focused: false,
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_string();
        self.cursor = self.value.len();
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn clear(&mut self) {
        self.value.clear();
        self.cursor = 0;
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                self.value.insert(self.cursor, c);
                self.cursor += 1;
            }
            KeyCode::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.value.remove(self.cursor);
                }
            }
            KeyCode::Delete => {
                if self.cursor < self.value.len() {
                    self.value.remove(self.cursor);
                }
            }
            KeyCode::Left => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor < self.value.len() {
                    self.cursor += 1;
                }
            }
            KeyCode::Home => {
                self.cursor = 0;
            }
            KeyCode::End => {
                self.cursor = self.value.len();
            }
            _ => {}
        }
    }
}

// ── Toggle ─────────────────────────────────────────────────────────────

/// Boolean toggle widget.
#[derive(Debug, Clone)]
pub struct Toggle {
    pub label: String,
    value: bool,
    pub focused: bool,
}

impl Toggle {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            value: false,
            focused: false,
        }
    }

    pub fn with_value(label: &str, value: bool) -> Self {
        Self {
            label: label.to_string(),
            value,
            focused: false,
        }
    }

    pub fn value(&self) -> bool {
        self.value
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) {
            self.value = !self.value;
        }
    }
}

// ── SingleSelect ───────────────────────────────────────────────────────

/// Pick one option from a list.
#[derive(Debug, Clone)]
pub struct SingleSelect {
    pub label: String,
    pub options: Vec<String>,
    highlight: usize,
    selected: Option<usize>,
    pub focused: bool,
}

impl SingleSelect {
    pub fn new(label: &str, options: Vec<String>) -> Self {
        Self {
            label: label.to_string(),
            options,
            highlight: 0,
            selected: None,
            focused: false,
        }
    }

    pub fn with_default(label: &str, options: Vec<String>, default: usize) -> Self {
        Self {
            label: label.to_string(),
            highlight: default.min(options.len().saturating_sub(1)),
            options,
            selected: None,
            focused: false,
        }
    }

    pub fn highlight(&self) -> usize {
        self.highlight
    }

    pub fn selected(&self) -> Option<&str> {
        self.selected.map(|i| self.options[i].as_str())
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.selected
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => {
                if self.highlight > 0 {
                    self.highlight -= 1;
                }
            }
            KeyCode::Down => {
                if self.highlight + 1 < self.options.len() {
                    self.highlight += 1;
                }
            }
            KeyCode::Enter => {
                self.selected = Some(self.highlight);
            }
            _ => {}
        }
    }
}

// ── SearchableList ─────────────────────────────────────────────────────

/// Item in a searchable list.
#[derive(Debug, Clone)]
pub struct ListItem {
    pub label: String,
    pub category: Option<String>,
    pub selected: bool,
}

/// Mode for the searchable list: typing in search or navigating items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListMode {
    Search,
    Navigate,
}

/// Multi-select list with search filtering and category grouping.
#[derive(Debug, Clone)]
pub struct SearchableList {
    items: Vec<ListItem>,
    pub search: TextInput,
    highlight: usize,
    pub mode: ListMode,
}

impl SearchableList {
    pub fn new(labels: Vec<&str>) -> Self {
        let items: Vec<ListItem> = labels
            .into_iter()
            .map(|l| ListItem {
                label: l.to_string(),
                category: None,
                selected: false,
            })
            .collect();
        Self {
            items,
            search: TextInput::new("Search"),
            highlight: 0,
            mode: ListMode::Search,
        }
    }

    pub fn categorized(categories: Vec<(&str, Vec<&str>)>) -> Self {
        let mut items: Vec<ListItem> = Vec::new();
        for (cat, labels) in categories {
            for label in labels {
                items.push(ListItem {
                    label: label.to_string(),
                    category: Some(cat.to_string()),
                    selected: false,
                });
            }
        }
        Self {
            items,
            search: TextInput::new("Search"),
            highlight: 0,
            mode: ListMode::Search,
        }
    }

    pub fn visible_items(&self) -> Vec<&ListItem> {
        let query: String = self.search.value().to_lowercase();
        self.items
            .iter()
            .filter(|item| {
                query.is_empty() || item.label.to_lowercase().contains(&query)
            })
            .collect()
    }

    pub fn selected(&self) -> Vec<&str> {
        self.items
            .iter()
            .filter(|item| item.selected)
            .map(|item| item.label.as_str())
            .collect()
    }

    pub fn highlight(&self) -> usize {
        self.highlight
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.mode {
            ListMode::Search => match key.code {
                KeyCode::Tab => {
                    self.mode = ListMode::Navigate;
                    self.highlight = 0;
                }
                _ => self.search.handle_key(key),
            },
            ListMode::Navigate => match key.code {
                KeyCode::Tab | KeyCode::BackTab => {
                    self.mode = ListMode::Search;
                }
                KeyCode::Up => {
                    if self.highlight > 0 {
                        self.highlight -= 1;
                    }
                }
                KeyCode::Down => {
                    let visible_count: usize = self.visible_items().len();
                    if self.highlight + 1 < visible_count {
                        self.highlight += 1;
                    }
                }
                KeyCode::Enter | KeyCode::Char(' ') => {
                    // Toggle the highlighted item
                    let visible: Vec<String> = self
                        .visible_items()
                        .iter()
                        .map(|item| item.label.clone())
                        .collect();
                    if let Some(label) = visible.get(self.highlight)
                        && let Some(item) = self
                            .items
                            .iter_mut()
                            .find(|item| &item.label == label)
                    {
                        item.selected = !item.selected;
                    }
                }
                _ => {}
            },
        }
    }
}

// ── ListInput ──────────────────────────────────────────────────────────

/// Mode for list input: adding new entries or browsing existing ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListInputMode {
    Adding,
    Browsing,
}

/// Dynamic list of text entries with add/remove capability.
#[derive(Debug, Clone)]
pub struct ListInput {
    pub label: String,
    entries: Vec<String>,
    pub input: TextInput,
    highlight: usize,
    pub mode: ListInputMode,
}

impl ListInput {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            entries: Vec::new(),
            input: TextInput::new("Add new"),
            highlight: 0,
            mode: ListInputMode::Adding,
        }
    }

    pub fn with_entries(label: &str, entries: Vec<String>) -> Self {
        Self {
            label: label.to_string(),
            entries,
            input: TextInput::new("Add new"),
            highlight: 0,
            mode: ListInputMode::Adding,
        }
    }

    pub fn entries(&self) -> &[String] {
        &self.entries
    }

    pub fn highlight(&self) -> usize {
        self.highlight
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.mode {
            ListInputMode::Adding => match key.code {
                KeyCode::Enter => {
                    let val: String = self.input.value().trim().to_string();
                    if !val.is_empty() {
                        self.entries.push(val);
                        self.input.clear();
                    }
                }
                KeyCode::Tab => {
                    if !self.entries.is_empty() {
                        self.mode = ListInputMode::Browsing;
                        self.highlight = 0;
                    }
                }
                _ => self.input.handle_key(key),
            },
            ListInputMode::Browsing => match key.code {
                KeyCode::Tab | KeyCode::BackTab => {
                    self.mode = ListInputMode::Adding;
                }
                KeyCode::Up => {
                    if self.highlight > 0 {
                        self.highlight -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.highlight + 1 < self.entries.len() {
                        self.highlight += 1;
                    }
                }
                KeyCode::Delete | KeyCode::Backspace => {
                    if !self.entries.is_empty() {
                        self.entries.remove(self.highlight);
                        if self.highlight >= self.entries.len() && self.highlight > 0 {
                            self.highlight -= 1;
                        }
                        if self.entries.is_empty() {
                            self.mode = ListInputMode::Adding;
                        }
                    }
                }
                _ => {}
            },
        }
    }
}

// ── PairedInput ────────────────────────────────────────────────────────

/// Mode for paired input fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PairedInputMode {
    Adding,
    Browsing,
}

/// Which field of the pair is focused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PairedField {
    First,
    Second,
}

/// Two-field entry widget (e.g., title + URL).
#[derive(Debug, Clone)]
pub struct PairedInput {
    pub label: String,
    pub first_label: String,
    pub second_label: String,
    entries: Vec<(String, String)>,
    pub first: TextInput,
    pub second: TextInput,
    highlight: usize,
    pub mode: PairedInputMode,
    pub focused_field: PairedField,
}

impl PairedInput {
    pub fn new(label: &str, first_label: &str, second_label: &str) -> Self {
        Self {
            label: label.to_string(),
            first_label: first_label.to_string(),
            second_label: second_label.to_string(),
            entries: Vec::new(),
            first: TextInput::new(first_label),
            second: TextInput::new(second_label),
            highlight: 0,
            mode: PairedInputMode::Adding,
            focused_field: PairedField::First,
        }
    }

    pub fn entries(&self) -> &[(String, String)] {
        &self.entries
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.mode {
            PairedInputMode::Adding => match key.code {
                KeyCode::Enter => {
                    let first_val: String = self.first.value().trim().to_string();
                    let second_val: String = self.second.value().trim().to_string();
                    if !first_val.is_empty() && !second_val.is_empty() {
                        self.entries.push((first_val, second_val));
                        self.first.clear();
                        self.second.clear();
                        self.focused_field = PairedField::First;
                    }
                }
                KeyCode::Tab => match self.focused_field {
                    PairedField::First => {
                        self.focused_field = PairedField::Second;
                    }
                    PairedField::Second => {
                        if !self.entries.is_empty() {
                            self.mode = PairedInputMode::Browsing;
                            self.highlight = 0;
                        } else {
                            self.focused_field = PairedField::First;
                        }
                    }
                },
                KeyCode::BackTab => match self.focused_field {
                    PairedField::First => {
                        if !self.entries.is_empty() {
                            self.mode = PairedInputMode::Browsing;
                            self.highlight = 0;
                        }
                    }
                    PairedField::Second => {
                        self.focused_field = PairedField::First;
                    }
                },
                _ => match self.focused_field {
                    PairedField::First => self.first.handle_key(key),
                    PairedField::Second => self.second.handle_key(key),
                },
            },
            PairedInputMode::Browsing => match key.code {
                KeyCode::Tab | KeyCode::BackTab => {
                    self.mode = PairedInputMode::Adding;
                    self.focused_field = PairedField::First;
                }
                KeyCode::Up => {
                    if self.highlight > 0 {
                        self.highlight -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.highlight + 1 < self.entries.len() {
                        self.highlight += 1;
                    }
                }
                KeyCode::Delete | KeyCode::Backspace => {
                    if !self.entries.is_empty() {
                        self.entries.remove(self.highlight);
                        if self.highlight >= self.entries.len() && self.highlight > 0 {
                            self.highlight -= 1;
                        }
                        if self.entries.is_empty() {
                            self.mode = PairedInputMode::Adding;
                            self.focused_field = PairedField::First;
                        }
                    }
                }
                _ => {}
            },
        }
    }
}

// ── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyModifiers;

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn char_key(c: char) -> KeyEvent {
        key(KeyCode::Char(c))
    }

    // ── TextInput tests ──

    #[test]
    fn test_text_input_new() {
        let input = TextInput::new("Username");
        assert_eq!(input.value(), "");
        assert_eq!(input.label, "Username");
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_text_input_typing() {
        let mut input = TextInput::new("Username");
        for c in "alice".chars() {
            input.handle_key(char_key(c));
        }
        assert_eq!(input.value(), "alice");
        assert_eq!(input.cursor(), 5);
    }

    #[test]
    fn test_text_input_backspace() {
        let mut input = TextInput::with_value("Username", "alice");
        input.handle_key(key(KeyCode::Backspace));
        assert_eq!(input.value(), "alic");
        assert_eq!(input.cursor(), 4);
    }

    #[test]
    fn test_text_input_backspace_empty() {
        let mut input = TextInput::new("Username");
        input.handle_key(key(KeyCode::Backspace));
        assert_eq!(input.value(), "");
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_text_input_cursor_left_right() {
        let mut input = TextInput::with_value("Username", "alice");
        input.handle_key(key(KeyCode::Left));
        input.handle_key(key(KeyCode::Left));
        assert_eq!(input.cursor(), 3);
        input.handle_key(key(KeyCode::Right));
        assert_eq!(input.cursor(), 4);
    }

    #[test]
    fn test_text_input_insert_middle() {
        let mut input = TextInput::with_value("Username", "alice");
        // Move cursor left 3 positions (to position 2)
        input.handle_key(key(KeyCode::Left));
        input.handle_key(key(KeyCode::Left));
        input.handle_key(key(KeyCode::Left));
        input.handle_key(char_key('X'));
        assert_eq!(input.value(), "alXice");
        assert_eq!(input.cursor(), 3);
    }

    #[test]
    fn test_text_input_home_end() {
        let mut input = TextInput::with_value("Username", "alice");
        input.handle_key(key(KeyCode::Home));
        assert_eq!(input.cursor(), 0);
        input.handle_key(key(KeyCode::End));
        assert_eq!(input.cursor(), 5);
    }

    #[test]
    fn test_text_input_with_initial() {
        let input = TextInput::with_value("Username", "alice");
        assert_eq!(input.value(), "alice");
    }

    #[test]
    fn test_text_input_clear() {
        let mut input = TextInput::with_value("Username", "alice");
        input.clear();
        assert_eq!(input.value(), "");
        assert_eq!(input.cursor(), 0);
    }

    // ── Toggle tests ──

    #[test]
    fn test_toggle_new() {
        let toggle = Toggle::new("Show Icons");
        assert!(!toggle.value());
    }

    #[test]
    fn test_toggle_flip() {
        let mut toggle = Toggle::new("Show Icons");
        toggle.handle_key(key(KeyCode::Enter));
        assert!(toggle.value());
        toggle.handle_key(key(KeyCode::Enter));
        assert!(!toggle.value());
    }

    #[test]
    fn test_toggle_space() {
        let mut toggle = Toggle::new("Show Icons");
        toggle.handle_key(char_key(' '));
        assert!(toggle.value());
    }

    #[test]
    fn test_toggle_with_value() {
        let toggle = Toggle::with_value("Show Icons", true);
        assert!(toggle.value());
    }

    // ── SingleSelect tests ──

    #[test]
    fn test_single_select_new() {
        let select =
            SingleSelect::new("Template", vec!["Minimal".into(), "Full".into(), "Multi".into()]);
        assert_eq!(select.highlight(), 0);
        assert!(select.selected().is_none());
    }

    #[test]
    fn test_single_select_navigate() {
        let mut select =
            SingleSelect::new("Template", vec!["A".into(), "B".into(), "C".into()]);
        select.handle_key(key(KeyCode::Down));
        assert_eq!(select.highlight(), 1);
        select.handle_key(key(KeyCode::Up));
        assert_eq!(select.highlight(), 0);
        select.handle_key(key(KeyCode::Up)); // Can't go above 0
        assert_eq!(select.highlight(), 0);
    }

    #[test]
    fn test_single_select_confirm() {
        let mut select =
            SingleSelect::new("Template", vec!["A".into(), "B".into(), "C".into()]);
        select.handle_key(key(KeyCode::Down));
        select.handle_key(key(KeyCode::Enter));
        assert_eq!(select.selected(), Some("B"));
        assert_eq!(select.selected_index(), Some(1));
    }

    #[test]
    fn test_single_select_with_default() {
        let select =
            SingleSelect::with_default("Template", vec!["A".into(), "B".into(), "C".into()], 1);
        assert_eq!(select.highlight(), 1);
    }

    // ── SearchableList tests ──

    #[test]
    fn test_searchable_list_new() {
        let list = SearchableList::new(vec!["Rust", "Ruby", "Python"]);
        assert_eq!(list.visible_items().len(), 3);
        assert!(list.selected().is_empty());
        assert_eq!(list.search.value(), "");
    }

    #[test]
    fn test_searchable_list_filter() {
        let mut list = SearchableList::new(vec!["Rust", "Ruby", "Python"]);
        list.search.handle_key(char_key('r'));
        list.search.handle_key(char_key('u'));
        let visible: Vec<&str> = list.visible_items().iter().map(|i| i.label.as_str()).collect();
        assert_eq!(visible, vec!["Rust", "Ruby"]);
    }

    #[test]
    fn test_searchable_list_filter_case_insensitive() {
        let mut list = SearchableList::new(vec!["Rust", "Ruby", "Python"]);
        list.search.handle_key(char_key('R'));
        list.search.handle_key(char_key('U'));
        let visible: Vec<&str> = list.visible_items().iter().map(|i| i.label.as_str()).collect();
        assert_eq!(visible, vec!["Rust", "Ruby"]);
    }

    #[test]
    fn test_searchable_list_filter_clear() {
        let mut list = SearchableList::new(vec!["Rust", "Ruby", "Python"]);
        list.search.handle_key(char_key('r'));
        assert_eq!(list.visible_items().len(), 2);
        list.search.handle_key(key(KeyCode::Backspace));
        assert_eq!(list.visible_items().len(), 3);
    }

    #[test]
    fn test_searchable_list_select() {
        let mut list = SearchableList::new(vec!["Rust", "Ruby", "Python"]);
        // Switch to navigate mode
        list.handle_key(key(KeyCode::Tab));
        assert_eq!(list.mode, ListMode::Navigate);
        // Select first item (Rust)
        list.handle_key(key(KeyCode::Enter));
        assert_eq!(list.selected(), vec!["Rust"]);
        // Deselect
        list.handle_key(key(KeyCode::Enter));
        assert!(list.selected().is_empty());
    }

    #[test]
    fn test_searchable_list_select_multiple() {
        let mut list = SearchableList::new(vec!["Rust", "Ruby", "Python"]);
        list.handle_key(key(KeyCode::Tab)); // Navigate mode
        list.handle_key(key(KeyCode::Enter)); // Select Rust
        list.handle_key(key(KeyCode::Down));
        list.handle_key(key(KeyCode::Down));
        list.handle_key(key(KeyCode::Enter)); // Select Python
        let sel: Vec<&str> = list.selected();
        assert_eq!(sel, vec!["Rust", "Python"]);
    }

    #[test]
    fn test_searchable_list_navigate() {
        let mut list = SearchableList::new(vec!["Rust", "Ruby", "Python"]);
        list.handle_key(key(KeyCode::Tab)); // Navigate mode
        assert_eq!(list.highlight(), 0);
        list.handle_key(key(KeyCode::Down));
        assert_eq!(list.highlight(), 1);
        list.handle_key(key(KeyCode::Up));
        assert_eq!(list.highlight(), 0);
        list.handle_key(key(KeyCode::Up)); // Can't go above 0
        assert_eq!(list.highlight(), 0);
    }

    #[test]
    fn test_searchable_list_filter_preserves_selection() {
        let mut list = SearchableList::new(vec!["Rust", "Ruby", "Python"]);
        // Select Rust
        list.handle_key(key(KeyCode::Tab));
        list.handle_key(key(KeyCode::Enter));
        // Switch back to search, filter to "py"
        list.handle_key(key(KeyCode::Tab));
        list.search.handle_key(char_key('p'));
        list.search.handle_key(char_key('y'));
        // Switch to navigate, select Python
        list.handle_key(key(KeyCode::Tab));
        list.handle_key(key(KeyCode::Enter));
        // Clear filter
        list.handle_key(key(KeyCode::Tab));
        list.search.clear();
        // Both should be selected
        let sel: Vec<&str> = list.selected();
        assert!(sel.contains(&"Rust"));
        assert!(sel.contains(&"Python"));
    }

    #[test]
    fn test_searchable_list_categorized() {
        let list = SearchableList::categorized(vec![
            ("Languages", vec!["Rust", "Python"]),
            ("Tools", vec!["Docker"]),
        ]);
        assert_eq!(list.visible_items().len(), 3);
        assert_eq!(
            list.visible_items()[0].category.as_deref(),
            Some("Languages")
        );
        assert_eq!(list.visible_items()[2].category.as_deref(), Some("Tools"));
    }

    // ── ListInput tests ──

    #[test]
    fn test_list_input_new() {
        let li = ListInput::new("Repos");
        assert!(li.entries().is_empty());
    }

    #[test]
    fn test_list_input_add() {
        let mut li = ListInput::new("Repos");
        for c in "alice/cool-cli".chars() {
            li.handle_key(char_key(c));
        }
        li.handle_key(key(KeyCode::Enter));
        assert_eq!(li.entries(), &["alice/cool-cli"]);
        assert_eq!(li.input.value(), ""); // Input cleared
    }

    #[test]
    fn test_list_input_add_empty_rejected() {
        let mut li = ListInput::new("Repos");
        li.handle_key(key(KeyCode::Enter));
        assert!(li.entries().is_empty());
    }

    #[test]
    fn test_list_input_remove() {
        let mut li = ListInput::with_entries("Repos", vec!["a".into(), "b".into()]);
        // Switch to browsing
        li.mode = ListInputMode::Browsing;
        li.highlight = 0;
        li.handle_key(key(KeyCode::Delete));
        assert_eq!(li.entries(), &["b"]);
    }

    #[test]
    fn test_list_input_navigate() {
        let mut li = ListInput::with_entries("Repos", vec!["a".into(), "b".into(), "c".into()]);
        li.mode = ListInputMode::Browsing;
        assert_eq!(li.highlight(), 0);
        li.handle_key(key(KeyCode::Down));
        assert_eq!(li.highlight(), 1);
        li.handle_key(key(KeyCode::Up));
        assert_eq!(li.highlight(), 0);
    }

    #[test]
    fn test_list_input_mode_switch() {
        let mut li = ListInput::with_entries("Repos", vec!["a".into()]);
        assert_eq!(li.mode, ListInputMode::Adding);
        li.handle_key(key(KeyCode::Tab));
        assert_eq!(li.mode, ListInputMode::Browsing);
        li.handle_key(key(KeyCode::Tab));
        assert_eq!(li.mode, ListInputMode::Adding);
    }

    // ── PairedInput tests ──

    #[test]
    fn test_paired_input_new() {
        let pi = PairedInput::new("Articles", "Title", "URL");
        assert!(pi.entries().is_empty());
        assert_eq!(pi.first.value(), "");
        assert_eq!(pi.second.value(), "");
    }

    #[test]
    fn test_paired_input_add() {
        let mut pi = PairedInput::new("Articles", "Title", "URL");
        // Type title
        for c in "My Post".chars() {
            pi.handle_key(char_key(c));
        }
        // Tab to second field
        pi.handle_key(key(KeyCode::Tab));
        assert_eq!(pi.focused_field, PairedField::Second);
        // Type URL
        for c in "https://example.com/post".chars() {
            pi.handle_key(char_key(c));
        }
        // Enter to add
        pi.handle_key(key(KeyCode::Enter));
        assert_eq!(
            pi.entries(),
            &[("My Post".to_string(), "https://example.com/post".to_string())]
        );
        // Fields should be cleared
        assert_eq!(pi.first.value(), "");
        assert_eq!(pi.second.value(), "");
        assert_eq!(pi.focused_field, PairedField::First);
    }

    #[test]
    fn test_paired_input_incomplete_rejected() {
        let mut pi = PairedInput::new("Articles", "Title", "URL");
        for c in "Title Only".chars() {
            pi.handle_key(char_key(c));
        }
        pi.handle_key(key(KeyCode::Enter));
        assert!(pi.entries().is_empty()); // Second field empty, rejected
    }

    #[test]
    fn test_paired_input_both_empty_rejected() {
        let mut pi = PairedInput::new("Articles", "Title", "URL");
        pi.handle_key(key(KeyCode::Enter));
        assert!(pi.entries().is_empty());
    }

    #[test]
    fn test_paired_input_remove() {
        let mut pi = PairedInput::new("Articles", "Title", "URL");
        pi.entries = vec![
            ("A".to_string(), "1".to_string()),
            ("B".to_string(), "2".to_string()),
        ];
        pi.mode = PairedInputMode::Browsing;
        pi.highlight = 0;
        pi.handle_key(key(KeyCode::Delete));
        assert_eq!(pi.entries(), &[("B".to_string(), "2".to_string())]);
    }

    #[test]
    fn test_paired_input_tab_cycle() {
        let mut pi = PairedInput::new("Articles", "Title", "URL");
        assert_eq!(pi.focused_field, PairedField::First);
        pi.handle_key(key(KeyCode::Tab));
        assert_eq!(pi.focused_field, PairedField::Second);
        // No entries, so Tab from Second goes back to First
        pi.handle_key(key(KeyCode::Tab));
        assert_eq!(pi.focused_field, PairedField::First);
    }
}
