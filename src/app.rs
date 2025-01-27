use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    view! {
        <main class="container">
            <aside>
                File Explorer
                <ul>
                    <li>File 1</li>
                    <li>File 2</li>
                    <li>File 3</li>
                </ul>
            </aside>
            <section>
                <header>
                    Bufferline
                </header>
                <article>
                    <h2>Buffer</h2>
                    <form class="row" on:submit=greet>
                        <input
                            id="greet-input"
                            placeholder="Enter a name..."
                            on:input=update_name
                        />
                        <button type="submit">"Greet"</button>
                    </form>
                    <p>{ move || greet_msg.get() }</p>
                </article>
                <footer>
                    Statusline
                </footer>
            </section>
        </main>
    }
}
