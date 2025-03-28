use leptos::*;

#[component]
pub fn InputQueryBy(
    url_to_go: ReadSignal<String>,
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
                        value_set(event_target_value(&ev));
                    }
                    on:keydown=move |event| {
                        if event.key() == "Enter" && !event_target_value(&event).is_empty() {
                            url_to_go();
                        }
                    }
                    prop:value=move || value_get()
                />
                <span class="icon is-small is-right">
                    <i class=class></i>
                </span>
            </p>
        </div>
    }
}