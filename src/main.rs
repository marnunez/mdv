use std::cell::RefCell;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use gtk4::gdk;
use gtk4::gio;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, CssProvider, Entry, EventControllerKey, Label,
    Orientation, RevealerTransitionType,
};
use pulldown_cmark::{html, Options, Parser};
use webkit6::prelude::*;
use webkit6::WebView;

// ─── Config ──────────────────────────────────────────────────────────────────

const APP_ID: &str = "io.github.marnunez.mdv";
const SCROLL_STEP: f64 = 80.0;

// ─── State ───────────────────────────────────────────────────────────────────

struct AppState {
    pending_g: bool,
    search_active: bool,
    zoom_level: f64,
    hint_mode: bool,
    hint_chars: String,
}

impl AppState {
    fn new(_path: PathBuf) -> Self {
        Self {
            pending_g: false,
            search_active: false,
            zoom_level: 1.0,
            hint_mode: false,
            hint_chars: String::new(),
        }
    }
}

// ─── Markdown → HTML ─────────────────────────────────────────────────────────

fn md_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(markdown, options);
    let mut body = String::new();
    html::push_html(&mut body, parser);

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<style>
{css}
</style>
</head>
<body>
{body}
<script>
{js}
</script>
</body>
</html>"#,
        css = CSS_DARK,
        body = body,
        js = SCROLL_JS
    )
}

const CSS_DARK: &str = r#"
* { box-sizing: border-box; }

body {
    font-family: 'Inter', 'Noto Sans', 'DejaVu Sans', system-ui, sans-serif;
    font-size: 16px;
    line-height: 1.7;
    color: #cdd6f4;
    background-color: #1e1e2e;
    max-width: 880px;
    margin: 0 auto;
    padding: 2rem 2.5rem;
}

h1, h2, h3, h4, h5, h6 {
    margin-top: 1.5em;
    margin-bottom: 0.5em;
    font-weight: 600;
    line-height: 1.3;
}

