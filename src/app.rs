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

#[function_component(App)]
pub fn app() -> Html {
    let audio_element = use_node_ref();
    let audio_context = use_mut_ref(|| None);

    let audio_element_clone = audio_element.clone();
    let onclick = Callback::from(move |_| {

        if audio_context.borrow().is_none() {
            let context = init(&audio_element_clone);
            *audio_context.borrow_mut() = Some(context);
        }

        let audio_context_borrow = audio_context.borrow();
        let audio_context_fr = audio_context_borrow.as_ref().unwrap();

        if audio_context_fr.state() == AudioContextState::Suspended {
            let future = audio_context_fr.resume().unwrap();
            spawn_local(async move {
                JsFuture::from(future).await.unwrap();
            });
        }
        let audio_element: HtmlMediaElement = audio_element_clone.cast().ok_or(JsValue::undefined()).unwrap();
        let future = audio_element.play().unwrap();
        spawn_local(async move {
            JsFuture::from(future).await.unwrap(); 
        });
        
    });

    html! {
        <main>
            <audio ref = {&audio_element} src="static/house2.mp3"></audio>
            <h1>{ "Hello World!" }</h1>
            <button onclick={onclick}> { " Play " }</button>
        </main>
    }
}

fn init(audio: &NodeRef) -> AudioContext {
    let audio_element: HtmlMediaElement = audio.cast().ok_or(JsValue::undefined()).unwrap();
    let audio_context = AudioContext::new().unwrap();
    let track = audio_context.create_media_element_source(&audio_element).unwrap();

    let mut gain_options = GainOptions::new();
    gain_options.gain(1.0);
    let gain_node = GainNode::new_with_options(&audio_context, &gain_options).unwrap();
    let destination = audio_context.destination();
    track.connect_with_audio_node(&gain_node).unwrap().connect_with_audio_node(&destination).unwrap();
    audio_context
}