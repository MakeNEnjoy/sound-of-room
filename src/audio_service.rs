use std::rc::Rc;

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

#[derive(Debug, PartialEq, Clone)]
pub struct AudioService {
    pub audio_context: Option<AudioContext>,
    pub audio_ref: NodeRef
}

impl AudioService {
    fn init(self: &Self) -> AudioService {
        let audio_element: HtmlMediaElement = self.audio_ref.cast().ok_or(JsValue::undefined()).unwrap();
        let audio_context = AudioContext::new().unwrap();

        let track = audio_context.create_media_element_source(&audio_element).unwrap();
        let mut gain_options = GainOptions::new();
        gain_options.gain(0.1);
        let gain_node = GainNode::new_with_options(&audio_context, &gain_options).unwrap();
        let destination = audio_context.destination();
        track.connect_with_audio_node(&gain_node).unwrap()
            .connect_with_audio_node(&destination).unwrap();

        let audio_service = AudioService {
            audio_context: Some(audio_context),
            audio_ref: self.audio_ref.clone()
        };
        audio_service.play();

        audio_service
    }

    fn play(self: &Self) {
        let audio_context = self.audio_context.as_ref().unwrap();
        if audio_context.state() == AudioContextState::Suspended {
            let future = audio_context.resume().unwrap();
            spawn_local(async move {
                JsFuture::from(future).await.unwrap();
            });
        }

        let audio_element: HtmlMediaElement = self.audio_ref.cast().ok_or(JsValue::undefined()).unwrap();
        let future = audio_element.play().unwrap();
        spawn_local(async move {
            JsFuture::from(future).await.unwrap(); 
        });
    }

}

pub enum AudioActions {
    PlayAudio
}

impl Reducible for AudioService {
    type Action = AudioActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AudioActions::PlayAudio => {
                match self.audio_context {
                    Some(_) => {
                        self.play();
                        self
                    }
                    None => self.init().into()
                }
            }
        }
    }
}

#[function_component]
pub fn AudioPlayer() -> Html {
    let audio_ref = use_node_ref();
    let ctx = use_reducer(|| AudioService {
        audio_context: None,
        audio_ref: audio_ref.clone()
    });

    let onclick = Callback::from(move |_| {
        ctx.dispatch(AudioActions::PlayAudio);
    });

    html! {
        <main>
            <audio ref = {&audio_ref} src="static/house2.mp3"></audio>
            <h1>{ "Hello World!" }</h1>
            <button onclick={onclick}> { " Play " }</button>
        </main>
    }
}