h1 { font-size: 2em; color: #cba6f7; border-bottom: 1px solid #313244; padding-bottom: 0.3em; }
h2 { font-size: 1.5em; color: #89b4fa; border-bottom: 1px solid #313244; padding-bottom: 0.2em; }
h3 { font-size: 1.25em; color: #a6e3a1; }
h4 { font-size: 1.1em; color: #f9e2af; }

p { margin: 0.8em 0; }

a { color: #89b4fa; text-decoration: underline; text-decoration-color: #45475a; text-underline-offset: 3px; }
a:hover { text-decoration: underline; color: #b4d0fb; }

strong { color: #f5e0dc; }
em { color: #f2cdcd; }

code {
    font-family: 'JetBrains Mono', 'Fira Code', 'DejaVu Sans Mono', monospace;
    background-color: #313244;
    color: #fab387;
    padding: 0.15em 0.4em;
    border-radius: 4px;
    font-size: 0.88em;
}

pre {
    background-color: #181825;
    border: 1px solid #313244;
    border-radius: 8px;
    padding: 1em 1.2em;
    overflow-x: auto;
    margin: 1em 0;
}

pre code {
    background: none;
    padding: 0;
    color: #cdd6f4;
    font-size: 0.88em;
}

blockquote {
    border-left: 4px solid #89b4fa;
    margin: 1em 0;
    padding: 0.5em 1em;
    background-color: #181825;
    color: #a6adc8;
    border-radius: 0 6px 6px 0;
}

blockquote p { margin: 0.3em 0; }

table {
    border-collapse: collapse;
    width: 100%;
    margin: 1em 0;
    font-size: 0.95em;
}

th, td {
    border: 1px solid #313244;
    padding: 0.5em 1em;
    text-align: left;
}

th {
    background-color: #313244;
    color: #cba6f7;
    font-weight: 600;
}

tr:nth-child(even) { background-color: #181825; }
tr:hover { background-color: #262637; }

ul, ol { padding-left: 1.5em; margin: 0.5em 0; }
li { margin: 0.25em 0; }
li > input[type="checkbox"] { margin-right: 0.5em; }

hr {
    border: none;
    border-top: 1px solid #313244;
    margin: 2em 0;
}

img {
    max-width: 100%;
    height: auto;
    border-radius: 6px;
}

del { color: #6c7086; }

::selection {
    background-color: #45475a;
    color: #cdd6f4;
}

::-webkit-scrollbar { width: 8px; }
::-webkit-scrollbar-track { background: #1e1e2e; }
::-webkit-scrollbar-thumb { background: #45475a; border-radius: 4px; }
::-webkit-scrollbar-thumb:hover { background: #585b70; }
"#;

const SCROLL_JS: &str = r#"
var _mdv = {
    target: 0,
    running: false,
    step: function(delta) {
        this.target += delta;
        if (!this.running) {
            this.running = true;
            this.animate();
        }
    },
    animate: function() {
        if (Math.abs(this.target) < 1) {
            this.target = 0;
            this.running = false;
            return;
        }
        var move = this.target * 0.35;
        if (Math.abs(move) < 1) move = this.target;
        document.scrollingElement.scrollTop += move;
        this.target -= move;
        requestAnimationFrame(this.animate.bind(this));
    },
    to: function(pos) {
        this.target = 0;
        this.running = false;
        document.scrollingElement.scrollTop = pos;
    },
    jumpBy: function(delta) {
        this.target = 0;
        this.running = false;
        document.scrollingElement.scrollTop += delta;
    }
};

// Auto-generate heading IDs for anchor navigation
document.querySelectorAll('h1,h2,h3,h4,h5,h6').forEach(function(h) {
    if (!h.id) {
        h.id = h.textContent.trim().toLowerCase()
            .replace(/[^\w\s-]/g, '').replace(/\s+/g, '-');
    }
});

var _mdv_hints = {
    active: false,
    elements: [],
    labels: [],
    typed: '',
    keys: 'asdfghjkl',

    show: function() {
        this.hide();
        var links = document.querySelectorAll('a[href]');
        if (links.length === 0) return;
        this.active = true;
        this.typed = '';
        var hints = this._gen(links.length);
        var vi = 0;
        for (var i = 0; i < links.length; i++) {
            var r = links[i].getBoundingClientRect();
            if (r.width === 0 || r.height === 0) continue;
            if (r.bottom < 0 || r.top > window.innerHeight) continue;
            var d = document.createElement('span');
            d.className = '_mdv-hint';
            d.textContent = hints[vi];
            d.dataset.hint = hints[vi];
            d.style.cssText = 'position:fixed;z-index:99999;background:#f9e2af;color:#1e1e2e;' +
                'font-family:monospace;font-size:11px;font-weight:bold;padding:1px 4px;' +
                'border-radius:3px;pointer-events:none;line-height:1.2;' +
                'left:' + Math.max(0, r.left - 4) + 'px;top:' + Math.max(0, r.top) + 'px;' +
                'box-shadow:0 1px 3px rgba(0,0,0,0.5);';
            document.body.appendChild(d);
            this.elements.push({link: links[i], hint: hints[vi]});
            this.labels.push(d);
            vi++;
        }
    },

    _gen: function(n) {
        var k = this.keys, h = [];
        if (n <= k.length) {
            for (var i = 0; i < n; i++) h.push(k[i]);
        } else {
            for (var i = 0; i < n; i++) {
                var a = Math.floor(i / k.length), b = i % k.length;
                h.push(a < k.length ? k[a] + k[b] : k[b]);
            }
        }
        return h;
    },

    filter: function(ch) {
        this.typed += ch;
        var matched = null, remaining = 0;
        for (var i = 0; i < this.elements.length; i++) {
            var hint = this.elements[i].hint;
            if (hint.startsWith(this.typed)) {
                this.labels[i].style.display = '';
                remaining++;
                if (hint === this.typed) matched = this.elements[i];
            } else {
                this.labels[i].style.display = 'none';
            }
        }
        if (matched) {
            var href = matched.link.getAttribute('href');
            if (href && href.startsWith('#')) {
                var id = href.substring(1);
                var el = document.getElementById(id);
                if (el) el.scrollIntoView({behavior: 'smooth', block: 'start'});
            } else {
                matched.link.click();
            }
            this.hide();
            return 'matched';
        }
        if (remaining === 0) {
            this.hide();
            return 'none';
        }
        return 'typing';
    },

    hide: function() {
        for (var i = 0; i < this.labels.length; i++) this.labels[i].remove();
        this.labels = [];
        this.elements = [];
        this.active = false;
        this.typed = '';
    }
};
"#;

// ─── Viewer ──────────────────────────────────────────────────────────────────

fn build_ui(app: &Application, file_path: PathBuf) {
    let state = Rc::new(RefCell::new(AppState::new(file_path.clone())));

    // Read and convert markdown
    let md_content = fs::read_to_string(&file_path).unwrap_or_else(|e| {
        eprintln!("mdv: cannot read '{}': {}", file_path.display(), e);
        std::process::exit(1);
    });
    let html = md_to_html(&md_content);

    // Window
    let filename = file_path
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_else(|| "mdv".into());

    let window = ApplicationWindow::builder()
        .application(app)
        .title(&format!("{} — mdv", filename))
        .default_width(900)
        .default_height(700)
        .build();

    // Main layout: overlay with webview + search bar at bottom
    let main_box = GtkBox::new(Orientation::Vertical, 0);

    // WebView
    let webview = WebView::new();
    let settings = WebViewExt::settings(&webview).unwrap();
    settings.set_enable_developer_extras(false);
    settings.set_enable_javascript(true);

    // Dark background while loading
    webview.set_background_color(&gdk::RGBA::new(0.118, 0.118, 0.180, 1.0));

    webview.set_vexpand(true);
    webview.set_hexpand(true);

    // Load HTML content
    webview.load_html(&html, Some("file:///"));

    main_box.append(&webview);

    // Search bar (hidden by default)
    let search_box = GtkBox::new(Orientation::Horizontal, 8);
    search_box.set_margin_start(8);
    search_box.set_margin_end(8);
    search_box.set_margin_top(4);
    search_box.set_margin_bottom(4);

    let search_label = Label::new(Some("/"));
    search_label.add_css_class("search-label");

    let search_entry = Entry::new();
    search_entry.set_hexpand(true);
    search_entry.set_placeholder_text(Some("Search..."));
    search_entry.add_css_class("search-entry");

    let search_status = Label::new(None);
    search_status.add_css_class("search-status");

    search_box.append(&search_label);
    search_box.append(&search_entry);
    search_box.append(&search_status);

    let search_revealer = gtk4::Revealer::new();
    search_revealer.set_transition_type(RevealerTransitionType::SlideUp);
    search_revealer.set_reveal_child(false);
    search_revealer.set_child(Some(&search_box));

    main_box.append(&search_revealer);

    window.set_child(Some(&main_box));

    // Apply GTK CSS for search bar styling
    let css_provider = CssProvider::new();
    css_provider.load_from_data(
        r#"
        .search-label {
            font-family: monospace;
            font-weight: bold;
            color: #89b4fa;
            font-size: 14px;
        }
        .search-entry {
            background-color: #313244;
            color: #cdd6f4;
            border: 1px solid #45475a;
            border-radius: 4px;
            padding: 4px 8px;
            font-family: monospace;
            font-size: 14px;
        }
        .search-status {
            color: #6c7086;
            font-family: monospace;
            font-size: 13px;
        }
        window {
            background-color: #1e1e2e;
        }
    "#,
    );
    gtk4::style_context_add_provider_for_display(
        &gdk::Display::default().unwrap(),
        &css_provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // ─── Keyboard handling ───────────────────────────────────────────────

    // Key controller on the window captures keys before webview eats them
    let key_controller = EventControllerKey::new();
    key_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);

    let wv = webview.clone();
    let sr = search_revealer.clone();
    let se = search_entry.clone();
    let ss = search_status.clone();
    let st = state.clone();
    let win = window.clone();
    let fp = file_path.clone();

    key_controller.connect_key_pressed(move |_, keyval, _keycode, modifiers| {
        let mut state = st.borrow_mut();

        // If search bar is visible, only handle Escape/Enter — let entry get all other keys
        if sr.reveals_child() {
            match keyval {
                gdk::Key::Escape => {
                    hide_search(&sr, &se, &ss, &wv, &mut state);
                    wv.grab_focus();
                    return glib::Propagation::Stop;
                }
                gdk::Key::Return | gdk::Key::KP_Enter => {
                    // Confirm search, close input, keep highlights
                    state.search_active = false;
                    sr.set_reveal_child(false);
                    wv.grab_focus();
                    return glib::Propagation::Stop;
                }
                _ => return glib::Propagation::Proceed,
            }
        }

        // Hint mode: forward letter keys to JS hint filter
        if state.hint_mode {
            match keyval {
                gdk::Key::Escape => {
                    eval(&wv, "_mdv_hints.hide()");
                    state.hint_mode = false;
                    state.hint_chars.clear();
                }
                _ => {
                    if let Some(c) = keyval.to_unicode() {
                        if c.is_ascii_alphabetic() {
                            let ch = c.to_ascii_lowercase();
                            state.hint_chars.push(ch);
                            eval(&wv, &format!("_mdv_hints.filter('{}')", ch));
                            if state.hint_chars.len() >= 2 {
                                state.hint_mode = false;
                                state.hint_chars.clear();
                            }
                        } else {
                            eval(&wv, "_mdv_hints.hide()");
                            state.hint_mode = false;
                            state.hint_chars.clear();
                        }
                    }
                }
            }
            return glib::Propagation::Stop;
        }

        // Cancel pending g if a different key is pressed
        let was_pending_g = state.pending_g;
        if keyval != gdk::Key::g {
            state.pending_g = false;
        }

        let ctrl = modifiers.contains(gdk::ModifierType::CONTROL_MASK);
        let shift = modifiers.contains(gdk::ModifierType::SHIFT_MASK);

        match keyval {
            // Scroll
            gdk::Key::j => scroll(&wv, SCROLL_STEP),
            gdk::Key::k => scroll(&wv, -SCROLL_STEP),
            gdk::Key::d => scroll_half_page(&wv, true),
            gdk::Key::u => scroll_half_page(&wv, false),
            gdk::Key::space if !shift => scroll_page(&wv, true),
            gdk::Key::space if shift => scroll_page(&wv, false),
            gdk::Key::b if !ctrl => scroll_page(&wv, false),

            // Top / Bottom
            gdk::Key::g if !shift => {
                if was_pending_g {
                    scroll_to_top(&wv);
                    state.pending_g = false;
                } else {
                    state.pending_g = true;
                }
            }
            gdk::Key::G | gdk::Key::End => scroll_to_bottom(&wv),
            gdk::Key::Home => scroll_to_top(&wv),

            // Zoom
            gdk::Key::plus | gdk::Key::equal => {
                state.zoom_level = (state.zoom_level + 0.1).min(3.0);
                wv.set_zoom_level(state.zoom_level);
            }
            gdk::Key::minus if !ctrl => {
                state.zoom_level = (state.zoom_level - 0.1).max(0.3);
                wv.set_zoom_level(state.zoom_level);
            }
            gdk::Key::_0 => {
                state.zoom_level = 1.0;
                wv.set_zoom_level(1.0);
            }

            // Search
            gdk::Key::slash => {
                show_search(&sr, &se, &mut state);
            }
            gdk::Key::f if ctrl => {
                show_search(&sr, &se, &mut state);
            }
            gdk::Key::n if !shift => search_next(&wv),
            gdk::Key::N => search_prev(&wv),
            gdk::Key::Escape => {
                // Clear search highlights if any
                if let Some(fc) = wv.find_controller() {
                    fc.search_finish();
                }
            }

            // Link hints
            gdk::Key::f if !ctrl => {
                eval(&wv, "_mdv_hints.show()");
                state.hint_mode = true;
                state.hint_chars.clear();
            }

            // Reload
            gdk::Key::r if !ctrl => reload_file(&wv, &fp, &win),

            // Quit
            gdk::Key::q => win.close(),

            _ => return glib::Propagation::Proceed,
        }

        glib::Propagation::Stop
    });

    window.add_controller(key_controller);

    // ─── Search entry live update ────────────────────────────────────────

    let wv2 = webview.clone();
    let ss2 = search_status.clone();
    search_entry.connect_changed(move |entry| {
        let text = entry.text();
        if text.is_empty() {
            let fc = wv2.find_controller().unwrap();
            fc.search_finish();
            ss2.set_text("");
        } else {
            let fc = wv2.find_controller().unwrap();
            fc.search(
                &text,
                webkit6::FindOptions::CASE_INSENSITIVE.bits() | webkit6::FindOptions::WRAP_AROUND.bits(),
                u32::MAX,
            );
        }
    });

    // ─── Link handling ───────────────────────────────────────────────────

    webview.connect_decide_policy(|_wv, decision, decision_type| {
        use webkit6::PolicyDecisionType;

        if decision_type == PolicyDecisionType::NavigationAction {
            if let Some(nav_decision) = decision.downcast_ref::<webkit6::NavigationPolicyDecision>()
            {
                if let Some(nav_action) = nav_decision.navigation_action() {
                    if let Some(request) = nav_action.request() {
                        if let Some(uri) = request.uri() {
                            let uri_str = uri.to_string();
                            // Allow initial load and anchor links
                            if uri_str == "file:///" || uri_str.starts_with("file:///") {
                                return false; // allow
                            }
                            // Open external links in default browser
                            if uri_str.starts_with("http://") || uri_str.starts_with("https://") {
                                let _ = gio::AppInfo::launch_default_for_uri(
                                    &uri_str,
                                    gio::AppLaunchContext::NONE,
                                );
                                decision.ignore();
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    });

    // Find controller match count updates
    let ss3 = search_status.clone();
    if let Some(fc) = webview.find_controller() {
        fc.connect_counted_matches(move |_fc, count| {
            if count > 0 {
                ss3.set_text(&format!("{} matches", count));
            } else {
                ss3.set_text("no matches");
            }
        });
    }

    window.present();
}

// ─── Actions ─────────────────────────────────────────────────────────────────

fn scroll(wv: &WebView, pixels: f64) {
    eval(wv, &format!("_mdv.step({})", pixels));
}

fn scroll_half_page(wv: &WebView, down: bool) {
    let sign = if down { "" } else { "-" };
    eval(wv, &format!("_mdv.jumpBy({}Math.round(window.innerHeight * 0.5))", sign));
}

fn scroll_page(wv: &WebView, down: bool) {
    let sign = if down { "" } else { "-" };
    eval(wv, &format!("_mdv.jumpBy({}Math.round(window.innerHeight * 0.9))", sign));
}

fn scroll_to_top(wv: &WebView) {
    eval(wv, "_mdv.to(0)");
}

fn scroll_to_bottom(wv: &WebView) {
    eval(wv, "_mdv.to(document.scrollingElement.scrollHeight)");
}

fn eval(wv: &WebView, js: &str) {
    wv.evaluate_javascript(js, None, None, gio::Cancellable::NONE, |_| {});
}

fn show_search(
    revealer: &gtk4::Revealer,
    entry: &Entry,
    state: &mut AppState,
) {
    state.search_active = true;
    revealer.set_reveal_child(true);
    entry.grab_focus();
    entry.select_region(0, -1);
}

fn hide_search(
    revealer: &gtk4::Revealer,
    entry: &Entry,
    status: &Label,
    wv: &WebView,
    state: &mut AppState,
) {
    state.search_active = false;
    revealer.set_reveal_child(false);
    entry.set_text("");
    status.set_text("");
    if let Some(fc) = wv.find_controller() {
        fc.search_finish();
    }
}

fn search_next(wv: &WebView) {
    if let Some(fc) = wv.find_controller() {
        fc.search_next();
    }
}

fn search_prev(wv: &WebView) {
    if let Some(fc) = wv.find_controller() {
        fc.search_previous();
    }
}

fn reload_file(wv: &WebView, path: &PathBuf, _window: &ApplicationWindow) {
    match fs::read_to_string(path) {
        Ok(content) => {
            let html = md_to_html(&content);
            wv.load_html(&html, Some("file:///"));
        }
        Err(e) => {
            eprintln!("mdv: reload failed: {}", e);
        }
    }
}

// ─── Main ────────────────────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: mdv <file.md>");
        std::process::exit(1);
    }

    if args[1] == "--help" || args[1] == "-h" {
        println!("mdv — zathura for markdown");
        println!();
        println!("Usage: mdv <file.md>");
        println!();
        println!("Keybindings:");
        println!("  j/k         Scroll down/up");
        println!("  d/u         Half page down/up");
        println!("  Space/b     Page down/up");
        println!("  gg          Go to top");
        println!("  G           Go to bottom");
        println!("  /           Search");
        println!("  n/N         Next/previous match");
        println!("  +/-         Zoom in/out");
        println!("  0           Reset zoom");
        println!("  f           Follow link (hint mode)");
        println!("  r           Reload file");
        println!("  q           Quit");
        std::process::exit(0);
    }

    let file_path = PathBuf::from(&args[1])
        .canonicalize()
        .unwrap_or_else(|e| {
            eprintln!("mdv: {}: {}", args[1], e);
            std::process::exit(1);
        });

    if !file_path.exists() {
        eprintln!("mdv: file not found: {}", file_path.display());
        std::process::exit(1);
    }

    let app = Application::builder().application_id(APP_ID).build();

    let fp = file_path.clone();
    app.connect_activate(move |app| {
        build_ui(app, fp.clone());
    });

    // Pass empty args to GTK (we handle our own)
    app.run_with_args::<String>(&[]);
}
