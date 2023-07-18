use yew::prelude::*;

use crate::audio_service::AudioPlayer;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <AudioPlayer />
    }
}