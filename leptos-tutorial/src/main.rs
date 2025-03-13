use leptos::prelude::*;

// The #[component] macro marks a function as a reusable component
// Components are the building blocks of your user interface
// They define a reusable unit of behavior
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button
            on:click=move |_| {
                *set_count.write() += 10;
            }
            // set the `style` attribute
            style="position: absolute"
            // and toggle individual CSS properties with `style:`
            style:left=move || format!("{}px", count.get() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", count.get(), 100)
            style:max-width="400px"
            // Set a CSS variable for stylesheet use
            style=("--columns", move || count.get().to_string())
        >
            "Click to Move"
        </button>
    }
}

// This `main` function is the entry point into the app
// It just mounts our component to the <body>
// Because we defined it as `fn App`, we can now use it in a
// template as <App/>
fn main() {
    leptos::mount::mount_to_body(App)
}
