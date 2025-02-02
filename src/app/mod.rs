use leptos::prelude::*;

mod buffer;
mod explorer;
mod lines;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="bg-dark-base">
            <explorer::Tree />
            <section>
                <lines::Buffer />
                <article class="bg-dark-mantle">
                    <lines::Numbers />
                    <buffer::View />
                </article>
                <lines::Status />
            </section>
        </main>
    }
}
