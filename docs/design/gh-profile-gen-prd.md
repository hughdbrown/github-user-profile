# gh-profile-gen — Product Requirements Document

## Overview

`gh-profile-gen` is a Rust CLI application that generates comprehensive GitHub profile README.md files. It supports two workflows: (1) rendering a README from a hand-editable TOML configuration file, and (2) an interactive TUI wizard that walks users through configuring their profile and outputs both a TOML file and a rendered README. The tool covers 30+ content types discovered across 50+ top GitHub profiles, including dynamic stats cards, tech stack badges, social links, featured projects, and blog post integration.

## Goals

- **G1**: Define a `ProfileConfig` data model (serde-serializable to TOML) covering all content sections: header, about, social links, skills/tech stack, GitHub stats, featured projects, blog/content, dynamic integrations, sponsors, extras, and layout/theming.
- **G2**: Provide a `gh-profile-gen init` command that generates a starter `profile.toml` with all fields present and commented examples.
- **G3**: Provide a `gh-profile-gen render <file.toml>` command that reads a TOML config and outputs a complete README.md.
- **G4**: Provide a `gh-profile-gen preview <file.toml>` command that renders the markdown and displays it in the terminal.
- **G5**: Build URL builders for all supported external services (github-readme-stats, streak-stats, typing-svg, shields.io, profile views, spotify, etc.).
- **G6**: Build a multi-step TUI wizard (`gh-profile-gen` / `gh-profile-gen --advanced`) using ratatui that walks users through configuration and generates output.
- **G7**: Support layout templates (Minimal, Full-Featured, Developer Card, Multi-Column) and GitHub dark/light mode image switching.

## Non-Goals

- **NG1**: GitHub Action generation for dynamic content (blog-post-workflow, waka-readme, etc.) — deferred to v2.
- **NG2**: Fetching live data from GitHub API, crates.io, or any external service at generation time.
- **NG3**: Retro 90s template (complex visual design, low priority).
- **NG4**: Interactive/gamified content (playable games, community wall).
- **NG5**: Auto-detection of pinned repos from GitHub API.
- **NG6**: Validation that provided URLs/usernames actually exist.

## Acceptance Criteria (as test descriptions)

### Data Model & Serialization
1. `test_profile_config_round_trip`: A `ProfileConfig` serialized to TOML and deserialized back produces an identical struct.
2. `test_profile_config_defaults`: A default `ProfileConfig` has all optional sections as `None` or empty vecs, and required fields have sensible defaults.
3. `test_profile_config_partial_toml`: A TOML file with only `[meta]` and `[about]` sections deserializes successfully; missing sections use defaults.
4. `test_profile_config_all_sections`: A TOML file with every section populated deserializes without error.

### URL Builders
5. `test_github_stats_url`: Given username "alice", theme "tokyonight", show_icons=true, hide_border=false → produces correct github-readme-stats URL.
6. `test_top_langs_url`: Given username "alice", layout "compact", langs_count=8 → produces correct top-languages URL.
7. `test_streak_stats_url`: Given username "alice", theme "tokyonight" → produces correct streak-stats.demolab.com URL.
8. `test_typing_svg_url`: Given lines=["Hello", "World"], font="Fira Code", color="f75c7e" → produces correct readme-typing-svg URL.
9. `test_shields_badge_url`: Given label="Rust", color="000000", logo="rust" → produces correct shields.io badge URL.
10. `test_profile_views_url`: Given username "alice" → produces correct komarev.com URL.
11. `test_spotify_url`: Given uid "USER" → produces correct spotify-github-profile URL.
12. `test_pin_card_url`: Given username "alice", repo "cool-cli", theme "tokyonight" → produces correct pin card URL.

### Markdown Rendering
13. `test_render_header_typing_svg`: A config with typing SVG header produces centered `<p>` with `<img>` tag containing the correct URL.
14. `test_render_header_text`: A config with text header produces `## Hey! I'm {name}` format.
15. `test_render_about_section`: A config with about fields produces bullet list with emoji prefixes (🔭, 🌱, 💬, ⚡).
16. `test_render_social_badges`: A config with Twitter and LinkedIn produces for-the-badge style shield badges with correct URLs.
17. `test_render_skills_badges`: A config with Rust, Python, Docker skills produces shields.io badges with simple-icons logos.
18. `test_render_stats_cards`: A config with stats and streak enabled produces two `![](url)` image embeds.
19. `test_render_featured_projects`: A config with two repos produces github-readme-stats pin cards.
20. `test_render_blog_markers`: A config with blog RSS URL produces `<!-- BLOG-POST-LIST:START -->` / `<!-- BLOG-POST-LIST:END -->` comment markers.
21. `test_render_manual_blog_posts`: A config with manual blog entries produces a numbered list with links.
22. `test_render_sponsors`: A config with GitHub Sponsors enabled produces a sponsor badge/link.
23. `test_render_extras_pgp`: A config with PGP fingerprint produces a PGP key badge.
24. `test_render_collapsible_section`: A config with collapsible content produces `<details><summary>` HTML.
25. `test_render_custom_markdown`: A config with raw markdown blocks inserts them verbatim.
26. `test_render_empty_sections_omitted`: Sections with no data configured produce no output (no empty headers).
27. `test_render_dark_light_mode`: A config with dark mode support produces `#gh-dark-mode-only` / `#gh-light-mode-only` suffixed images.

