use yew::prelude::*;

use crate::audio_player::AudioPlayer;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <AudioPlayer />
    }
}