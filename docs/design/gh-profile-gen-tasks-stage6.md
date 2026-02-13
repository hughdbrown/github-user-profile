# Stage 6: TUI Wizard — Detailed Task List

**Depends on**: Stages 1-5 complete and stable (data model, URL builders, renderers, templates, CLI all working).

**Files touched**: src/ui/mod.rs, src/ui/app.rs, src/ui/wizard.rs, src/ui/widgets.rs, src/ui/theme.rs, src/main.rs (add wizard launch), Cargo.toml (add ratatui + crossterm)

## Acceptance Criteria (TUI-specific)

These extend the PRD's acceptance criteria with TUI-specific behavior:

- `test_wizard_state_initial`: A new `WizardState` starts at step `Welcome` with a default `ProfileConfig`.
- `test_wizard_state_advance`: Calling `next_step()` from `Welcome` moves to `Identity`.
- `test_wizard_state_back`: Calling `prev_step()` from `Identity` moves to `Welcome`. Calling `prev_step()` from `Welcome` stays at `Welcome`.
- `test_wizard_state_advance_from_last`: Calling `next_step()` from `PreviewGenerate` stays at `PreviewGenerate`.
- `test_wizard_state_generate_early`: Calling `generate()` from any step produces a `ProfileConfig` with whatever has been filled in so far.
- `test_wizard_identity_updates_meta`: Setting username="alice", name="Alice" in Identity step updates `wizard.config().meta`.
- `test_wizard_about_updates_config`: Setting role, current_work in About step updates `wizard.config().about`.
- `test_wizard_social_add_remove`: Adding a Twitter URL and removing it updates `wizard.config().social` correctly.
- `test_wizard_skills_search_filter`: Typing "rust" in the skills search filters the list to show only items containing "rust" (case-insensitive).
- `test_wizard_skills_toggle`: Toggling "Rust" on adds it to selected skills; toggling again removes it.
- `test_wizard_stats_toggles`: Toggling stats_card on sets `wizard.config().stats.stats_card = true`.
- `test_wizard_projects_add`: Adding "alice/cool-cli" appends to `wizard.config().projects.repos`.
- `test_wizard_layout_select`: Selecting "Minimal" sets `wizard.config().layout.template = Template::Minimal`.
- `test_wizard_basic_mode_no_toml`: In Basic mode, `generate()` returns `GenerateResult` with `write_toml = false`.
- `test_wizard_advanced_mode_writes_toml`: In Advanced mode, `generate()` returns `GenerateResult` with `write_toml = true`.
- `test_text_input_typing`: A `TextInput` widget receiving character events accumulates text correctly.
- `test_text_input_backspace`: Backspace on a `TextInput` removes the last character. Backspace on empty is a no-op.
- `test_text_input_cursor`: Cursor position tracks correctly with left/right arrow keys and typing.
- `test_toggle_input_flip`: A `Toggle` widget receiving Enter/Space flips its value.
- `test_searchable_list_filter`: A `SearchableList` with items ["Rust", "Ruby", "Python"] filtered by "ru" shows ["Rust", "Ruby"].
- `test_searchable_list_select`: Pressing Enter on a highlighted item in `SearchableList` toggles its selected state.
- `test_list_input_add_remove`: A `ListInput` widget supports adding a string entry and removing it by index.

---

## Sub-stage 6A: Dependencies + Terminal Infrastructure + App State Machine

Add ratatui/crossterm to the project and build the foundational app loop and state machine. No visual rendering yet — this is all testable logic.

### Task 6A.1: Add TUI dependencies to Cargo.toml

- **Tests**: None (config change only).
- **Code**: In `Cargo.toml`, add:
  ```toml
  ratatui = "0.29"
  crossterm = "0.28"
  termimad = "0.30"
  ```
  `termimad` renders markdown to styled terminal output for the preview pane.
- **Verify**: `cargo check`

### Task 6A.2: Theme constants

- **Tests**: In `src/ui/theme.rs`:
  - `test_theme_colors_exist`: Verify that `Theme::default()` provides non-panic access to `primary`, `secondary`, `bg`, `fg`, `accent`, `error`, `border`, `highlight` colors.
- **Code**: In `src/ui/theme.rs`:
  - `struct Theme` with `ratatui::style::Color` fields for: primary, secondary, bg, fg, accent, error, border, highlight.
  - `impl Default for Theme` with a cohesive color palette (tokyonight-inspired).
  - `Theme::title_style() -> Style`, `Theme::input_style() -> Style`, `Theme::selected_style() -> Style`, `Theme::help_style() -> Style` — convenience methods returning `ratatui::style::Style`.
- **Files**: Create `src/ui/mod.rs`, `src/ui/theme.rs`
- **Verify**: `cargo test --lib ui::theme`

### Task 6A.3: Wizard step enum and state machine

- **Tests**: In `src/ui/wizard.rs`:
  - `test_wizard_step_order`: `WizardStep::Welcome.next()` == `Identity`, `Identity.next()` == `About`, ... through `PreviewGenerate.next()` == `PreviewGenerate`.
  - `test_wizard_step_prev`: `WizardStep::Identity.prev()` == `Welcome`, `Welcome.prev()` == `Welcome`.
  - `test_wizard_step_count`: `WizardStep::ALL.len()` == 12.
  - `test_wizard_step_label`: Each step has a human-readable label (e.g., `Welcome.label()` == "Welcome").
