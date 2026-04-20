# mdv Stress Test Document

> A deliberately busy Markdown file for testing scrolling, search, link hints, headings, lists, tables, and general rendering.

## Table of Contents

- [Overview](#overview)
- [Quick Links](#quick-links)
- [Installation](#installation)
- [Configuration](#configuration)
- [Command Reference](#command-reference)
- [Service Matrix](#service-matrix)
- [Troubleshooting](#troubleshooting)
- [Long-form Notes](#long-form-notes)
- [Appendix A](#appendix-a)
- [Appendix B](#appendix-b)

---

## Overview

This file exists to exercise `mdv` under mildly adversarial conditions.

It contains:

- many headings
- many links
- nested lists
- tables
- block quotes
- inline code such as `cargo test`, `nix build`, and `xdg-open`
- fenced code blocks
- repeated phrases for search testing, including the word **widget** several times
- some very long links and labels

If everything is working, you should be able to:

1. search for `widget`
2. press `n` and `N`
3. enter link-hint mode with `f`
4. follow links without visual chaos

### Repeated search targets

Search targets: widget, widget, widget, scroll, overlay, overlay, markdown, markdown, markdown.

### A short quotation

> The purpose of a viewer is to reveal the text, not to become the chief subject of it.
>
> A surprisingly difficult ambition.

---

## Quick Links

### Primary links

- [Rust](https://www.rust-lang.org/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [GTK4 Rust bindings](https://gtk-rs.org/gtk4-rs/stable/latest/book/)
- [NixOS](https://nixos.org/)
- [Nixpkgs search](https://search.nixos.org/packages)
- [Home Manager](https://nix-community.github.io/home-manager/)
- [pulldown-cmark](https://docs.rs/pulldown-cmark/latest/pulldown_cmark/)
- [Markdown Guide](https://www.markdownguide.org/)
- [ripgrep](https://github.com/BurntSushi/ripgrep)

### Slightly absurd links

- [A very long label that should still be recognisable when hint labels are drawn after the link text](https://example.com/very/long/path/for/testing/overlay/placement?with=query&and=parameters&because=why-not)
- [Short](https://example.com/a)
- [Medium length example link](https://example.com/medium-length)
- [Link with numbers 12345](https://example.com/12345)
- [Link with dashes and punctuation](https://example.com/alpha-beta_gamma)
- [Search engines](https://duckduckgo.com/) / [Google](https://google.com/) / [Bing](https://bing.com/)

### Dense paragraph with inline links

You can read about [GTK overlays](https://docs.gtk.org/gtk4/class.Overlay.html), compare them with [fixed positioning](https://docs.gtk.org/gtk4/class.Fixed.html), inspect [TextView](https://docs.gtk.org/gtk4/class.TextView.html), and then wonder whether [Pango](https://docs.gtk.org/Pango/) is making your life easier or merely more ornate.

---

## Installation

### Nix

```bash
git clone git@github.com:marnunez/mdv.git
cd mdv
nix build .#default
./result/bin/mdv examples/stress-test.md
```

### Cargo

```bash
cargo run --release -- examples/stress-test.md
```

### Sanity checklist

- [ ] Build succeeds
- [ ] File opens
- [ ] Search works
- [ ] Link hints appear in plausible places
- [ ] Enter on search does not explode

---

## Configuration

### Theme

The viewer currently uses a dark theme inspired by Catppuccin Mocha.

#### Colours

- background: `#1e1e2e`
- foreground: `#cdd6f4`
- accent: `#cba6f7`
- link: `#89b4fa`
- code: `#fab387`

### Behaviour

#### Keyboard navigation

- `j` / `k` for line movement
- `d` / `u` for half pages
- `Space` / `b` for pages
- `gg` / `G` for top and bottom
- `/` for search
- `f` for link hints
- `r` for reload
- `q` for quit

#### Search phrases

Useful things to search for in this file:

- `widget`
- `overlay`
- `Appendix`
- `service`
- `Nix`
- `link`

---

## Command Reference

### File inspection

```bash
rg -n "widget|overlay|markdown" examples/stress-test.md
```

### Build

```bash
nix build .#default
```

### Test

```bash
cargo test
```

### Run

```bash
./result/bin/mdv examples/stress-test.md
```

### Open a link manually

```bash
xdg-open https://gtk-rs.org/
```

---

## Service Matrix

| Service | URL | Purpose | Notes |
|--------|-----|---------|-------|
| Rust docs | https://doc.rust-lang.org/ | Language docs | Usually excellent |
| crates.io | https://crates.io/ | Package registry | A civilised dependency bazaar |
| docs.rs | https://docs.rs/ | API docs | Occasionally cryptic |
| GTK docs | https://docs.gtk.org/gtk4/ | Widget docs | Necessary and sometimes stern |
| NixOS | https://nixos.org/ | Distribution | Declarative, unforgiving, useful |
| Home Manager | https://github.com/nix-community/home-manager | User config | Helps keep order |
| GitHub | https://github.com/ | Source hosting | Also a theatre of opinions |

### Nested lists for layout testing

1. First item
   - child A
   - child B with [a link](https://example.com/child-b)
   - child C
2. Second item
   1. numbered child one
   2. numbered child two
   3. numbered child three with [another link](https://example.com/numbered)
3. Third item
   - final note mentioning widget and overlay again

---

## Troubleshooting

### Search does nothing

Possible causes:

- the query is empty
- there are no matches
- the overlay is not focused
- a bug exists, which would be awkward but educational

### Link hints are misplaced

Possible causes:

- text metrics are slightly off
- the anchor offset is wrong
- overlay coordinate translation is wrong
- the universe retains a mild hostility to neat UI alignment

### Rendering looks odd

Check:

- heading spacing
- quote indentation
- code block spans
- inline code such as `fn main()` and `let widget = true;`

---

## Long-form Notes

### Section 1

Markdown viewers tend to look simple right up until the moment one asks them to support search, keyboard navigation, accurate scrolling, and in-document overlays. Then one discovers that what seemed like a plain text problem is, in fact, a small theatre of layout state, input routing, rendering trade-offs, and widget idiosyncrasies.

### Section 2

This paragraph includes several links in close proximity: [one](https://example.com/one), [two](https://example.com/two), [three](https://example.com/three), [four](https://example.com/four), [five](https://example.com/five), and [six](https://example.com/six). It is useful for testing whether hint labels collide with each other or drift onto adjacent words.

### Section 3

Another repeated phrase block follows.

overlay overlay overlay
widget widget widget
markdown markdown markdown
search search search
hint hint hint

### Section 4

A paragraph with enough length to wrap across multiple lines should help test where hint labels appear when the link is near the edge of the text area and the text wraps in a slightly theatrical fashion around the available width of the view. Try the link here: [wrapped-link-target](https://example.com/wrapped-link-target-for-positioning-tests).

### Section 5

Block quote again:

> A viewer that redraws everything merely to whisper a hint is perhaps being a touch melodramatic.
>
> But at least the melodrama is now testable.

---

## Appendix A

### A1

- [alpha](https://example.com/alpha)
- [beta](https://example.com/beta)
- [gamma](https://example.com/gamma)
- [delta](https://example.com/delta)
- [epsilon](https://example.com/epsilon)
- [zeta](https://example.com/zeta)
- [eta](https://example.com/eta)
- [theta](https://example.com/theta)
- [iota](https://example.com/iota)
- [kappa](https://example.com/kappa)

### A2

```rust
fn main() {
    let widget = "text-view";
    let overlay = true;
    println!("{} {}", widget, overlay);
}
```

### A3

Search for `alpha`, `beta`, `gamma`, or `delta` if you want easy navigation targets.

---

## Appendix B

### B1

Final cluster of links for hint-volume testing:

[01](https://example.com/01) [02](https://example.com/02) [03](https://example.com/03) [04](https://example.com/04) [05](https://example.com/05) [06](https://example.com/06) [07](https://example.com/07) [08](https://example.com/08) [09](https://example.com/09) [10](https://example.com/10)

### B2

And a closing sentence with [one last link](https://example.com/finale), because one should never underestimate the value of a decent finale.
