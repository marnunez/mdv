# mdv

> Zathura for markdown. Minimal, keyboard-driven markdown viewer with vim keybindings.

`mdv` is a lightweight markdown viewer built with GTK4 in Rust. It parses markdown with `pulldown-cmark` and renders it natively in a GTK text view with editorial night/day themes, vim-style navigation, incremental search, reload, internal heading links, and visible-only link hint mode.

Designed as the markdown equivalent of zathura: no menus, no toolbars, just keyboard-driven reading.

## Features

- Vim-style navigation
- Incremental search with match count
- Visible-only link hint mode
- Internal heading links (`#anchors`)
- Theme toggle between Editorial Night and Editorial Day
- Native GTK rendering — no embedded browser engine
- Headings, lists, block quotes, code blocks, tables, and links
- External links open in your default browser
- Reload file from disk
- Mouse text selection and copy

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
| `Enter` | Open link at cursor/search position |
| `t` | Toggle theme |
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
- pulldown-cmark

## Status

Working native GTK viewer with syntax highlighting, internal anchors, visible-only hints, and editorial night/day themes. On `nixos-unstable`, the packaged runtime closure is roughly **188 MiB** after removing WebKitGTK and trimming the GTK wrapper.
