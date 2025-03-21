use leptos::prelude::*;

/// Shows progress toward a goal.
#[component]
fn ProgressBar(
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// How much progress should be displayed.
    #[prop(into)]
    progress: Signal<i32>
) -> impl IntoView
{
    view! {
        <progress max=max value=progress />
        <br />
    }
}

#[component]
fn App() -> impl IntoView {
    let values = vec![0, 1, 2];
view! {
    // this will just render "012"
    <p>{values.clone()}</p>
    // or we can wrap them in <li>
    <ul>{values.into_iter().map(|n| view! { <li>{n}</li> }).collect_view()}</ul>
}
}

// This `main` function is the entry point into the app
// It just mounts our component to the <body>
// Because we defined it as `fn App`, we can now use it in a
// template as <App/>
fn main() {
    leptos::mount::mount_to_body(App)
}
