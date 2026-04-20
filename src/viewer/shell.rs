use std::cell::RefCell;
use std::rc::Rc;

use gtk4::gdk;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, CssProvider, Entry, Fixed, Label, Orientation,
    Overlay, PolicyType, Revealer, ScrolledWindow, TextView, Viewport, WrapMode,
};

#[derive(Clone)]
pub(crate) struct LinkHintWidget {
    pub(crate) label: Label,
}

#[derive(Clone)]
pub(crate) struct Ui {
    pub(crate) window: ApplicationWindow,
    pub(crate) text_view: TextView,
    pub(crate) scrolled: ScrolledWindow,
    pub(crate) overlay_revealer: Revealer,
    pub(crate) overlay_title: Label,
    pub(crate) overlay_entry: Entry,
    pub(crate) overlay_info: Label,
    pub(crate) status_label: Label,
    pub(crate) css_provider: CssProvider,
    pub(crate) hint_layer: Fixed,
    pub(crate) link_hint_widgets: Rc<RefCell<Vec<LinkHintWidget>>>,
}

pub(crate) fn build_window(app: &Application) -> Ui {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("mdv")
        .default_width(980)
        .default_height(780)
        .build();

    let root = GtkBox::new(Orientation::Vertical, 0);
    root.add_css_class("mdv-root");

    let overlay_revealer = Revealer::builder().reveal_child(false).build();
    let overlay_box = GtkBox::new(Orientation::Horizontal, 12);
    overlay_box.add_css_class("mdv-topbar");

    let overlay_title = Label::new(None);
    overlay_title.set_xalign(0.0);
    overlay_title.add_css_class("mdv-overlay-title");

    let overlay_entry = Entry::new();
    overlay_entry.set_hexpand(true);
    overlay_entry.add_css_class("mdv-overlay-entry");

    let overlay_info = Label::new(None);
    overlay_info.set_xalign(0.0);
    overlay_info.set_wrap(true);
    overlay_info.add_css_class("mdv-overlay-info");

    overlay_box.append(&overlay_title);
    overlay_box.append(&overlay_entry);
    overlay_box.append(&overlay_info);
    overlay_revealer.set_child(Some(&overlay_box));

    let scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vscrollbar_policy(PolicyType::Automatic)
        .hexpand(true)
        .vexpand(true)
        .build();
    scrolled.add_css_class("mdv-scroller");

    let page = GtkBox::new(Orientation::Vertical, 0);
    page.add_css_class("mdv-page");
    page.set_hexpand(true);
    page.set_vexpand(true);
    page.set_margin_start(34);
    page.set_margin_end(34);
    page.set_margin_top(18);
    page.set_margin_bottom(20);

    let text_view = TextView::new();
    text_view.remove_css_class("view");
    text_view.set_editable(false);
    text_view.set_cursor_visible(false);
    text_view.set_wrap_mode(WrapMode::WordChar);
    text_view.set_left_margin(0);
    text_view.set_right_margin(0);
    text_view.set_top_margin(0);
    text_view.set_bottom_margin(0);
    text_view.add_css_class("mdv-view");
    text_view.grab_focus();
    page.append(&text_view);

    let viewport = Viewport::new(None::<&gtk4::Adjustment>, None::<&gtk4::Adjustment>);
    viewport.remove_css_class("view");
    viewport.add_css_class("mdv-viewport");
    viewport.set_child(Some(&page));
    scrolled.set_child(Some(&viewport));

    let view_overlay = Overlay::new();
    view_overlay.add_css_class("mdv-content");
    view_overlay.set_child(Some(&scrolled));

    let hint_layer = Fixed::new();
    hint_layer.set_hexpand(true);
    hint_layer.set_vexpand(true);
    hint_layer.set_can_target(false);
    hint_layer.set_visible(false);
    view_overlay.add_overlay(&hint_layer);
    view_overlay.set_measure_overlay(&hint_layer, false);
    view_overlay.set_clip_overlay(&hint_layer, true);

    let status_label = Label::new(None);
    status_label.set_xalign(0.0);
    status_label.add_css_class("mdv-status");

    root.append(&overlay_revealer);
    root.append(&view_overlay);
    root.append(&status_label);

    window.set_child(Some(&root));
    window.add_css_class("mdv-window");

    let css_provider = CssProvider::new();
    if let Some(display) = gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &css_provider,
            gtk4::STYLE_PROVIDER_PRIORITY_USER,
        );
    }

    Ui {
        window,
        text_view,
        scrolled,
        overlay_revealer,
        overlay_title,
        overlay_entry,
        overlay_info,
        status_label,
        css_provider,
        hint_layer,
        link_hint_widgets: Rc::new(RefCell::new(Vec::new())),
    }
}
