use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (x, set_x) = create_signal(0);
    
    view! {
        <button
            on:click = move |_| {
                // on nightly this is set_count(3);
                set_count.update(|n| *n += 1);
            }
            class:red=move || count.get() % 2 == 1
            // class=("button-20", move || count.get()%2 == 1)
        >
            "Click me: "
            // on nightly this is {move || count()};
            {move || count}
        </button>

        <button
            on:click={move |_| {
                set_x.update(|n| *n += 10);
            }}
            // set the `style` attribute
            style="position: absolute"
            // and toggle individual CSS properties with `style:`
            style:left=move || format!("{}px", x.get() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", x.get(), 100)
            style:max-width="400px"
            // Set a CSS variable for stylesheet use
            style=("--columns", x)
        >
            "Click to Move"
        </button>
        <progress max="50"
        // signals are functions, so `value=count` and `value=move || count.get()`
        // are interchangeable.
        value=count
        />
}
}