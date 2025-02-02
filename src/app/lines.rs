use leptos::prelude::*;

#[component]
pub fn Numbers() -> impl IntoView {
    view! {
        <div data-line-numbers class="bg-dark-surface0">
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
    }
}

#[component]
pub fn Buffer() -> impl IntoView {
    view! {
        <header class="p-2 bg-surface-1">
            Bufferline
        </header>
    }
}

#[component]
pub fn Status() -> impl IntoView {
    view! {
        <footer class="p-2 bg-surface2">
            Statusline
        </footer>
    }
}