- **Code**: In `src/ui/wizard.rs`:
  - `enum WizardStep { Welcome, Identity, About, Social, Skills, Stats, Projects, Blog, Dynamic, Extras, Layout, PreviewGenerate }`
  - `impl WizardStep`: `fn next(&self) -> Self`, `fn prev(&self) -> Self`, `fn label(&self) -> &'static str`, `fn index(&self) -> usize`.
  - `const ALL: &[WizardStep]` — ordered array of all steps.
- **Verify**: `cargo test --lib ui::wizard`

### Task 6A.4: WizardState — top-level state container

- **Tests**: In `src/ui/wizard.rs` (or `src/ui/app.rs`):
  - `test_wizard_state_initial`: `WizardState::new(Mode::Basic)` starts at `Welcome` with default `ProfileConfig`.
  - `test_wizard_state_advance`: After `state.next_step()`, current step is `Identity`.
  - `test_wizard_state_back`: From `Identity`, `state.prev_step()` returns to `Welcome`.
  - `test_wizard_state_generate_early`: From `About` step, `state.build_config()` returns a `ProfileConfig` with whatever fields are set.
  - `test_wizard_state_mode_basic`: `WizardState::new(Mode::Basic).mode()` == `Mode::Basic`.
  - `test_wizard_state_mode_advanced`: `WizardState::new(Mode::Advanced).mode()` == `Mode::Advanced`.
- **Code**: In `src/ui/app.rs`:
  - `enum Mode { Basic, Advanced }`
  - `struct WizardState` with fields: `current_step: WizardStep`, `mode: Mode`, `config: ProfileConfig`, plus per-step state structs (initially empty, populated in later tasks).
  - Methods: `new(mode) -> Self`, `next_step()`, `prev_step()`, `current_step() -> WizardStep`, `mode() -> Mode`, `config(&self) -> &ProfileConfig`, `config_mut(&mut self) -> &mut ProfileConfig`, `build_config(self) -> ProfileConfig`.
- **Verify**: `cargo test --lib ui::app`

### Task 6A.5: Terminal setup/teardown and event loop skeleton

- **Tests**: No unit tests — terminal setup/teardown is inherently side-effectful. Manual verification only.
- **Code**: In `src/ui/app.rs` (or a new `src/ui/terminal.rs`):
  - `fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>>` — enables raw mode, enters alternate screen, creates terminal.
  - `fn restore_terminal(terminal: &mut Terminal<...>) -> Result<()>` — disables raw mode, leaves alternate screen. Must be called in all exit paths (normal and panic).
  - `fn run_wizard(mode: Mode) -> Result<ProfileConfig>` — the main loop:
    1. `setup_terminal()`
    2. Loop: poll events → dispatch to current step handler → if quit/generate, break
    3. `restore_terminal()`
    4. Return the built `ProfileConfig`
  - Event handling skeleton: match on `KeyCode::Esc` (back), `KeyCode::Enter` (advance/confirm), `KeyCode::Tab`/`BackTab` (field navigation), `Ctrl-G` (generate now), `Ctrl-C`/`q` (quit).
- **Code**: In `src/main.rs`:
  - Add `wizard` subcommand (or default no-subcommand behavior) that calls `run_wizard()`.
  - On return, call existing `render()` to produce README.md.
  - In Advanced mode, also call `save_config()` to write `profile.toml`.
- **Verify**: `cargo run` launches the TUI and Ctrl-C exits cleanly without corrupting the terminal.

**Risks for Sub-stage 6A**: The terminal setup/teardown must be robust — a panic that doesn't restore the terminal leaves the user's shell in raw mode. Use `std::panic::set_hook` or a Drop guard to ensure cleanup. This is the most common source of "damaged terminal" bugs in ratatui apps.

---

## Sub-stage 6B: Core Reusable Widgets

Build the input widgets that wizard steps will compose. Each widget has internal state (testable) and a `render()` method (visual, not unit-tested).

### Task 6B.1: TextInput widget

- **Tests**: In `src/ui/widgets.rs`:
  - `test_text_input_new`: `TextInput::new("Username")` has empty value, label "Username", cursor at 0.
  - `test_text_input_typing`: Sending chars 'a', 'l', 'i', 'c', 'e' → `value()` == "alice", cursor at 5.
  - `test_text_input_backspace`: After typing "alice", backspace → "alic", cursor at 4.
  - `test_text_input_backspace_empty`: Backspace on empty input is no-op, cursor stays at 0.
  - `test_text_input_cursor_left_right`: After typing "alice", left 2 → cursor at 3, right 1 → cursor at 4.
  - `test_text_input_insert_middle`: After typing "alice", left 3, type 'X' → "alXice", cursor at 3.
  - `test_text_input_home_end`: Home → cursor 0, End → cursor at len.
  - `test_text_input_with_initial`: `TextInput::with_value("Username", "alice")` → `value()` == "alice".
  - `test_text_input_clear`: After typing "alice", `clear()` → empty, cursor 0.
- **Code**: In `src/ui/widgets.rs`:
  - `struct TextInput { label: String, value: String, cursor: usize, focused: bool }`
  - Methods: `new(label)`, `with_value(label, value)`, `value() -> &str`, `handle_key(KeyEvent)`, `clear()`, `set_focused(bool)`.
  - `fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme)` — renders a labeled text input with cursor indicator.
