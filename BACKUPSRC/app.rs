use yew::prelude::*;
use web_sys::{
    AudioContext,
    HtmlMediaElement
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{
    spawn_local,
    JsFuture
};

#[function_component(App)]
pub fn app() -> Html {
    let audio_element = use_node_ref();

    let onclick = setup_audio_nodes(&audio_element);
    html! {
        <main>
            <audio ref = {&audio_element} src="static/house2.mp3"></audio>
            <h1>{ "Hello World!" }</h1>
            <button {onclick}> { " Play " }</button>
        </main>
    }
}

fn setup_audio_nodes(audio: &NodeRef) -> Callback<MouseEvent> {
    let audio = audio.clone();
    Callback::from(move |_| {
        let audio_element: HtmlMediaElement = audio.cast().ok_or(JsValue::undefined()).unwrap();
        // let audio_context = AudioContext::new().unwrap();
        // let track = audio_context.create_media_element_source(&audio_element).unwrap();
        // let destination = audio_context.destination();
        // track.connect_with_audio_node(&destination).unwrap();
        // if let Ok(future) = audio_context.resume() {
            spawn_local(async move {
                // let _ = JsFuture::from(future).await;
                let _ = audio_element.play();
            });

        // }
    })
}