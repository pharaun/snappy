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

#[function_component(App)]
pub fn app() -> Html {
    let snaps = vec![
        Snapshot {
            name: "minecraft".to_string(),
            from_dir: "c://foobar/minecraft".to_string(),
            to_dir: "/nas//asdf".to_string(),
        },
        Snapshot {
            name: "valheim".to_string(),
            from_dir: "c://foobar/valheim".to_string(),
            to_dir: "/nas//asdf".to_string(),
        },
    ];

    let snap_html = snaps.iter().map(|snap| html! {
        <div class="container is-fluid block">
            <div class="notification">
                <span class="icon-text">
                    <span><h1 class="title">{snap.name.clone()}</h1></span>
                    <p class="buttons ml-2">
                        <button class="button">
                            <span class="icon">
                                <span class="icon">
                                    <ion-icon name="create-outline" size="large"></ion-icon>
                                </span>
                            </span>
                        </button>
                    </p>
                </span>

                <label class="label">{"Snapshot from"}</label>
                <div class="file has-name is-right is-fullwidth">
                  <label class="file-label">
                    <input class="file-input" type="text" name="resume" />
                    <span class="file-cta">
                      <span class="file-icon"><ion-icon name="folder-outline"></ion-icon></span>
                      <span class="file-label">{"Choose a directory…"}</span>
                    </span>
                    <span class="file-name">{snap.from_dir.clone()}</span>
                  </label>
                </div>

                <label class="label">{"Snapshot to"}</label>
                <div class="file has-name is-right is-fullwidth">
                  <label class="file-label">
                    <input class="file-input" type="text" name="resume" />
                    <span class="file-cta">
                      <span class="file-icon"><ion-icon name="folder-outline"></ion-icon></span>
                      <span class="file-label">{"Choose a directory…"}</span>
                    </span>
                    <span class="file-name">{snap.to_dir.clone()}</span>
                  </label>
                </div>

            </div>
        </div>
    }).collect::<Html>();

    html! {
        <>
        <section class="hero is-small is-primary">
            <div class="hero-body">
                <p class="title">{"Snappy"}</p>
            </div>
        </section>
        <section class="section">
            {snap_html}
        </section>
        </>
    }
}
