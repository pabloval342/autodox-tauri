use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
// use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::Closure;
use web_sys::{DragEvent, Element, MouseEvent, window};
use yew::{html, Html};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use shared::invoke;

use crate::*;
use crate::components::Menu;
use crate::router::Route;

#[derive(PartialEq, Properties)]
pub struct MarkdownProps {
    // pub id: u64,
}

#[function_component(Markdown)]
pub fn markdown(props: &MarkdownProps) -> Html {
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);
    let _position = position.clone();
    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _position.set(Some(_e));
    });

    let items: Vec<Html> = vec![
        html! {<a>
        <input type="checkbox" id="vehicle1" name="vehicle1" value="Bike1"/>
          <label for="vehicle1">{"Show marks"}</label>
          </a>},
        html! {<a>
        <input type="checkbox" id="vehicle2" name="vehicle2" value="Bike2"/>
          <label for="vehicle2">{"Show html"}</label>
          </a>},
        html! {<a>
        <input type="checkbox" id="vehicle3" name="vehicle3" value="Bike3"/>
          <label for="vehicle3">{"Show render"}</label>
          </a>},
    ];

    html! {<>
        <Menu click_on={Some(true)} event={position.clone()}{items}/>
        <li {onmouseup} class="btn right_clickable"> <i class="fa-brands fa-markdown"></i></li>
    </>}
}