- **Verify**: `cargo test --lib ui::widgets::test_text_input`

### Task 6B.2: Toggle widget

- **Tests**: In `src/ui/widgets.rs`:
  - `test_toggle_new`: `Toggle::new("Show Icons")` has value `false`.
  - `test_toggle_flip`: After `handle_key(Enter)`, value is `true`. Again → `false`.
  - `test_toggle_space`: `handle_key(Space)` also flips the toggle.
  - `test_toggle_with_value`: `Toggle::with_value("Show Icons", true)` → `value()` == `true`.
- **Code**: In `src/ui/widgets.rs`:
  - `struct Toggle { label: String, value: bool, focused: bool }`
  - Methods: `new(label)`, `with_value(label, val)`, `value() -> bool`, `handle_key(KeyEvent)`, `set_focused(bool)`.
  - `fn render(...)` — renders `[x] Label` or `[ ] Label` with focus indicator.
- **Verify**: `cargo test --lib ui::widgets::test_toggle`

### Task 6B.3: SearchableList widget (multi-select with search filter)

- **Tests**: In `src/ui/widgets.rs`:
  - `test_searchable_list_new`: `SearchableList::new(vec!["Rust", "Ruby", "Python"])` has all items visible, none selected, search empty.
  - `test_searchable_list_filter`: After typing "ru" in search → visible items are ["Rust", "Ruby"].
  - `test_searchable_list_filter_case_insensitive`: Typing "RU" also matches ["Rust", "Ruby"].
  - `test_searchable_list_filter_clear`: Clearing search → all items visible again.
  - `test_searchable_list_select`: Navigate to "Rust", press Enter → "Rust" is in `selected()`. Press Enter again → deselected.
  - `test_searchable_list_select_multiple`: Select "Rust" and "Python" → `selected()` contains both.
  - `test_searchable_list_navigate`: Down arrow moves highlight from index 0 to 1. Up from 0 is no-op.
  - `test_searchable_list_navigate_wraps`: Down past last item stays at last (or wraps to first — decide convention).
  - `test_searchable_list_filter_preserves_selection`: Select "Rust", filter to "py", select "Python", clear filter → `selected()` == ["Rust", "Python"].
  - `test_searchable_list_categorized`: `SearchableList::categorized(vec![("Languages", vec!["Rust", "Python"]), ("Tools", vec!["Docker"])])` shows categories as headers.
- **Code**: In `src/ui/widgets.rs`:
  - `struct SearchableList { items: Vec<ListItem>, search: TextInput, highlight: usize, mode: ListMode }` where `ListItem { label: String, category: Option<String>, selected: bool }`.
  - `enum ListMode { Search, Navigate }` — Tab switches between search box and list navigation.
  - Methods: `new(items)`, `categorized(categories)`, `handle_key(KeyEvent)`, `selected() -> Vec<&str>`, `visible_items() -> Vec<&ListItem>`.
  - `fn render(...)` — search box at top, scrollable list below with `[x]`/`[ ]` indicators and category headers.
- **Verify**: `cargo test --lib ui::widgets::test_searchable_list`

### Task 6B.4: ListInput widget (dynamic add/remove text entries)

- **Tests**: In `src/ui/widgets.rs`:
  - `test_list_input_new`: `ListInput::new("Repos")` has empty entries list.
  - `test_list_input_add`: Type "alice/cool-cli", press Enter → `entries()` == ["alice/cool-cli"], input clears.
  - `test_list_input_add_empty_rejected`: Press Enter with empty input → nothing added.
  - `test_list_input_remove`: With two entries, navigate to first, press Delete/Backspace → removed, one entry remains.
  - `test_list_input_navigate`: Down/Up arrows move selection among existing entries.
  - `test_list_input_mode_switch`: Tab switches between "add new" input mode and "browse existing" list mode.
- **Code**: In `src/ui/widgets.rs`:
  - `struct ListInput { label: String, entries: Vec<String>, input: TextInput, highlight: usize, mode: ListInputMode }`
  - `enum ListInputMode { Adding, Browsing }`
  - Methods: `new(label)`, `with_entries(label, entries)`, `entries() -> &[String]`, `handle_key(KeyEvent)`.
  - `fn render(...)` — text input at top for adding, list of entries below with delete capability.
- **Verify**: `cargo test --lib ui::widgets::test_list_input`

### Task 6B.5: SingleSelect widget (pick one from a list)

- **Tests**: In `src/ui/widgets.rs`:
  - `test_single_select_new`: `SingleSelect::new("Template", vec!["Minimal", "Full", "Multi-Column"])` has first item highlighted, none confirmed.
  - `test_single_select_navigate`: Down moves highlight. Up from 0 is no-op.
  - `test_single_select_confirm`: Press Enter → `selected()` returns highlighted item.
  - `test_single_select_with_default`: `SingleSelect::with_default(..., 1)` starts with index 1 highlighted.
- **Code**: In `src/ui/widgets.rs`:
  - `struct SingleSelect { label: String, options: Vec<String>, highlight: usize, selected: Option<usize>, focused: bool }`
  - Methods: `new(label, options)`, `with_default(label, options, default)`, `handle_key(KeyEvent)`, `selected() -> Option<&str>`, `selected_index() -> Option<usize>`.
  - `fn render(...)` — vertical list with radio-button style `(o)`/`( )` indicators.
