pub mod backends {
    use wasm_bindgen_futures::spawn_local;
    use crate::utils::DeviceInfo;
    use crate::utils::FileDirectory;
    use crate::utils::FileNode;
    use crate::utils::FileTree;
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use serde_wasm_bindgen::from_value;
    use shared::{invoke_async, Tree};
    use uuid::Uuid;
    use yewdux::prelude::Dispatch;


    async fn call_postgres<T, U>(command: String, args: Option<&U>) -> Result<T, String>
        where
            T: DeserializeOwned,
            U: Serialize,
    {
        let x = invoke_async::<U>(command, args)
            .await
            .map_err(|e| format!("{:?}", e))?;
        from_value(x).map_err(|e| e.to_string())
    }

    enum FileLocation {
        LOCAL,
        CLOUD,
        SYNCED,
    }

    pub fn change_directory(parent_id: String, child_id: String)
    // -> Result<(), String>
    {
        // let mut response: Option<Result<(), String>> = None;
        spawn_local(async move {
            self::local_change_directory(parent_id, child_id).await;
        });
        // return response.unwrap();
    }

    async fn local_change_directory(parent_id: String, child_id: String) -> Result<(), String> {
        let info = Dispatch::<DeviceInfo>::new();
        let file_dispatch = Dispatch::<FileTree>::new();
        if info.get().web || info.get().online {
            unimplemented!();
        }
        if !info.get().web {
            return self::call_postgres(
                "change_directory".to_string(),
                Some(&serde_json::json!({ "childId": child_id , "parentId" : parent_id})),
            )
                .await
                .map(|e| {
                    file_dispatch.reduce_mut(|f| {
                        for i in f.files.adjacency.values_mut() {
                            i.remove(&Uuid::parse_str(&child_id).unwrap());
                        }
                        f.files.push_edge(
                            Uuid::parse_str(&parent_id).unwrap(),
                            Uuid::parse_str(&child_id).unwrap(),
                        );
                    });
                    e
                });
        }
        return Err("user is offline".to_string());
    }

    //  async fn cloud_change_directory(parent_id: String, child_id: String) -> Result<(), String> {
    //     ....
    // }

    //  async fn synced_change_directory(parent_id: String, child_id: String) -> Result<(), String> {
    //     backends::local_change_directory(parent_id,child_id);
    //     backends::local_change_directory(parent_id,child_id);
    // }

    //
    //
    // fn add_file() {
    //     let mut new_file: Option<None> = Some(None);
    //
    //     if IS_WEB {
    //         params = [("method", "add_file"), ("name", "untitled")];
    //         let res = reqwest::post("host.io?canister_id=aaa-aa").from(params);
    //         new_file = res.file
    //     } else {
    //         let file = crate::backend::files::create_file(
    //             state.files.id,
    //             state.files.root.unwrap(),
    //             "untitled".to_string(),
    //         )
    //             .await;
    //         new_file = file
    //     }
    //
    //     if let Ok(f) = file {
    //         state.files.push_children(state.files.root.unwrap(), f.id, f);
    //     }
    //
    // }
    //
    // fn get_files() -> Vec<FileTree> {
    //     let files = [];
    //     if IS_LOGEDIN {
    //         params = [("method", "files")];
    //         let res = reqwest::post("host.io?canister_id=aaa-aa").from(params);
    //         files.extends(res.files)
    //     }
    //     if IS_WEB == false {
    //         files.extends(get_directory());
    //     }
    //     return files;
    // }
    //
    // fn delete_file(id: Uuid, file_location: FileLocation) {
    //     if file_location == FileLocation.LOCAL {
    //         tauri::commands: delete_file(id);
    //     }
    //     if file_location == FileLocation.CLOUD {
    //         params = [("method", "delete_file"), ("id", id)];
    //         let res = reqwest::post("host.io?canister_id=aaa-aa").from(params);
    //     }
    //
    //     if file_location == FileLocation.SYNCED {
    //         tauri::commands: delete_file(id);
    //         params = [("method", "delete_file"), ("id", id)];
    //         let res = reqwest::post("host.io?canister_id=aaa-aa").from(params);
    //     }
    // }
}