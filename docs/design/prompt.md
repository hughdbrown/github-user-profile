# GitHub Profile README App — Research & Specification

## Part 1: Research Findings — Content Types in GitHub Profile READMEs

Research conducted across 50+ profile READMEs from top Rust developers (orhun, dtolnay, BurntSushi), Python developers (simonw, gautamkrishnar, rednafi), AI content creators, and profiles featured in the [awesome-github-profile-readme](https://github.com/saturn-abhishek/awesome-github-profile-readme) collection (29K+ stars, 700+ commits cataloging profiles).

### Profiles Examined (50+)

**Rust developers:** orhun, dtolnay, BurntSushi, qu4k, teoxoy, Wrapperup, maximousblk, guilyx, gargakshit
**Python developers:** simonw, gautamkrishnar, rednafi, WaylonWalker, mmphego, dayyass, Zhenye-Na, sciencepal
**AI/ML creators:** Rishit-dagli, anmol098, rahul-jha98, terrytangyuan, halfrost
**Prolific OSS:** sindresorhus, anuraghazra, DenverCoder1, ABSphreak, MartinHeinz, adamalston, bdougie, sw-yx, codestackr, athul, roaldnefs, JessicaLim8, tw93, blackcater, Spiderpig86, ouuan, d-koppenhagen, lifeparticle, timburgan, JonathanGin52, rossjrw, marcizhu, natemoo-re, jh3y, f (Fatih Kadir Akın), BrunnerLivio, fnky, caneco, kha7iq, vinitshahdeo, MacroPower, andyruwruw, cheesits456, trinib

### Content Types Catalog

#### 1. Basic / Expected Content
| Content Type | Description | Example Profiles |
|---|---|---|
| **Bio / About Me** | Short personal intro, current role, interests | Nearly all profiles |
| **Highlighted Projects** | Pinned repos or curated project lists | anuraghazra, orhun, dtolnay |
| **Contact Information** | Email, personal website links | gautamkrishnar, DenverCoder1 |
| **Social Media Links** | Twitter/X, LinkedIn, Mastodon, Bluesky, Instagram, Discord | simonw, DenverCoder1, gautamkrishnar |
| **Personal/Promotional Website** | Link to portfolio or homepage | anuraghazra, simonw, gautamkrishnar |

#### 2. Dynamic / Automated Content
| Content Type | Description | Example Profiles |
|---|---|---|
| **GitHub Stats Cards** | Dynamically generated stats (commits, PRs, stars, streak) | anuraghazra (creator), DenverCoder1 |
| **Top Languages Card** | Most-used languages pie/bar chart | anuraghazra |
| **GitHub Streak Stats** | Current streak, longest streak, total contributions | DenverCoder1 (creator), orhun |
| **Contribution Graph / Snake Animation** | Animated SVG of contribution graph as a snake game | Platane/snk action users |
| **WakaTime Coding Stats** | Weekly coding time by language, editor, OS | athul (waka-readme creator), anmol098 |
| **Latest Blog Posts (RSS)** | Auto-updated from dev.to, Medium, personal blog | simonw, gautamkrishnar (blog-post-workflow creator) |
| **Latest YouTube Videos** | Auto-pulled from YouTube channel RSS | codestackr, DenverCoder1 |
| **Spotify Now Playing** | Currently playing track or recently played | kittinan, novatorem, andyruwruw |
| **Latest Releases** | Auto-updated list of recent GitHub releases | simonw |
| **TIL (Today I Learned)** | Auto-updated from a TIL repository | simonw |
| **Recent GitHub Activity** | Recent commits, PRs, issues, stars | jamesgeorge007/github-activity-readme users |
| **StackOverflow Activity** | Recent SO answers/reputation | gautamkrishnar |
| **npm Download Counts** | Total downloads across npm packages | maddhruv |
| **Todoist Stats** | Task completion stats from Todoist | abhisheknaiidu |
| **Dev Metrics (All-in-One)** | Early bird/night owl, most productive day, coding breakdown | anmol098 (waka-readme-stats) |

#### 3. Visual / Design Elements
| Content Type | Description | Example Profiles |
|---|---|---|
| **Typing SVG Animation** | Animated typing effect for taglines | DenverCoder1 (creator), ABSphreak |
| **Custom Header Banner** | Designed header image/SVG | anuraghazra, DenverCoder1 |
| **Skill/Tech Badges** | shields.io badges for languages, tools, frameworks | ileriayo, harish-sethuraman, br3ndonland |
| **Tech Stack Icons** | Small icons for each technology | anuraghazra, hussainweb, peterthehan |
| **Animated GIFs** | Fun GIFs as personality expression | sindresorhus (retro 90s aesthetic), saadeghi |
| **Dark/Light Mode Support** | Different images for GitHub dark vs light themes | orhun (via `#gh-dark-mode-only` suffix) |
| **Retro/Nostalgic Design** | 90s web aesthetic, visitor counters, under construction GIFs | sindresorhus, BrunnerLivio, fnky |
| **Fancy Fonts / ASCII Art** | Custom typography or terminal-style ASCII art | xiaoluoboding, fnky |
| **Profile View Counter** | Visitor count badge | komarev/github-profile-views-counter, orhun |

#### 4. Interactive / Gamified Content
| Content Type | Description | Example Profiles |
|---|---|---|
| **Playable Games** | Chess, Connect4, tic-tac-toe via GitHub Issues | timburgan, JonathanGin52, rossjrw, marcizhu |
| **Community Wall / Guestbook** | Visitors can add themselves via Issues/PRs | timburgan |

#### 5. Professional / Credentialing
| Content Type | Description | Example Profiles |
|---|---|---|
| **GitHub Star Badge** | GitHub Star program recognition | DenverCoder1 |
| **Sponsors Section** | Top sponsors / sponsor button | DenverCoder1, orhun, dtolnay |
| **Certifications & Awards** | Professional certs, hackathon wins | various |
| **Open Source Story** | Narrative about OSS journey & impact | gautamkrishnar |
| **Current Work / Learning** | What they're building/studying now | gautamkrishnar, BurntSushi |
| **PGP Key** | Public key fingerprint badge | orhun |
| **LinkedIn Profile Card** | Dynamically generated LinkedIn summary image | soroushchehresa/github-readme-linkedin |
| **Resume / CV Link** | Direct link to downloadable resume | various |

#### 6. Content Aggregation / Cross-Platform
| Content Type | Description | Example Profiles |
|---|---|---|
| **Newsletter Signup** | Link to Substack/newsletter | simonw |
| **Ko-fi / Buy Me a Coffee** | Donation/tip links | DenverCoder1 |
| **Discord Server Invite** | Community Discord link | DenverCoder1 |
| **Xbox/Steam GamerTag** | Gaming profile links | gautamkrishnar |
| **Mastodon/Bluesky/Fediverse** | Decentralized social links | simonw |
| **Dev.to / Hashnode Profile** | Developer blogging platform links | gautamkrishnar |
| **StackOverflow Profile** | SO reputation/link | gautamkrishnar |

#### 7. Structural / Layout Patterns
| Content Type | Description | Example Profiles |
|---|---|---|
| **Multi-Column Table Layout** | 2-3 columns for different content streams | simonw (releases, blog, TIL) |
| **Collapsible Sections** | `<details>` tags for expandable content | DenverCoder1, orhun |
| **Pinned Repo Cards** | github-readme-stats pin cards | anuraghazra |
| **GitHub Contributor Stats** | Stats for repos actually contributed to (not just owned) | HwangTaehyun |

---

## Part 2: Web Services & Tools Inventory

### Existing Services (Ready to Use)

| Content Type | Service/Tool | How It Works |
|---|---|---|
| **GitHub Stats Card** | [github-readme-stats](https://github.com/anuraghazra/github-readme-stats) | Vercel-hosted SVG endpoint: `![](https://github-readme-stats.vercel.app/api?username=USER)` |
| **Top Languages** | github-readme-stats | Same service: `/api/top-langs/?username=USER` |
| **Pinned Repo Card** | github-readme-stats | `/api/pin/?username=USER&repo=REPO` |
| **Streak Stats** | [github-readme-streak-stats](https://github.com/DenverCoder1/github-readme-streak-stats) | `![](https://streak-stats.demolab.com/?user=USER)` |
| **Typing SVG** | [readme-typing-svg](https://github.com/DenverCoder1/readme-typing-svg) | `![](https://readme-typing-svg.demolab.com/?lines=Hello;World)` |
| **Skill Badges** | [shields.io](https://shields.io/) + [simple-icons](https://simpleicons.org/) | `![](https://img.shields.io/badge/Rust-000?logo=rust)` |
| **Markdown Badges** | [markdown-badges](https://github.com/Ileriayo/markdown-badges) | Pre-made badge collection, copy-paste |
| **Profile Views** | [komarev/github-profile-views-counter](https://github.com/antonkomarev/github-profile-views-counter) | `![](https://komarev.com/ghpvc/?username=USER)` |
| **Contribution Snake** | [Platane/snk](https://github.com/Platane/snk) | GitHub Action generates snake SVG from contribution graph |
| **WakaTime Stats** | [waka-readme](https://github.com/athul/waka-readme) | GitHub Action updates README with WakaTime data |
| **All Dev Metrics** | [waka-readme-stats](https://github.com/anmol098/waka-readme-stats) | GitHub Action: early bird/night owl, productive day, etc. |
| **Blog Posts (RSS)** | [blog-post-workflow](https://github.com/gautamkrishnar/blog-post-workflow) | GitHub Action replaces markers with latest RSS items |
| **GitHub Activity** | [github-activity-readme](https://github.com/jamesgeorge007/github-activity-readme) | GitHub Action updates README with recent activity |
| **Spotify Now Playing** | [spotify-github-profile](https://github.com/kittinan/spotify-github-profile) | Vercel app generates now-playing SVG card |
| **Medium Articles** | [github-readme-medium](https://github.com/omidnikrah/github-readme-medium) | Shows latest Medium post |
| **LinkedIn Card** | [github-readme-linkedin](https://github.com/soroushchehresa/github-readme-linkedin) | Dynamic LinkedIn profile image |
| **StackOverflow Stats** | [github-readme-stackoverflow](https://github.com/omidnikrah/github-readme-stackoverflow) | SO reputation/badges card |
| **npm Downloads** | [github-readme-npm-downloads](https://github.com/maddhruv/github-readme-npm-downloads) | Shows npm package download counts |
| **YouTube Stats** | [youtube-stats-card](https://github.com/DenverCoder1/github-readme-youtube-cards) | YouTube video cards for README |
| **Todoist Stats** | [todoist-readme](https://github.com/abhisheknaiidu/todoist-readme) | Daily Todoist stats |
| **Contributor Stats** | [github-contributor-stats](https://github.com/HwangTaehyun/github-contributor-stats) | Stats for repos you actually committed to |
| **Custom Icon Badges** | [custom-icon-badges](https://github.com/DenverCoder1/custom-icon-badges) | shields.io with custom SVG icons |
| **Profile README Generator** | [github-profile-readme-generator](https://github.com/rahuldkjain/github-profile-readme-generator) | Web UI to create README (React app) |

### Needs Custom Implementation

| Content Type | What Would Need to Be Built |
|---|---|
| **Crates.io Stats** | Script to fetch crate download counts from crates.io API, generate badge/table. Could be a GitHub Action or Vercel function. |
| **PyPI Package Stats** | Similar to npm-downloads but for Python packages. Fetch from PyPI JSON API. |
| **Automated Project Showcase** | Script that reads pinned repos via GitHub GraphQL API and generates formatted cards with descriptions, stars, language. |
| **Conference Talks List** | RSS/JSON feed from personal site or manual YAML → rendered markdown table via GitHub Action. |
| **Reading List / Book Shelf** | Integration with Goodreads/OpenLibrary API → GitHub Action to update README. |
| **Commit Streak Counter** | GitHub Action that queries contribution data and writes "currently on X-day streak" (orhun does this). |

---

## Part 3: Claude Code Prompt for Rust TUI App

```
You are building a Rust terminal UI (TUI) application called `gh-profile-gen` that helps users create a comprehensive GitHub profile README.md.

## Overview

Build a Rust CLI application using the `ratatui` crate for terminal UI and `crossterm` for terminal backend. The app should have two modes:

1. **Basic Mode**: Interactive wizard that asks questions and directly outputs a complete README.md
2. **Advanced Mode**: Generates a TOML project file that can be hand-edited, then regenerated into README.md at any time

## Architecture

```
gh-profile-gen/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point, CLI args (clap)
│   ├── app.rs               # App state machine
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── wizard.rs         # Step-by-step TUI wizard
│   │   ├── widgets.rs        # Custom ratatui widgets
│   │   └── theme.rs          # Colors and styling
│   ├── config/
│   │   ├── mod.rs
│   │   ├── profile.rs        # Profile data model (serde)
│   │   └── toml_io.rs        # TOML read/write
│   ├── render/
│   │   ├── mod.rs
│   │   ├── markdown.rs       # Markdown generation engine
│   │   ├── sections.rs       # Each README section renderer
│   │   └── templates.rs      # Built-in layout templates
│   └── services/
│       ├── mod.rs
│       └── urls.rs           # URL builders for external services
```

## Dependencies

```toml
[dependencies]
ratatui = "0.29"
crossterm = "0.28"
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
toml = "0.8"
```

## Data Model

Define a `ProfileConfig` struct (serde-serializable to TOML) with these sections:

```rust
struct ProfileConfig {
    meta: Meta,           // username, name, mode
    header: Header,       // banner style, typing SVG text, tagline
    about: About,         // bio, current work, learning, fun fact, pronouns
    social: Social,       // all social links
    skills: Skills,       // languages, frameworks, tools, databases
    stats: Stats,         // which stat cards to include
    projects: Projects,   // featured projects
    blog: Blog,           // blog feed config
    dynamic: Dynamic,     // spotify, wakatime, youtube, etc.
    layout: Layout,       // template choice, dark mode support
    sponsors: Sponsors,   // sponsor section config
    extras: Extras,       // PGP key, gaming profiles, etc.
}
```

### Content Sections the App Must Support

**Header Section:**
- Custom banner image URL or generated SVG header
- Typing SVG animation with configurable lines (via readme-typing-svg)
- Simple text greeting
- Wave/hand emoji greeting

**About Me Section:**
- Current role and company
- What they're working on
- What they're learning
- How to reach them
- Fun fact
- Pronouns
- Location and timezone

**Social Links Section:**
Render as icon row or badge row. Support:
- GitHub, Twitter/X, LinkedIn, Mastodon, Bluesky, Instagram, YouTube, Discord, Dev.to, Hashnode, Medium, StackOverflow, Reddit, Twitch, personal website, email, Ko-fi/Buy Me a Coffee, RSS feed

**Skills / Tech Stack Section:**
Render as shields.io badges or icon grid. Categories:
- Languages (Rust, Python, TypeScript, Go, C++, etc.)
- Frameworks (React, Vue, Actix, Rocket, Django, Flask, etc.)
- Tools (Docker, Git, Neovim, VS Code, etc.)
- Databases (PostgreSQL, Redis, SQLite, MongoDB, etc.)
- Cloud/Infra (AWS, GCP, Vercel, Cloudflare, etc.)

User picks from a searchable list in the TUI. Each skill maps to a simple-icons slug for badge generation.

**GitHub Stats Section:**
Optional cards (user toggles each on/off):
- GitHub stats card (github-readme-stats)
- Top languages card
- Streak stats card (github-readme-streak-stats)
- Contributor stats card
- GitHub trophies (github-profile-trophy)
- Contribution graph snake animation
- Profile views counter

Each card: configurable theme (dark, radical, tokyonight, etc.), hide_border toggle.

**Featured Projects Section:**
- Manual list of repos to highlight (owner/repo pairs)
- Rendered as github-readme-stats pin cards or as a markdown table with name, description, stars badge, language badge
- Option for auto-generated from pinned repos

**Blog / Content Section:**
- RSS feed URL(s) for auto-update via blog-post-workflow GitHub Action
- Manual list of articles with title + URL
- YouTube channel integration (latest video cards)
- Newsletter signup link

**Dynamic / Real-time Section:**
- Spotify now playing (spotify-github-profile)
- WakaTime coding stats (waka-readme or waka-readme-stats)
- Recent GitHub activity (github-activity-readme)
- Latest releases (custom GitHub Action like simonw's)
- StackOverflow activity

**Sponsors Section:**
- GitHub Sponsors badge/link
- Ko-fi / Buy Me a Coffee badge
- Custom sponsor tier display

**Extras Section:**
- PGP key fingerprint badge
- Gaming profiles (Xbox, Steam, PSN)
- Certifications and awards
- Conference talks
- Collapsible details sections
- Custom raw markdown blocks (escape hatch for anything not covered)

**Layout & Theming:**
- Template choices: Minimal, Full-Featured, Developer Card, Retro 90s, Multi-Column (simonw-style)
- Dark/light mode image support (`#gh-dark-mode-only` / `#gh-light-mode-only`)
- Alignment options (centered vs left-aligned)
- Horizontal rule style

## TUI Wizard Flow

The wizard should be a multi-step form using ratatui:

1. **Welcome screen** — choose Basic or Advanced mode
2. **Identity** — GitHub username, display name, tagline
3. **About** — role, company, current work, learning, fun fact
4. **Social links** — show list, user enters URLs for each they want
5. **Skills** — searchable multi-select from categorized list
6. **Stats** — toggle switches for each stat card type, pick theme
7. **Projects** — add repos to feature (owner/repo)
8. **Blog & Content** — RSS URLs, YouTube channel, newsletter
9. **Dynamic integrations** — Spotify, WakaTime, etc.
10. **Extras** — PGP, gaming, certifications, custom blocks
11. **Layout** — pick template, dark mode support
12. **Preview & Generate** — show markdown preview, write to file

Navigation: Tab/Shift-Tab between fields, Enter to advance, Esc to go back, Ctrl-G to generate at any point (skipping remaining steps).

In Advanced mode, after step 12 also write `profile.toml`. User can later run `gh-profile-gen render profile.toml` to regenerate.

## CLI Interface

```
gh-profile-gen                    # Launch TUI wizard (basic mode)
gh-profile-gen --advanced         # Launch TUI wizard (advanced mode, saves TOML)
gh-profile-gen render <file.toml> # Render TOML to README.md
gh-profile-gen init               # Generate a starter profile.toml with all fields commented
gh-profile-gen preview <file.toml> # Preview rendered markdown in terminal
```

## Markdown Generation

The render engine should:
1. Read ProfileConfig (from wizard state or TOML file)
2. For each enabled section, generate markdown using the appropriate template
3. Insert comment markers for GitHub Action integrations (e.g., `<!-- BLOG-POST-LIST:START -->`)
4. Generate a companion `.github/workflows/profile-update.yml` if any dynamic content is enabled
5. Support outputting to stdout or to a file path

URL builders for external services should be in a separate module. Example:
```rust
fn github_stats_url(username: &str, theme: &str, show_icons: bool, hide_border: bool) -> String {
    format!("https://github-readme-stats.vercel.app/api?username={}&theme={}&show_icons={}&hide_border={}",
        username, theme, show_icons, hide_border)
}
```

## GitHub Action Generation

When dynamic content is configured, generate a GitHub Actions workflow file that:
- Runs on schedule (configurable cron, default every 6 hours)
- Uses the appropriate actions (blog-post-workflow, waka-readme, etc.)
- Commits changes back to the profile repo

## Quality Requirements

- Full error handling with `anyhow` for the binary, `thiserror` for library errors
- Unit tests for URL builders and markdown generation
- Integration test that generates a complete README from a sample TOML
- Clippy clean, rustfmt formatted
- Documentation comments on all public items
- README.md for the tool itself with usage examples

## Example Output

For a Rust developer named "Alice" with GitHub username "alice-dev", the generated README should look something like:

```markdown
<p align="center">
  <a href="https://readme-typing-svg.demolab.com/">
    <img src="https://readme-typing-svg.demolab.com/?lines=Rust+Developer;Open+Source+Enthusiast;Systems+Programmer&font=Fira+Code&center=true&width=440&height=45&color=f75c7e&vCenter=true&pause=1000&size=22" />
  </a>
</p>

## Hey! 👋 I'm Alice

🦀 Rust developer passionate about systems programming and CLI tools.

- 🔭 Currently working on **my-awesome-project**
- 🌱 Learning **WebAssembly** and **embedded Rust**
- 💬 Ask me about Rust, CLI tools, or terminal UIs

### Connect with me
[![Twitter](https://img.shields.io/badge/Twitter-1DA1F2?style=for-the-badge&logo=twitter&logoColor=white)](https://twitter.com/alice)
[![LinkedIn](https://img.shields.io/badge/LinkedIn-0077B5?style=for-the-badge&logo=linkedin&logoColor=white)](https://linkedin.com/in/alice)

### Tech Stack
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Python](https://img.shields.io/badge/Python-3776AB?style=for-the-badge&logo=python&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)

### GitHub Stats
![Alice's GitHub stats](https://github-readme-stats.vercel.app/api?username=alice-dev&show_icons=true&theme=tokyonight)
![GitHub Streak](https://streak-stats.demolab.com/?user=alice-dev&theme=tokyonight)

### Featured Projects
<a href="https://github.com/alice-dev/cool-cli">
  <img align="center" src="https://github-readme-stats.vercel.app/api/pin/?username=alice-dev&repo=cool-cli&theme=tokyonight" />
</a>

### 📕 Latest Blog Posts
<!-- BLOG-POST-LIST:START -->
<!-- BLOG-POST-LIST:END -->

### 🎵 Spotify
[![spotify-github-profile](https://spotify-github-profile.kittinan.vercel.app/api/view?uid=USER&cover_image=true)](https://spotify-github-profile.kittinan.vercel.app/api/view?uid=USER&redirect=true)
```

Build the complete application. Start with the data model and CLI parsing, then the markdown renderer, then the TUI wizard. Test each layer independently.