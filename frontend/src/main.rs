#[cfg(target_arch = "wasm32")]
fn main() {
    yew::Renderer::<muxduit_frontend::app::App>::new().render();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("Muxduit WASM is a browser app. Run it with `trunk serve`.");
}