- **Verify**: `cargo test --lib ui::widgets::test_single_select`

### Task 6B.6: PairedInput widget (two-field add: label + value)

- **Tests**: In `src/ui/widgets.rs`:
  - `test_paired_input_new`: `PairedInput::new("Articles", "Title", "URL")` has empty entries, both fields empty.
  - `test_paired_input_add`: Type "My Post" in first field, Tab to second, type "https://example.com/post", Enter → `entries()` == vec![("My Post", "https://example.com/post")]. Both fields clear.
  - `test_paired_input_incomplete_rejected`: Enter with only first field filled → does not add (both fields required).
  - `test_paired_input_both_empty_rejected`: Enter with both fields empty → nothing added.
  - `test_paired_input_remove`: With two entries, navigate to first in browse mode, Delete → removed.
  - `test_paired_input_tab_cycle`: Tab cycles: first field → second field → first field (within add mode). Tab from browse mode enters add mode.
  - `test_paired_input_mode_switch`: Tab in browse mode switches to add mode. Shift-Tab in add mode switches to browse mode.
- **Code**: In `src/ui/widgets.rs`:
  - `struct PairedInput { label: String, first_label: String, second_label: String, entries: Vec<(String, String)>, first: TextInput, second: TextInput, highlight: usize, mode: PairedInputMode, focused_field: PairedField }`
  - `enum PairedInputMode { Adding, Browsing }`
  - `enum PairedField { First, Second }`
  - Methods: `new(label, first_label, second_label)`, `with_entries(...)`, `entries() -> &[(String, String)]`, `handle_key(KeyEvent)`.
  - `fn render(...)` — two text inputs side-by-side at top (labeled "Title" and "URL"), list of paired entries below as "Title — URL" rows.
- **Verify**: `cargo test --lib ui::widgets::test_paired_input`

**Risks for Sub-stage 6B**: The `SearchableList` widget is the most complex. Maintaining selection state across filter changes requires care — selected items are tracked by identity (label string), not by index. Test this thoroughly.

---

## Sub-stage 6C: Wizard Steps 1-3 (Welcome, Identity, About)

These are the simplest steps — mostly text inputs and a mode selector. Build them to establish the pattern that all later steps will follow.

### Task 6C.1: Step trait / step handler pattern

- **Tests**: In `src/ui/wizard.rs`:
  - `test_step_handler_dispatch`: Given `WizardStep::Welcome`, `dispatch_step()` returns the correct handler type.
- **Code**: In `src/ui/wizard.rs`:
  - Define the pattern for step handlers. Two options:
    - **Option A (trait)**: `trait StepHandler { fn handle_key(&mut self, key: KeyEvent, config: &mut ProfileConfig) -> StepAction; fn render(&self, frame: &mut Frame, area: Rect, config: &ProfileConfig, theme: &Theme); }`
    - **Option B (enum)**: Each step is a variant of `enum StepState` holding its widget state, with match arms in `handle_key` and `render`.
  - `enum StepAction { Continue, NextStep, PrevStep, Generate, Quit }` — returned by key handlers to tell the app loop what to do.
  - Recommend **Option B** for simplicity — no dynamic dispatch, all state in one enum.
- **Verify**: `cargo test --lib ui::wizard::test_step_handler`

### Task 6C.2: Welcome step (mode selection)

- **Tests**: In `src/ui/wizard.rs`:
  - `test_welcome_step_initial`: Welcome step shows two options: Basic, Advanced. Basic is highlighted by default.
  - `test_welcome_step_select_advanced`: Navigate down, press Enter → returns `StepAction::NextStep` and sets mode to `Advanced`.
  - `test_welcome_step_select_basic`: Press Enter on default → returns `StepAction::NextStep` and mode is `Basic`.
  - `test_welcome_step_quit`: Press Ctrl-C → returns `StepAction::Quit`.
- **Code**: In `src/ui/wizard.rs`:
  - `struct WelcomeState { select: SingleSelect }` — wraps a `SingleSelect` with options ["Basic — Generate README.md directly", "Advanced — Generate README.md + editable profile.toml"].
  - `fn handle_key(key, config) -> StepAction` — delegates to SingleSelect, on confirm sets mode and returns NextStep.
  - `fn render(frame, area, theme)` — centered welcome message, app name/version, mode selector below.
- **Verify**: `cargo test --lib ui::wizard::test_welcome_step`

### Task 6C.3: Identity step (username, display name, tagline)

- **Tests**: In `src/ui/wizard.rs`:
  - `test_identity_step_initial`: Three text inputs: username (empty), display name (empty), tagline (empty). Username is focused.
  - `test_identity_step_fill`: Type "alice" in username → Tab → type "Alice" in name → Tab → type "Rust dev" in tagline → Enter → `config.meta.username` == "alice", `config.meta.name` == Some("Alice"), `config.header.tagline` == Some("Rust dev").
  - `test_identity_step_tab_cycle`: Tab cycles focus through username → display name → tagline → username.
  - `test_identity_step_username_required`: Press Enter with empty username → does NOT advance (stays on step). With non-empty username → advances.
  - `test_identity_step_back`: Esc → returns `PrevStep`.
