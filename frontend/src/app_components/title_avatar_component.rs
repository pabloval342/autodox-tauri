use yew::prelude::*;
use crate::IS_LOGEDIN;
use crate::components::{Menu, Avatar};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::prelude::*;
use shared::log;


#[wasm_bindgen(module = "/src/app_components/identify.js")]
extern "C" {
    #[wasm_bindgen(js_name = identify)]
    pub async fn identify() -> JsValue;
}


#[function_component(TitleAvatarComponent)]
pub fn title_avatar_component() -> Html {
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);

    let _position = position.clone();
    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _position.set(Some(_e));
    });
    let items: Vec<Html> = vec![
        html! {<><i class="fa-solid fa-user"></i>{"Profile info"}</>},
        html! {<><i class="fa-solid fa-eye"></i>{"Who can find me"}</>},
        html! {<><i class="fa-solid fa-gear"></i>{"Settings"}</>},
        html! {<><i class="fa-solid fa-right-from-bracket"></i>{"logout"}</>},
    ];

    let onclick = Callback::from(move |_e: MouseEvent| {
        spawn_local(async move {
            // let x = invoke_async("open_new_window".to_string()).await;
            log!("calling dfinity identity!");
            let user_token = identify().await;
            log!(user_token);
        });
    });


    if *IS_LOGEDIN {
        return html! { <>
                <Menu
                event={position.clone()}
                 {items}
                  />
                <span class="right_clickable main_avatar" {onmouseup}>
                <Avatar />
                </span>
                </>
                };
    } else {
        return html! {<span {onclick} class="btn" ><i class="fa-solid fa-right-to-bracket"></i>{"login"}</span>};
    }
}
