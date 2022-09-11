use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


/*
#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

pub fn app() -> Html {
    let greet_input_ref = use_ref(|| NodeRef::default());

    let name = use_state(|| String::new());

    let greet_msg = use_state(|| String::new());
    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }

                    let new_msg = invoke(
                        "greet",
                        JsValue::from_serde(&GreetArgs { name: &*name }).unwrap(),
                    )
                    .await;
                    log(&new_msg.as_string().unwrap());
                    greet_msg.set(new_msg.as_string().unwrap());
                });

                || {}
            },
            name2,
        );
    }

    let greet = {
        let name = name.clone();
        let greet_input_ref = greet_input_ref.clone();
        Callback::from(move |_| {
            name.set(greet_input_ref.cast::<web_sys::HtmlInputElement>().unwrap().value());
        })
    };

    html! {
        <div class="row">
            <input id="greet-input" ref={&*greet_input_ref} placeholder="Enter a name..." />
            <button type="button" onclick={greet}>{"Greet"}</button>
        </div>

        <p><b>{ &*greet_msg }</b></p>
    }
}
*/


#[derive(Serialize, Deserialize)]
struct Snapshot {
    name: String,
    from_dir: String,
    to_dir: String,
}


fn dir_picker(label: String, path: String) -> Html {
    html! {
        <>
        <label class="label">{label}</label>
        <div class="file has-name is-right is-fullwidth">
          <label class="file-label">
            <input class="file-input" type="text" name="resume" />
            <span class="file-cta">
              <span class="file-icon"><ion-icon name="folder-outline"></ion-icon></span>
              <span class="file-label">{"Choose a directory…"}</span>
            </span>
            <span class="file-name">{path}</span>
          </label>
        </div>
        </>
    }
}


#[function_component(App)]
pub fn app() -> Html {
    let snaps = vec![
        Snapshot {
            name: "minecraft".to_string(),
            from_dir: "c://foobar/minecraft".to_string(),
            to_dir: "/nas//asdf".to_string(),
        },
    ];

    // Edit mode html i think
    let snap_edit = snaps.iter().map(|snap| html! {
        <div class="container is-fluid block">
            <div class="notification">
                <nav class="level">
                    <div class="level-left">
                        <div class="level-item">
                            <h1 class="title">{"NAME"}</h1>
                        </div>
                    </div>

                    <div class="level-right">
                        <p class="level-item"><a class="button">{"Edit"}</a></p>
                        <p class="level-item"><a class="button is-success">{"Run"}</a></p>
                    </div>
                </nav>

                <form class="box">
                    <div class="block">
                        {dir_picker(
                            "Name".to_string(),
                            snap.name.clone(),
                        )}

                        {dir_picker(
                            "Snapshot from".to_string(),
                            snap.from_dir.clone(),
                        )}

                        {dir_picker(
                            "Snapshot to".to_string(),
                            snap.to_dir.clone(),
                        )}
                    </div>

                    <div class="field is-grouped is-grouped-right">
                        <p class="control">
                            <button class="button is-link">{"Save"}</button>
                        </p>
                        <p class="control">
                            <button class="button">{"Cancel"}</button>
                        </p>
                        <p class="control">
                            <button class="button is-danger">{"Delete"}</button>
                        </p>
                    </div>
                </form>

            </div>
        </div>
    }).collect::<Html>();

    // Rest mode html
    let snap_rest = html! {
        <div class="container is-fluid block">
            <div class="notification">

                <nav class="level">
                    <div class="level-left">
                        <div class="level-item">
                            <h1 class="title">{"NAME"}</h1>
                        </div>
                    </div>

                    <div class="level-right">
                        <p class="level-item"><a class="button">{"Edit"}</a></p>
                        <p class="level-item"><a class="button is-success">{"Run"}</a></p>
                    </div>
                </nav>

            </div>
        </div>
    };

    // running mode html
    let snap_running = html! {
        <div class="container is-fluid block">
            <div class="notification">

                <nav class="level">
                    <div class="level-left">
                        <div class="level-item">
                            <h1 class="title">{"NAME"}</h1>
                        </div>
                    </div>

                    <div class="level-right">
                        <p class="level-item"><a class="button">{"Edit"}</a></p>
                        <p class="level-item"><a class="button is-success is-loading">{"Run"}</a></p>
                    </div>
                </nav>

                <form class="box">
                    <div class="block">
                        <div class="control">
                            <textarea class="textarea" readonly=true>{"Log linesmore logs"}</textarea>
                        </div>
                    </div>
                    <div class="field is-grouped is-grouped-right">
                        <p class="control">
                            <button class="button is-danger">{"Cancel"}</button>
                        </p>
                    </div>
                </form>
            </div>
        </div>
    };

    html! {
        <>
        <section class="hero is-small is-primary">
            <div class="hero-body">
                <p class="title">{"Snappy"}</p>
            </div>
        </section>
        <section class="section">
            {snap_rest}
            {snap_edit}
            {snap_running}
        </section>
        </>
    }
}