- **Code**: In `src/ui/wizard.rs`:
  - `struct IdentityState { username: TextInput, display_name: TextInput, tagline: TextInput, focused_field: usize }`
  - Tab/BackTab cycles `focused_field` 0..3.
  - Enter validates username non-empty, copies values to `config.meta` and `config.header`, returns `NextStep`.
  - Esc returns `PrevStep`.
  - `fn render(...)` — step title "Identity", three labeled text inputs stacked vertically, help text at bottom.
- **Verify**: `cargo test --lib ui::wizard::test_identity_step`

### Task 6C.4: About step (role, company, current work, learning, fun fact, pronouns, location)

- **Tests**: In `src/ui/wizard.rs`:
  - `test_about_step_initial`: Seven text inputs, all optional. First is focused.
  - `test_about_step_fill_partial`: Fill role and current_work, leave others empty, press Enter → `config.about.role` == Some("Backend Engineer"), `config.about.current_work` == Some("my-project"), other fields None.
  - `test_about_step_all_optional`: Press Enter with all fields empty → advances (all fields are optional), `config.about` has all None values.
  - `test_about_step_tab_cycle`: Tab cycles through all 7 fields.
  - `test_about_step_back`: Esc → `PrevStep`.
  - `test_about_step_preserves_on_back`: Fill role, go to next step, come back → role field still has its value.
- **Code**: In `src/ui/wizard.rs`:
  - `struct AboutState { fields: Vec<TextInput>, focused_field: usize }` — 7 fields: role, company, current_work, learning, reach_me, fun_fact, pronouns.
  - Enter copies non-empty values to `config.about` as `Some(value)`, empty as `None`.
  - `fn render(...)` — step title "About Me", scrollable list of labeled text inputs. Help text: "All fields optional — press Enter to skip."
- **Verify**: `cargo test --lib ui::wizard::test_about_step`

### Task 6C.5: Render the step progress bar / navigation chrome

- **Tests**: None (purely visual).
- **Code**: In `src/ui/wizard.rs` or `src/ui/app.rs`:
  - `fn render_chrome(frame, area, current_step, theme)` — renders:
    - Top bar: step progress indicator (e.g., "Step 3/12 — About" or a graphical progress bar showing filled/unfilled segments).
    - Bottom bar: keyboard shortcuts help line. Context-sensitive per step:
      - Text input steps: "Tab: next field | Enter: continue | Esc: back | Ctrl-G: generate now | Ctrl-C: quit"
      - List/toggle steps: "Enter: toggle | →: continue | Esc: back | Ctrl-G: generate now | Ctrl-C: quit"
  - The main wizard render function calls `render_chrome()` first, then delegates the inner area to the current step's render.
- **Verify**: Visual — `cargo run` shows progress bar and help line on every step.

**Risks for Sub-stage 6C**: The "preserves on back" behavior (6C.4) means step state must live in `WizardState`, not be recreated on each step visit. Initialize all step states upfront in `WizardState::new()`.

---

## Sub-stage 6D: Wizard Steps 4-5 (Social Links, Skills)

These steps use the more complex widgets: list input for social links, searchable multi-select for skills.

### Task 6D.1: Social links step

- **Tests**: In `src/ui/wizard.rs`:
  - `test_social_step_initial`: Shows list of social platforms (Twitter, LinkedIn, Mastodon, ...) with empty URL fields.
  - `test_social_step_fill`: Navigate to Twitter row, type URL, Tab to LinkedIn, type URL → `config.social.twitter` == Some("https://twitter.com/alice"), `config.social.linkedin` == Some("https://linkedin.com/in/alice").
  - `test_social_step_skip_empty`: Leave all URLs empty, press Enter → `config.social` has all None. Step advances.
  - `test_social_step_scroll`: With 18 platforms, the list scrolls vertically. Down arrow past visible area scrolls the view.
  - `test_social_step_back`: Esc → `PrevStep`.
- **Code**: In `src/ui/wizard.rs`:
  - `struct SocialState { fields: Vec<(String, TextInput)>, focused: usize, scroll_offset: usize }` — one `TextInput` per platform.
  - The platform list: GitHub, Twitter/X, LinkedIn, Mastodon, Bluesky, Instagram, YouTube, Discord, Dev.to, Hashnode, Medium, StackOverflow, Reddit, Twitch, Personal Website, Email, Ko-fi, RSS Feed.
  - Enter copies non-empty values into the corresponding `config.social` fields.
  - `fn render(...)` — scrollable two-column layout: platform name on left, text input on right.
- **Verify**: `cargo test --lib ui::wizard::test_social_step`

### Task 6D.2: Skills step (searchable categorized multi-select)

- **Tests**: In `src/ui/wizard.rs`:
  - `test_skills_step_initial`: Shows categorized skill list with search box. Nothing selected.
  - `test_skills_step_search`: Type "rust" → only items containing "rust" shown (e.g., "Rust" under Languages, "Actix" not shown).
  - `test_skills_step_select`: Navigate to "Rust", press Enter → selected. Navigate to "Python", press Enter → selected. `config.skills.languages` == vec!["Rust", "Python"].
  - `test_skills_step_deselect`: Select "Rust", then toggle again → deselected. Not in config.
  - `test_skills_step_categories`: Items are grouped under "Languages", "Frameworks", "Tools", "Databases", "Cloud/Infra" headers.
  - `test_skills_step_back_preserves`: Select skills, go back, return → selections preserved.
