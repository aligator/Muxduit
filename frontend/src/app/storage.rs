//! Thin `localStorage` helpers shared by the views that persist small UI
//! settings (output folder, overwrite flag, theme choice).

fn local_storage() -> Option<web_sys::Storage> {
    web_sys::window().and_then(|window| window.local_storage().ok().flatten())
}

pub(crate) fn ls_get(key: &str) -> Option<String> {
    local_storage().and_then(|store| store.get_item(key).ok().flatten())
}

pub(crate) fn ls_set(key: &str, value: &str) {
    if let Some(store) = local_storage() {
        let _ = store.set_item(key, value);
    }
}

pub(crate) fn ls_remove(key: &str) {
    if let Some(store) = local_storage() {
        let _ = store.remove_item(key);
    }
}
