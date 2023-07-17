mod app;
// mod audio_service;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
