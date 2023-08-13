mod app;
mod audio_player;
mod dependent_module;
mod wasm_audio;
mod transparent_node;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}

// implement this next: https://rustwasm.github.io/wasm-bindgen/examples/wasm-audio-worklet.html