- **Code**: In `src/ui/wizard.rs`:
  - `struct SkillsState { list: SearchableList }` — wraps a categorized `SearchableList`.
  - Built-in skill catalog as a const: Languages (Rust, Python, TypeScript, JavaScript, Go, C, C++, Java, Kotlin, Swift, Ruby, PHP, Elixir, Haskell, Scala, Zig, Lua, Dart, R, Shell/Bash), Frameworks (React, Vue, Svelte, Angular, Next.js, Actix, Axum, Rocket, Django, Flask, FastAPI, Rails, Spring, Express, Gin, Echo), Tools (Docker, Git, Neovim, VS Code, GitHub Actions, Terraform, Ansible, Kubernetes, Nginx, Linux), Databases (PostgreSQL, MySQL, SQLite, MongoDB, Redis, DynamoDB, CockroachDB, ClickHouse), Cloud/Infra (AWS, GCP, Azure, Vercel, Cloudflare, DigitalOcean, Fly.io, Heroku).
  - Enter toggles the highlighted item. **Right arrow advances to the next step.** This convention applies here and in any other step where Enter has a widget-specific meaning.
  - On advance, copies selected items into `config.skills` by category.
  - `fn render(...)` — search box at top, categorized scrollable list, selected count indicator. Help bar shows "Enter: toggle | →: continue | Esc: back".
- **Verify**: `cargo test --lib ui::wizard::test_skills_step`

**Risks for Sub-stage 6D**: The right-arrow-to-advance convention must be documented in the help bar on every step that uses it (Skills, Stats toggles, Projects list). Ensure right arrow doesn't conflict with TextInput cursor movement — when a TextInput is focused, right arrow moves the cursor; only when the list/toggle area is focused does right arrow advance the step.

---

## Sub-stage 6E: Wizard Steps 6-10 (Stats, Projects, Blog, Dynamic, Extras)

These steps are structurally simpler — mostly toggle switches and list inputs. Several can reuse patterns established in 6C and 6D.

### Task 6E.1: Stats step (toggle switches + theme selector)

- **Tests**: In `src/ui/wizard.rs`:
  - `test_stats_step_initial`: All toggles off. Theme is "tokyonight" by default.
  - `test_stats_step_toggle`: Toggle "GitHub Stats Card" on → `config.stats.stats_card` == true.
  - `test_stats_step_theme`: Select "radical" theme → `config.stats.theme` == "radical".
  - `test_stats_step_multiple`: Enable stats_card, streak, top_langs → all three true in config.
  - `test_stats_step_hide_border`: Toggle hide_border → `config.stats.hide_border` == true.
- **Code**: In `src/ui/wizard.rs`:
  - `struct StatsState { toggles: Vec<(String, Toggle)>, theme_select: SingleSelect, focused: usize }`.
  - Toggles: GitHub Stats Card, Top Languages, Streak Stats, Contributor Stats, GitHub Trophies, Contribution Snake, Profile Views Counter, Hide Border.
  - Theme options: default, dark, radical, merko, gruvbox, tokyonight, onedark, cobalt, synthwave, highcontrast, dracula.
  - `fn render(...)` — toggles in a vertical list, theme selector below.
- **Verify**: `cargo test --lib ui::wizard::test_stats_step`

### Task 6E.2: Projects step (add repos)

- **Tests**: In `src/ui/wizard.rs`:
  - `test_projects_step_add`: Type "alice/cool-cli", Enter → added to list. `config.projects.repos` == vec!["alice/cool-cli"].
  - `test_projects_step_add_multiple`: Add two repos → both in config.
  - `test_projects_step_remove`: Add a repo, select it, press Delete → removed.
  - `test_projects_step_empty_ok`: Enter with empty list → `config.projects.repos` == vec![]. Step advances.
  - `test_projects_step_display_style`: Toggle between "Pin Cards" and "Markdown Table" → `config.projects.display` changes.
- **Code**: In `src/ui/wizard.rs`:
  - `struct ProjectsState { list: ListInput, display_toggle: SingleSelect }`.
  - `fn render(...)` — list input for repos, display style selector, help text: "Enter repos as owner/repo (e.g., alice/cool-cli)".
- **Verify**: `cargo test --lib ui::wizard::test_projects_step`

### Task 6E.3: Blog & Content step

- **Tests**: In `src/ui/wizard.rs`:
  - `test_blog_step_rss`: Type RSS URL → `config.blog.rss_urls` == vec!["https://..."].
  - `test_blog_step_manual_articles`: Add two articles via PairedInput (title + URL) → `config.blog.articles` has two entries with correct title and URL.
  - `test_blog_step_youtube`: Type YouTube channel URL → `config.blog.youtube` == Some("...").
  - `test_blog_step_newsletter`: Type newsletter URL → `config.blog.newsletter` == Some("...").
  - `test_blog_step_empty_ok`: All empty → advances with no blog config set.
