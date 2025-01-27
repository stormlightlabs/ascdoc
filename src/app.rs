use icondata::{AiFolderOpenOutlined, SiAsciidoctor};
use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use leptos_icons::Icon;
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
            <aside class="flex flex-col gap-y-1">
                <section class="flex items-center gap-x-1">
                    <Icon icon={AiFolderOpenOutlined} {..} class="text-xl" />
                    <span class="font-medium">Root</span>
                </section>
                <section class="ml-6 space-y-1">
                    <div class="flex items-center gap-x-1">
                        <Icon icon={AiFolderOpenOutlined} {..} class="text-xl" />
                        <span class="col-auto">Folder 1</span>
                    </div>
                    <div class="ml-6 flex items-center gap-x-1">
                        <Icon icon={SiAsciidoctor} {..} class="text-xs" />
                        <span>File 1</span>
                    </div>
                    <div class="ml-6 flex items-center gap-x-1">
                        <Icon icon={SiAsciidoctor} {..} class="text-xs" />
                        <span>File 2</span>
                    </div>
                </section>
                <section class="ml-6 space-y-1">
                    <div class="flex items-center gap-x-1">
                        <Icon icon={AiFolderOpenOutlined} {..} class="text-xl" />
                        <span class="col-auto">Folder 2</span>
                    </div>
                </section>
                <section class="ml-6 space-y-1">
                    <div class="flex items-center gap-x-1">
                        <Icon icon={AiFolderOpenOutlined} {..} class="text-xl" />
                        <span class="col-auto">Folder 3</span>
                    </div>
                </section>
                <section class="ml-6 space-y-1">
                    <div class="flex items-center gap-x-1">
                        <Icon icon={AiFolderOpenOutlined} {..} class="text-xl" />
                        <span class="col-auto">Folder 4</span>
                    </div>
                </section>
            </aside>
            <section>
                <header class="p-2">
                    Bufferline
                </header>
                <article>
                    <div data-line-numbers>
                        <span>1</span>
                        <span>2</span>
                        <span>3</span>
                        <span>4</span>
                        <span>5</span>
                        <span>6</span>
                        <span>7</span>
                        <span>8</span>
                        <span>9</span>
                        <span>10</span>
                    </div>
                    <div data-buffer>
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
                    </div>
                </article>
                <footer class="p-2">
                    Statusline
                </footer>
            </section>
        </main>
    }
}
