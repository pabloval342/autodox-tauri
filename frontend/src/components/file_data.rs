use uuid::Uuid;
use yew::prelude::*;
use yewdux::prelude::*;

use shared::schema::FileDirectory;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: Uuid,
}

#[function_component(FileData)]
pub fn file_data(props: &Props) -> Html {
    let dispatch = Dispatch::<FileDirectory>::new();
    html! {
        <>
        { &dispatch.get().files.vertices.get(&props.id).unwrap().data }
        </>
    }
}