- **Code**: In `src/ui/wizard.rs`:
  - `struct BlogState { rss_list: ListInput, articles: PairedInput, youtube: TextInput, newsletter: TextInput, focused: usize }`.
  - Articles use `PairedInput::new("Articles", "Title", "URL")` — two-field input with Title and URL side-by-side.
  - `fn render(...)` — four sections: RSS feeds (ListInput), manual articles (PairedInput), YouTube (TextInput), newsletter (TextInput).
- **Verify**: `cargo test --lib ui::wizard::test_blog_step`

### Task 6E.4: Dynamic integrations step

- **Tests**: In `src/ui/wizard.rs`:
  - `test_dynamic_step_spotify`: Type Spotify UID → `config.dynamic.spotify_uid` == Some("USER").
  - `test_dynamic_step_wakatime`: Toggle WakaTime on → `config.dynamic.wakatime` == true.
  - `test_dynamic_step_activity`: Toggle GitHub Activity on → `config.dynamic.github_activity` == true.
  - `test_dynamic_step_stackoverflow`: Type SO user ID → `config.dynamic.stackoverflow_uid` == Some("12345").
  - `test_dynamic_step_empty_ok`: All empty/off → advances.
- **Code**: In `src/ui/wizard.rs`:
  - `struct DynamicState { spotify: TextInput, wakatime: Toggle, github_activity: Toggle, stackoverflow: TextInput, focused: usize }`.
  - `fn render(...)` — mix of text inputs and toggles for each dynamic service.
- **Verify**: `cargo test --lib ui::wizard::test_dynamic_step`

### Task 6E.5: Extras step

- **Tests**: In `src/ui/wizard.rs`:
  - `test_extras_step_pgp`: Type PGP fingerprint → `config.extras.pgp_fingerprint` == Some("...").
  - `test_extras_step_gaming`: Type Xbox gamertag → `config.extras.xbox` == Some("...").
  - `test_extras_step_custom_markdown`: Add a raw markdown block → `config.extras.custom_blocks` contains it.
  - `test_extras_step_empty_ok`: All empty → advances.
- **Code**: In `src/ui/wizard.rs`:
  - `struct ExtrasState { pgp: TextInput, xbox: TextInput, steam: TextInput, psn: TextInput, custom_blocks: ListInput, focused: usize }`.
  - Custom markdown blocks accept multi-line input. Simplest approach: single-line entries that the user can use `\n` escapes in, or a note that multi-line custom blocks are better done via TOML editing.
  - `fn render(...)` — text inputs for each extra, list input for custom blocks.
- **Verify**: `cargo test --lib ui::wizard::test_extras_step`

Tasks 6E.1 through 6E.5 are **independent of each other** and can be implemented in parallel.

**Risks for Sub-stage 6E**: The blog step's `PairedInput` widget (Task 6B.6) needs enough horizontal space for two side-by-side text inputs. On narrow terminals (<80 cols), consider stacking them vertically. Test at minimum 80-column width.

---

## Sub-stage 6F: Wizard Steps 11-12 (Layout, Preview & Generate)

The final steps. Layout is a selection screen. Preview renders the markdown inline and triggers file output.

### Task 6F.1: Layout step (template + options)

- **Tests**: In `src/ui/wizard.rs`:
  - `test_layout_step_initial`: Default template is "Full-Featured". Dark mode off. Alignment "left".
  - `test_layout_step_select_minimal`: Select "Minimal" → `config.layout.template` == Template::Minimal.
  - `test_layout_step_dark_mode`: Toggle dark mode on → `config.layout.dark_mode` == true.
  - `test_layout_step_centered`: Select "Centered" alignment → `config.layout.centered` == true.
- **Code**: In `src/ui/wizard.rs`:
  - `struct LayoutState { template_select: SingleSelect, dark_mode: Toggle, centered: Toggle, focused: usize }`.
  - Template options: Minimal, Full-Featured, Developer Card, Multi-Column.
  - `fn render(...)` — template selector (with short descriptions of each), dark mode toggle, alignment toggle.
- **Verify**: `cargo test --lib ui::wizard::test_layout_step`

### Task 6F.2: Preview & Generate step (with rendered markdown preview)

- **Tests**: In `src/ui/wizard.rs`:
  - `test_generate_step_basic_mode`: In Basic mode, `generate_result()` has `write_toml == false`, `readme_content` is non-empty.
  - `test_generate_step_advanced_mode`: In Advanced mode, `generate_result()` has `write_toml == true`, `toml_content` is non-empty.
  - `test_generate_step_calls_renderer`: The raw markdown matches calling `render::render(&config)` directly.
  - `test_generate_step_termimad_render`: The preview text is non-empty and differs from raw markdown (termimad adds ANSI styling).
- **Code**: In `src/ui/wizard.rs`:
  - `struct PreviewGenerateState { raw_markdown: String, rendered_preview: String, preview_scroll: usize, generated: bool }`.
  - `struct GenerateResult { readme_content: String, toml_content: Option<String>, write_toml: bool }`.
  - On entering this step:
    1. Call `render::render(&config)` from Stages 3-4 to produce raw markdown.
    2. Pass the raw markdown through `termimad::text(...)` (or `MadSkin::term_text()`) to produce terminal-styled output for the preview.
  - Display the rendered preview in a scrollable ratatui `Paragraph` widget with `.scroll()`. termimad output includes ANSI codes for headers, bold, links, code blocks, lists — these render natively in ratatui `Paragraph` with `Style` spans.
  - Keybindings: Up/Down/PageUp/PageDown to scroll preview. Enter/Ctrl-G to confirm and write files. Esc to go back.
  - On confirm: write the **raw markdown** (not the terminal-rendered version) to `README.md`.
  - `fn render(...)` — full-width rendered markdown preview with scroll indicator, action bar at bottom ("Enter: generate | Esc: back | ↑↓: scroll").
