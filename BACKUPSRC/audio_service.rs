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
use std::rc::Rc;
use yew::suspense::{Suspension, SuspensionResult};

// struct MP3Player {
//     audio_file: NodeRef
// }

// impl Component for MP3Player {
//     type Properties = ();
//     type Message = ();

//     fn create(ctx: &Context<Self>) -> Self {
//         MP3Player { audio_file: NodeRef::default() }
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         // let scope = ctx.link();
//         let audio_context = ctx.link()
//             .context();
        
//         html! {
//             <audio ref = {&self.audio_file} src="static/house2.mp3"></audio>
//         }
//     }
// }

// fn setup_node() -> SuspensionResult<>

#[function_component(Radio)]
fn radio() -> HtmlResult {
    let audio_context = use_context::<AudioContext>().ok_or();
    let track = audio_context.create_media_element_source(&audio_element).ok_or();
    let destination = audio_context.destination();
    track.connect_with_audio_node(&destination).ok_or();
    Ok(html! {
        <audio ref = {&audio_element} src="static/house2.mp3"></audio>
        <button {onclick}> { " Play " }</button>
    })
}

#[function_component(MP3Player)]
fn mp3_player() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Radio />
        </Suspense>
    }
}

// If I ever have to do this with a struct component.
// This is the example: https://github.com/yewstack/yew/blob/master/examples/contexts/src/struct_component_subscriber.rs
#[function_component(AudioService)]
pub fn audio_service() -> Html {
    let audio_context = use_memo(|_| AudioContext::new().unwrap(), ());

    html! {
        <ContextProvider<Rc<AudioContext>> context={audio_context}>

        </ContextProvider<Rc<AudioContext>>>
    }
}

