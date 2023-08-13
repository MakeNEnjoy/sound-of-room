use yew::prelude::*;
use web_sys::{
    AudioContext,
    HtmlMediaElement,
    GainNode,
    GainOptions,
    AudioContextState,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{
    spawn_local,
    JsFuture
};
use crate::wasm_audio::{prepare_wasm_audio_spawn, wasm_audio_node};
use crate::transparent_node::TransparentNode;

fn init(audio_context: &AudioContext, audio_ref: &NodeRef) {
    let audio_element: HtmlMediaElement = audio_ref.cast().ok_or(JsValue::undefined()).unwrap();

    let track = audio_context.create_media_element_source(&audio_element).unwrap();
    let mut gain_options = GainOptions::new();
    gain_options.gain(0.1);
    let gain_node = GainNode::new_with_options(&audio_context, &gain_options).unwrap();

    let mut transparent_node_data = TransparentNode::new();
    let transparent_node = wasm_audio_node(
        &audio_context,
        Box::new(move |in_buf, out_buf| transparent_node_data.process(in_buf, out_buf)))
        .unwrap();
    
    let destination = audio_context.destination();
    track.connect_with_audio_node(&gain_node).unwrap()
        .connect_with_audio_node(&transparent_node).unwrap()
        .connect_with_audio_node(&destination).unwrap();
}

fn play(audio_context: &AudioContext, audio_ref: &NodeRef) {
    if audio_context.state() == AudioContextState::Suspended {
        let future = audio_context.resume().unwrap();
        spawn_local(async move {
            JsFuture::from(future).await.unwrap();
        });
    }

    let audio_element: HtmlMediaElement = audio_ref.cast().ok_or(JsValue::undefined()).unwrap();
    let future = audio_element.play().unwrap();
    spawn_local(async move {
        JsFuture::from(future).await.unwrap(); 
    });
}

#[function_component]
pub fn AudioPlayer() -> Html {
    let audio_ref = use_node_ref();
    let audio_context = use_memo(|_| AudioContext::new().unwrap(), ());
    prepare_wasm_audio_spawn(&audio_context);
    let context_is_initialised = use_state(|| false);

    let onclick = {
        let audio_ref = audio_ref.clone();
        Callback::from(move |_| {
            if !(*context_is_initialised) {
                init(&audio_context, &audio_ref);
                context_is_initialised.set(true);
            }
            play(&audio_context, &audio_ref);
        })
    };

    html! {
        <main>
            <audio ref = {&audio_ref} src="static/house2.mp3"></audio>
            <h1>{ "Hello World!" }</h1>
            <button onclick={onclick}> { " Play " }</button>
        </main>
    }
}