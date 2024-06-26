use wasm_bindgen::prelude::*;
use web_sys::{MouseEvent, window};
use yew::prelude::*;


use crate::*;
use crate::app_components::{Download, Markdown, PageOptions, TitleAvatarComponent};
use crate::components::{CurrDirectory, TitleBar};

pub fn get_titlebar(x: UseStateHandle<String>) -> Html {
    let light_mod = use_state(|| false);

    let is_expanded = x.chars().count();
    let doc = window().unwrap_throw().document().unwrap_throw();
    let current_directory = html! {<CurrDirectory/>};

    let _light_mod = light_mod.clone();
    let _doc = doc.clone();
    let handle_light_mod: Callback<MouseEvent> = Callback::from(move |e: MouseEvent| {
        let _ = _doc.query_selector("html")
            .unwrap()
            .unwrap()
            .class_list()
            .toggle("light-mod");
        _light_mod.set(!(*_light_mod));
    });

    let toggle_asidebar: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        if x.chars().count() > 1 {
            x.set("".to_string());
            let _ = &doc.query_selector(".editor_title")
                .unwrap()
                .unwrap()
                .set_attribute("style", "margin-left:0px; width:100%");
            let _ = &doc.query_selector(".text_editor_container")
                .unwrap()
                .unwrap()
                .set_attribute("style", "margin-left:0px; width:100%");
        } else {
            x.set("width:250px".to_string());
            let _ = &doc.query_selector(".editor_title")
                .unwrap()
                .unwrap()
                .set_attribute("style", "margin-left:250px; margin-right:2%; width:80%");
            let _ = &doc.query_selector(".text_editor_container")
                .unwrap()
                .unwrap()
                .set_attribute("style", "margin-left:250px; margin-right:2%; width:80%");
        }
    });

    let right_content: Html = html! {
       <>
               <Download/>
               <i
               onclick={handle_light_mod}
               class={format!("btn {}",if (*light_mod) {"fa-solid fa-moon"} else {"fa-solid fa-sun"})}
               ></i>

           <TitleAvatarComponent/>

           <PageOptions/>
       </>
    };
    html! {
        <TitleBar
            style={(if !(*IS_WEB) {"padding-left: 75px; cursor: grab;"} else {""}).to_string()}
            title={current_directory}
            {right_content}
         >
            <li class="btn" onclick={toggle_asidebar}>
            {if is_expanded > 1{
                html!{<i class="fa-solid fa-x"></i>}
            } else{
                html!{<i class="fa-solid fa-bars"></i>}
            }}
            </li>
            <Markdown/>
        </TitleBar >
    }
}
