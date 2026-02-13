# gh-profile-gen

Generate comprehensive GitHub profile README.md files from a simple TOML configuration.

## Features

- **30+ content types**: Headers, about sections, social badges, skill icons, GitHub stats, pinned projects, blog posts, dynamic integrations, and more
- **4 layout templates**: Minimal, Full-Featured, Developer Card, Multi-Column
- **shields.io badges**: Auto-generated for-the-badge style badges with simple-icons
- **GitHub stats integration**: Stats cards, streak stats, top languages, trophies, contribution graphs
- **Dynamic content**: Spotify now-playing, WakaTime stats, GitHub activity, StackOverflow flair
- **TUI wizard** (in progress): Interactive terminal UI for building profiles step-by-step
- **TOML round-trip**: Edit your profile config as a human-readable TOML file

## Installation

```bash
cargo install --path .
```

## Quick Start

```bash
# Generate a starter profile.toml with all fields documented
gh-profile-gen init

# Edit profile.toml with your information
$EDITOR profile.toml

# Render to README.md
gh-profile-gen render profile.toml

# Preview in terminal
gh-profile-gen preview profile.toml
```

## Usage

### `init` — Generate a starter config

```bash
gh-profile-gen init                    # Creates profile.toml
gh-profile-gen init -o my-profile.toml # Custom output path
gh-profile-gen init --force            # Overwrite existing file
```

### `render` — Render TOML to README.md

```bash
gh-profile-gen render profile.toml              # Writes README.md
gh-profile-gen render profile.toml -o output.md # Custom output path
gh-profile-gen render profile.toml --stdout      # Print to stdout
```

### `preview` — Preview rendered markdown

```bash
gh-profile-gen preview profile.toml
```

## Configuration

The TOML config has these sections (all optional except `[meta]`):

| Section | Description |
|---------|-------------|
| `[meta]` | **Required.** GitHub username, display name |
| `[header]` | Header style: text, typing SVG, wave, banner image |
| `[about]` | Role, company, current work, learning, fun fact |
| `[social]` | Links to 18 social platforms |
| `[skills]` | Programming languages, frameworks, tools, databases, cloud |
| `[stats]` | GitHub stats cards, streaks, top languages, trophies |
| `[projects]` | Pinned repos as cards or markdown table |
| `[blog]` | RSS feeds, manual articles, YouTube, newsletter |
| `[dynamic]` | Spotify, WakaTime, GitHub activity, StackOverflow |
| `[layout]` | Template selection, dark mode, centering |
| `[sponsors]` | Sponsor button links |
| `[extras]` | PGP keys, gaming tags, custom markdown blocks |

Run `gh-profile-gen init` to see a fully commented example.

## Building from Source

```bash
git clone https://github.com/hughdbrown/gh-profile-gen.git
cd gh-profile-gen
cargo build --release
```

## Development

```bash
cargo check          # Fast type checking
cargo test           # Run all tests
cargo clippy         # Lint
cargo fmt --check    # Format check
```

## License

MIT