### Layout Templates
28. `test_layout_minimal`: Minimal template includes only header, about, and social — no stats, no extras.
29. `test_layout_full`: Full-Featured template includes all configured sections.
30. `test_layout_multi_column`: Multi-Column template wraps content in HTML table with 2-3 columns.
31. `test_layout_centered`: Centered layout wraps sections in `<p align="center">` or `<div align="center">`.

### CLI
32. `test_cli_init_creates_toml`: Running `init` creates a `profile.toml` file with commented-out example fields.
33. `test_cli_render_produces_readme`: Running `render sample.toml` writes a valid README.md.
34. `test_cli_render_stdout`: Running `render sample.toml --stdout` prints markdown to stdout instead of writing a file.
35. `test_cli_render_missing_file`: Running `render nonexistent.toml` prints a clear error message and exits with code 1.
36. `test_cli_preview`: Running `preview sample.toml` outputs rendered markdown to the terminal.

### Integration
37. `test_full_pipeline`: Load the example `profile.toml` from test fixtures, render to README.md, verify it contains expected sections and URLs.
38. `test_init_then_render`: Run `init`, then `render` the generated TOML — produces a valid (mostly empty) README with just defaults.

## Technical Decisions

### Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `clap` | 4 | CLI argument parsing with derive macros |
| `serde` | 1 | Serialization/deserialization with derive |
| `toml` | 0.8+ | TOML parsing and generation |
| `anyhow` | 1 | Error handling in binary (main, CLI commands) |
| `thiserror` | 2 | Typed errors for library code |
| `ratatui` | 0.29+ | Terminal UI (Stage 6 only) |
| `crossterm` | 0.28+ | Terminal backend for ratatui (Stage 6 only) |
| `termimad` | 0.30+ | Markdown-to-terminal rendering for preview pane (Stage 6 only) |

**Decision: Use semver-compatible ranges** (e.g., `clap = "4"` not `clap = "4.5.58"`) to avoid unnecessary lock-in. Cargo.lock will pin exact versions.

### Architecture

The spec's proposed directory structure is sound. One adjustment: split `render/sections.rs` into individual section modules once it exceeds ~300 lines, but start with a single file.

```
src/
├── main.rs              # Entry point, CLI (clap)
├── error.rs             # thiserror error types
├── config/
│   ├── mod.rs
│   ├── profile.rs       # ProfileConfig and all sub-structs
│   └── toml_io.rs       # load_config(), save_config(), generate_starter()
├── services/
│   ├── mod.rs
│   └── urls.rs           # URL builder functions for external services
├── render/
│   ├── mod.rs
│   ├── markdown.rs       # Top-level render(config) -> String
│   ├── sections.rs       # Individual section renderers
│   └── templates.rs      # Layout template logic
└── ui/                   # (Stage 6 — TUI wizard)
    ├── mod.rs
    ├── app.rs
    ├── wizard.rs
    ├── widgets.rs
    └── theme.rs
```

### Data Model Design

All sections in `ProfileConfig` should be `Option<SectionType>` so that:
- Missing TOML sections deserialize as `None`
- `None` sections produce no output during rendering
- The `init` command can generate a complete template with all sections

Exception: `meta` (contains `username`) is required — not optional.

### Markdown Generation Strategy

Each section renderer is a pure function: `fn render_X(config: &XConfig, meta: &Meta) -> String`. The top-level renderer calls each in order based on the layout template, filters out empty strings, joins with `\n\n---\n\n` or just `\n\n` depending on template.

### shields.io Badge Convention

Use `for-the-badge` style consistently for social links and skill badges. Format:
```
https://img.shields.io/badge/{LABEL}-{COLOR}?style=for-the-badge&logo={LOGO}&logoColor=white
```

The simple-icons slug mapping (e.g., "Rust" → "rust", "TypeScript" → "typescript") will be a const lookup table in `services/urls.rs`.

## Design and Operation

### User Perspective

