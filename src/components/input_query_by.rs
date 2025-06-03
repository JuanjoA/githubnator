use leptos::*;
use leptos::ev;
use leptos::prelude::*;
use leptos_dom::helpers::event_target_value;

#[component]
pub fn InputQueryBy(
    value_get: ReadSignal<String>,
    value_set: WriteSignal<String>,
    placeholder: String,
    class: String,
) -> impl IntoView {
    view! {
        <div class="field has-text-centered px-2 py-2">
            <p class="control has-icons-right">
                <input
                    type="text"
                    class="input is-primary"
                    placeholder=placeholder
                    on:input=move |ev| {
                        value_set.set(event_target_value(&ev));
                    }
                    on:keydown=move |event: ev::KeyboardEvent| {
                        if event.key() == "Enter" && !event_target_value(&event).is_empty() {
                            // Trigger action based on url_to_go
                        }
                    }
                    prop:value=move || value_get.get()
                />
                <span class="icon is-small is-right">
                    <i class=class></i>
                </span>
            </p>
        </div>
    }
}