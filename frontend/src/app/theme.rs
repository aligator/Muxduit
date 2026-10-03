//! Light/dark theming. The stylesheet defaults to light and switches to dark
//! via `prefers-color-scheme`; this module lets the user override that with a
//! `data-theme` attribute on `<html>`, persisted in localStorage.

use yew::prelude::*;

use super::storage::{ls_get, ls_remove, ls_set};
use super::widgets::icon;

const LS_THEME: &str = "muxduit_theme";

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum ThemeMode {
    System,
    Light,
    Dark,
}

impl ThemeMode {
    fn from_storage() -> Self {
        match ls_get(LS_THEME).as_deref() {
            Some("light") => Self::Light,
            Some("dark") => Self::Dark,
            _ => Self::System,
        }
    }

    fn next(self) -> Self {
        match self {
            Self::System => Self::Light,
            Self::Light => Self::Dark,
            Self::Dark => Self::System,
        }
    }

    fn attribute(self) -> Option<&'static str> {
        match self {
            Self::System => None,
            Self::Light => Some("light"),
            Self::Dark => Some("dark"),
        }
    }

    fn icon(self) -> &'static str {
        match self {
            Self::System => "brightness_auto",
            Self::Light => "light_mode",
            Self::Dark => "dark_mode",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::System => "Theme: system (click for light)",
            Self::Light => "Theme: light (click for dark)",
            Self::Dark => "Theme: dark (click for system)",
        }
    }
}

/// Writes (or clears) `data-theme` on `<html>` and remembers the choice.
fn apply(mode: ThemeMode) {
    let Some(root) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.document_element())
    else {
        return;
    };
    match mode.attribute() {
        Some(value) => {
            let _ = root.set_attribute("data-theme", value);
            ls_set(LS_THEME, value);
        }
        None => {
            let _ = root.remove_attribute("data-theme");
            ls_remove(LS_THEME);
        }
    }
}

#[function_component(ThemeToggle)]
pub(crate) fn theme_toggle() -> Html {
    let mode = use_state(ThemeMode::from_storage);

    // Re-apply on mount so a stored choice survives reloads, and on every
    // change made from the button.
    {
        let mode = *mode;
        use_effect_with(mode as u8, move |_| {
            apply(mode);
            || ()
        });
    }

    let onclick = {
        let mode = mode.clone();
        Callback::from(move |_| mode.set(mode.next()))
    };

    html! {
        <button class="icon-button subtle" title={mode.label()} {onclick}>
            { icon(mode.icon()) }
        </button>
    }
}
