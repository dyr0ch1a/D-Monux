# Monux

**Terminal utility inspired by Obsidian**  
A lightweight alternative that can work even in pure **TTY** sessions (っ˘ω˘ς )

Monux is a TUI note-taking app written in Rust, created for those who want minimalism and the ability to work without a graphical environment.

## Basic Terms

- **Service Directory**  
  The folder where the config and database live (default: `~/.monux/`)

- **Vault**  
  Note storage. Everything that resides in the root of the vault directory as `.md` files + the service directory `.monux` inside it (´・ω・`)

## Development Organization

The project is divided into four crates (work in progress):

- **monux-core**  
  The heart of the project. Database initialization, service directory, basic logic.

- **monux-cli**  
  Command-line interface for managing Monux.

- **monux-markdown**  
  Parser and pseudo-rendering of markdown tailored for TUI.

- **monux-tui**  
  The terminal interface itself, built on ratatui.

## Current Roadmap

### monux-core
- [ ] Create service directory  
- [ ] Initialize Vault  
- [ ] Handle startup errors (so it doesn't fail silently when something's wrong) (´；ω；`)

### monux-cli
- [ ] `init` — create service directory + initial vault setup  
- [ ] `version` — show program version and short help/info  
- [ ] `tui` — launch TUI session  
- [ ] `head` — check and diagnose user config (like "doctor", but cuter)

### monux-markdown
- [ ] Read `.md` files  
- [ ] Parse for links `[[wiki-style]]` and regular `[markdown](links)`  
- [ ] Pseudo-preview (text-based rendering for TUI)

### monux-tui
- [ ] Main window showing file content  
- [ ] File tree of the current Vault (on the left, like in Obsidian)  
- [ ] Panel with backlinks  
- [ ] Vim-like navigation and keybindings (っ´▽`)っ

## Technologies

- **Rust** — the foundation of everything  
- **sled** — lightweight embedded key-value transactional database  
- **ratatui** — main library for building the TUI  
- **crossterm** — terminal backend  
- **std::fs / std::path** — for all file-system needs (´∀｀*)

