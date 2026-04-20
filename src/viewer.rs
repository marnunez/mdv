mod document;
mod overlay;
mod shell;
mod state;

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, EventControllerKey};

use crate::links::{open_link_at_cursor, sync_link_hints};
use crate::navigation::{
    half_page_down, half_page_up, line_down, line_up, page_down, page_up, scroll_to_bottom,
    scroll_to_top,
};
use crate::search::search_next;
use crate::theme::apply_css;

use document::apply_initial_theme;
use overlay::{
    handle_overlay_key, show_link_hint_overlay, show_search_overlay, wire_overlay_events,
};
use shell::build_window;

pub(crate) use document::{reload_document, update_status};
pub(crate) use shell::{LinkHintWidget, Ui};
pub(crate) use state::{AppState, OverlayMode, SearchMatch};

pub fn build_ui(app: &Application, file_path: PathBuf) {
    let ui = build_window(app);
    let window = ui.window.clone();

    let state = Rc::new(RefCell::new(AppState::new(file_path)));
    apply_initial_theme(&ui, &state);
    reload_document(&ui, &state);
    wire_overlay_events(&ui, &state);
    wire_keybindings(&ui, &state, &window);
    wire_hint_tracking(&ui, &state);
    window.present();
}

fn wire_hint_tracking(ui: &Ui, state: &Rc<RefCell<AppState>>) {
    let ui_tick = ui.clone();
    let state_tick = state.clone();
    ui.text_view.add_tick_callback(move |_, _| {
        let state = state_tick.borrow();
        if state.overlay_mode == OverlayMode::LinkHints {
            sync_link_hints(&ui_tick, &state);
        }
        glib::ControlFlow::Continue
    });
}

fn wire_keybindings(ui: &Ui, state: &Rc<RefCell<AppState>>, window: &ApplicationWindow) {
    let key_controller = EventControllerKey::new();
    key_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);

    let ui_key = ui.clone();
    let state_rc = state.clone();
    key_controller.connect_key_pressed(move |_, keyval, _keycode, modifiers| {
        let mut state = state_rc.borrow_mut();

        if state.overlay_mode != OverlayMode::None {
            return handle_overlay_key(&ui_key, &mut state, keyval);
        }

        if modifiers.contains(gdk::ModifierType::CONTROL_MASK)
            && matches!(keyval, gdk::Key::f | gdk::Key::F)
        {
            show_search_overlay(&ui_key, &mut state);
            return glib::Propagation::Stop;
        }

        match keyval {
            gdk::Key::q => {
                ui_key.window.close();
                glib::Propagation::Stop
            }
            gdk::Key::j => {
                line_down(&ui_key);
                state.pending_g = false;
                glib::Propagation::Stop
            }
            gdk::Key::k => {
                line_up(&ui_key);
                state.pending_g = false;
                glib::Propagation::Stop
            }
            gdk::Key::d => {
                half_page_down(&ui_key);
                state.pending_g = false;
                glib::Propagation::Stop
            }
            gdk::Key::u => {
                half_page_up(&ui_key);
                state.pending_g = false;
                glib::Propagation::Stop
            }
            gdk::Key::space => {
                page_down(&ui_key);
                state.pending_g = false;
                glib::Propagation::Stop
            }
            gdk::Key::b => {
                page_up(&ui_key);
                state.pending_g = false;
                glib::Propagation::Stop
            }
            gdk::Key::g => {
                if state.pending_g {
                    scroll_to_top(&ui_key);
                    state.pending_g = false;
                } else {
                    state.pending_g = true;
                }
                glib::Propagation::Stop
            }
            gdk::Key::G => {
                scroll_to_bottom(&ui_key);
                state.pending_g = false;
                glib::Propagation::Stop
            }
            gdk::Key::slash => {
                state.pending_g = false;
                show_search_overlay(&ui_key, &mut state);
                glib::Propagation::Stop
            }
            gdk::Key::n => {
                state.pending_g = false;
                search_next(&ui_key, &mut state, 1);
                glib::Propagation::Stop
            }
            gdk::Key::N => {
                state.pending_g = false;
                search_next(&ui_key, &mut state, -1);
                glib::Propagation::Stop
            }
            gdk::Key::f => {
                state.pending_g = false;
                show_link_hint_overlay(&ui_key, &mut state);
                glib::Propagation::Stop
            }
            gdk::Key::plus | gdk::Key::KP_Add => {
                state.pending_g = false;
                state.zoom_level = (state.zoom_level + 0.1).min(2.5);
                apply_css(&ui_key.css_provider, state.zoom_level);
                update_status(&ui_key, &state, None);
                glib::Propagation::Stop
            }
            gdk::Key::minus | gdk::Key::KP_Subtract => {
                state.pending_g = false;
                state.zoom_level = (state.zoom_level - 0.1).max(0.6);
                apply_css(&ui_key.css_provider, state.zoom_level);
                update_status(&ui_key, &state, None);
                glib::Propagation::Stop
            }
            gdk::Key::_0 | gdk::Key::KP_0 => {
                state.pending_g = false;
                state.zoom_level = 1.0;
                apply_css(&ui_key.css_provider, state.zoom_level);
                update_status(&ui_key, &state, None);
                glib::Propagation::Stop
            }
            gdk::Key::r => {
                state.pending_g = false;
                drop(state);
                reload_document(&ui_key, &state_rc);
                glib::Propagation::Stop
            }
            gdk::Key::Return | gdk::Key::KP_Enter => {
                state.pending_g = false;
                if open_link_at_cursor(&ui_key, &state) {
                    glib::Propagation::Stop
                } else {
                    glib::Propagation::Proceed
                }
            }
            _ => {
                state.pending_g = false;
                glib::Propagation::Proceed
            }
        }
    });

    window.add_controller(key_controller);
}
