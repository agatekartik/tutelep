use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click = move |_| {
                // on nightly this is set_count(3);
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: "
            // on nightly this is {move || count()};
            {move || count.get()}
        </button>
    }
}