- **Code**: In `src/ui/app.rs`:
  - Wire the generate step's confirm action to file I/O:
    1. `restore_terminal()` (leave TUI)
    2. Write `README.md` using `std::fs::write` with the **raw markdown**
    3. If Advanced mode, write `profile.toml` using `config::toml_io::save_config()`
    4. Print success message to stdout: "Wrote README.md" / "Wrote README.md and profile.toml"
- **Verify**: `cargo test --lib ui::wizard::test_generate_step` for state logic. Manual verification for the full flow: `cargo run`, fill in a few fields, navigate to Preview (verify headers/badges/links render with terminal styling), press Enter, verify files contain raw markdown.

### Task 6F.3: Wire Ctrl-G (generate early) across all steps

- **Tests**: In `src/ui/wizard.rs`:
  - `test_ctrl_g_from_about`: From the About step with username set, pressing Ctrl-G → `StepAction::Generate`. Resulting `ProfileConfig` has the username and any about fields filled in.
  - `test_ctrl_g_without_username`: From the Welcome step (no username), pressing Ctrl-G → does NOT generate (username is required). Shows an error indicator.
- **Code**: In each step's `handle_key` (or in the shared dispatch logic in `app.rs`):
  - Match `KeyCode::Char('g')` with `KeyModifiers::CONTROL` → if `config.meta.username` is non-empty, return `StepAction::Generate`. Otherwise, flash an error message: "Username is required to generate."
  - In the app loop, `StepAction::Generate` triggers the same file-writing logic as the PreviewGenerate step's confirm.
- **Verify**: `cargo test --lib ui::wizard::test_ctrl_g`

**Risks for Sub-stage 6F**: `termimad` renders markdown to terminal-styled text with ANSI codes. The integration with ratatui requires converting termimad output to ratatui `Spans`/`Line` types. Two approaches: (a) use `termimad::MadSkin::write_in()` to write to a buffer, then wrap in a ratatui `Paragraph`; (b) use `termimad::Area` for direct rendering, bypassing ratatui for the preview area. Option (a) is cleaner since ratatui controls the full layout. If termimad's ANSI output doesn't integrate cleanly with ratatui's `Paragraph`, fall back to a `tui-markdown` crate or render markdown with simple manual styling (headers bold, code dimmed, links underlined).

---

## Coverage Validation

| Acceptance Criterion | Task |
|---------------------|------|
| WizardState initial/advance/back | 6A.3, 6A.4 |
| Generate early (Ctrl-G) | 6F.3 |
| Identity → meta config | 6C.3 |
| About → about config | 6C.4 |
| Social add/remove | 6D.1 |
| Skills search/filter/toggle | 6D.2 |
| Stats toggles | 6E.1 |
| Projects add | 6E.2 |
| Layout select | 6F.1 |
| Basic mode no TOML / Advanced writes TOML | 6F.2 |
| TextInput typing/backspace/cursor | 6B.1 |
| Toggle flip | 6B.2 |
| SearchableList filter/select | 6B.3 |
| ListInput add/remove | 6B.4 |

All acceptance criteria are covered by at least one task.

---

## Summary

| Sub-stage | Tasks | Files | Theme |
|-----------|-------|-------|-------|
| **6A** | 5 tasks | Cargo.toml, src/ui/mod.rs, src/ui/theme.rs, src/ui/wizard.rs, src/ui/app.rs, src/main.rs | Infrastructure + state machine |
| **6B** | 6 tasks | src/ui/widgets.rs | Reusable input widgets (TextInput, Toggle, SearchableList, ListInput, SingleSelect, PairedInput) |
| **6C** | 5 tasks | src/ui/wizard.rs | Steps 1-3 (Welcome, Identity, About) + chrome |
| **6D** | 2 tasks | src/ui/wizard.rs | Steps 4-5 (Social, Skills) |
| **6E** | 5 tasks | src/ui/wizard.rs | Steps 6-10 (Stats, Projects, Blog, Dynamic, Extras) |
| **6F** | 3 tasks | src/ui/wizard.rs, src/ui/app.rs | Steps 11-12 (Layout, Preview & Generate) + Ctrl-G |

**Total**: 26 tasks across 6 sub-stages.

**Parallelism opportunities**:
- 6A and 6B are independent — can be done in parallel.
- 6E.1 through 6E.5 are independent of each other.
- 6C, 6D, 6E all depend on 6A + 6B.
- 6F depends on everything before it.

**Resolved decisions**:
1. **Step advancement in list/toggle steps**: Right arrow (→) advances to the next step. Enter is reserved for toggling items / confirming selections within the step. Help bar documents this on every applicable step.
2. **Blog article input**: Two-field `PairedInput` widget (Title + URL side-by-side). New widget added as Task 6B.6.
3. **Preview rendering**: `termimad` crate renders markdown with terminal styling (bold headers, underlined links, dimmed code blocks). Raw markdown is still written to the output file.
