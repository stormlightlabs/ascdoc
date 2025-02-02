use icondata::{AiFolderOpenOutlined, SiAsciidoctor};
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn Tree() -> impl IntoView {
    view! {
        <aside class="flex flex-col gap-y-1 bg-">
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
    }
}
