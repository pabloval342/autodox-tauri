use uuid::Uuid;
use web_sys::{console::log_1, Element, MouseEvent};
use yew::prelude::*;
use yewdux::prelude::*;

use shared::schema::FileDirectory;

use crate::components::Menu;

#[derive(PartialEq, Properties)]
pub struct FileComponentProps {
    pub onclick: Callback<MouseEvent>,
    pub onclickfile: Callback<MouseEvent>,
    pub name: String,
    pub class: String,
    pub id: Uuid,
}

#[function_component(FileComponent)]
pub fn file_component(props: &FileComponentProps) -> Html {
    // HasMap
    // {
    // type:"dropover", // or dropunder or dropbellow,
    // dragged_id: uuid,
    // target_id: uuid
    // }
    //let drop_data = use_state(|| "".to_string());
    //let is_drag_over = use_state(|| "".to_string());
    let is_drag_under = use_state(|| "".to_string());

    let is_dragged = use_state(|| "".to_string());
    let is_enter = use_state(|| "".to_string());
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);

    let caret = use_state(|| "".to_string());
    let id = props.id.clone().to_string();

    let _position = position.clone();
    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        if _e.which() == 3 {
            _position.set(Some(_e));
        }
    });

    let _caret = caret.clone();
    let toggle_caret = {
        move |_e: MouseEvent| {
            if _caret.len() == 0 {
                _caret.set("caret-down".to_string())
            } else {
                _caret.set("".to_string())
            }
        }
    };

    let _is_dragged = is_dragged.clone();
    let _id = id.clone();
    let ondragstart: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _e.data_transfer()
            .unwrap()
            .set_data("dragged_item", &_id)
            .unwrap();
        _is_dragged.set("dragged".to_string())
    });

    let _is_dragged = is_dragged.clone();
    let _is_enter = is_enter.clone();
    let ondragend: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _is_dragged.set("".to_string());
        _is_enter.set("".to_string());
    });

    let _is_enter = is_enter.clone();
    let _is_dragged = is_dragged.clone();
    let ondragenter: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        if (*_is_dragged).len() == 0 {
            _is_enter.set("dragging_over".to_string());
        }
    });

    let _is_enter = is_enter.clone();
    let ondragleave: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _is_enter.set("".to_string());
    });

    let _is_enter = is_enter.clone();
    let _is_dragged = is_dragged.clone();
    let _is_drag_under = is_drag_under.clone();
    let ondrop: Callback<DragEvent> = {
        let id = id.clone();
        let file_dispatch = Dispatch::<FileDirectory>::new();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            let curr: Element = e.target_unchecked_into();
            let _ = curr.class_list().toggle("dragging_over");
            let dragged = e.data_transfer().unwrap().get_data("dragged_item").unwrap();
            let id = id.clone();
            let mut old_parent_id: Uuid = Uuid::new_v4();
            let dragged_uuid = Uuid::parse_str(dragged.as_str()).unwrap();
            for (id, value) in &file_dispatch.get().files.adjacency {
                if value.contains(&dragged_uuid) {
                    old_parent_id = *id;
                }
            }
            crate::backend::change_directory(
                id,
                dragged,
                file_dispatch.get().id.to_string(),
                old_parent_id.to_string(),
            );
        })
    };

    let _is_drag_under = is_drag_under.clone();
    let _is_dragged = is_dragged.clone();
    let ondragenter_under: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        if (*_is_dragged).len() == 0 {
            _is_drag_under.set("height: 20px; opacity:1;".to_string());
        }
    });

    let ondragover: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _e.prevent_default();
    });

    let _is_drag_under = is_drag_under.clone();
    let _id = id.clone();
    let ondrop_under: Callback<DragEvent> = Callback::from(move |e: DragEvent| {
        e.prevent_default();
        let curr: Element = e.target_unchecked_into();
        let _ = curr.set_attribute("style", "height: 3px; opacity:0;");

        let _dragged = e.data_transfer().unwrap().get_data("dragged_item");
    });
    let ondelete = {
        let id = id.clone();
        Callback::from(move |_e: MouseEvent| {
            // TODO: complete this
            log_1(&format!("{:?}", id).into());
        })
    };

    let _is_drag_under = is_drag_under.clone();
    let ondragleave_under: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _is_drag_under.set("".to_string());
    });

    // let ondragenter_above: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
    //     _is_drag_above.set("height: 20px; opacity:1;".to_string());
    // });

    // let ondragleave_above: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
    //     _is_drag_above.set("".to_string());
    // });

    // let ondrop_above: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
    //     _e.prevent_default();
    //     _is_drag_above.set("".to_string());
    // });

    html! {
        <div>
        // TODO
        //  {if is_first_file {
        //         html!{
        //         <div
        //            ondrop={ondrop_above}
        //            ondragenter={ondragenter_above}
        //            ondragleave={ondragleave_above}
        //            class="drag_under" />
        //         }
        //  }}

        <div style="position: relative; width:100%; display: block;">
           {if props.class.contains("caret"){
           html!{<button class={format!("{} crate_button",(*caret))}
           onmouseup={toggle_caret}
           onclick = { props.onclick.clone() } ><i class="fa-solid fa-caret-right"></i></button>}
           } else{ html!{} }
           }
           <li
           ondragover={ondragover.clone()}
           {ondrop}
           {ondragenter}
           {ondragleave}
           {ondragstart}
           {ondragend}
           {onmouseup}
           id = { id }
           onclick={props.onclickfile.clone()}
           draggable="true"
           class={format!("right_clickable file_component hovering active {} {} {}",(*is_dragged).clone(),(*is_enter).clone(), "")}
           style="margin-left: 30px; min-width: 0px; align-items: center; height: 100%; display: block;"
           >
           {props.name.clone()}
           </li>
           <i style="height:100%" class="btn create_file fa-solid fa-plus"></i>
        </div>

            <div
            ondragover={ondragover.clone()}
            style={format!("{}",(*is_drag_under).clone())}
            ondrop={ondrop_under}
            ondragenter={ondragenter_under}
            ondragleave={ondragleave_under}
            class="drag_under" />

           <Menu
           items={vec![
           html! {<a><i class="fa-solid fa-signature"></i>{"Rename"}</a>},
           html! {<a><i class="fa-solid fa-upload"/>{"Share"}</a>},
           html! {<a><i class="fa-solid fa-eye"/>{"Permissions"}</a>},
           html! {<a><i class="fa-solid fa-trash" onclick = {ondelete}/>{"Delete"}</a>},
           html! {<a><i class="fa-brands fa-medium"></i>{"Category"}</a>},

           ]}
           event={position.clone()}
           />

        </div>

    }
}