**`gh-profile-gen init`**
- Creates `profile.toml` in the current directory (or specified path with `-o`)
- The TOML file has every section with example values commented out
- User hand-edits the file, uncommenting and filling in what they want

**`gh-profile-gen render <file.toml>`**
- Reads the TOML, validates required fields (at minimum: `meta.username`)
- Renders a complete README.md
- Writes to `README.md` in the current directory (or `-o <path>`)
- Flag `--stdout` prints to stdout instead

**`gh-profile-gen preview <file.toml>`**
- Same as render but prints to stdout (alias for `render --stdout`)

**`gh-profile-gen` (no subcommand) / `gh-profile-gen --advanced`**
- Launches TUI wizard (Stage 6)

### System Perspective — Render Pipeline

```
TOML file → deserialize → ProfileConfig → template selects sections →
  each section renderer → Vec<String> → join with separators → README.md string → write file
```

### Error Handling

| Error | Handling |
|-------|----------|
| TOML file not found | `anyhow` context: "Could not open {path}: {io_error}" |
| TOML parse error | `anyhow` context: "Failed to parse {path}: {toml_error}" |
| Missing `meta.username` | `thiserror` variant: `ConfigError::MissingUsername` |
| Invalid template name | `thiserror` variant: `ConfigError::UnknownTemplate(String)` |
| Output file write error | `anyhow` context: "Could not write {output_path}: {io_error}" |

### Edge Cases

- **Empty config (only meta)**: Renders a minimal README with just the username as a heading.
- **All sections enabled**: Renders everything in template order — no section interferes with another.
- **Unicode in bio/tagline**: Passed through verbatim — markdown handles UTF-8.
- **Very long skill lists**: Rendered as-is; no wrapping logic (GitHub renders the badges inline and wraps naturally).
- **Missing logo slug**: If a skill name doesn't have a simple-icons mapping, render the badge without a logo.

## Test Strategy

**Unit tests** (most coverage here):
- URL builder functions: pure input→output, one test per builder
- Section renderers: pure `&Config → String`, test expected markdown output
- TOML round-trip: serialize/deserialize identity
- Template selection: given a layout enum, correct sections are included

**Integration tests** (in `tests/` directory):
- Full pipeline: sample TOML fixture → render → verify output contains expected patterns
- CLI integration: run the binary with `assert_cmd` crate, check exit codes and output

**Test fixtures** (in `tests/fixtures/`):
- `minimal.toml` — only `[meta]`
- `full.toml` — every section populated
- `expected_minimal.md` — expected output for minimal
- `expected_full.md` — expected output for full (or use `contains` assertions rather than exact match)

**Not needed**: No concurrency tests, no performance tests, no network tests. This is a pure offline generator.

## Rollback and Safety

This is a greenfield project with no existing users. No rollback concerns. The tool only writes files the user explicitly requests (`profile.toml`, `README.md`). The `init` command should refuse to overwrite an existing `profile.toml` unless `--force` is passed.

## Implementation Stages

### Stage 1: Project Scaffold + Data Model + TOML I/O
- Create Cargo.toml, directory structure, error types
- Define `ProfileConfig` and all sub-structs with serde derives
- Implement `load_config()`, `save_config()`, `generate_starter_toml()`
- **Deliverable**: `cargo test` passes — config round-trips through TOML
- **Files**: Cargo.toml, src/main.rs, src/error.rs, src/config/mod.rs, src/config/profile.rs, src/config/toml_io.rs

### Stage 2: URL Builders
- Implement all URL builder functions for external services
- **Deliverable**: `cargo test` passes — all URL builders produce correct output
- **Files**: src/services/mod.rs, src/services/urls.rs

### Stage 3: Markdown Section Renderers
- Implement renderers for each content section (header, about, social, skills, stats, projects, blog, sponsors, extras)
- **Deliverable**: `cargo test` passes — each section renders expected markdown
- **Files**: src/render/mod.rs, src/render/sections.rs

### Stage 4: Layout Templates + Top-Level Renderer
- Implement template logic (Minimal, Full, Developer Card, Multi-Column)
- Wire section renderers into top-level `render()` function
- Add dark/light mode image support
- **Deliverable**: `cargo test` passes — full render produces expected README
- **Files**: src/render/markdown.rs, src/render/templates.rs

### Stage 5: CLI Commands
- Implement `init`, `render`, `preview` subcommands with clap
- Wire CLI to config loading and rendering
- Add integration tests with `assert_cmd`
- **Deliverable**: `cargo run -- init` and `cargo run -- render profile.toml` work end-to-end
- **Files**: src/main.rs, tests/cli_integration.rs, tests/fixtures/*.toml

### Stage 6: TUI Wizard (future — not in this task list)
- Multi-step ratatui wizard
- Depends on Stages 1-5 being stable
