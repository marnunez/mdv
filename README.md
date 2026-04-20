# mdv

> Zathura for markdown. Minimal, keyboard-driven markdown viewer with vim keybindings.

`mdv` is a lightweight markdown viewer built with GTK4 + WebKitGTK in Rust. It renders markdown to HTML in an embedded WebView with a Catppuccin Mocha dark theme, smooth scrolling, incremental search, link hint mode, and vim-style navigation.

Designed as the markdown equivalent of zathura: no menus, no toolbars, just keyboard-driven reading.

## Features

- Vim-style navigation
- Smooth scrolling
- Incremental search with match count
- Vimium-style link hints
- Auto-generated heading anchors
- External links open in your default browser
- Catppuccin Mocha dark theme
- Syntax-highlighted code blocks via WebKit rendering

## Keybindings

| Key | Action |
|-----|--------|
| `j` / `k` | Scroll down / up |
| `d` / `u` | Half page down / up |
| `Space` / `b` | Page down / up |
| `gg` | Go to top |
| `G` | Go to bottom |
| `/` or `Ctrl+F` | Search |
| `n` / `N` | Next / previous match |
| `f` | Link hint mode |
| `+` / `-` | Zoom in / out |
| `0` | Reset zoom |
| `r` | Reload file from disk |
| `q` | Quit |

## Build and run

### With Cargo

```bash
cargo run --release -- path/to/file.md
```

### With Nix

```bash
nix run . -- path/to/file.md
```

## Install with Nix

Run without installing:

```bash
nix run github:marnunez/mdv -- path/to/file.md
```

Install into your profile:

```bash
nix profile install github:marnunez/mdv
```

Use as a flake input:

```nix
inputs.mdv.url = "github:marnunez/mdv";
```

Then:

```nix
home.packages = [
  inputs.mdv.packages.${pkgs.stdenv.hostPlatform.system}.default
];
```

## Development shell

```bash
nix develop
```

## Tech stack

- Rust
- GTK4
- WebKitGTK 6
- pulldown-cmark

## Status

Working prototype.
