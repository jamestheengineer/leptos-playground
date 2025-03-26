use leptos::prelude::*;


#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = signal("Controlled".to_string());
    let email = RwSignal::new("".to_string());
    let spam_me = RwSignal::new(true);
    
    view! {
        <input type="text"
            bind:value=(name, set_name)
        />
        <input type="email"
            bind:value=email
        />
        <label>
            "Please send me lots of spam email."
            <input type="checkbox"
                bind:checked=spam_me
            />
        </label>
        <p>"Name is: " {name}</p>
        <p>"Email is: " {email}</p>
        <Show when=move || spam_me.get()>
            <p>"You’ll receive cool bonus content!"</p>
        </Show>
    }
}
fn main() {
    leptos::mount::mount_to_body(App)
}
